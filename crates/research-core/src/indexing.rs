mod benchmark;
mod document_ids;
mod documents;
mod query;
mod search;
#[cfg(test)]
mod tests;
mod types;

pub use benchmark::benchmark_research_indexing;
pub use documents::{
    build_research_index_documents, build_research_index_documents_with_pdf_text,
    build_research_index_manifest, plan_incremental_research_reindex, repair_research_index,
};
pub use query::{normalize_search_query, parse_research_search_request, to_shared_search_query};
pub use search::{
    search_research_snapshot, search_research_snapshot_with_hits,
    search_research_snapshot_with_pdf_text,
};
pub use types::*;
