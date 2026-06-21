use tench_research_core::{
    append_analysis_to_note as core_append_analysis_to_note,
    build_analysis_engine_request as core_build_analysis_engine_request,
    notes_for_paper as core_notes_for_paper, search_papers as core_search_papers,
    AnalysisEngineRequestPlan, AnalysisRequest, AnalysisThread, Note, Paper, PaperSearchFilter,
    ResearchSnapshot, Timestamp,
};

#[tauri::command]
pub fn research_snapshot() -> ResearchSnapshot {
    ResearchSnapshot {
        papers: Vec::new(),
        collections: Vec::new(),
        tags: Vec::new(),
        notes: Vec::new(),
        analysis_threads: Vec::new(),
    }
}

#[tauri::command]
pub fn research_i18n_required_keys() -> Vec<String> {
    crate::i18n::research_i18n_required_keys()
}

#[tauri::command]
pub fn research_i18n_catalog(locale: String) -> tench_app_core::I18nCatalog {
    crate::i18n::research_i18n_catalog(&locale)
}

#[tauri::command]
pub fn research_i18n_coverage(locale: String) -> tench_app_core::I18nCoverageReport {
    let catalog = research_i18n_catalog(locale);
    let required = research_i18n_required_keys();
    catalog.coverage_report(&required)
}

#[tauri::command]
pub fn search_papers(papers: Vec<Paper>, filter: PaperSearchFilter) -> Vec<Paper> {
    core_search_papers(&papers, &filter)
}

#[tauri::command]
pub fn notes_for_paper(notes: Vec<Note>, paper_id: String) -> Vec<Note> {
    core_notes_for_paper(&notes, &paper_id)
}

#[tauri::command]
pub fn prepare_research_analysis_engine_request(
    request: AnalysisRequest,
    papers: Vec<Paper>,
    model: Option<String>,
    stream: bool,
) -> Result<AnalysisEngineRequestPlan, String> {
    core_build_analysis_engine_request(&request, &papers, model.as_deref(), stream)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn complete_research_analysis_from_engine_output(
    plan: AnalysisEngineRequestPlan,
    prompt: String,
    assistant_content: String,
    created_at: Timestamp,
) -> Result<AnalysisThread, String> {
    tench_research_core::analysis_thread_from_engine_output(
        &plan,
        prompt,
        assistant_content,
        created_at,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn append_analysis_to_note(note: Note, thread: AnalysisThread) -> Note {
    core_append_analysis_to_note(&note, &thread)
}

#[tauri::command]
pub fn research_ai_prompt_templates() -> Vec<tench_research_core::ResearchAiPromptTemplate> {
    tench_research_core::research_ai_prompt_templates()
}

#[tauri::command]
pub fn prepare_research_ai_workflow_engine_request(
    request: tench_research_core::ResearchAiWorkflowRequest,
    model: Option<String>,
    stream: bool,
) -> Result<tench_research_core::ResearchAiWorkflowPlan, String> {
    tench_research_core::build_research_ai_workflow_engine_request(
        &request,
        model.as_deref(),
        stream,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn complete_research_ai_workflow_text_draft(
    plan: tench_research_core::ResearchAiWorkflowPlan,
    engine_output_markdown: String,
    created_at: Timestamp,
) -> Result<tench_research_core::ResearchAiDraftOutput, String> {
    tench_research_core::complete_research_ai_workflow_text_draft(
        &plan,
        engine_output_markdown,
        created_at,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_research_ai_draft_status(
    draft: tench_research_core::ResearchAiDraftOutput,
    status: tench_research_core::DraftStatus,
) -> tench_research_core::ResearchAiDraftOutput {
    tench_research_core::set_research_ai_draft_status(draft, status)
}

#[tauri::command]
pub fn can_save_research_ai_draft_as_note(
    draft: tench_research_core::ResearchAiDraftOutput,
) -> bool {
    tench_research_core::can_save_research_ai_draft_as_note(&draft)
}

#[tauri::command]
pub fn approve_research_ai_draft_as_note(
    draft: tench_research_core::ResearchAiDraftOutput,
    note_id: tench_research_core::ResearchNoteId,
    title: Option<String>,
    now: Timestamp,
) -> Result<tench_research_core::ResearchNote, String> {
    tench_research_core::approve_research_ai_draft_as_note(&draft, note_id, title, now)
        .map_err(|error| error.to_string())
}
