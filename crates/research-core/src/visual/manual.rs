use super::*;
use crate::ResearchLocale;

pub fn build_manual_paper_analysis_visual(
    id: VisualSpecId,
    library_id: impl Into<String>,
    kind: ResearchVisualKind,
    title: impl Into<String>,
    locale: Option<ResearchLocale>,
    manual_data: ResearchVisualManualData,
) -> Result<ResearchVisualSpec, String> {
    if !is_manual_analysis_visual_kind(kind) {
        return Err(format!(
            "manual paper analysis visual does not support {:?}",
            kind
        ));
    }
    manual_data.validate_for_kind(kind)?;
    let title = title.into();
    let spec = ResearchVisualSpec {
        id,
        kind,
        title: VisualLocalizedText {
            value: title.clone(),
            locale,
        },
        data_query: VisualQuery {
            library_id: library_id.into(),
            reference_ids: manual_data
                .nodes
                .iter()
                .filter_map(|node| node.reference_id.clone())
                .chain(
                    manual_data
                        .events
                        .iter()
                        .filter_map(|event| event.reference_id.clone()),
                )
                .collect(),
            note_ids: manual_data
                .nodes
                .iter()
                .filter_map(|node| node.note_id.clone())
                .chain(
                    manual_data
                        .events
                        .iter()
                        .filter_map(|event| event.note_id.clone()),
                )
                .collect(),
            filters: Vec::new(),
            aggregation: Some(VisualAggregation {
                group_by: "manual_analysis".to_string(),
                metric: "manual_weight".to_string(),
                limit: Some(500),
            }),
        },
        encodings: manual_analysis_encodings(kind),
        state: VisualState::default(),
        animation: Some(VisualAnimation {
            layout_transition: true,
            filter_transition: true,
            timeline_playback: matches!(kind, ResearchVisualKind::ExperimentTimeline),
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![
            VisualInteraction::Select,
            VisualInteraction::Pan,
            VisualInteraction::Zoom,
            VisualInteraction::EditNode,
            VisualInteraction::EditEdge,
        ],
        accessibility: VisualAccessibility {
            summary: format!(
                "{title} manual visual with {} nodes, {} edges, {} cells, and {} events.",
                manual_data.nodes.len(),
                manual_data.edges.len(),
                manual_data.cells.len(),
                manual_data.events.len()
            ),
            table_fallback_ref: Some(format!("table://manual-analysis/{}", kind_label(kind))),
            screen_reader_label: Some(title),
        },
        source: VisualSource::UserAuthored,
        manual_data: Some(manual_data),
    };
    spec.validate_for_non_ai_release()?;
    Ok(spec)
}

fn is_manual_analysis_visual_kind(kind: ResearchVisualKind) -> bool {
    matches!(
        kind,
        ResearchVisualKind::PaperAnalysisMap
            | ResearchVisualKind::MethodFlow
            | ResearchVisualKind::ClaimEvidenceGraph
            | ResearchVisualKind::ExperimentTimeline
            | ResearchVisualKind::ResultComparisonChart
            | ResearchVisualKind::EvidenceMatrix
    )
}

fn manual_analysis_encodings(kind: ResearchVisualKind) -> Vec<VisualEncoding> {
    match kind {
        ResearchVisualKind::ResultComparisonChart | ResearchVisualKind::EvidenceMatrix => vec![
            VisualEncoding {
                channel: VisualChannel::X,
                field: "manual_column".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Hide,
            },
            VisualEncoding {
                channel: VisualChannel::Y,
                field: "manual_row".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Hide,
            },
            VisualEncoding {
                channel: VisualChannel::Color,
                field: "manual_value".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Zero,
            },
        ],
        ResearchVisualKind::ExperimentTimeline => vec![
            VisualEncoding {
                channel: VisualChannel::X,
                field: "manual_year".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
            VisualEncoding {
                channel: VisualChannel::Label,
                field: "manual_event_label".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Hide,
            },
        ],
        _ => vec![
            VisualEncoding {
                channel: VisualChannel::Size,
                field: "manual_weight".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Zero,
            },
            VisualEncoding {
                channel: VisualChannel::Label,
                field: "manual_label".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Hide,
            },
        ],
    }
}

fn kind_label(kind: ResearchVisualKind) -> &'static str {
    match kind {
        ResearchVisualKind::BarChart => "bar-chart",
        ResearchVisualKind::Histogram => "histogram",
        ResearchVisualKind::Timeline => "timeline",
        ResearchVisualKind::InfluenceGraph => "influence-graph",
        ResearchVisualKind::KeywordMap => "keyword-map",
        ResearchVisualKind::EvidenceMatrix => "evidence-matrix",
        ResearchVisualKind::PdfOverlay => "pdf-overlay",
        ResearchVisualKind::AnnotationHeatmap => "annotation-heatmap",
        ResearchVisualKind::PaperAnalysisMap => "paper-analysis-map",
        ResearchVisualKind::MethodFlow => "method-flow",
        ResearchVisualKind::ClaimEvidenceGraph => "claim-evidence-graph",
        ResearchVisualKind::ExperimentTimeline => "experiment-timeline",
        ResearchVisualKind::ResultComparisonChart => "result-comparison-chart",
    }
}
