use super::*;
use tench_shared_types::EngineMethod;

#[test]
fn filters_by_query_and_tag() {
    let snapshot = example_snapshot();
    let results = search_papers(
        &snapshot.papers,
        &PaperSearchFilter {
            query: Some("attention".to_string()),
            tag: Some("transformers".to_string()),
            ..PaperSearchFilter::default()
        },
    );
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "paper-attention");
}

#[test]
fn builds_engine_request_for_analysis_without_local_output() {
    let snapshot = example_snapshot();
    let plan = build_analysis_engine_request(
        &AnalysisRequest {
            paper_id: "paper-attention".to_string(),
            prompt: "summarize".to_string(),
            page: None,
            selection: None,
        },
        &snapshot.papers,
        None,
        false,
    )
    .expect("analysis plan");
    assert_eq!(plan.job.kind, "research.ai.analysis");
    assert_eq!(
        plan.engine_request.method,
        EngineMethod::ChatCompletionsCreate
    );
    assert_eq!(plan.engine_request.params["model"], "tench/research");
    assert_eq!(plan.engine_request.params["messages"][0]["role"], "system");
    assert!(plan.requires_user_approval);
}

#[test]
fn research_ai_prompt_templates_cover_phase12_features() {
    let templates = research_ai_prompt_templates();

    assert_eq!(templates.len(), 10);
    for kind in phase12_ai_feature_kinds() {
        let template = templates
            .iter()
            .find(|template| template.kind == kind)
            .expect("template");
        assert!(template.requires_user_approval);
        assert!(template.system_prompt.contains("supplied local context"));
        assert!(!template.output_contract.trim().is_empty());
    }
}

#[test]
fn research_ai_workflows_use_engine_ipc_and_require_approval() {
    let request = ResearchAiWorkflowRequest {
        kind: ResearchAiFeatureKind::DraftMethodFlow,
        library_id: "lib_1".to_string(),
        reference_id: ReferenceId::from("ref_1"),
        title: "Methods paper".to_string(),
        context: "Methods: collect data, train model, evaluate.".to_string(),
        selection: None,
        annotations: vec!["p. 3: training procedure".to_string()],
        notes: vec!["Check baseline details.".to_string()],
        locale: ResearchLocale::parse("en-US"),
    };

    let plan =
        build_research_ai_workflow_engine_request(&request, None, true).expect("workflow plan");

    assert_eq!(plan.job.kind, "research.ai.workflow");
    assert_eq!(plan.transport, ResearchEngineTransport::Ipc);
    assert_eq!(plan.visual_kind, Some(ResearchVisualKind::MethodFlow));
    assert_eq!(
        plan.output_destination,
        ResearchAiOutputDestination::DraftVisual
    );
    assert_eq!(
        plan.engine_request.method,
        EngineMethod::ChatCompletionsCreate
    );
    assert_eq!(plan.engine_request.params["stream"], true);
    assert_eq!(plan.engine_request.params["messages"][0]["role"], "system");
    assert!(plan.requires_user_approval);
    assert_eq!(plan.job.payload["transport"], "ipc");
}

#[test]
fn research_ai_draft_saves_as_note_only_after_approval() {
    let request = ResearchAiWorkflowRequest {
        kind: ResearchAiFeatureKind::NoteFromAnnotations,
        library_id: "lib_1".to_string(),
        reference_id: ReferenceId::from("ref_1"),
        title: "Annotated paper".to_string(),
        context: "Annotation context.".to_string(),
        selection: None,
        annotations: vec!["p. 2: important result".to_string()],
        notes: Vec::new(),
        locale: None,
    };
    let plan = build_research_ai_workflow_engine_request(&request, Some("local-model"), false)
        .expect("workflow plan");
    let draft = complete_research_ai_workflow_text_draft(
        &plan,
        "## Reading note\n\nImportant result needs verification.",
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    )
    .expect("draft");

    assert!(!can_save_research_ai_draft_as_note(&draft));
    let draft = set_research_ai_draft_status(draft, DraftStatus::Accepted);
    let note = approve_research_ai_draft_as_note(
        &draft,
        ResearchNoteId::from("note_ai"),
        Some("Approved AI note".to_string()),
        Timestamp("2026-05-04T00:01:00Z".to_string()),
    )
    .expect("note");

    assert_eq!(note.reference_id, Some(ReferenceId::from("ref_1")));
    assert_eq!(note.title, "Approved AI note");
    assert!(note.body_markdown.contains("Important result"));
}

#[test]
fn research_ai_tag_suggestions_are_editable_draft_data() {
    let request = ResearchAiWorkflowRequest {
        kind: ResearchAiFeatureKind::SuggestTags,
        library_id: "lib_1".to_string(),
        reference_id: ReferenceId::from("ref_1"),
        title: "Tag paper".to_string(),
        context: "Graph neural network retrieval with evaluation.".to_string(),
        selection: None,
        annotations: Vec::new(),
        notes: Vec::new(),
        locale: None,
    };
    let plan = build_research_ai_workflow_engine_request(&request, None, false).expect("workflow");
    let draft = complete_research_ai_workflow_text_draft(
        &plan,
        "- Retrieval\n- graph neural networks\n- retrieval",
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    )
    .expect("draft");

    assert_eq!(
        draft.suggested_tags,
        vec!["retrieval", "graph neural networks"]
    );
    assert_eq!(draft.status, DraftStatus::Proposed);
}

#[test]
fn appends_approved_analysis_to_note() {
    let snapshot = example_snapshot();
    let plan = build_analysis_engine_request(
        &AnalysisRequest {
            paper_id: "paper-attention".to_string(),
            prompt: "summarize".to_string(),
            page: None,
            selection: None,
        },
        &snapshot.papers,
        None,
        false,
    )
    .expect("analysis plan");
    let thread = analysis_thread_from_engine_output(
        &plan,
        "summarize",
        "The paper introduces attention-only sequence modeling.",
        Timestamp("2026-04-28T10:00:02Z".to_string()),
    )
    .expect("approved engine output");
    let note = append_analysis_to_note(&snapshot.notes[0], &thread);
    assert!(note.content_markdown.contains("Research analysis"));
    assert!(note.word_count > snapshot.notes[0].word_count);
}
