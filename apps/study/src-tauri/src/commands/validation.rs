use std::path::PathBuf;

#[tauri::command]
pub fn validate_study_curriculum(
    curriculum: tench_study_core::Curriculum,
) -> tench_study_core::CurriculumValidationReport {
    curriculum.validate()
}

#[tauri::command]
pub fn validate_study_content_pack(
    manifest: tench_study_core::ContentPackManifest,
) -> tench_study_core::PackValidationReport {
    manifest.validate_for_release()
}

#[tauri::command]
pub fn validate_study_visual(visual: tench_study_core::LearningVisualSpec) -> Result<(), String> {
    visual.validate_for_release()
}

#[tauri::command]
pub fn validate_study_glossary_term(term: tench_study_core::GlossaryTerm) -> Result<(), String> {
    term.validate_for_release()
}

#[tauri::command]
pub fn detect_study_import_format(value: String) -> Option<tench_study_core::StudyImportFormat> {
    tench_study_core::detect_study_import_format(&value)
}

#[tauri::command]
pub fn plan_study_file_import_paths(
    paths: Vec<String>,
    options: tench_study_core::StudyFileImportOptions,
) -> Result<tench_study_core::StudyFileImportPlan, String> {
    let paths = paths.into_iter().map(PathBuf::from).collect::<Vec<_>>();
    tench_study_core::plan_study_file_import_paths(&paths, &options)
}

#[tauri::command]
pub fn study_card_exchange_format_for_import(
    format: tench_study_core::StudyImportFormat,
) -> Option<tench_study_core::StudyCardExchangeFormat> {
    tench_study_core::study_card_exchange_format(format)
}

#[tauri::command]
pub fn study_note_exchange_format_for_import(
    format: tench_study_core::StudyImportFormat,
) -> Option<tench_study_core::StudyNoteExchangeFormat> {
    tench_study_core::study_note_exchange_format(format)
}

#[tauri::command]
pub fn grade_study_practice_item(
    item: tench_study_core::PracticeItem,
    submission: tench_study_core::AnswerSubmission,
) -> tench_study_core::GradingResult {
    item.grade(&submission)
}
