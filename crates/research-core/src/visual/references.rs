use super::*;
use crate::ReferenceItem;

pub fn build_reference_timeline_visual(
    id: VisualSpecId,
    library_id: impl Into<String>,
    references: &[ReferenceItem],
) -> ResearchVisualSpec {
    let years = references
        .iter()
        .filter_map(|reference| reference.issued.year.map(i32::from))
        .collect::<Vec<_>>();
    let timeline_range = years
        .iter()
        .min()
        .zip(years.iter().max())
        .map(|(start, end)| (*start, *end));

    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::Timeline,
        title: VisualLocalizedText {
            value: "Reference timeline".to_string(),
            locale: None,
        },
        data_query: VisualQuery {
            library_id: library_id.into(),
            reference_ids: references
                .iter()
                .map(|reference| reference.id.clone())
                .collect(),
            note_ids: Vec::new(),
            filters: Vec::new(),
            aggregation: Some(VisualAggregation {
                group_by: "issued.year".to_string(),
                metric: "reference_count".to_string(),
                limit: None,
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::X,
                field: "issued.year".to_string(),
                metric_source: Some(MetricSource::ImportedMetadata),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
            VisualEncoding {
                channel: VisualChannel::Y,
                field: "reference_count".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Zero,
            },
        ],
        state: VisualState {
            timeline_range,
            ..VisualState::default()
        },
        animation: Some(VisualAnimation {
            layout_transition: true,
            filter_transition: true,
            timeline_playback: true,
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![
            VisualInteraction::Select,
            VisualInteraction::Pan,
            VisualInteraction::Zoom,
            VisualInteraction::ScrubTimeline,
        ],
        accessibility: VisualAccessibility {
            summary: format!(
                "Timeline of {} references by publication year",
                references.len()
            ),
            table_fallback_ref: Some("table://references/by-year".to_string()),
            screen_reader_label: Some("Reference timeline".to_string()),
        },
        source: VisualSource::MetadataDerived,
        manual_data: None,
    }
}

pub fn build_reference_influence_graph_visual(
    id: VisualSpecId,
    library_id: impl Into<String>,
    references: &[ReferenceItem],
) -> ResearchVisualSpec {
    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::InfluenceGraph,
        title: VisualLocalizedText {
            value: "Reference influence graph".to_string(),
            locale: None,
        },
        data_query: VisualQuery {
            library_id: library_id.into(),
            reference_ids: references
                .iter()
                .map(|reference| reference.id.clone())
                .collect(),
            note_ids: Vec::new(),
            filters: Vec::new(),
            aggregation: Some(VisualAggregation {
                group_by: "citation_or_metadata_link".to_string(),
                metric: "link_strength".to_string(),
                limit: Some(500),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::Size,
                field: "influence_score".to_string(),
                metric_source: Some(MetricSource::ImportedMetadata),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
            VisualEncoding {
                channel: VisualChannel::Color,
                field: "cluster".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
            VisualEncoding {
                channel: VisualChannel::Label,
                field: "title".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Hide,
            },
        ],
        state: VisualState::default(),
        animation: Some(VisualAnimation {
            layout_transition: true,
            filter_transition: true,
            timeline_playback: false,
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![
            VisualInteraction::Select,
            VisualInteraction::Pan,
            VisualInteraction::Zoom,
            VisualInteraction::ExpandCluster,
        ],
        accessibility: VisualAccessibility {
            summary: format!(
                "Influence graph prepared for {} references. Missing citation metadata is shown as unknown.",
                references.len()
            ),
            table_fallback_ref: Some("table://references/influence".to_string()),
            screen_reader_label: Some("Reference influence graph".to_string()),
        },
        source: VisualSource::CitationDerived,
        manual_data: None,
    }
}
