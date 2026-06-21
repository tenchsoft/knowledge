use tench_research_core::*;

#[tauri::command]
pub fn import_research_pdf_paths(
    snapshot: ResearchSnapshotV2,
    root_dir: String,
    paths: Vec<String>,
    options: tench_research_core::PdfImportOptions,
    batch_id: tench_research_core::ImportBatchId,
    dedupe_policy: tench_research_core::DedupePolicy,
    now: String,
) -> Result<tench_research_core::PdfImportOutcome, String> {
    let paths = paths
        .into_iter()
        .map(std::path::PathBuf::from)
        .collect::<Vec<_>>();
    tench_research_core::import_pdf_paths(
        snapshot,
        root_dir,
        &paths,
        options,
        batch_id,
        dedupe_policy,
        now,
    )
}

#[tauri::command]
pub fn find_research_duplicate_candidates(
    existing: Vec<ReferenceItem>,
    imported: Vec<ReferenceItem>,
) -> Vec<tench_research_core::DuplicateCandidate> {
    tench_research_core::find_duplicate_candidates(&existing, &imported)
}

#[tauri::command]
pub fn research_shared_search_query(
    request: ResearchSearchRequest,
) -> tench_search_core::SearchQuery {
    tench_research_core::to_shared_search_query(&request)
}

#[tauri::command]
pub fn parse_research_search_request(query: String, limit: u16) -> ResearchSearchRequest {
    tench_research_core::parse_research_search_request(&query, limit)
}

#[tauri::command]
pub fn search_research_snapshot(
    snapshot: ResearchSnapshotV2,
    request: ResearchSearchRequest,
) -> Vec<tench_search_core::SearchResult> {
    tench_research_core::search_research_snapshot(&snapshot, &request)
}

#[tauri::command]
pub fn build_research_index_documents(
    snapshot: ResearchSnapshotV2,
    pdf_texts: Vec<tench_research_core::PdfDocumentText>,
) -> Vec<tench_research_core::ResearchIndexDocument> {
    tench_research_core::build_research_index_documents_with_pdf_text(&snapshot, &pdf_texts)
}

#[tauri::command]
pub fn build_research_index_manifest(
    documents: Vec<tench_research_core::ResearchIndexDocument>,
) -> Vec<tench_research_core::ResearchIndexManifestEntry> {
    tench_research_core::build_research_index_manifest(&documents)
}

#[tauri::command]
pub fn search_research_snapshot_with_pdf_text(
    snapshot: ResearchSnapshotV2,
    pdf_texts: Vec<tench_research_core::PdfDocumentText>,
    request: ResearchSearchRequest,
) -> Vec<tench_search_core::SearchResult> {
    tench_research_core::search_research_snapshot_with_pdf_text(&snapshot, &pdf_texts, &request)
}

#[tauri::command]
pub fn search_research_snapshot_with_hits(
    snapshot: ResearchSnapshotV2,
    pdf_texts: Vec<tench_research_core::PdfDocumentText>,
    request: ResearchSearchRequest,
) -> Vec<tench_research_core::ResearchSearchResult> {
    tench_research_core::search_research_snapshot_with_hits(&snapshot, &pdf_texts, &request)
}

#[tauri::command]
pub fn plan_incremental_research_reindex(
    snapshot: ResearchSnapshotV2,
    pdf_texts: Vec<tench_research_core::PdfDocumentText>,
    existing_manifest: Vec<tench_research_core::ResearchIndexManifestEntry>,
    failed_jobs: Vec<tench_research_core::ResearchIndexFailure>,
    updated_at: Option<String>,
) -> tench_research_core::ResearchIncrementalIndexPlan {
    tench_research_core::plan_incremental_research_reindex(
        &snapshot,
        &pdf_texts,
        &existing_manifest,
        failed_jobs,
        updated_at,
    )
}

#[tauri::command]
pub fn repair_research_index(
    snapshot: ResearchSnapshotV2,
    pdf_texts: Vec<tench_research_core::PdfDocumentText>,
    existing_document_ids: Vec<String>,
    failed_jobs: Vec<tench_research_core::ResearchIndexFailure>,
    updated_at: Option<String>,
) -> tench_research_core::ResearchIndexRepairReport {
    tench_research_core::repair_research_index(
        &snapshot,
        &pdf_texts,
        &existing_document_ids,
        failed_jobs,
        updated_at,
    )
}

#[tauri::command]
pub fn benchmark_research_indexing(
    input: tench_research_core::ResearchIndexBenchmarkInput,
) -> tench_research_core::ResearchIndexBenchmarkReport {
    tench_research_core::benchmark_research_indexing(input)
}
