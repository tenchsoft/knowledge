use tench_research_core::*;

#[tauri::command]
pub fn run_research_manuscript_checks(manuscript: ResearchManuscript) -> Vec<WritingCheckResult> {
    tench_research_core::run_non_ai_writing_checks(&manuscript)
}

#[tauri::command]
pub fn create_research_manuscript_skeleton(
    id: tench_research_core::ManuscriptId,
    library_id: tench_research_core::LibraryId,
    title: tench_research_core::LocalizedField,
    target: tench_research_core::ManuscriptTarget,
    locale: tench_research_core::ResearchLocale,
    now: Timestamp,
) -> ResearchManuscript {
    tench_research_core::create_manuscript_skeleton(id, library_id, title, target, locale, now)
}

#[tauri::command]
pub fn create_research_manuscript_from_template(
    id: tench_research_core::ManuscriptId,
    library_id: tench_research_core::LibraryId,
    title: tench_research_core::LocalizedField,
    template: ManuscriptTemplateKind,
    locale: tench_research_core::ResearchLocale,
    now: Timestamp,
) -> ResearchManuscript {
    tench_research_core::create_manuscript_from_template(
        id, library_id, title, template, locale, now,
    )
}

#[tauri::command]
pub fn insert_research_manuscript_text(
    manuscript: ResearchManuscript,
    insertion: ManuscriptTextInsertion,
) -> Result<ResearchManuscript, String> {
    tench_research_core::insert_manuscript_text(manuscript, insertion)
}

#[tauri::command]
pub fn insert_research_manuscript_citation(
    manuscript: ResearchManuscript,
    insertion: ManuscriptCitationInsertion,
    references: Vec<ReferenceItem>,
) -> Result<ResearchManuscript, String> {
    tench_research_core::insert_manuscript_citation(manuscript, insertion, &references)
}

#[tauri::command]
pub fn link_note_to_research_manuscript_section(
    manuscript: ResearchManuscript,
    link: tench_research_core::ManuscriptSectionNoteLink,
) -> Result<ResearchManuscript, String> {
    tench_research_core::link_note_to_manuscript_section(manuscript, link)
}

#[tauri::command]
pub fn convert_research_annotation_to_manuscript_quote(
    manuscript: ResearchManuscript,
    insertion: tench_research_core::ManuscriptAnnotationQuoteInsertion,
) -> Result<ResearchManuscript, String> {
    tench_research_core::convert_annotation_to_manuscript_quote(manuscript, insertion)
}

#[tauri::command]
pub fn insert_research_manuscript_table(
    manuscript: ResearchManuscript,
    insertion: tench_research_core::ManuscriptTableInsertion,
) -> Result<ResearchManuscript, String> {
    tench_research_core::insert_manuscript_table(manuscript, insertion)
}

#[tauri::command]
pub fn insert_research_manuscript_equation(
    manuscript: ResearchManuscript,
    insertion: tench_research_core::ManuscriptEquationInsertion,
) -> Result<ResearchManuscript, String> {
    tench_research_core::insert_manuscript_equation(manuscript, insertion)
}

#[tauri::command]
pub fn insert_research_manuscript_asset_placement(
    manuscript: ResearchManuscript,
    placement: tench_research_core::ManuscriptAssetPlacement,
) -> Result<ResearchManuscript, String> {
    tench_research_core::insert_manuscript_asset_placement(manuscript, placement)
}

#[tauri::command]
pub fn create_research_manuscript_asset(
    manuscript: ResearchManuscript,
    asset: tench_research_core::ManuscriptAsset,
    now: Timestamp,
) -> Result<ResearchManuscript, String> {
    tench_research_core::create_manuscript_asset(manuscript, asset, now)
}

#[tauri::command]
pub fn update_research_manuscript_asset(
    manuscript: ResearchManuscript,
    asset: tench_research_core::ManuscriptAsset,
    now: Timestamp,
) -> Result<ResearchManuscript, String> {
    tench_research_core::update_manuscript_asset(manuscript, asset, now)
}

#[tauri::command]
pub fn build_research_manuscript_asset_numbering(
    manuscript: ResearchManuscript,
) -> Vec<tench_research_core::ManuscriptAssetNumbering> {
    tench_research_core::build_manuscript_asset_numbering(&manuscript)
}

#[tauri::command]
pub fn insert_research_manuscript_cross_reference(
    manuscript: ResearchManuscript,
    insertion: tench_research_core::ManuscriptCrossReferenceInsertion,
) -> Result<ResearchManuscript, String> {
    tench_research_core::insert_manuscript_cross_reference(manuscript, insertion)
}

#[tauri::command]
pub fn refresh_research_manuscript_bibliography(
    manuscript: ResearchManuscript,
    references: Vec<ReferenceItem>,
    now: Timestamp,
) -> ResearchManuscript {
    tench_research_core::refresh_manuscript_bibliography(manuscript, &references, now)
}

#[tauri::command]
pub fn export_research_manuscript(
    manuscript: ResearchManuscript,
    references: Vec<ReferenceItem>,
    format: WritingExportFormat,
    now: Timestamp,
) -> Result<ManuscriptExport, String> {
    tench_research_core::export_manuscript(manuscript, &references, format, now)
}

#[tauri::command]
pub fn create_research_manuscript_snapshot(
    manuscript: ResearchManuscript,
    id: tench_research_core::SnapshotId,
    now: Timestamp,
) -> ManuscriptSnapshot {
    tench_research_core::create_manuscript_snapshot(&manuscript, id, now)
}

#[tauri::command]
pub fn compare_research_manuscript_snapshots(
    before: ManuscriptSnapshot,
    after: ManuscriptSnapshot,
) -> ManuscriptDiff {
    tench_research_core::compare_manuscript_snapshots(&before, &after)
}
