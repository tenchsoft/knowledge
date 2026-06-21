use std::path::PathBuf;
use std::sync::Mutex;

use crate::state::{AnswerRecord, StudyState, StudyStats};

/// Return the path to the study state file inside the Tauri app data dir.
fn state_path(_app: &tauri::AppHandle) -> PathBuf {
    tench_study_core::study_storage_dir(tench_study_core::StudyStorageArea::Progress)
        .join("study_state.json")
}

/// Auto-save the current state to disk.
fn auto_save(app: &tauri::AppHandle, state: &StudyState) -> Result<(), String> {
    let _ = app;
    tench_study_core::write_study_json(
        tench_study_core::StudyStorageArea::Progress,
        "study_state.json",
        state,
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn study_i18n_required_keys() -> Vec<String> {
    crate::i18n::study_i18n_required_keys()
}

#[tauri::command]
pub fn study_i18n_catalog(locale: String) -> tench_app_core::I18nCatalog {
    crate::i18n::study_i18n_catalog(&locale)
}

#[tauri::command]
pub fn study_i18n_coverage(locale: String) -> tench_app_core::I18nCoverageReport {
    let catalog = study_i18n_catalog(locale);
    let required = study_i18n_required_keys();
    catalog.coverage_report(&required)
}

#[tauri::command]
pub fn prepare_study_ai_engine_request(
    request: tench_study_core::StudyAiRequest,
    model: Option<String>,
    stream: bool,
) -> Result<tench_study_core::StudyAiEngineRequestPlan, String> {
    tench_study_core::build_study_ai_engine_request(&request, model.as_deref(), stream)
}

#[tauri::command]
pub fn study_ai_prompt_templates() -> Vec<tench_study_core::StudyAiPromptTemplate> {
    tench_study_core::study_ai_prompt_templates()
}

#[tauri::command]
pub fn complete_study_ai_draft_from_engine_output(
    plan: tench_study_core::StudyAiEngineRequestPlan,
    assistant_content: String,
    created_at: String,
) -> Result<tench_study_core::StudyAiDraft, String> {
    tench_study_core::study_ai_draft_from_engine_output(&plan, assistant_content, created_at)
}

#[tauri::command]
pub fn can_commit_study_ai_draft(draft: tench_study_core::StudyAiDraft) -> bool {
    draft.can_commit_to_learner_data()
}

#[tauri::command]
pub fn set_study_ai_draft_status(
    draft: tench_study_core::StudyAiDraft,
    status: tench_study_core::StudyAiDraftStatus,
    edited_content_markdown: Option<String>,
) -> Result<tench_study_core::StudyAiDraft, String> {
    tench_study_core::set_study_ai_draft_status(draft, status, edited_content_markdown)
}

#[tauri::command]
pub fn approve_study_ai_draft_for_commit(
    draft: tench_study_core::StudyAiDraft,
    destination: Option<tench_study_core::StudyAiCommitDestination>,
) -> Result<tench_study_core::StudyAiCommitPlan, String> {
    tench_study_core::approve_study_ai_draft_for_commit(draft, destination)
}

#[tauri::command]
pub fn load_study_state(app: tauri::AppHandle) -> Result<StudyState, String> {
    let _ = state_path(&app);
    tench_study_core::read_study_json(
        tench_study_core::StudyStorageArea::Progress,
        "study_state.json",
    )
    .map(|state| state.unwrap_or_default())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_study_state(app: tauri::AppHandle, state: StudyState) -> Result<(), String> {
    let _ = state_path(&app);
    tench_study_core::write_study_json(
        tench_study_core::StudyStorageArea::Progress,
        "study_state.json",
        &state,
    )
    .map(|_| ())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_study_profile(profile: tench_study_core::LearnerProfile) -> Result<String, String> {
    let file_name = format!("{}.json", profile.id.as_str());
    tench_study_core::write_study_json(
        tench_study_core::StudyStorageArea::LearnerProfiles,
        &file_name,
        &profile,
    )
    .map(|path| path.to_string_lossy().to_string())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_study_profile(
    learner_id: tench_study_core::LearnerId,
) -> Result<Option<tench_study_core::LearnerProfile>, String> {
    let file_name = format!("{}.json", learner_id.as_str());
    tench_study_core::read_study_json(
        tench_study_core::StudyStorageArea::LearnerProfiles,
        &file_name,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_study_learner_progress(
    progress: tench_study_core::LearnerProgress,
) -> Result<String, String> {
    let file_name = format!(
        "{}-{}.json",
        progress.learner_id.as_str(),
        progress.node_id.as_str()
    );
    tench_study_core::write_study_json(
        tench_study_core::StudyStorageArea::Progress,
        &file_name,
        &progress,
    )
    .map(|path| path.to_string_lossy().to_string())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_study_learner_progress(
    learner_id: tench_study_core::LearnerId,
    node_id: tench_study_core::CurriculumNodeId,
) -> Result<Option<tench_study_core::LearnerProgress>, String> {
    let file_name = format!("{}-{}.json", learner_id.as_str(), node_id.as_str());
    tench_study_core::read_study_json(tench_study_core::StudyStorageArea::Progress, &file_name)
        .map_err(|error| error.to_string())
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub fn record_answer(
    app: tauri::AppHandle,
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
    concept_id: String,
    correct: bool,
    problem_id: String,
    wrong_answer: String,
    correct_answer: String,
    cause_tag: String,
    related_concept: String,
    solution: String,
    problem_text: String,
    problem_matrices: String,
) -> Result<(), String> {
    let mut state = state_mutex.lock().map_err(|e| e.to_string())?;
    state.record_answer(AnswerRecord {
        concept_id,
        correct,
        problem_id,
        wrong_answer,
        correct_answer,
        cause_tag,
        related_concept,
        solution,
        problem_text,
        problem_matrices,
    });

    auto_save(&app, &state)
}

#[tauri::command]
pub fn update_spaced_repetition(
    app: tauri::AppHandle,
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
    concept_id: String,
    quality: u32,
) -> Result<(), String> {
    let mut state = state_mutex.lock().map_err(|e| e.to_string())?;
    state.update_spaced_repetition(concept_id, quality);

    auto_save(&app, &state)
}

#[tauri::command]
pub fn get_due_review_concepts(
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
) -> Result<Vec<String>, String> {
    let state = state_mutex.lock().map_err(|e| e.to_string())?;
    Ok(state.due_review_concepts(&chrono::Utc::now().to_rfc3339()))
}

#[tauri::command]
pub fn mark_reviewed(
    app: tauri::AppHandle,
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
    review_id: String,
) -> Result<(), String> {
    let mut state = state_mutex.lock().map_err(|e| e.to_string())?;
    state.mark_reviewed(&review_id);
    auto_save(&app, &state)
}

#[tauri::command]
pub fn increment_session(
    app: tauri::AppHandle,
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
) -> Result<(), String> {
    let mut state = state_mutex.lock().map_err(|e| e.to_string())?;
    state.increment_session();
    auto_save(&app, &state)
}

#[tauri::command]
pub fn add_elapsed_seconds(
    app: tauri::AppHandle,
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
    seconds: u64,
) -> Result<(), String> {
    let mut state = state_mutex.lock().map_err(|e| e.to_string())?;
    state.add_elapsed_seconds(seconds);
    auto_save(&app, &state)
}

#[tauri::command]
pub fn update_streak(
    app: tauri::AppHandle,
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
    streak: u32,
) -> Result<(), String> {
    let mut state = state_mutex.lock().map_err(|e| e.to_string())?;
    state.update_streak(streak);
    auto_save(&app, &state)
}

#[tauri::command]
pub fn get_study_stats(
    state_mutex: tauri::State<'_, Mutex<StudyState>>,
) -> Result<StudyStats, String> {
    let state = state_mutex.lock().map_err(|e| e.to_string())?;
    Ok(state.stats())
}
