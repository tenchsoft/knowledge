use serde::{Deserialize, Serialize};
use tench_fs_core::{FileAccessGrant, FileEntry, FolderScanOptions};
use tench_job_core::JobDescriptor;

use crate::{AttachmentStoragePolicy, Citekey, ReferenceId, ResearchLocale, ResearchSnapshotV2};

crate::research_id_type!(ImportBatchId);
crate::research_id_type!(ExportBatchId);
crate::research_id_type!(ImportIssueId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchImportFormat {
    Pdf,
    BibTex,
    Ris,
    CslJson,
    EndNoteXml,
    Doi,
    Isbn,
    Arxiv,
    WebTranslator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchExportFormat {
    BibTex,
    Ris,
    CslJson,
    EndNoteXml,
    PlainTextBibliography,
    MarkdownBibliography,
    HtmlBibliography,
    RtfBibliography,
    LibraryBackupArchive,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum ImportSource {
    File {
        entry: FileEntry,
        format: ResearchImportFormat,
    },
    Folder {
        root: String,
        options: FolderScanOptions,
    },
    Identifier {
        value: String,
        format: ResearchImportFormat,
    },
    Text {
        value: String,
        format: ResearchImportFormat,
    },
    WebTranslator {
        url: String,
        translator_id: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImportPlan {
    pub id: ImportBatchId,
    pub sources: Vec<ImportSource>,
    pub dedupe_policy: DedupePolicy,
    #[serde(default)]
    pub grants: Vec<FileAccessGrant>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PdfImportOptions {
    pub recursive: bool,
    pub include_hidden: bool,
    #[serde(default)]
    pub max_depth: Option<u8>,
    #[serde(default)]
    pub storage_policy: Option<AttachmentStoragePolicy>,
    #[serde(default)]
    pub locale: Option<ResearchLocale>,
}

impl Default for PdfImportOptions {
    fn default() -> Self {
        Self {
            recursive: true,
            include_hidden: false,
            max_depth: Some(16),
            storage_policy: None,
            locale: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DedupePolicy {
    KeepBoth,
    MergeMetadata,
    PreferExisting,
    PreferImported,
    AskUser,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportReport {
    pub batch_id: ImportBatchId,
    pub imported: Vec<ReferenceId>,
    pub duplicates: Vec<DuplicateCandidate>,
    pub issues: Vec<ImportIssue>,
    #[serde(default)]
    pub jobs: Vec<JobDescriptor>,
}

impl ImportReport {
    pub fn has_blockers(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == ImportIssueSeverity::Error)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfImportOutcome {
    pub snapshot: ResearchSnapshotV2,
    pub report: ImportReport,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DuplicateCandidate {
    pub existing_id: ReferenceId,
    pub imported_id: ReferenceId,
    pub reason: DuplicateReason,
    pub confidence: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DuplicateReason {
    Doi,
    Isbn,
    ArxivId,
    Pmid,
    TitleYearCreator,
    AttachmentHash,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImportIssue {
    pub id: ImportIssueId,
    pub severity: ImportIssueSeverity,
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub retryable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportIssueSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExportRequest {
    pub id: ExportBatchId,
    pub format: ResearchExportFormat,
    pub selection: ExportSelection,
    #[serde(default)]
    pub locale: Option<crate::ResearchLocale>,
    #[serde(default)]
    pub style_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum ExportSelection {
    All,
    References { ids: Vec<ReferenceId> },
    Collection { id: crate::ResearchCollectionId },
    Bibliography { citekeys: Vec<Citekey> },
}
