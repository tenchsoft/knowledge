use std::collections::BTreeSet;

use tench_research_core::ReadingStatus;
use tench_ui::prelude::{Color, Rect};

use super::builders::all_research_ui_commands;
use super::status_label;

#[derive(Debug, Clone)]
pub struct Paper {
    pub title: String,
    pub authors: String,
    pub venue: String,
    pub file_name: String,
    pub pages: u32,
    pub abstract_text: String,
    pub year: u32,
    pub tags: Vec<String>,
    pub status: ReadingStatus,
    pub favorite: bool,
    pub notes: String,
    pub references: Vec<String>,
    pub collection_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub count: usize,
    pub expanded: bool,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FocusTarget {
    None,
    SearchBox,
    QaInput,
    PdfSearch,
    DoiInput,
    ManuscriptCiteSearch,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SortMode {
    TitleAsc,
    TitleDesc,
    YearAsc,
    YearDesc,
    AuthorAsc,
    AuthorDesc,
    DateAddedAsc,
    DateAddedDesc,
}

impl SortMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::TitleAsc => "Title (A-Z)",
            Self::TitleDesc => "Title (Z-A)",
            Self::YearAsc => "Year (old)",
            Self::YearDesc => "Year (new)",
            Self::AuthorAsc => "Author (A-Z)",
            Self::AuthorDesc => "Author (Z-A)",
            Self::DateAddedAsc => "Added (old)",
            Self::DateAddedDesc => "Added (new)",
        }
    }
}

pub struct ResearchState {
    pub papers: Vec<Paper>,
    pub collections: Vec<Collection>,
    pub tags: Vec<String>,
    pub statuses: Vec<ResearchStatusFilter>,
    pub selected_paper: usize,
    pub search_query: String,
    pub active_inspector_tab: usize,
    pub reader_mode: ReaderMode,
    pub citation_format: CitationFormat,
    pub favorites_only: bool,
    pub import_status: ImportStatus,
    pub analysis_messages: Vec<(String, String)>,
    pub visual_summary_lines: Vec<String>,
    pub visual_draw_plan: Option<tench_research_core::ResearchVisualDrawPlan>,
    pub manuscript_summary_lines: Vec<String>,
    pub writing_visual_draw_plan: Option<tench_research_core::ResearchVisualDrawPlan>,
    pub selected_papers: BTreeSet<usize>,
    pub context_menu: Option<ResearchContextMenu>,
    pub command_palette: ResearchCommandPaletteState,
    pub dropped_import_paths: Vec<String>,
    pub progress_history: Vec<ResearchProgressEvent>,
    pub duplicate_merge: Option<ResearchDuplicateMergeUi>,
    pub missing_attachment_repair: ResearchAttachmentRepairUi,
    pub backup_restore: ResearchBackupRestoreUi,
    pub i18n_missing_keys: Vec<String>,

    // Phase 1: focus and collection selection
    pub focus: FocusTarget,
    pub selected_collection: Option<String>,
    pub sort_mode: SortMode,

    // Phase 5: Q&A input
    pub qa_input: String,

    // Phase 7: advanced search
    pub advanced_search: Option<AdvancedSearchState>,
    pub show_advanced_search: bool,
    pub saved_searches: Vec<SavedSearch>,

    // Phase 10: toast
    pub toasts: Vec<ToastMessage>,

    // Phase 10: shortcut help modal
    pub show_shortcut_help: bool,

    // Phase 3: PDF reader state
    pub pdf_current_page: u32,
    pub pdf_page_count: u32,
    pub pdf_zoom: f64,
    pub pdf_rotation: f64,
    pub pdf_continuous_scroll: bool,

    // Phase 3: PDF search state
    pub pdf_search_query: String,
    pub pdf_search_results: Vec<PdfSearchResult>,
    pub pdf_search_active_index: Option<usize>,

    // Phase 3: PDF rendered page image data (set by app shell from Tauri commands)
    pub pdf_page_image_data: Option<PdfPageImageData>,

    // Phase 4: PDF annotations
    pub pdf_annotations: Vec<PdfAnnotationState>,
    pub pdf_selected_annotation: Option<String>,
    pub pdf_annotation_tool: PdfAnnotationTool,

    // Phase 4: annotation list sidebar
    pub pdf_show_annotation_list: bool,

    // Phase 6: DOI/fetch
    pub doi_input: String,
    pub citation_export_format: CitationExportFormat,

    // Phase 9: manuscript
    pub manuscript_sections: Vec<ManuscriptSection>,
    pub manuscript_cite_search: String,
    pub manuscript_active_section: Option<usize>,
    pub manuscript_show_cite_search: bool,

    // Phase 7: smart collections
    pub smart_collections: Vec<SmartCollection>,

    // Phase 10: welcome
    pub show_welcome: bool,
}

#[derive(Debug, Clone)]
pub struct PdfPageImageData {
    pub width_px: u32,
    pub height_px: u32,
    pub pixels_rgba: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PdfSearchResult {
    pub page: u32,
    pub rect: Rect,
    pub text: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResearchStatusFilter {
    All,
    Status(ReadingStatus),
}

impl ResearchStatusFilter {
    pub fn label(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Status(status) => status_label(status),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ReaderMode {
    Detail,
    Pdf,
    Importing,
}

impl ReaderMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Detail => "detail",
            Self::Pdf => "pdf",
            Self::Importing => "importing",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CitationFormat {
    BibTex,
    Apa,
    Mla,
}

impl CitationFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::BibTex => "BibTeX",
            Self::Apa => "APA",
            Self::Mla => "MLA",
        }
    }
}

impl std::fmt::Display for CitationFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.label())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImportStatus {
    Ready,
    Queued,
}

impl ImportStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Queued => "Import queued",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResearchContextMenu {
    pub target: ResearchContextTarget,
    pub commands: Vec<ResearchUiCommand>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResearchContextTarget {
    Paper { index: usize },
    PaperList,
    Collection { name: String },
    Tag { label: String },
    Attachment { path: String },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResearchUiCommand {
    OpenSelected,
    ToggleFavorite,
    CopyCitation,
    ExportSelected,
    ImportDroppedFiles,
    BatchSetStatusReviewed,
    BatchAddTag,
    MergeDuplicates,
    RepairMissingAttachment,
    CreateBackup,
    RestoreBackup,
    ShowI18nCoverage,
}

impl ResearchUiCommand {
    pub fn label(self) -> &'static str {
        match self {
            Self::OpenSelected => "open_selected",
            Self::ToggleFavorite => "toggle_favorite",
            Self::CopyCitation => "copy_citation",
            Self::ExportSelected => "export_selected",
            Self::ImportDroppedFiles => "import_dropped_files",
            Self::BatchSetStatusReviewed => "batch_set_status_reviewed",
            Self::BatchAddTag => "batch_add_tag",
            Self::MergeDuplicates => "merge_duplicates",
            Self::RepairMissingAttachment => "repair_missing_attachment",
            Self::CreateBackup => "create_backup",
            Self::RestoreBackup => "restore_backup",
            Self::ShowI18nCoverage => "show_i18n_coverage",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResearchCommandPaletteState {
    pub open: bool,
    pub query: String,
    pub commands: Vec<ResearchUiCommand>,
}

impl Default for ResearchCommandPaletteState {
    fn default() -> Self {
        Self {
            open: false,
            query: String::new(),
            commands: all_research_ui_commands(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResearchProgressEvent {
    pub id: String,
    pub kind: ResearchProgressKind,
    pub label: String,
    pub status: ResearchProgressStatus,
    pub completed: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResearchProgressKind {
    Import,
    Export,
    Backup,
    Restore,
    Repair,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResearchProgressStatus {
    Queued,
    Running,
    Complete,
    Failed,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResearchDuplicateMergeUi {
    pub candidate_count: usize,
    pub selected_pair: Option<(String, String)>,
    pub last_action: Option<String>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ResearchAttachmentRepairUi {
    pub missing_count: usize,
    pub unresolved_count: usize,
    pub last_scan_label: Option<String>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ResearchBackupRestoreUi {
    pub last_backup_path: Option<String>,
    pub last_restore_label: Option<String>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct AdvancedSearchState {
    pub title_query: String,
    pub author_query: String,
    pub year_from: Option<u32>,
    pub year_to: Option<u32>,
    pub venue_query: String,
    pub tag_query: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ToastMessage {
    pub id: String,
    pub message: String,
    pub kind: ToastKind,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PdfAnnotationTool {
    None,
    Highlight,
    Underline,
    Strikeout,
    StickyNote,
    TextSelect,
}

#[derive(Debug, Clone)]
pub struct PdfAnnotationState {
    pub id: String,
    pub page: u32,
    pub rect: Rect,
    pub kind: PdfAnnotationTool,
    pub color: Color,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CitationExportFormat {
    BibTex,
    Ris,
    Apa,
    Chicago,
    Mla,
}

impl CitationExportFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::BibTex => "BibTeX",
            Self::Ris => "RIS",
            Self::Apa => "APA",
            Self::Chicago => "Chicago",
            Self::Mla => "MLA",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManuscriptSection {
    pub id: String,
    pub title: String,
    pub content: String,
    pub citations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SmartCollection {
    pub id: String,
    pub name: String,
    pub rule: SmartCollectionRule,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub enum SmartCollectionRule {
    /// Papers added in the last N days
    RecentlyAdded { days: u32 },
    /// Papers with a specific tag
    Tagged { tag: String },
    /// Papers not yet read
    Unread,
    /// Papers marked as favorite
    Favorites,
    /// Papers by a specific author
    ByAuthor { author: String },
    /// Papers from a specific year range
    YearRange { from: u32, to: u32 },
}

#[derive(Debug, Clone)]
pub struct SavedSearch {
    pub id: String,
    pub name: String,
    pub query: String,
    pub advanced: Option<AdvancedSearchState>,
}
