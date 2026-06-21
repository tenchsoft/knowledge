use serde::{Deserialize, Serialize};
use tench_document_core::TenchDocument;

use crate::{
    AttachmentId, LocalizedField, PdfAnnotation, ReferenceId, ResearchLocale, ResearchNoteId,
    Timestamp,
};

crate::research_id_type!(ManuscriptId);
crate::research_id_type!(ManuscriptAuthorId);
crate::research_id_type!(TemplateId);
crate::research_id_type!(SectionId);
crate::research_id_type!(ManuscriptAssetId);
crate::research_id_type!(CitationId);
crate::research_id_type!(SnapshotId);
crate::research_id_type!(ManuscriptCrossReferenceId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchManuscript {
    pub id: ManuscriptId,
    pub library_id: crate::LibraryId,
    pub title: LocalizedField,
    #[serde(default)]
    pub subtitle: Option<LocalizedField>,
    #[serde(default)]
    pub authors: Vec<ManuscriptAuthor>,
    pub target: ManuscriptTarget,
    pub locale: ResearchLocale,
    #[serde(default)]
    pub template_id: Option<TemplateId>,
    pub outline: ManuscriptOutline,
    pub document: TenchDocument,
    pub citation_state: ManuscriptCitationState,
    #[serde(default)]
    pub assets: Vec<ManuscriptAsset>,
    #[serde(default)]
    pub cross_references: Vec<ManuscriptCrossReference>,
    #[serde(default)]
    pub checks: Vec<WritingCheckResult>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptAuthor {
    pub id: ManuscriptAuthorId,
    pub display_name: String,
    #[serde(default)]
    pub affiliation: Option<String>,
    #[serde(default)]
    pub orcid: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptTarget {
    pub kind: TargetKind,
    pub name: String,
    #[serde(default)]
    pub citation_style: Option<String>,
    pub bibliography_locale: ResearchLocale,
    #[serde(default)]
    pub word_limit: Option<u32>,
    #[serde(default)]
    pub abstract_limit: Option<u32>,
    #[serde(default)]
    pub section_rules: Vec<SectionRule>,
    #[serde(default)]
    pub figure_table_rules: Vec<AssetRule>,
    #[serde(default)]
    pub export_formats: Vec<WritingExportFormat>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    Journal,
    Conference,
    Thesis,
    Report,
    Custom,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SectionRule {
    pub kind: SectionKind,
    pub required: bool,
    #[serde(default)]
    pub word_limit: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetRule {
    pub kind: AssetKind,
    pub alt_text_required: bool,
    #[serde(default)]
    pub max_size_bytes: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritingExportFormat {
    Docx,
    Pdf,
    Markdown,
    Html,
    Latex,
    Archive,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManuscriptTemplateKind {
    JournalArticle,
    ConferencePaper,
    LiteratureReview,
    ThesisChapter,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptTextInsertion {
    pub section_id: SectionId,
    pub text: String,
    #[serde(default)]
    pub cited_references: Vec<ReferenceId>,
    pub now: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptCitationInsertion {
    pub id: CitationId,
    #[serde(default)]
    pub reference_ids: Vec<ReferenceId>,
    #[serde(default)]
    pub section_id: Option<SectionId>,
    pub mode: CitationMode,
    pub now: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptSectionNoteLink {
    pub section_id: SectionId,
    pub note_id: ResearchNoteId,
    pub now: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptAnnotationQuoteInsertion {
    pub section_id: SectionId,
    #[serde(default)]
    pub note_id: Option<ResearchNoteId>,
    pub annotation: PdfAnnotation,
    pub now: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptTableInsertion {
    pub section_id: SectionId,
    #[serde(default)]
    pub caption: Option<LocalizedField>,
    #[serde(default)]
    pub rows: Vec<Vec<String>>,
    pub now: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptEquationInsertion {
    pub section_id: SectionId,
    #[serde(default)]
    pub label: Option<String>,
    pub latex: String,
    pub now: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptAssetPlacement {
    pub section_id: SectionId,
    pub asset_id: ManuscriptAssetId,
    pub now: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptCrossReferenceInsertion {
    pub id: ManuscriptCrossReferenceId,
    pub section_id: SectionId,
    pub target: ManuscriptCrossReferenceTarget,
    pub now: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptExport {
    pub format: WritingExportFormat,
    pub file_name: String,
    pub body: String,
    #[serde(default)]
    pub body_bytes: Vec<u8>,
    pub media_type: String,
    pub bibliography: BibliographySnapshot,
    #[serde(default)]
    pub diagnostics: Vec<WritingCheckResult>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptSnapshot {
    pub id: SnapshotId,
    pub manuscript_id: ManuscriptId,
    pub title: LocalizedField,
    pub body_plain_text: String,
    pub bibliography: BibliographySnapshot,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptDiff {
    pub before_id: SnapshotId,
    pub after_id: SnapshotId,
    pub word_delta: i64,
    #[serde(default)]
    pub added_lines: Vec<String>,
    #[serde(default)]
    pub removed_lines: Vec<String>,
    pub bibliography_changed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ManuscriptOutline {
    #[serde(default)]
    pub sections: Vec<OutlineSection>,
    #[serde(default)]
    pub required_sections: Vec<RequiredSection>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutlineSection {
    pub id: SectionId,
    pub title: LocalizedField,
    pub kind: SectionKind,
    pub status: SectionStatus,
    #[serde(default)]
    pub source_notes: Vec<ResearchNoteId>,
    #[serde(default)]
    pub cited_references: Vec<ReferenceId>,
    #[serde(default)]
    pub target_words: Option<u32>,
    #[serde(default)]
    pub children: Vec<OutlineSection>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequiredSection {
    pub kind: SectionKind,
    pub label: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionKind {
    Title,
    Abstract,
    Introduction,
    Background,
    RelatedWork,
    Methods,
    Results,
    Discussion,
    Conclusion,
    References,
    Appendix,
    Custom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionStatus {
    Planned,
    Drafting,
    NeedsCitation,
    NeedsRevision,
    Complete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptCitationState {
    pub style_id: String,
    pub locale: ResearchLocale,
    #[serde(default)]
    pub citations: Vec<InlineCitation>,
    pub bibliography: BibliographySnapshot,
    #[serde(default)]
    pub unresolved_citations: Vec<CitationIssue>,
    #[serde(default)]
    pub citekey_map: Vec<CitekeyBinding>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InlineCitation {
    pub id: CitationId,
    pub reference_ids: Vec<ReferenceId>,
    pub section_id: Option<SectionId>,
    pub mode: CitationMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationMode {
    InText,
    Footnote,
    Endnote,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct BibliographySnapshot {
    #[serde(default)]
    pub rendered: String,
    #[serde(default)]
    pub reference_ids: Vec<ReferenceId>,
    #[serde(default)]
    pub generated_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CitationIssue {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub reference_id: Option<ReferenceId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CitekeyBinding {
    pub reference_id: ReferenceId,
    pub citekey: crate::Citekey,
    pub locked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptAsset {
    pub id: ManuscriptAssetId,
    pub kind: AssetKind,
    pub label: String,
    pub caption: LocalizedField,
    pub source: AssetSource,
    #[serde(default)]
    pub permissions: Option<PermissionNote>,
    #[serde(default)]
    pub linked_references: Vec<ReferenceId>,
    #[serde(default)]
    pub linked_notes: Vec<ResearchNoteId>,
    #[serde(default)]
    pub alt_text: Option<String>,
    pub order: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptAssetNumbering {
    pub asset_id: ManuscriptAssetId,
    pub kind: AssetKind,
    pub number: u32,
    pub label: String,
    pub caption: LocalizedField,
    pub order: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManuscriptCrossReference {
    pub id: ManuscriptCrossReferenceId,
    pub section_id: SectionId,
    pub target: ManuscriptCrossReferenceTarget,
    pub label: String,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum ManuscriptCrossReferenceTarget {
    Asset { asset_id: ManuscriptAssetId },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    Figure,
    Table,
    Equation,
    Supplement,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetSource {
    pub kind: AssetSourceKind,
    #[serde(default)]
    pub attachment_id: Option<AttachmentId>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub note_id: Option<ResearchNoteId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetSourceKind {
    Attachment,
    LocalFile,
    Note,
    Generated,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PermissionNote {
    pub text: String,
    pub checked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WritingCheckResult {
    pub severity: WritingCheckSeverity,
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub section_id: Option<SectionId>,
    #[serde(default)]
    pub reference_id: Option<ReferenceId>,
    pub export_blocker: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WritingCheckSeverity {
    Info,
    Warning,
    Error,
}
