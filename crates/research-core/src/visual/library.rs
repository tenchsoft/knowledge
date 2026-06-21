use super::table::table_cell;
use super::*;
use std::collections::BTreeMap;

use crate::{
    AttachmentId, CitationRenderOutput, PdfAnnotation, ReadingStatus, ReferenceItem,
    ResearchIndexState, ResearchSnapshotV2,
};

pub fn build_library_overview_visual(
    id: VisualSpecId,
    snapshot: &ResearchSnapshotV2,
    index_state: Option<&ResearchIndexState>,
    duplicate_candidate_count: u32,
) -> ResearchVisualSpec {
    let pdf_count = snapshot
        .attachments
        .iter()
        .filter(|attachment| matches!(attachment.kind, crate::AttachmentKind::Pdf))
        .count();
    let reading_count = snapshot
        .references
        .iter()
        .filter(|reference| reference.status == ReadingStatus::Reading)
        .count();
    let reviewed_count = snapshot
        .references
        .iter()
        .filter(|reference| reference.status == ReadingStatus::Reviewed)
        .count();
    let unorganized_count = snapshot
        .references
        .iter()
        .filter(|reference| reference.tags.is_empty() && reference.collections.is_empty())
        .count();
    let failed_index_count = index_state
        .map(|state| state.stats.failed_items)
        .unwrap_or_default();
    let pending_index_count = index_state
        .map(|state| state.stats.pending_items)
        .unwrap_or_default();
    let cells = vec![
        overview_cell("references", snapshot.references.len()),
        overview_cell("pdf_attachments", pdf_count),
        overview_cell("reading", reading_count),
        overview_cell("reviewed", reviewed_count),
        overview_cell("unorganized", unorganized_count),
        overview_cell("index_pending", pending_index_count),
        overview_cell("index_failed", failed_index_count),
        overview_cell("duplicate_candidates", duplicate_candidate_count),
    ];

    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::BarChart,
        title: VisualLocalizedText {
            value: "Library overview".to_string(),
            locale: Some(snapshot.library.settings.default_locale.clone()),
        },
        data_query: VisualQuery {
            library_id: snapshot.library.id.as_str().to_string(),
            reference_ids: Vec::new(),
            note_ids: Vec::new(),
            filters: cells
                .iter()
                .map(|cell| VisualFilter {
                    field: cell.row.clone(),
                    value: cell.value.round().to_string(),
                })
                .collect(),
            aggregation: Some(VisualAggregation {
                group_by: "library_status_metric".to_string(),
                metric: "count".to_string(),
                limit: Some(32),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::X,
                field: "metric".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
            VisualEncoding {
                channel: VisualChannel::Y,
                field: "count".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Zero,
            },
            VisualEncoding {
                channel: VisualChannel::Label,
                field: "metric_label".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
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
        ],
        accessibility: VisualAccessibility {
            summary: format!(
                "Library overview for {} references, {pdf_count} PDFs, {reading_count} reading, {reviewed_count} reviewed, {unorganized_count} unorganized, {pending_index_count} pending index items, {failed_index_count} failed index items, and {duplicate_candidate_count} duplicate candidates.",
                snapshot.references.len()
            ),
            table_fallback_ref: Some(format!(
                "table://libraries/{}/overview",
                snapshot.library.id.as_str()
            )),
            screen_reader_label: Some("Library overview".to_string()),
        },
        source: VisualSource::MetadataDerived,
        manual_data: Some(ResearchVisualManualData {
            nodes: Vec::new(),
            edges: Vec::new(),
            cells,
            events: Vec::new(),
        }),
    }
}

pub fn build_reference_keyword_map_visual(
    id: VisualSpecId,
    library_id: impl Into<String>,
    references: &[ReferenceItem],
    limit: usize,
) -> ResearchVisualSpec {
    let library_id = library_id.into();
    let limit = limit.clamp(1, 500);
    let mut tag_counts = BTreeMap::<String, u32>::new();
    let mut pair_counts = BTreeMap::<(String, String), u32>::new();
    for reference in references {
        let mut tags = reference
            .tags
            .iter()
            .map(|tag| tag.as_str().trim().to_string())
            .filter(|tag| !tag.is_empty())
            .collect::<Vec<_>>();
        tags.sort();
        tags.dedup();
        for tag in &tags {
            *tag_counts.entry(tag.clone()).or_default() += 1;
        }
        for left_index in 0..tags.len() {
            for right_index in (left_index + 1)..tags.len() {
                let key = (tags[left_index].clone(), tags[right_index].clone());
                *pair_counts.entry(key).or_default() += 1;
            }
        }
    }
    let mut nodes = tag_counts
        .into_iter()
        .map(|(tag, count)| ManualVisualNode {
            id: format!("tag:{tag}"),
            label: tag,
            group: Some("tag".to_string()),
            weight: count as f32,
            reference_id: None,
            note_id: None,
        })
        .collect::<Vec<_>>();
    nodes.sort_by(|left, right| {
        right
            .weight
            .partial_cmp(&left.weight)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(left.label.cmp(&right.label))
    });
    nodes.truncate(limit);
    let kept = nodes
        .iter()
        .map(|node| node.label.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut edges = pair_counts
        .into_iter()
        .filter(|((left, right), _)| kept.contains(left.as_str()) && kept.contains(right.as_str()))
        .map(|((left, right), count)| ManualVisualEdge {
            from: format!("tag:{left}"),
            to: format!("tag:{right}"),
            label: Some("co-occurs".to_string()),
            strength: (count as f32 / references.len().max(1) as f32).clamp(0.05, 1.0),
        })
        .collect::<Vec<_>>();
    edges.sort_by(|left, right| {
        right
            .strength
            .partial_cmp(&left.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    edges.truncate(limit.saturating_mul(2));

    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::KeywordMap,
        title: VisualLocalizedText {
            value: "Keyword and tag map".to_string(),
            locale: None,
        },
        data_query: VisualQuery {
            library_id: library_id.clone(),
            reference_ids: Vec::new(),
            note_ids: Vec::new(),
            filters: vec![VisualFilter {
                field: "source_reference_count".to_string(),
                value: references.len().to_string(),
            }],
            aggregation: Some(VisualAggregation {
                group_by: "tag_co_occurrence".to_string(),
                metric: "co_occurrence_strength".to_string(),
                limit: Some(limit as u32),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::Size,
                field: "tag_count".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Zero,
            },
            VisualEncoding {
                channel: VisualChannel::Label,
                field: "tag_label".to_string(),
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
                "Keyword map with {} tags and {} co-occurrence links from {} references.",
                nodes.len(),
                edges.len(),
                references.len()
            ),
            table_fallback_ref: Some(format!("table://libraries/{library_id}/keyword-map")),
            screen_reader_label: Some("Keyword and tag map".to_string()),
        },
        source: VisualSource::MetadataDerived,
        manual_data: Some(ResearchVisualManualData {
            nodes,
            edges,
            cells: Vec::new(),
            events: Vec::new(),
        }),
    }
}

pub fn build_pdf_annotation_heatmap_visual(
    id: VisualSpecId,
    library_id: impl Into<String>,
    attachment_id: AttachmentId,
    annotations: &[PdfAnnotation],
) -> ResearchVisualSpec {
    let library_id = library_id.into();
    let mut page_counts = BTreeMap::<u32, u32>::new();
    let mut reference_ids = Vec::new();
    for annotation in annotations
        .iter()
        .filter(|annotation| annotation.attachment_id == attachment_id)
    {
        *page_counts.entry(annotation.page).or_default() += 1;
        if !reference_ids.contains(&annotation.reference_id) {
            reference_ids.push(annotation.reference_id.clone());
        }
    }
    let cells = page_counts
        .into_iter()
        .map(|(page, count)| ManualVisualCell {
            row: format!("page:{page}"),
            column: "annotation_count".to_string(),
            value: count as f32,
            label: Some(format!("Page {page}: {count} annotations")),
        })
        .collect::<Vec<_>>();

    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::AnnotationHeatmap,
        title: VisualLocalizedText {
            value: "PDF annotation heatmap".to_string(),
            locale: None,
        },
        data_query: VisualQuery {
            library_id: library_id.clone(),
            reference_ids,
            note_ids: Vec::new(),
            filters: vec![VisualFilter {
                field: "attachment_id".to_string(),
                value: attachment_id.as_str().to_string(),
            }],
            aggregation: Some(VisualAggregation {
                group_by: "pdf_page".to_string(),
                metric: "annotation_count".to_string(),
                limit: Some(1_000),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::Y,
                field: "page".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Hide,
            },
            VisualEncoding {
                channel: VisualChannel::Opacity,
                field: "annotation_count".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Zero,
            },
        ],
        state: VisualState::default(),
        animation: Some(VisualAnimation {
            layout_transition: false,
            filter_transition: true,
            timeline_playback: false,
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![
            VisualInteraction::Select,
            VisualInteraction::Pan,
            VisualInteraction::Zoom,
        ],
        accessibility: VisualAccessibility {
            summary: format!(
                "Annotation heatmap for attachment {} with {} annotated pages.",
                attachment_id.as_str(),
                cells.len()
            ),
            table_fallback_ref: Some(format!(
                "table://libraries/{library_id}/attachments/{}/annotation-heatmap",
                attachment_id.as_str()
            )),
            screen_reader_label: Some("PDF annotation heatmap".to_string()),
        },
        source: VisualSource::AnnotationDerived,
        manual_data: Some(ResearchVisualManualData {
            nodes: Vec::new(),
            edges: Vec::new(),
            cells,
            events: Vec::new(),
        }),
    }
}

pub fn build_citation_warning_visual(
    id: VisualSpecId,
    library_id: impl Into<String>,
    output: &CitationRenderOutput,
) -> ResearchVisualSpec {
    let library_id = library_id.into();
    let mut cells = output
        .warnings
        .iter()
        .enumerate()
        .map(|(index, warning)| ManualVisualCell {
            row: format!("warning:{}", index + 1),
            column: "message".to_string(),
            value: 1.0,
            label: Some(warning.clone()),
        })
        .collect::<Vec<_>>();
    if cells.is_empty() {
        cells.push(ManualVisualCell {
            row: "ready".to_string(),
            column: "message".to_string(),
            value: 0.0,
            label: Some("No citation warnings".to_string()),
        });
    }
    let reference_ids = output
        .inline_citations
        .iter()
        .map(|preview| preview.reference_id.clone())
        .collect::<Vec<_>>();

    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::EvidenceMatrix,
        title: VisualLocalizedText {
            value: "Citation preview warnings".to_string(),
            locale: Some(output.locale_used.clone()),
        },
        data_query: VisualQuery {
            library_id: library_id.clone(),
            reference_ids,
            note_ids: Vec::new(),
            filters: vec![
                VisualFilter {
                    field: "style_id".to_string(),
                    value: output.style_id_used.as_str().to_string(),
                },
                VisualFilter {
                    field: "warning_count".to_string(),
                    value: output.warnings.len().to_string(),
                },
            ],
            aggregation: Some(VisualAggregation {
                group_by: "citation_warning".to_string(),
                metric: "warning_count".to_string(),
                limit: Some(100),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::Y,
                field: "warning".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
            VisualEncoding {
                channel: VisualChannel::Color,
                field: "severity".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
        ],
        state: VisualState::default(),
        animation: Some(VisualAnimation {
            layout_transition: false,
            filter_transition: true,
            timeline_playback: false,
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![VisualInteraction::Select, VisualInteraction::Pan],
        accessibility: VisualAccessibility {
            summary: format!(
                "Citation preview has {} warnings for style {}.",
                output.warnings.len(),
                output.style_id_used.as_str()
            ),
            table_fallback_ref: Some(format!("table://libraries/{library_id}/citation-warnings")),
            screen_reader_label: Some("Citation preview warnings".to_string()),
        },
        source: VisualSource::CitationDerived,
        manual_data: Some(ResearchVisualManualData {
            nodes: Vec::new(),
            edges: Vec::new(),
            cells,
            events: Vec::new(),
        }),
    }
}

pub fn aggregate_research_library_visuals(
    id_prefix: impl AsRef<str>,
    snapshot: &ResearchSnapshotV2,
    index_state: Option<&ResearchIndexState>,
    duplicate_candidate_count: u32,
    max_references: usize,
) -> ResearchVisualAggregationBundle {
    let max_references = max_references.clamp(1, 1_000);
    let included = snapshot.references.len().min(max_references);
    let included_refs = snapshot
        .references
        .iter()
        .take(included)
        .cloned()
        .collect::<Vec<_>>();
    let id_prefix = id_prefix.as_ref();
    let overview = build_library_overview_visual(
        VisualSpecId::from(format!("{id_prefix}-overview")),
        snapshot,
        index_state,
        duplicate_candidate_count,
    );
    let timeline = build_reference_timeline_visual(
        VisualSpecId::from(format!("{id_prefix}-timeline")),
        snapshot.library.id.as_str(),
        &included_refs,
    );
    let keyword = build_reference_keyword_map_visual(
        VisualSpecId::from(format!("{id_prefix}-keyword-map")),
        snapshot.library.id.as_str(),
        &included_refs,
        250,
    );
    let summary_rows = vec![
        ResearchVisualTableRow {
            label: "references".to_string(),
            cells: vec![
                table_cell("total", snapshot.references.len()),
                table_cell("included", included),
                table_cell(
                    "omitted",
                    snapshot.references.len().saturating_sub(included),
                ),
            ],
        },
        ResearchVisualTableRow {
            label: "attachments".to_string(),
            cells: vec![table_cell("total", snapshot.attachments.len())],
        },
    ];
    ResearchVisualAggregationBundle {
        library_id: snapshot.library.id.as_str().to_string(),
        reference_count: snapshot.references.len(),
        included_reference_count: included,
        omitted_reference_count: snapshot.references.len().saturating_sub(included),
        visuals: vec![overview, timeline, keyword],
        summary_rows,
    }
}

fn overview_cell(row: impl Into<String>, value: impl IntoOverviewValue) -> ManualVisualCell {
    ManualVisualCell {
        row: row.into(),
        column: "count".to_string(),
        value: value.into_overview_value(),
        label: None,
    }
}

trait IntoOverviewValue {
    fn into_overview_value(self) -> f32;
}

impl IntoOverviewValue for usize {
    fn into_overview_value(self) -> f32 {
        self as f32
    }
}

impl IntoOverviewValue for u32 {
    fn into_overview_value(self) -> f32 {
        self as f32
    }
}

impl IntoOverviewValue for u64 {
    fn into_overview_value(self) -> f32 {
        self as f32
    }
}
