use tench_research_core::*;

#[tauri::command]
pub fn export_research_pdf_annotations_markdown(items: Vec<PdfAnnotationExportItem>) -> String {
    tench_research_core::export_annotations_markdown(&items)
}

#[tauri::command]
pub fn research_pdf_cache_path(root_dir: String, key: PdfCacheKey) -> String {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::pdf_cache_path(&layout, &key)
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
pub fn invalidate_research_pdf_annotation_cache(
    mut manifest: PdfCacheManifest,
    attachment_id: tench_research_core::AttachmentId,
) -> PdfCacheManifest {
    manifest.invalidate_annotation_overlays(&attachment_id);
    manifest
}

#[tauri::command]
pub fn inspect_research_pdf_document_bytes(
    attachment_id: tench_research_core::AttachmentId,
    bytes: Vec<u8>,
) -> tench_research_core::PdfOpenResult {
    tench_research_core::inspect_pdf_document_bytes(attachment_id, &bytes)
}

#[tauri::command]
pub fn extract_research_pdf_literal_text(
    attachment_id: tench_research_core::AttachmentId,
    bytes: Vec<u8>,
    locale: Option<tench_research_core::ResearchLocale>,
) -> Result<tench_research_core::PdfDocumentText, String> {
    tench_research_core::extract_pdf_literal_text(attachment_id, &bytes, locale)
}

#[tauri::command]
pub fn render_research_pdf_page(
    request: tench_research_core::PdfRenderRequest,
    document_text: Option<tench_research_core::PdfDocumentText>,
) -> tench_research_core::RenderedPdfPage {
    tench_research_core::render_pdf_page(request, document_text.as_ref())
}

#[tauri::command]
pub fn render_research_pdf_page_from_bytes(
    request: tench_research_core::PdfRenderRequest,
    bytes: Vec<u8>,
    locale: Option<tench_research_core::ResearchLocale>,
) -> Result<tench_research_core::RenderedPdfPage, String> {
    tench_research_core::render_pdf_page_from_bytes(request, &bytes, locale)
}

#[tauri::command]
pub fn build_research_pdf_thumbnail_strip(
    request: tench_research_core::PdfThumbnailRequest,
    document_text: Option<tench_research_core::PdfDocumentText>,
) -> tench_research_core::PdfThumbnailStrip {
    tench_research_core::build_pdf_thumbnail_strip(request, document_text.as_ref())
}

#[tauri::command]
pub fn queue_research_pdf_render_job(
    request: tench_research_core::PdfRenderRequest,
    batch_id: String,
) -> tench_job_core::JobDescriptor {
    tench_research_core::pdf_render_job_descriptor(&request, batch_id)
}

#[tauri::command]
pub fn apply_research_pdf_reader_action(
    state: tench_research_core::PdfReaderState,
    action: tench_research_core::PdfReaderAction,
    page_count: u32,
    document_text: Option<tench_research_core::PdfDocumentText>,
) -> tench_research_core::PdfReaderState {
    tench_research_core::apply_pdf_reader_action(state, action, page_count, document_text.as_ref())
}

#[tauri::command]
pub fn search_research_pdf_text(
    document: tench_research_core::PdfDocumentText,
    query: String,
    limit: usize,
) -> tench_research_core::PdfSearchState {
    tench_research_core::search_pdf_text_with_limit(&document, &query, limit)
}

#[tauri::command]
pub fn copy_research_pdf_selection_text(
    state: tench_research_core::PdfReaderState,
) -> Option<String> {
    tench_research_core::copy_pdf_selection_text(&state)
}

#[tauri::command]
pub fn build_research_pdf_annotation_overlay_plan(
    attachment_id: tench_research_core::AttachmentId,
    page: u32,
    annotations: Vec<tench_research_core::PdfAnnotation>,
    selected_annotation_id: Option<tench_research_core::AnnotationId>,
) -> tench_research_core::PdfOverlayDrawPlan {
    tench_research_core::build_pdf_annotation_overlay_plan(
        attachment_id,
        page,
        &annotations,
        selected_annotation_id.as_ref(),
    )
}

#[tauri::command]
pub fn build_research_pdf_reader_overlay_plan(
    state: tench_research_core::PdfReaderState,
    annotations: Vec<tench_research_core::PdfAnnotation>,
    selected_annotation_id: Option<tench_research_core::AnnotationId>,
) -> tench_research_core::PdfOverlayDrawPlan {
    tench_research_core::build_pdf_reader_overlay_plan(
        &state,
        &annotations,
        selected_annotation_id.as_ref(),
    )
}
