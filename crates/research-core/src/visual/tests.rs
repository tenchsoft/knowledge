use super::*;
use crate::{
    AttachmentId, CitationRenderOutput, ReadingStatus, ReferenceId, ResearchIndexState,
    ResearchLocale, ResearchNoteId, Timestamp,
};
use tench_shared_types::EngineMethod;

fn visual(source: VisualSource) -> ResearchVisualSpec {
    ResearchVisualSpec {
        id: VisualSpecId::from("visual_1"),
        kind: ResearchVisualKind::ClaimEvidenceGraph,
        title: VisualLocalizedText {
            value: "Claims".to_string(),
            locale: None,
        },
        data_query: VisualQuery {
            library_id: "lib_1".to_string(),
            reference_ids: Vec::new(),
            note_ids: Vec::new(),
            filters: Vec::new(),
            aggregation: None,
        },
        encodings: Vec::new(),
        state: VisualState::default(),
        animation: None,
        interactions: vec![VisualInteraction::Select, VisualInteraction::EditNode],
        accessibility: VisualAccessibility {
            summary: "Claim and evidence graph".to_string(),
            table_fallback_ref: None,
            screen_reader_label: None,
        },
        source,
        manual_data: None,
    }
}

#[test]
fn ai_visual_draft_requires_acceptance_for_canonical_commit() {
    let draft = AiVisualDraft {
        id: VisualDraftId::from("draft_1"),
        source_reference_id: ReferenceId::from("ref_1"),
        source_attachment_id: None,
        source_ranges: Vec::new(),
        visual_spec: visual(VisualSource::LlmDerivedDraft),
        confidence: Some(0.7),
        warnings: Vec::new(),
        created_by: EngineRunId::from("run_1"),
        status: DraftStatus::Proposed,
        created_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
    };

    assert!(!draft.can_commit_to_canonical_content());
}

#[test]
fn ai_visual_engine_request_creates_uncommitted_draft() {
    let request = AiVisualRequest {
        source_reference_id: ReferenceId::from("ref_1"),
        source_attachment_id: Some(AttachmentId::from("pdf_1")),
        source_ranges: vec![SourceRange {
            kind: SourceRangeKind::PdfText,
            page: Some(3),
            start: Some(10),
            end: Some(80),
        }],
        requested_kind: ResearchVisualKind::ClaimEvidenceGraph,
        prompt: "Map claims and evidence from the selected section.".to_string(),
        library_id: "lib_1".to_string(),
    };

    let plan = build_ai_visual_engine_request(&request, None, false).expect("plan");
    let draft = ai_visual_draft_from_engine_spec(
        &plan,
        visual(VisualSource::UserAuthored),
        Some(0.61),
        vec![AiVisualWarning {
            code: "needs_review".to_string(),
            message: "User should verify evidence labels.".to_string(),
        }],
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    )
    .expect("draft");

    assert_eq!(plan.job.kind, "research.ai.visual_draft");
    assert_eq!(
        plan.engine_request.method,
        EngineMethod::ChatCompletionsCreate
    );
    assert!(plan.requires_user_approval);
    assert_eq!(draft.visual_spec.source, VisualSource::LlmDerivedDraft);
    assert_eq!(draft.source_ranges.len(), 1);
    assert!(!draft.can_commit_to_canonical_content());
}

#[test]
fn visual_state_actions_update_viewport_filters_and_timeline() {
    let state = VisualState::default()
        .apply_action(ResearchVisualAction::SetViewport {
            pan_x: 10.0,
            pan_y: -4.0,
            zoom: 100.0,
        })
        .apply_action(ResearchVisualAction::AddFilter {
            field: "year".to_string(),
            value: "2026".to_string(),
        })
        .apply_action(ResearchVisualAction::SetTimelineRange {
            start: 2030,
            end: 2020,
        });

    assert_eq!(state.viewport.zoom, 64.0);
    assert_eq!(state.active_filters.len(), 1);
    assert_eq!(state.timeline_range, Some((2020, 2030)));
}

#[test]
fn metadata_visual_builders_create_release_valid_specs() {
    let refs = vec![
        crate::reference_from_minimal_metadata(
            "ref_1",
            crate::ReferenceKind::JournalArticle,
            "Older",
            Some(2020),
            "2026-05-04T00:00:00Z",
        ),
        crate::reference_from_minimal_metadata(
            "ref_2",
            crate::ReferenceKind::JournalArticle,
            "Newer",
            Some(2026),
            "2026-05-04T00:00:00Z",
        ),
    ];

    let timeline = build_reference_timeline_visual(VisualSpecId::from("timeline"), "lib", &refs);
    let influence =
        build_reference_influence_graph_visual(VisualSpecId::from("influence"), "lib", &refs);

    timeline.validate_for_non_ai_release().unwrap();
    influence.validate_for_non_ai_release().unwrap();
    assert_eq!(timeline.state.timeline_range, Some((2020, 2026)));
    assert_eq!(influence.data_query.reference_ids.len(), 2);
}

#[test]
fn phase10_visual_builders_cover_overview_keyword_heatmap_citation_and_aggregation() {
    let now = Timestamp("2026-05-04T00:00:00Z".to_string());
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let mut snapshot = crate::new_research_library_snapshot(
        crate::LibraryId::from("lib"),
        "Library",
        "/tmp/lib",
        locale.clone(),
        now.clone(),
    );
    for index in 0..6 {
        let mut reference = crate::reference_from_minimal_metadata(
            format!("ref_{index}"),
            crate::ReferenceKind::JournalArticle,
            format!("Reference {index}"),
            Some(2020 + index as u16),
            now.0.clone(),
        );
        reference.tags.push(crate::ResearchTagId::from("nlp"));
        if index % 2 == 0 {
            reference.tags.push(crate::ResearchTagId::from("vision"));
        }
        reference.status = if index == 0 {
            ReadingStatus::Reading
        } else {
            ReadingStatus::Reviewed
        };
        snapshot.references.push(reference);
    }
    snapshot.attachments.push(crate::Attachment {
        id: AttachmentId::from("att_1"),
        reference_id: ReferenceId::from("ref_0"),
        kind: crate::AttachmentKind::Pdf,
        title: "Paper PDF".to_string(),
        stored_path: "attachments/att_1.pdf".to_string(),
        original_path: None,
        mime_type: "application/pdf".to_string(),
        size_bytes: 1024,
        content_hash: "hash".to_string(),
        page_count: Some(3),
        text_indexed: true,
        created_at: now.clone(),
        updated_at: now.clone(),
    });
    let annotations = vec![
        crate::PdfAnnotation {
            id: crate::AnnotationId::from("ann_1"),
            attachment_id: AttachmentId::from("att_1"),
            reference_id: ReferenceId::from("ref_0"),
            kind: crate::PdfAnnotationKind::Highlight,
            page: 1,
            rects: Vec::new(),
            color: crate::ColorRgba {
                r: 255,
                g: 220,
                b: 0,
                a: 255,
            },
            selected_text: Some("evidence".to_string()),
            note_markdown: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        },
        crate::PdfAnnotation {
            id: crate::AnnotationId::from("ann_2"),
            attachment_id: AttachmentId::from("att_1"),
            reference_id: ReferenceId::from("ref_0"),
            kind: crate::PdfAnnotationKind::Note,
            page: 1,
            rects: Vec::new(),
            color: crate::ColorRgba {
                r: 120,
                g: 180,
                b: 255,
                a: 255,
            },
            selected_text: None,
            note_markdown: Some("note".to_string()),
            created_at: now.clone(),
            updated_at: now.clone(),
        },
    ];
    let index_state = ResearchIndexState {
        stats: tench_search_core::IndexStats {
            indexed_items: 6,
            pending_items: 2,
            failed_items: 1,
            updated_at: Some(now.0.clone()),
        },
        pending_references: vec![ReferenceId::from("ref_0")],
        failed_jobs: Vec::new(),
    };

    let overview = build_library_overview_visual(
        VisualSpecId::from("overview"),
        &snapshot,
        Some(&index_state),
        3,
    );
    let keyword = build_reference_keyword_map_visual(
        VisualSpecId::from("keyword"),
        "lib",
        &snapshot.references,
        10,
    );
    let heatmap = build_pdf_annotation_heatmap_visual(
        VisualSpecId::from("heatmap"),
        "lib",
        AttachmentId::from("att_1"),
        &annotations,
    );
    let citation_visual = build_citation_warning_visual(
        VisualSpecId::from("citation"),
        "lib",
        &CitationRenderOutput {
            inline_citations: vec![crate::CitationPreview {
                reference_id: ReferenceId::from("ref_0"),
                citekey: crate::Citekey::from("smith2026"),
                rendered: "(Smith, 2026)".to_string(),
            }],
            bibliography: None,
            warnings: vec!["Reference ref_0 has no DOI.".to_string()],
            style_id_used: crate::CitationStyleId::from("apa"),
            locale_used: locale,
            output_format: crate::CitationOutputFormat::PlainText,
        },
    );

    for spec in [&overview, &keyword, &heatmap, &citation_visual] {
        spec.validate_for_non_ai_release().unwrap();
        let plan = build_research_visual_draw_plan(spec, false).expect("draw plan");
        assert!(!plan.table_fallback.is_empty(), "{:?}", spec.kind);
    }
    assert!(build_research_visual_draw_plan(&keyword, false)
        .expect("keyword draw")
        .commands
        .iter()
        .any(|command| matches!(command, ResearchVisualDrawCommand::GraphEdge { .. })));

    let bundle = aggregate_research_library_visuals("bundle", &snapshot, Some(&index_state), 3, 2);
    assert_eq!(bundle.reference_count, 6);
    assert_eq!(bundle.included_reference_count, 2);
    assert_eq!(bundle.omitted_reference_count, 4);
    assert!(bundle
        .visuals
        .iter()
        .all(|visual| visual.data_query.reference_ids.len() <= 2));
}

#[test]
fn manual_paper_analysis_visual_renders_without_ai() {
    let manual_data = ResearchVisualManualData {
        nodes: vec![
            ManualVisualNode {
                id: "method".to_string(),
                label: "Method".to_string(),
                group: Some("method".to_string()),
                weight: 1.0,
                reference_id: Some(ReferenceId::from("ref_1")),
                note_id: None,
            },
            ManualVisualNode {
                id: "result".to_string(),
                label: "Result".to_string(),
                group: Some("result".to_string()),
                weight: 2.0,
                reference_id: Some(ReferenceId::from("ref_1")),
                note_id: Some(ResearchNoteId::from("note_1")),
            },
        ],
        edges: vec![ManualVisualEdge {
            from: "method".to_string(),
            to: "result".to_string(),
            label: Some("produces".to_string()),
            strength: 0.8,
        }],
        cells: Vec::new(),
        events: Vec::new(),
    };

    let spec = build_manual_paper_analysis_visual(
        VisualSpecId::from("manual-flow"),
        "lib",
        ResearchVisualKind::MethodFlow,
        "Manual method flow",
        ResearchLocale::parse("en-US"),
        manual_data,
    )
    .expect("manual visual");
    let plan = build_research_visual_draw_plan(&spec, false).expect("draw plan");

    assert_eq!(spec.source, VisualSource::UserAuthored);
    assert!(spec.manual_data.is_some());
    assert_eq!(
        plan.commands
            .iter()
            .filter(|command| matches!(command, ResearchVisualDrawCommand::GraphNode { .. }))
            .count(),
        2
    );
    assert!(plan.commands.iter().any(|command| matches!(
        command,
        ResearchVisualDrawCommand::GraphEdge { from, to, .. }
        if from == "method" && to == "result"
    )));
    assert_eq!(
        plan.table_fallback_ref.as_deref(),
        Some("table://manual-analysis/method-flow")
    );
    assert!(plan.table_fallback.iter().any(|row| row.label == "Method"));
}

#[test]
fn manual_experiment_timeline_uses_event_data() {
    let spec = build_manual_paper_analysis_visual(
        VisualSpecId::from("manual-timeline"),
        "lib",
        ResearchVisualKind::ExperimentTimeline,
        "Experiment timeline",
        None,
        ResearchVisualManualData {
            nodes: Vec::new(),
            edges: Vec::new(),
            cells: Vec::new(),
            events: vec![
                ManualVisualEvent {
                    id: "prep".to_string(),
                    label: "Prepare samples".to_string(),
                    year: Some(2024),
                    position: 0.0,
                    reference_id: None,
                    note_id: None,
                },
                ManualVisualEvent {
                    id: "result".to_string(),
                    label: "Analyze results".to_string(),
                    year: Some(2026),
                    position: 1.0,
                    reference_id: Some(ReferenceId::from("ref_2")),
                    note_id: None,
                },
            ],
        },
    )
    .expect("timeline visual");

    let plan = build_research_visual_draw_plan(&spec, false).expect("draw plan");

    assert!(plan.commands.iter().any(|command| matches!(
        command,
        ResearchVisualDrawCommand::TimelineAxis {
            start_year: 2024,
            end_year: 2026
        }
    )));
    assert_eq!(
        plan.commands
            .iter()
            .filter(|command| matches!(command, ResearchVisualDrawCommand::TimelineBin { .. }))
            .count(),
        2
    );
    assert!(plan
        .table_fallback
        .iter()
        .any(|row| row.label == "Analyze results"));
}

#[test]
fn visual_draw_plan_limits_graph_and_preserves_accessibility() {
    let refs = (0..6)
        .map(|index| {
            crate::reference_from_minimal_metadata(
                format!("ref_{index}"),
                crate::ReferenceKind::JournalArticle,
                format!("Reference {index}"),
                Some(2020 + index as u16),
                "2026-05-04T00:00:00Z",
            )
        })
        .collect::<Vec<_>>();
    let mut influence =
        build_reference_influence_graph_visual(VisualSpecId::from("influence"), "lib", &refs);
    influence.data_query.aggregation.as_mut().unwrap().limit = Some(3);
    influence.state = influence.state.apply_action(ResearchVisualAction::Select {
        id: Some("ref_1".to_string()),
    });

    let plan = build_research_visual_draw_plan(&influence, true).expect("draw plan");

    assert!(plan.reduced_motion);
    assert_eq!(
        plan.table_fallback_ref.as_deref(),
        Some("table://references/influence")
    );
    assert_eq!(
        plan.commands
            .iter()
            .filter(|command| matches!(command, ResearchVisualDrawCommand::GraphNode { .. }))
            .count(),
        3
    );
    assert_eq!(
        plan.table_fallback
            .iter()
            .filter(|row| row.cells.iter().any(|cell| cell.key == "radius"))
            .count(),
        3
    );
    assert!(plan.commands.iter().any(|command| matches!(
        command,
        ResearchVisualDrawCommand::GraphNode {
            id,
            selected: true,
            ..
        } if id == "ref_1"
    )));
}

#[test]
fn manuscript_visuals_expose_readiness_citation_density_and_evidence_links() {
    let locale = crate::ResearchLocale::parse("en-US").expect("locale");
    let mut manuscript = crate::create_manuscript_from_template(
        crate::ManuscriptId::from("ms_1"),
        crate::LibraryId::from("lib_1"),
        crate::LocalizedField::plain("Draft"),
        crate::ManuscriptTemplateKind::JournalArticle,
        locale,
        crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let section_id = manuscript.outline.sections[0].id.clone();
    manuscript.outline.sections[0].status = crate::SectionStatus::Complete;
    manuscript.outline.sections[0]
        .cited_references
        .push(ReferenceId::from("ref_1"));
    manuscript.outline.sections[0]
        .source_notes
        .push(ResearchNoteId::from("note_1"));
    manuscript
        .citation_state
        .citations
        .push(crate::InlineCitation {
            id: crate::CitationId::from("cite_1"),
            reference_ids: vec![ReferenceId::from("ref_1")],
            section_id: Some(section_id),
            mode: crate::CitationMode::InText,
        });
    manuscript
        .citation_state
        .bibliography
        .reference_ids
        .push(ReferenceId::from("ref_1"));

    let readiness =
        build_manuscript_readiness_dashboard_visual(VisualSpecId::from("ready"), &manuscript);
    let density =
        build_manuscript_citation_density_visual(VisualSpecId::from("density"), &manuscript);
    let evidence =
        build_manuscript_claim_evidence_visual(VisualSpecId::from("evidence"), &manuscript);

    readiness.validate_for_non_ai_release().unwrap();
    density.validate_for_non_ai_release().unwrap();
    evidence.validate_for_non_ai_release().unwrap();
    assert_eq!(readiness.kind, ResearchVisualKind::ResultComparisonChart);
    assert_eq!(density.kind, ResearchVisualKind::EvidenceMatrix);
    assert_eq!(evidence.kind, ResearchVisualKind::ClaimEvidenceGraph);
    assert!(readiness
        .data_query
        .filters
        .iter()
        .any(|filter| filter.field == "sections_complete" && filter.value == "1"));
    assert_eq!(
        density.data_query.reference_ids,
        vec![ReferenceId::from("ref_1")]
    );
    assert_eq!(
        evidence.data_query.note_ids,
        vec![ResearchNoteId::from("note_1")]
    );
}
