use serde::{Deserialize, Serialize};
use serde_json::json;
use tench_job_core::{JobDescriptor, JobState};

use crate::{
    AnnotationId, AttachmentId, ColorRgba, LibraryLayout, PageRect, PdfAnnotation,
    PdfAnnotationKind, Timestamp,
};

crate::research_id_type!(PdfSearchResultId);

mod extract;
mod render;
mod search;

pub use extract::{extract_pdf_literal_text, inspect_pdf_document_bytes};
pub use render::{
    build_pdf_thumbnail_strip, pdf_page_cache_window, pdf_render_job_descriptor, render_pdf_page,
    render_pdf_page_from_bytes,
};
pub use search::{
    copy_pdf_selection, copy_pdf_selection_text, search_pdf_text, search_pdf_text_with_limit,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfReaderState {
    pub attachment_id: AttachmentId,
    pub current_page: u32,
    pub zoom: f32,
    #[serde(default)]
    pub pan_x: f32,
    #[serde(default)]
    pub pan_y: f32,
    pub mode: PdfReaderMode,
    pub theme: PdfReaderTheme,
    #[serde(default)]
    pub search: Option<PdfSearchState>,
    #[serde(default)]
    pub selected_text: Option<PdfTextSelection>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfDocumentText {
    pub attachment_id: AttachmentId,
    #[serde(default)]
    pub pages: Vec<PdfPageText>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfPageText {
    pub page: u32,
    pub text: String,
    #[serde(default)]
    pub locale: Option<crate::ResearchLocale>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PdfDocumentMetadata {
    pub attachment_id: AttachmentId,
    pub status: PdfDocumentStatus,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub page_count: Option<u32>,
    #[serde(default)]
    pub pdf_version: Option<String>,
    pub text_extractable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PdfDocumentStatus {
    Ready,
    Encrypted,
    Corrupt,
    Unsupported,
    ImageOnly,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PdfOpenResult {
    pub metadata: PdfDocumentMetadata,
    #[serde(default)]
    pub warning: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum PdfReaderAction {
    SetPage { page: u32 },
    NextPage,
    PreviousPage,
    SetZoom { zoom: f32 },
    ZoomBy { delta: f32 },
    Pan { dx: f32, dy: f32 },
    SetMode { mode: PdfReaderMode },
    SetTheme { theme: PdfReaderTheme },
    Search { query: String },
    SelectText { selection: Option<PdfTextSelection> },
    ClearSelection,
    ClearSearch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PdfReaderMode {
    SinglePage,
    Continuous,
    FitWidth,
    FitPage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PdfReaderTheme {
    Paper,
    Dark,
    Sepia,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfSearchState {
    pub query: String,
    pub results: Vec<PdfSearchResult>,
    pub active_result: Option<PdfSearchResultId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfSearchResult {
    pub id: PdfSearchResultId,
    pub page: u32,
    pub rects: Vec<PageRect>,
    pub snippet: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfTextSelection {
    pub page: u32,
    pub rects: Vec<PageRect>,
    pub text: String,
    #[serde(default)]
    pub locale: Option<crate::ResearchLocale>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PdfCacheKey {
    pub attachment_id: AttachmentId,
    pub attachment_hash: String,
    pub page: Option<u32>,
    pub annotation_updated_at: Option<Timestamp>,
    pub kind: PdfCacheKind,
}

impl PdfCacheKey {
    pub fn stable_file_name(&self) -> String {
        let page = self
            .page
            .map(|page| page.to_string())
            .unwrap_or_else(|| "document".to_string());
        let annotation = self
            .annotation_updated_at
            .as_ref()
            .map(|timestamp| stable_cache_token(&timestamp.0))
            .unwrap_or_else(|| "base".to_string());
        format!(
            "{}-{}-{}-{}.cache",
            stable_cache_token(self.attachment_id.as_str()),
            stable_cache_token(&self.attachment_hash),
            page,
            annotation
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PdfCacheKind {
    RenderedPageBitmap,
    Thumbnail,
    ExtractedText,
    AnnotationOverlay,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfRenderRequest {
    pub attachment_id: AttachmentId,
    pub attachment_hash: String,
    pub page: u32,
    pub zoom: f32,
    pub max_dimension_px: u32,
    #[serde(default)]
    pub theme: Option<PdfReaderTheme>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderedPdfPage {
    pub attachment_id: AttachmentId,
    pub page: u32,
    pub width_px: u32,
    pub height_px: u32,
    pub pixel_format: PdfPixelFormat,
    pub pixels: Vec<u8>,
    pub cache_key: PdfCacheKey,
    pub render_quality: PdfRenderQuality,
    pub accessibility_summary: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfThumbnailRequest {
    pub attachment_id: AttachmentId,
    pub attachment_hash: String,
    pub current_page: u32,
    pub page_count: u32,
    pub radius: u32,
    pub max_dimension_px: u32,
    #[serde(default)]
    pub theme: Option<PdfReaderTheme>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfThumbnail {
    pub page: u32,
    pub width_px: u32,
    pub height_px: u32,
    pub pixel_format: PdfPixelFormat,
    pub pixels: Vec<u8>,
    pub cache_key: PdfCacheKey,
    pub selected: bool,
    pub accessibility_summary: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfThumbnailStrip {
    pub attachment_id: AttachmentId,
    pub current_page: u32,
    pub page_count: u32,
    #[serde(default)]
    pub pages: Vec<u32>,
    #[serde(default)]
    pub thumbnails: Vec<PdfThumbnail>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PdfRenderQuality {
    NativeBitmap,
    TextPreview,
    DocumentShell,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PdfPixelFormat {
    Rgba8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfAnnotationExportItem {
    pub id: AnnotationId,
    pub kind: PdfAnnotationKind,
    pub page: u32,
    pub color: ColorRgba,
    pub selected_text: Option<String>,
    pub note_markdown: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct PdfCacheManifest {
    #[serde(default)]
    pub entries: Vec<PdfCacheEntry>,
}

impl PdfCacheManifest {
    pub fn upsert(&mut self, entry: PdfCacheEntry) {
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|existing| existing.key == entry.key)
        {
            *existing = entry;
        } else {
            self.entries.push(entry);
        }
    }

    pub fn invalidate_for_attachment(&mut self, attachment_id: &AttachmentId) -> usize {
        let before = self.entries.len();
        self.entries
            .retain(|entry| &entry.key.attachment_id != attachment_id);
        before - self.entries.len()
    }

    pub fn invalidate_annotation_overlays(&mut self, attachment_id: &AttachmentId) -> usize {
        let before = self.entries.len();
        self.entries.retain(|entry| {
            &entry.key.attachment_id != attachment_id
                || entry.key.kind != PdfCacheKind::AnnotationOverlay
        });
        before - self.entries.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PdfCacheEntry {
    pub key: PdfCacheKey,
    pub relative_path: String,
    pub bytes: u64,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfOverlayDrawPlan {
    pub attachment_id: AttachmentId,
    pub page: u32,
    #[serde(default)]
    pub commands: Vec<PdfOverlayCommand>,
    pub accessibility_summary: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum PdfOverlayCommand {
    Highlight {
        annotation_id: AnnotationId,
        rect: PageRect,
        color: ColorRgba,
        selected: bool,
    },
    Underline {
        annotation_id: AnnotationId,
        rect: PageRect,
        color: ColorRgba,
    },
    Strikeout {
        annotation_id: AnnotationId,
        rect: PageRect,
        color: ColorRgba,
    },
    StickyNote {
        annotation_id: AnnotationId,
        x: f32,
        y: f32,
        label: String,
    },
    Bookmark {
        annotation_id: AnnotationId,
        label: String,
    },
    SearchResult {
        result_id: PdfSearchResultId,
        rect: PageRect,
        active: bool,
    },
    TextSelection {
        rect: PageRect,
    },
}

pub fn apply_pdf_reader_action(
    mut state: PdfReaderState,
    action: PdfReaderAction,
    page_count: u32,
    document_text: Option<&PdfDocumentText>,
) -> PdfReaderState {
    let max_page = page_count.max(1);
    match action {
        PdfReaderAction::SetPage { page } => {
            state.current_page = page.clamp(1, max_page);
        }
        PdfReaderAction::NextPage => {
            state.current_page = (state.current_page + 1).min(max_page);
        }
        PdfReaderAction::PreviousPage => {
            state.current_page = state.current_page.saturating_sub(1).max(1);
        }
        PdfReaderAction::SetZoom { zoom } => {
            state.zoom = zoom.clamp(0.1, 8.0);
        }
        PdfReaderAction::ZoomBy { delta } => {
            state.zoom = (state.zoom + delta).clamp(0.1, 8.0);
        }
        PdfReaderAction::Pan { dx, dy } => {
            state.pan_x += dx;
            state.pan_y += dy;
        }
        PdfReaderAction::SetMode { mode } => {
            state.mode = mode;
        }
        PdfReaderAction::SetTheme { theme } => {
            state.theme = theme;
        }
        PdfReaderAction::Search { query } => {
            state.search = document_text.map(|text| search_pdf_text(text, &query));
        }
        PdfReaderAction::SelectText { selection } => {
            state.selected_text = selection;
        }
        PdfReaderAction::ClearSelection => {
            state.selected_text = None;
        }
        PdfReaderAction::ClearSearch => {
            state.search = None;
        }
    }
    state
}

pub fn build_pdf_annotation_overlay_plan(
    attachment_id: AttachmentId,
    page: u32,
    annotations: &[PdfAnnotation],
    selected_annotation_id: Option<&AnnotationId>,
) -> PdfOverlayDrawPlan {
    let mut commands = Vec::new();
    for annotation in annotations
        .iter()
        .filter(|annotation| annotation.attachment_id == attachment_id && annotation.page == page)
    {
        match annotation.kind {
            PdfAnnotationKind::Highlight => {
                for rect in &annotation.rects {
                    commands.push(PdfOverlayCommand::Highlight {
                        annotation_id: annotation.id.clone(),
                        rect: *rect,
                        color: annotation.color,
                        selected: selected_annotation_id == Some(&annotation.id),
                    });
                }
            }
            PdfAnnotationKind::Underline => {
                for rect in &annotation.rects {
                    commands.push(PdfOverlayCommand::Underline {
                        annotation_id: annotation.id.clone(),
                        rect: *rect,
                        color: annotation.color,
                    });
                }
            }
            PdfAnnotationKind::Strikeout => {
                for rect in &annotation.rects {
                    commands.push(PdfOverlayCommand::Strikeout {
                        annotation_id: annotation.id.clone(),
                        rect: *rect,
                        color: annotation.color,
                    });
                }
            }
            PdfAnnotationKind::Note | PdfAnnotationKind::Drawing => {
                let anchor = annotation.rects.first().copied().unwrap_or(PageRect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.0,
                    height: 0.0,
                });
                commands.push(PdfOverlayCommand::StickyNote {
                    annotation_id: annotation.id.clone(),
                    x: anchor.x,
                    y: anchor.y,
                    label: annotation
                        .note_markdown
                        .as_deref()
                        .unwrap_or("Note")
                        .chars()
                        .take(80)
                        .collect(),
                });
            }
            PdfAnnotationKind::Bookmark => {
                commands.push(PdfOverlayCommand::Bookmark {
                    annotation_id: annotation.id.clone(),
                    label: annotation
                        .selected_text
                        .as_deref()
                        .unwrap_or("Bookmark")
                        .chars()
                        .take(80)
                        .collect(),
                });
            }
        }
    }
    PdfOverlayDrawPlan {
        attachment_id,
        page,
        accessibility_summary: format!("{} annotations on page {}", commands.len(), page),
        commands,
    }
}

pub fn build_pdf_reader_overlay_plan(
    state: &PdfReaderState,
    annotations: &[PdfAnnotation],
    selected_annotation_id: Option<&AnnotationId>,
) -> PdfOverlayDrawPlan {
    let mut plan = build_pdf_annotation_overlay_plan(
        state.attachment_id.clone(),
        state.current_page,
        annotations,
        selected_annotation_id,
    );

    if let Some(search) = &state.search {
        for result in search
            .results
            .iter()
            .filter(|result| result.page == state.current_page)
        {
            for rect in &result.rects {
                plan.commands.push(PdfOverlayCommand::SearchResult {
                    result_id: result.id.clone(),
                    rect: *rect,
                    active: search.active_result.as_ref() == Some(&result.id),
                });
            }
        }
    }

    if let Some(selection) = &state.selected_text {
        if selection.page == state.current_page {
            for rect in &selection.rects {
                plan.commands
                    .push(PdfOverlayCommand::TextSelection { rect: *rect });
            }
        }
    }

    plan.accessibility_summary = format!(
        "{} overlay commands on page {}",
        plan.commands.len(),
        state.current_page
    );
    plan
}

pub fn pdf_cache_path(layout: &LibraryLayout, key: &PdfCacheKey) -> std::path::PathBuf {
    let area = match key.kind {
        PdfCacheKind::RenderedPageBitmap => &layout.index_dir,
        PdfCacheKind::Thumbnail => &layout.thumbnails_dir,
        PdfCacheKind::ExtractedText => &layout.index_dir,
        PdfCacheKind::AnnotationOverlay => &layout.index_dir,
    };
    area.join(key.stable_file_name())
}

pub fn export_annotations_markdown(items: &[PdfAnnotationExportItem]) -> String {
    items
        .iter()
        .map(|item| {
            let mut block = format!("- page {} {:?}\n", item.page, item.kind);
            if let Some(text) = &item.selected_text {
                block.push_str(&format!("  > {}\n", text.replace('\n', " ")));
            }
            if let Some(note) = &item.note_markdown {
                block.push_str(&format!("  {}\n", note.trim()));
            }
            block
        })
        .collect::<Vec<_>>()
        .join("")
}

fn stable_cache_token(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .take(48)
        .collect::<String>()
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests;
