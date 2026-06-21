#[tauri::command]
pub fn study_subject_catalog() -> tench_study_core::SubjectCatalog {
    tench_study_core::SubjectCatalog::production_scope()
}

#[tauri::command]
pub fn study_builtin_curricula() -> tench_study_core::BuiltinCurriculumSet {
    tench_study_core::builtin_curricula()
}

#[tauri::command]
pub fn study_builtin_visual_specs() -> Vec<tench_study_core::LearningVisualSpec> {
    tench_study_core::builtin_visual_specs_for_all()
}

#[tauri::command]
pub fn study_builtin_assessments() -> Vec<tench_study_core::AssessmentDraft> {
    tench_study_core::builtin_assessments_for_all()
}

#[tauri::command]
pub fn study_builtin_content_coverage_report() -> tench_study_core::BuiltinContentCoverageReport {
    tench_study_core::builtin_content_coverage_report()
}

#[tauri::command]
pub fn study_builtin_glossary_terms() -> Vec<tench_study_core::GlossaryTerm> {
    let curricula = tench_study_core::builtin_curricula();
    tench_study_core::glossary_terms_from_all_curricula(&curricula.curricula)
}

#[tauri::command]
pub fn study_dashboard_snapshot(
    profile: tench_study_core::LearnerProfile,
    curricula: Vec<tench_study_core::Curriculum>,
    progress: Vec<tench_study_core::LearnerProgress>,
    review_items: Vec<tench_study_core::ReviewQueueItem>,
    today: tench_study_core::ReviewDate,
    daily_limit: u16,
) -> tench_study_core::StudyDashboardSnapshot {
    tench_study_core::build_study_dashboard_snapshot(
        &profile,
        &curricula,
        &progress,
        &review_items,
        today,
        daily_limit,
    )
}

#[tauri::command]
pub fn study_shared_search_query(
    request: tench_study_core::StudySearchRequest,
) -> tench_search_core::SearchQuery {
    tench_study_core::study_to_shared_search_query(&request)
}

#[tauri::command]
pub fn search_study_curriculum(
    curriculum: tench_study_core::Curriculum,
    request: tench_study_core::StudySearchRequest,
) -> Vec<tench_search_core::SearchResult> {
    tench_study_core::curriculum_search_results(&curriculum, &request)
}

#[tauri::command]
pub fn search_study_notes_cards(
    notes: Vec<tench_study_core::StudyNote>,
    cards: Vec<tench_study_core::StudyCard>,
    request: tench_study_core::StudySearchRequest,
) -> Vec<tench_search_core::SearchResult> {
    tench_study_core::study_notes_cards_search_results(&notes, &cards, &request)
}

#[tauri::command]
pub fn search_study_glossary_terms(
    terms: Vec<tench_study_core::GlossaryTerm>,
    query: String,
    locale: Option<tench_study_core::ContentLocale>,
    limit: usize,
) -> Vec<tench_study_core::GlossarySearchResult> {
    tench_study_core::search_glossary_terms(&terms, &query, locale.as_ref(), limit)
}
