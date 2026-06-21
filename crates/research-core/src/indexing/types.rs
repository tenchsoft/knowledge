use serde::{Deserialize, Serialize};
use tench_search_core::{IndexStats, SearchDomain, SearchResult};

use crate::{AttachmentId, ReferenceId, ResearchLocale};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIndexState {
    pub stats: IndexStats,
    #[serde(default)]
    pub pending_references: Vec<ReferenceId>,
    #[serde(default)]
    pub failed_jobs: Vec<ResearchIndexFailure>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIndexFailure {
    pub reference_id: Option<ReferenceId>,
    pub attachment_id: Option<crate::AttachmentId>,
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIndexManifestEntry {
    pub document_id: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIncrementalIndexPlan {
    pub state: ResearchIndexState,
    #[serde(default)]
    pub upsert_documents: Vec<ResearchIndexDocument>,
    #[serde(default)]
    pub remove_document_ids: Vec<String>,
    #[serde(default)]
    pub manifest: Vec<ResearchIndexManifestEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIndexRepairReport {
    pub state: ResearchIndexState,
    #[serde(default)]
    pub documents: Vec<ResearchIndexDocument>,
    #[serde(default)]
    pub removed_document_ids: Vec<String>,
    #[serde(default)]
    pub manifest: Vec<ResearchIndexManifestEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchSearchRequest {
    pub query: String,
    #[serde(default)]
    pub locale: Option<ResearchLocale>,
    #[serde(default)]
    pub filters: Vec<SearchFilter>,
    #[serde(default)]
    pub sort: SearchSort,
    pub limit: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SearchFilter {
    pub field: SearchField,
    pub value: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchField {
    Author,
    Tag,
    Year,
    Venue,
    Status,
    Collection,
    HasAttachment,
    Citekey,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SearchSort {
    pub field: SearchSortField,
    pub direction: SortDirection,
}

impl Default for SearchSort {
    fn default() -> Self {
        Self {
            field: SearchSortField::Relevance,
            direction: SortDirection::Desc,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchSortField {
    Relevance,
    AddedDate,
    UpdatedDate,
    Year,
    Title,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIndexDocument {
    pub id: String,
    pub reference_id: Option<ReferenceId>,
    pub domain: SearchDomain,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub location: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchSearchResult {
    pub result: SearchResult,
    #[serde(default)]
    pub hits: Vec<ResearchSearchHit>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchSearchHit {
    pub field: ResearchSearchHitField,
    pub range: ResearchTextRange,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub reference_id: Option<ReferenceId>,
    #[serde(default)]
    pub attachment_id: Option<AttachmentId>,
    #[serde(default)]
    pub page: Option<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchSearchHitField {
    Title,
    Metadata,
    Note,
    Annotation,
    PdfPage,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchTextRange {
    pub start_grapheme: u32,
    pub end_grapheme: u32,
    pub snippet: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIndexBenchmarkInput {
    pub reference_count: usize,
    pub pdf_attachment_count: usize,
    pub pages_per_pdf: usize,
    pub chars_per_pdf_page: usize,
    pub query: String,
    pub search_limit: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchIndexBenchmarkReport {
    pub reference_count: usize,
    pub pdf_attachment_count: usize,
    pub document_count: usize,
    pub indexed_bytes: u64,
    pub build_micros: u128,
    pub search_micros: u128,
    pub query_result_count: usize,
    pub ten_k_reference_target_micros: u128,
    pub thousand_pdf_target_micros: u128,
    pub meets_10k_reference_target: bool,
    pub meets_1k_pdf_target: bool,
}

pub const RESEARCH_10K_REFERENCE_INDEX_TARGET_MICROS: u128 = 2_000_000;
pub const RESEARCH_1K_PDF_INDEX_TARGET_MICROS: u128 = 5_000_000;
