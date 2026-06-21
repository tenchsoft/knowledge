use super::table::table_cell;
use super::*;

pub fn build_research_visual_draw_plan(
    spec: &ResearchVisualSpec,
    reduced_motion: bool,
) -> Result<ResearchVisualDrawPlan, String> {
    spec.validate_for_non_ai_release()?;
    let mut commands = Vec::new();
    let used_manual_data = append_manual_visual_commands(&mut commands, spec);
    if !used_manual_data {
        match spec.kind {
            ResearchVisualKind::Timeline => {
                let (start_year, end_year) = spec.state.timeline_range.unwrap_or((0, 0));
                commands.push(ResearchVisualDrawCommand::TimelineAxis {
                    start_year,
                    end_year,
                });
                let bin_count = spec.data_query.reference_ids.len().min(500);
                for (index, reference_id) in spec
                    .data_query
                    .reference_ids
                    .iter()
                    .take(bin_count)
                    .enumerate()
                {
                    let year = if start_year == 0 && end_year == 0 {
                        0
                    } else {
                        start_year + (index as i32 % (end_year - start_year + 1).max(1))
                    };
                    commands.push(ResearchVisualDrawCommand::TimelineBin {
                        id: reference_id.as_str().to_string(),
                        year,
                        count: 1,
                        selected: spec.state.selected_id.as_deref() == Some(reference_id.as_str()),
                    });
                }
            }
            ResearchVisualKind::InfluenceGraph
            | ResearchVisualKind::KeywordMap
            | ResearchVisualKind::ClaimEvidenceGraph => {
                let limit = spec
                    .data_query
                    .aggregation
                    .as_ref()
                    .and_then(|aggregation| aggregation.limit)
                    .unwrap_or(500) as usize;
                let visible_ids = spec
                    .data_query
                    .reference_ids
                    .iter()
                    .take(limit)
                    .collect::<Vec<_>>();
                for reference_id in &visible_ids {
                    commands.push(ResearchVisualDrawCommand::GraphNode {
                        id: reference_id.as_str().to_string(),
                        label: reference_id.as_str().to_string(),
                        radius: if spec.state.selected_id.as_deref() == Some(reference_id.as_str())
                        {
                            9.0
                        } else {
                            6.0
                        },
                        selected: spec.state.selected_id.as_deref() == Some(reference_id.as_str()),
                        hovered: spec.state.hovered_id.as_deref() == Some(reference_id.as_str()),
                    });
                }
                for pair in visible_ids.windows(2) {
                    commands.push(ResearchVisualDrawCommand::GraphEdge {
                        from: pair[0].as_str().to_string(),
                        to: pair[1].as_str().to_string(),
                        strength: 0.5,
                    });
                }
            }
            ResearchVisualKind::EvidenceMatrix | ResearchVisualKind::ResultComparisonChart => {
                for reference_id in &spec.data_query.reference_ids {
                    commands.push(ResearchVisualDrawCommand::MatrixCell {
                        row: reference_id.as_str().to_string(),
                        column: spec
                            .data_query
                            .aggregation
                            .as_ref()
                            .map(|aggregation| aggregation.metric.clone())
                            .unwrap_or_else(|| "value".to_string()),
                        value: 1.0,
                        selected: spec.state.selected_id.as_deref() == Some(reference_id.as_str()),
                    });
                }
            }
            ResearchVisualKind::PdfOverlay
            | ResearchVisualKind::AnnotationHeatmap
            | ResearchVisualKind::PaperAnalysisMap
            | ResearchVisualKind::MethodFlow
            | ResearchVisualKind::ExperimentTimeline
            | ResearchVisualKind::BarChart
            | ResearchVisualKind::Histogram => {
                commands.push(ResearchVisualDrawCommand::OverlayRegion {
                    id: spec.id.as_str().to_string(),
                    label: spec.title.value.clone(),
                    opacity: if reduced_motion { 0.85 } else { 1.0 },
                });
            }
        }
    }
    if commands.is_empty() {
        commands.push(ResearchVisualDrawCommand::TextLabel {
            id: spec.id.as_str().to_string(),
            label: "No data available".to_string(),
        });
    }

    let table_fallback = research_visual_table_fallback(&commands);

    Ok(ResearchVisualDrawPlan {
        visual_id: spec.id.clone(),
        kind: spec.kind,
        title: spec.title.value.clone(),
        viewport: spec.state.viewport,
        commands,
        accessibility_summary: spec.accessibility.summary.clone(),
        table_fallback_ref: spec.accessibility.table_fallback_ref.clone(),
        table_fallback,
        reduced_motion,
    })
}

pub fn research_visual_table_fallback(
    commands: &[ResearchVisualDrawCommand],
) -> Vec<ResearchVisualTableRow> {
    commands
        .iter()
        .map(|command| match command {
            ResearchVisualDrawCommand::TimelineAxis {
                start_year,
                end_year,
            } => ResearchVisualTableRow {
                label: "Timeline axis".to_string(),
                cells: vec![
                    table_cell("start_year", start_year),
                    table_cell("end_year", end_year),
                ],
            },
            ResearchVisualDrawCommand::TimelineBin {
                id,
                year,
                count,
                selected,
            } => ResearchVisualTableRow {
                label: id.clone(),
                cells: vec![
                    table_cell("year", year),
                    table_cell("count", count),
                    table_cell("selected", selected),
                ],
            },
            ResearchVisualDrawCommand::GraphNode {
                id,
                label,
                radius,
                selected,
                hovered,
            } => ResearchVisualTableRow {
                label: label.clone(),
                cells: vec![
                    table_cell("id", id),
                    table_cell("radius", format!("{radius:.2}")),
                    table_cell("selected", selected),
                    table_cell("hovered", hovered),
                ],
            },
            ResearchVisualDrawCommand::GraphEdge { from, to, strength } => ResearchVisualTableRow {
                label: format!("{from} -> {to}"),
                cells: vec![
                    table_cell("from", from),
                    table_cell("to", to),
                    table_cell("strength", format!("{strength:.2}")),
                ],
            },
            ResearchVisualDrawCommand::MatrixCell {
                row,
                column,
                value,
                selected,
            } => ResearchVisualTableRow {
                label: format!("{row} / {column}"),
                cells: vec![
                    table_cell("row", row),
                    table_cell("column", column),
                    table_cell("value", format!("{value:.2}")),
                    table_cell("selected", selected),
                ],
            },
            ResearchVisualDrawCommand::OverlayRegion { id, label, opacity } => {
                ResearchVisualTableRow {
                    label: label.clone(),
                    cells: vec![
                        table_cell("id", id),
                        table_cell("opacity", format!("{opacity:.2}")),
                    ],
                }
            }
            ResearchVisualDrawCommand::TextLabel { id, label } => ResearchVisualTableRow {
                label: label.clone(),
                cells: vec![table_cell("id", id), table_cell("text", label)],
            },
        })
        .collect()
}

fn append_manual_visual_commands(
    commands: &mut Vec<ResearchVisualDrawCommand>,
    spec: &ResearchVisualSpec,
) -> bool {
    let Some(manual_data) = &spec.manual_data else {
        return false;
    };

    match spec.kind {
        ResearchVisualKind::PaperAnalysisMap
        | ResearchVisualKind::MethodFlow
        | ResearchVisualKind::ClaimEvidenceGraph
        | ResearchVisualKind::InfluenceGraph
        | ResearchVisualKind::KeywordMap => {
            for node in &manual_data.nodes {
                commands.push(ResearchVisualDrawCommand::GraphNode {
                    id: node.id.clone(),
                    label: node.label.clone(),
                    radius: 5.0 + node.weight.clamp(0.0, 5.0),
                    selected: spec.state.selected_id.as_deref() == Some(node.id.as_str()),
                    hovered: spec.state.hovered_id.as_deref() == Some(node.id.as_str()),
                });
            }
            for edge in &manual_data.edges {
                commands.push(ResearchVisualDrawCommand::GraphEdge {
                    from: edge.from.clone(),
                    to: edge.to.clone(),
                    strength: edge.strength.clamp(0.0, 1.0),
                });
            }
        }
        ResearchVisualKind::EvidenceMatrix
        | ResearchVisualKind::ResultComparisonChart
        | ResearchVisualKind::BarChart
        | ResearchVisualKind::Histogram
        | ResearchVisualKind::AnnotationHeatmap => {
            for cell in &manual_data.cells {
                commands.push(ResearchVisualDrawCommand::MatrixCell {
                    row: cell.row.clone(),
                    column: cell.column.clone(),
                    value: cell.value.clamp(0.0, 1.0),
                    selected: spec.state.selected_id.as_deref() == Some(cell.row.as_str()),
                });
            }
        }
        ResearchVisualKind::ExperimentTimeline | ResearchVisualKind::Timeline => {
            let years = manual_data
                .events
                .iter()
                .filter_map(|event| event.year)
                .collect::<Vec<_>>();
            let range = spec
                .state
                .timeline_range
                .or_else(|| {
                    years
                        .iter()
                        .min()
                        .zip(years.iter().max())
                        .map(|(a, b)| (*a, *b))
                })
                .unwrap_or((0, 0));
            commands.push(ResearchVisualDrawCommand::TimelineAxis {
                start_year: range.0,
                end_year: range.1,
            });
            for event in &manual_data.events {
                commands.push(ResearchVisualDrawCommand::TimelineBin {
                    id: event.id.clone(),
                    year: event.year.unwrap_or_else(|| {
                        range.0
                            + ((range.1 - range.0) as f32 * event.position.clamp(0.0, 1.0)) as i32
                    }),
                    count: 1,
                    selected: spec.state.selected_id.as_deref() == Some(event.id.as_str()),
                });
                commands.push(ResearchVisualDrawCommand::TextLabel {
                    id: format!("label-{}", event.id),
                    label: event.label.clone(),
                });
            }
        }
        _ => {}
    }

    !commands.is_empty()
}
