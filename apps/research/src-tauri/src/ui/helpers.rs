//! Standalone helper functions for the Research UI.

use tench_ui::kurbo::Vec2;
use tench_ui::prelude::*;

use super::state;

// ---------------------------------------------------------------------------
// PDF surface builder
// ---------------------------------------------------------------------------

pub fn build_pdf_surface_for_paper(
    paper: &state::Paper,
    pdf_state: &state::ResearchState,
) -> PdfSurface {
    let page_count = paper.pages.max(1);
    let current_page = pdf_state.pdf_current_page.clamp(1, page_count);
    let page_size = Size::new(612.0, 792.0);

    // Convert rendered image data from app shell into peniko::ImageData
    let image_data = pdf_state.pdf_page_image_data.as_ref().and_then(|img| {
        if img.pixels_rgba.is_empty() {
            return None;
        }
        let blob = tench_ui::peniko::Blob::new(std::sync::Arc::new(img.pixels_rgba.clone()));
        Some(tench_ui::peniko::ImageData {
            data: blob,
            format: tench_ui::peniko::ImageFormat::Rgba8,
            alpha_type: tench_ui::peniko::ImageAlphaType::AlphaPremultiplied,
            width: img.width_px,
            height: img.height_px,
        })
    });

    let page = PdfSurfacePage {
        page: current_page,
        label: format!("{}/{}", current_page, page_count),
        size: page_size,
        image_data,
    };

    // Convert annotations to overlays
    let mut overlays: Vec<PdfSurfaceOverlay> = pdf_state
        .pdf_annotations
        .iter()
        .filter(|ann| ann.page == current_page)
        .map(|ann| {
            let kind = match ann.kind {
                state::PdfAnnotationTool::Highlight => PdfSurfaceOverlayKind::Highlight,
                state::PdfAnnotationTool::Underline => PdfSurfaceOverlayKind::Underline,
                state::PdfAnnotationTool::Strikeout => PdfSurfaceOverlayKind::Strikeout,
                state::PdfAnnotationTool::StickyNote => PdfSurfaceOverlayKind::StickyNote,
                state::PdfAnnotationTool::TextSelect => PdfSurfaceOverlayKind::TextSelection,
                state::PdfAnnotationTool::None => PdfSurfaceOverlayKind::Highlight,
            };
            PdfSurfaceOverlay {
                id: ann.id.clone(),
                page: ann.page,
                rect: ann.rect,
                kind,
                color: ann.color,
                label: ann.text.clone(),
            }
        })
        .collect();

    // Add search result overlays for current page
    for (i, result) in pdf_state
        .pdf_search_results
        .iter()
        .enumerate()
        .filter(|(_, r)| r.page == current_page)
    {
        let active = pdf_state.pdf_search_active_index == Some(i);
        overlays.push(PdfSurfaceOverlay {
            id: format!("search-{i}"),
            page: result.page,
            rect: result.rect,
            kind: PdfSurfaceOverlayKind::SearchResult { active },
            color: Color::rgb8(0xFF, 0xEB, 0x3B),
            label: Some(result.text.clone()),
        });
    }

    PdfSurface::new(vec![page])
        .with_viewport(PdfSurfaceViewport {
            current_page,
            zoom: pdf_state.pdf_zoom,
            pan: Vec2::ZERO,
            page_gap: 24.0,
            theme: PdfSurfaceTheme::Paper,
        })
        .with_overlays(overlays)
        .with_accessibility_summary(format!(
            "annotations/search/rotation {:.0}",
            pdf_state.pdf_rotation
        ))
}

// ---------------------------------------------------------------------------
// Localization helpers
// ---------------------------------------------------------------------------

pub fn localized_research_status(catalog: &tench_app_core::I18nCatalog, status: &str) -> String {
    let key = match status {
        "All" => "research.status.all",
        "Read" => "research.status.read",
        "Reading" => "research.status.reading",
        "To Read" => "research.status.to_read",
        "Archived" => "research.status.archived",
        "Ready" => "research.status.ready",
        "Import queued" => "research.status.import_queued",
        _ => return status.to_string(),
    };
    catalog.resolve(key).unwrap_or(status).to_string()
}

pub fn localized_research_reader_mode(catalog: &tench_app_core::I18nCatalog, mode: &str) -> String {
    let key = match mode {
        "detail" => "research.reader.detail",
        "pdf" => "research.reader.pdf",
        "importing" => "research.reader.importing",
        _ => return mode.to_string(),
    };
    catalog.resolve(key).unwrap_or(mode).to_string()
}

// ---------------------------------------------------------------------------
// Visual surface command translation
// ---------------------------------------------------------------------------

pub fn research_visual_surface_commands(
    plan: &tench_research_core::ResearchVisualDrawPlan,
) -> Vec<VisualSurfaceCommand> {
    let mut commands = Vec::new();
    for (index, command) in plan.commands.iter().enumerate() {
        match command {
            tench_research_core::ResearchVisualDrawCommand::TimelineAxis {
                start_year,
                end_year,
            } => commands.push(VisualSurfaceCommand {
                id: "timeline-axis".to_string(),
                kind: VisualSurfaceCommandKind::Axis2d {
                    x_label: format!("{start_year}-{end_year}"),
                    y_label: "refs".to_string(),
                },
                label: None,
                color: Color::rgb8(0x8A, 0x8A, 0x8A),
            }),
            tench_research_core::ResearchVisualDrawCommand::TimelineBin {
                id,
                count,
                selected,
                ..
            } => {
                let x = 0.10 + (index % 8) as f64 * 0.10;
                let height = (0.12 + f64::from(*count).min(8.0) * 0.04).min(0.40);
                commands.push(VisualSurfaceCommand {
                    id: id.clone(),
                    kind: VisualSurfaceCommandKind::Shape2d {
                        unit_rect: Rect::new(x, 0.72 - height, x + 0.06, 0.72),
                        progress: 1.0,
                        selected: *selected,
                    },
                    label: None,
                    color: Color::rgb8(0x60, 0xA5, 0xFA),
                });
            }
            tench_research_core::ResearchVisualDrawCommand::GraphNode {
                id,
                label,
                selected,
                hovered,
                ..
            } => {
                let column = index % 4;
                let row = index / 4;
                let x = 0.16 + column as f64 * 0.18;
                let y = 0.24 + (row % 3) as f64 * 0.16;
                commands.push(VisualSurfaceCommand {
                    id: id.clone(),
                    kind: VisualSurfaceCommandKind::Shape2d {
                        unit_rect: Rect::new(x, y, x + 0.08, y + 0.08),
                        progress: if *hovered { 1.0 } else { 0.65 },
                        selected: *selected,
                    },
                    label: Some(label.clone()),
                    color: Color::rgb8(0x34, 0xD3, 0x99),
                });
            }
            tench_research_core::ResearchVisualDrawCommand::MatrixCell {
                row,
                column,
                value,
                selected,
            } => {
                let x = 0.12 + (index % 5) as f64 * 0.12;
                let y = 0.24 + (index / 5 % 4) as f64 * 0.12;
                commands.push(VisualSurfaceCommand {
                    id: format!("{row}-{column}"),
                    kind: VisualSurfaceCommandKind::Shape2d {
                        unit_rect: Rect::new(x, y, x + 0.10, y + 0.10),
                        progress: *value,
                        selected: *selected,
                    },
                    label: None,
                    color: Color::rgb8(0xA7, 0x8B, 0xFA),
                });
            }
            tench_research_core::ResearchVisualDrawCommand::OverlayRegion {
                id,
                label,
                opacity,
            } => commands.push(VisualSurfaceCommand {
                id: id.clone(),
                kind: VisualSurfaceCommandKind::Shape2d {
                    unit_rect: Rect::new(0.14, 0.24, 0.86, 0.70),
                    progress: *opacity,
                    selected: false,
                },
                label: Some(label.clone()),
                color: Color::rgb8(0xF5, 0x9E, 0x0B),
            }),
            tench_research_core::ResearchVisualDrawCommand::TextLabel { id, label } => {
                commands.push(VisualSurfaceCommand {
                    id: id.clone(),
                    kind: VisualSurfaceCommandKind::TextLabel {
                        unit_position: Point::new(0.12, 0.18 + (index % 6) as f64 * 0.08),
                        text: label.clone(),
                    },
                    label: None,
                    color: Color::rgb8(0xD4, 0xD4, 0xD4),
                });
            }
            tench_research_core::ResearchVisualDrawCommand::GraphEdge { .. } => {}
        }
    }
    commands
}
