use tench_research_core::*;

#[tauri::command]
pub fn validate_research_visual(visual: ResearchVisualSpec) -> Result<(), String> {
    visual.validate_for_non_ai_release()
}

#[tauri::command]
pub fn build_research_reference_timeline_visual(
    id: tench_research_core::VisualSpecId,
    library_id: String,
    references: Vec<ReferenceItem>,
) -> ResearchVisualSpec {
    tench_research_core::build_reference_timeline_visual(id, library_id, &references)
}

#[tauri::command]
pub fn build_research_reference_influence_graph_visual(
    id: tench_research_core::VisualSpecId,
    library_id: String,
    references: Vec<ReferenceItem>,
) -> ResearchVisualSpec {
    tench_research_core::build_reference_influence_graph_visual(id, library_id, &references)
}

#[tauri::command]
pub fn build_research_library_overview_visual(
    id: tench_research_core::VisualSpecId,
    snapshot: ResearchSnapshotV2,
    index_state: Option<tench_research_core::ResearchIndexState>,
    duplicate_candidate_count: u32,
) -> ResearchVisualSpec {
    tench_research_core::build_library_overview_visual(
        id,
        &snapshot,
        index_state.as_ref(),
        duplicate_candidate_count,
    )
}

#[tauri::command]
pub fn build_research_reference_keyword_map_visual(
    id: tench_research_core::VisualSpecId,
    library_id: String,
    references: Vec<ReferenceItem>,
    limit: usize,
) -> ResearchVisualSpec {
    tench_research_core::build_reference_keyword_map_visual(id, library_id, &references, limit)
}

#[tauri::command]
pub fn build_research_pdf_annotation_heatmap_visual(
    id: tench_research_core::VisualSpecId,
    library_id: String,
    attachment_id: tench_research_core::AttachmentId,
    annotations: Vec<tench_research_core::PdfAnnotation>,
) -> ResearchVisualSpec {
    tench_research_core::build_pdf_annotation_heatmap_visual(
        id,
        library_id,
        attachment_id,
        &annotations,
    )
}

#[tauri::command]
pub fn build_research_citation_warning_visual(
    id: tench_research_core::VisualSpecId,
    library_id: String,
    output: tench_research_core::CitationRenderOutput,
) -> ResearchVisualSpec {
    tench_research_core::build_citation_warning_visual(id, library_id, &output)
}

#[tauri::command]
pub fn aggregate_research_library_visuals(
    id_prefix: String,
    snapshot: ResearchSnapshotV2,
    index_state: Option<tench_research_core::ResearchIndexState>,
    duplicate_candidate_count: u32,
    max_references: usize,
) -> tench_research_core::ResearchVisualAggregationBundle {
    tench_research_core::aggregate_research_library_visuals(
        id_prefix,
        &snapshot,
        index_state.as_ref(),
        duplicate_candidate_count,
        max_references,
    )
}

#[tauri::command]
pub fn build_manual_research_paper_analysis_visual(
    id: tench_research_core::VisualSpecId,
    library_id: String,
    kind: tench_research_core::ResearchVisualKind,
    title: String,
    locale: Option<tench_research_core::ResearchLocale>,
    manual_data: tench_research_core::ResearchVisualManualData,
) -> Result<ResearchVisualSpec, String> {
    tench_research_core::build_manual_paper_analysis_visual(
        id,
        library_id,
        kind,
        title,
        locale,
        manual_data,
    )
}

#[tauri::command]
pub fn build_research_manuscript_readiness_dashboard_visual(
    id: tench_research_core::VisualSpecId,
    manuscript: ResearchManuscript,
) -> ResearchVisualSpec {
    tench_research_core::build_manuscript_readiness_dashboard_visual(id, &manuscript)
}

#[tauri::command]
pub fn build_research_manuscript_citation_density_visual(
    id: tench_research_core::VisualSpecId,
    manuscript: ResearchManuscript,
) -> ResearchVisualSpec {
    tench_research_core::build_manuscript_citation_density_visual(id, &manuscript)
}

#[tauri::command]
pub fn build_research_manuscript_claim_evidence_visual(
    id: tench_research_core::VisualSpecId,
    manuscript: ResearchManuscript,
) -> ResearchVisualSpec {
    tench_research_core::build_manuscript_claim_evidence_visual(id, &manuscript)
}

#[tauri::command]
pub fn can_commit_research_ai_visual_draft(draft: AiVisualDraft) -> bool {
    draft.can_commit_to_canonical_content()
}

#[tauri::command]
pub fn prepare_research_ai_visual_engine_request(
    request: AiVisualRequest,
    model: Option<String>,
    stream: bool,
) -> Result<AiVisualEngineRequestPlan, String> {
    tench_research_core::build_ai_visual_engine_request(&request, model.as_deref(), stream)
}

#[tauri::command]
pub fn complete_research_ai_visual_draft_from_engine_spec(
    plan: AiVisualEngineRequestPlan,
    visual_spec: ResearchVisualSpec,
    confidence: Option<f32>,
    warnings: Vec<tench_research_core::AiVisualWarning>,
    created_at: Timestamp,
) -> Result<AiVisualDraft, String> {
    tench_research_core::ai_visual_draft_from_engine_spec(
        &plan,
        visual_spec,
        confidence,
        warnings,
        created_at,
    )
}

#[tauri::command]
pub fn apply_research_visual_action(
    state: VisualState,
    action: ResearchVisualAction,
) -> VisualState {
    state.apply_action(action)
}

#[tauri::command]
pub fn build_research_visual_draw_plan(
    visual: ResearchVisualSpec,
    reduced_motion: bool,
) -> Result<tench_research_core::ResearchVisualDrawPlan, String> {
    tench_research_core::build_research_visual_draw_plan(&visual, reduced_motion)
}
