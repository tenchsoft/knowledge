#[tauri::command]
pub fn create_study_progress_backup(
    id: tench_study_core::StudyProgressBackupId,
    exported_at: String,
    payload: tench_study_core::StudyProgressBackupPayload,
) -> tench_study_core::StudyProgressBackup {
    tench_study_core::create_study_progress_backup(id, exported_at, payload)
}

#[tauri::command]
pub fn export_study_progress_backup_json(
    backup: tench_study_core::StudyProgressBackup,
) -> Result<String, String> {
    tench_study_core::export_study_progress_backup_json(&backup)
}

#[tauri::command]
pub fn import_study_progress_backup_json(
    text: String,
) -> Result<tench_study_core::StudyProgressBackup, String> {
    tench_study_core::import_study_progress_backup_json(&text)
}

#[tauri::command]
pub fn export_study_progress_backup_zip(
    backup: tench_study_core::StudyProgressBackup,
) -> Result<Vec<u8>, String> {
    tench_study_core::export_study_progress_backup_zip(&backup)
}

#[tauri::command]
pub fn import_study_progress_backup_zip(
    bytes: Vec<u8>,
) -> Result<tench_study_core::StudyProgressBackup, String> {
    tench_study_core::import_study_progress_backup_zip(&bytes)
}

#[tauri::command]
pub fn preview_study_progress_restore(
    backup: tench_study_core::StudyProgressBackup,
) -> Result<tench_study_core::StudyProgressRestoreReport, String> {
    tench_study_core::preview_study_progress_restore(&backup)
}

#[tauri::command]
pub fn restore_study_progress_backup_payload(
    backup: tench_study_core::StudyProgressBackup,
) -> Result<tench_study_core::StudyProgressBackupPayload, String> {
    tench_study_core::restore_study_progress_backup_payload(backup)
}

#[tauri::command]
pub fn save_study_progress_backup(
    backup: tench_study_core::StudyProgressBackup,
) -> Result<String, String> {
    tench_study_core::save_study_progress_backup(&backup)
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_study_progress_backup(
    id: tench_study_core::StudyProgressBackupId,
) -> Result<Option<tench_study_core::StudyProgressBackup>, String> {
    tench_study_core::load_study_progress_backup(&id).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn grade_study_exam_session(
    session: tench_study_core::ExamSession,
    items: Vec<tench_study_core::PracticeItem>,
    completed_at: String,
) -> Result<(tench_study_core::ExamSession, tench_study_core::ExamReport), String> {
    tench_study_core::grade_exam_session(session, &items, completed_at)
}

#[tauri::command]
pub fn build_study_exam_session(
    blueprint: tench_study_core::ExamBlueprint,
    items: Vec<tench_study_core::PracticeItem>,
) -> Result<tench_study_core::ExamBuildReport, String> {
    tench_study_core::build_exam_session(blueprint, &items)
}

#[tauri::command]
pub fn study_exam_timing_status(
    session: tench_study_core::ExamSession,
    elapsed_seconds: u32,
) -> tench_study_core::ExamTimingStatus {
    tench_study_core::exam_timing_status(&session, elapsed_seconds)
}

#[tauri::command]
pub fn grade_study_exam_session_with_rubrics(
    session: tench_study_core::ExamSession,
    items: Vec<tench_study_core::PracticeItem>,
    rubric_scores: Vec<tench_study_core::RubricScore>,
    completed_at: String,
) -> Result<(tench_study_core::ExamSession, tench_study_core::ExamReport), String> {
    tench_study_core::grade_exam_session_with_rubrics(session, &items, &rubric_scores, completed_at)
}

#[tauri::command]
pub fn build_study_exam_result_review(
    session: tench_study_core::ExamSession,
    items: Vec<tench_study_core::PracticeItem>,
) -> Result<tench_study_core::ExamResultReview, String> {
    tench_study_core::build_exam_result_review(&session, &items)
}

#[tauri::command]
pub fn export_study_exam_report(
    format: tench_study_core::StudyExamReportExportFormat,
    session: tench_study_core::ExamSession,
    report: tench_study_core::ExamReport,
    review: Option<tench_study_core::ExamResultReview>,
) -> Result<String, String> {
    tench_study_core::export_study_exam_report(format, &session, &report, review.as_ref())
}

#[tauri::command]
pub fn schedule_study_review_at(
    state: tench_study_core::SpacedRepetitionState,
    rating: tench_study_core::ReviewRating,
    reviewed_on: tench_study_core::ReviewDate,
) -> tench_study_core::SpacedRepetitionState {
    tench_study_core::schedule_review_at(state, rating, reviewed_on)
}

#[tauri::command]
pub fn apply_study_fsrs_hint(
    state: tench_study_core::SpacedRepetitionState,
    stability: f64,
    difficulty: f64,
) -> tench_study_core::SpacedRepetitionState {
    tench_study_core::apply_fsrs_hint(state, stability, difficulty)
}

#[tauri::command]
pub fn bury_study_review_until(
    state: tench_study_core::SpacedRepetitionState,
    buried_until: tench_study_core::ReviewDate,
) -> tench_study_core::SpacedRepetitionState {
    tench_study_core::bury_review_until(state, buried_until)
}

#[tauri::command]
pub fn set_study_review_suspended(
    state: tench_study_core::SpacedRepetitionState,
    suspended: bool,
) -> tench_study_core::SpacedRepetitionState {
    tench_study_core::set_review_suspended(state, suspended)
}

#[tauri::command]
pub fn build_study_review_queue(
    progress: Vec<tench_study_core::LearnerProgress>,
    review_items: Vec<tench_study_core::ReviewQueueItem>,
    policy: tench_study_core::ReviewQueuePolicy,
) -> tench_study_core::ReviewQueuePlan {
    tench_study_core::build_review_queue(&progress, &review_items, &policy)
}

#[tauri::command]
pub fn study_review_stats(
    progress: Vec<tench_study_core::LearnerProgress>,
    review_items: Vec<tench_study_core::ReviewQueueItem>,
    today: tench_study_core::ReviewDate,
) -> tench_study_core::ReviewStats {
    tench_study_core::review_stats(&progress, &review_items, &today)
}
