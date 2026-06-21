use super::super::{helpers, paper_list, state, ResearchApp, HEADER_H};
use tench_ui::kurbo::Vec2;
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

pub(super) struct InspectorPanelContext<'a> {
    pub(super) size: Size,
    pub(super) theme: &'a Theme,
    pub(super) center_right: f64,
    pub(super) right_w: f64,
    pub(super) spacing: f64,
    pub(super) spacing_large: f64,
    pub(super) paper: &'a state::Paper,
}

impl ResearchApp {
    pub(super) fn paint_inspector_panel(
        &self,
        p: &mut Painter<'_>,
        context: InspectorPanelContext<'_>,
    ) {
        let InspectorPanelContext {
            size,
            theme,
            center_right,
            right_w,
            spacing,
            spacing_large,
            paper,
        } = context;
        let t = |key: &'static str| self.i18n.resolve(key).unwrap_or(key);
        let header_h = HEADER_H;
        let rx = center_right;
        let mut ry = header_h + spacing_large;

        // Inspector tabs
        let mut tab_x = rx + spacing;
        let inspector_tabs = [
            t("research.inspector.notes_short"),
            t("research.inspector.summary_short"),
            t("research.inspector.qa_short"),
            t("research.inspector.visual_short"),
            t("research.inspector.write_short"),
            t("research.inspector.cite_short"),
        ];
        for (i, label) in inspector_tabs.iter().enumerate() {
            let tw = 36.0;
            let tab_rect = Rect::new(tab_x, ry - 10.0, tab_x + tw, ry + 6.0);
            if i == self.state.active_inspector_tab {
                p.fill_rounded_rect(tab_rect, theme.primary, theme.border_radius);
                p.draw_text(
                    label,
                    tab_x + 8.0,
                    ry,
                    theme.on_primary,
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    false,
                );
            } else {
                p.draw_text(
                    label,
                    tab_x + 8.0,
                    ry,
                    theme.secondary,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
            }
            tab_x += tw + spacing;
        }
        ry += 24.0;

        // Inspector content
        match self.state.active_inspector_tab {
            0 => {
                // Notes
                p.draw_text(
                    t("research.inspector.notes"),
                    rx + spacing,
                    ry,
                    theme.on_background,
                    theme.font_size,
                    FontWeight::BOLD,
                    false,
                );
                ry += 20.0;
                p.draw_text(
                    &paper.notes,
                    rx + spacing,
                    ry,
                    theme.on_surface,
                    theme.font_size,
                    FontWeight::NORMAL,
                    false,
                );
            }
            1 => {
                p.draw_text(
                    t("research.inspector.summary"),
                    rx + spacing,
                    ry,
                    theme.on_background,
                    theme.font_size,
                    FontWeight::BOLD,
                    false,
                );
                ry += 20.0;
                p.draw_text(
                    t("research.inspector.summary_text"),
                    rx + spacing,
                    ry,
                    theme.on_surface,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
            }
            2 => {
                p.draw_text(
                    t("research.inspector.qa"),
                    rx + spacing,
                    ry,
                    theme.on_background,
                    theme.font_size,
                    FontWeight::BOLD,
                    false,
                );
                ry += 20.0;

                // Chat messages
                for (role, message) in &self.state.analysis_messages {
                    let role_color = match role.as_str() {
                        "user" => theme.primary,
                        "assistant" => Color::rgb8(0x34, 0xD3, 0x99),
                        _ => theme.secondary,
                    };
                    p.draw_text(
                        role,
                        rx + spacing,
                        ry,
                        role_color,
                        theme.font_size_small,
                        FontWeight::BOLD,
                        false,
                    );
                    let wrapped = paper_list::wrap_text(message, 40);
                    for line in &wrapped {
                        ry += 14.0;
                        p.draw_text(
                            line,
                            rx + spacing,
                            ry,
                            theme.on_surface,
                            theme.font_size_small,
                            FontWeight::NORMAL,
                            false,
                        );
                    }
                    ry += 20.0;
                }

                // Q&A input field
                ry += spacing;
                let input_w = right_w - 80.0;
                let input_rect = Rect::new(
                    rx + spacing,
                    ry - 10.0,
                    rx + spacing + input_w,
                    ry - 10.0 + theme.input_height,
                );
                let is_qa_focused = self.state.focus == state::FocusTarget::QaInput;
                p.fill_rounded_rect(input_rect, theme.background, theme.border_radius);
                p.stroke_rounded_rect(
                    input_rect,
                    if is_qa_focused {
                        theme.primary
                    } else {
                        theme.border
                    },
                    if is_qa_focused { 2.0 } else { 1.0 },
                    theme.border_radius,
                );
                let qa_display = if self.state.qa_input.is_empty() {
                    "Ask about this paper..."
                } else {
                    &self.state.qa_input
                };
                p.draw_text(
                    qa_display,
                    rx + spacing + 8.0,
                    ry + 6.0,
                    if self.state.qa_input.is_empty() {
                        theme.disabled
                    } else {
                        theme.on_background
                    },
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );

                // Send button
                let send_x = rx + spacing + input_w + 4.0;
                let send_rect = Rect::new(
                    send_x,
                    ry - 10.0,
                    send_x + 36.0,
                    ry - 10.0 + theme.input_height,
                );
                p.fill_rounded_rect(send_rect, theme.primary, theme.border_radius);
                p.draw_text(
                    "Send",
                    send_x + 4.0,
                    ry + 6.0,
                    theme.on_primary,
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    false,
                );

                // Quick action buttons
                ry += theme.input_height + spacing;
                let quick_actions = ["Summarize", "Key Points", "Limitations"];
                let mut qa_x = rx + spacing;
                for action in &quick_actions {
                    let action_w = 72.0;
                    let action_rect = Rect::new(qa_x, ry - 8.0, qa_x + action_w, ry + 6.0);
                    p.fill_rounded_rect(action_rect, theme.surface, theme.border_radius);
                    p.stroke_rounded_rect(action_rect, theme.border, 1.0, theme.border_radius);
                    p.draw_text(
                        action,
                        qa_x + 6.0,
                        ry,
                        theme.on_surface,
                        theme.font_size_small - 1.0,
                        FontWeight::NORMAL,
                        false,
                    );
                    qa_x += action_w + 4.0;
                }
            }
            3 => {
                p.draw_text(
                    t("research.inspector.visuals"),
                    rx + spacing,
                    ry,
                    theme.on_background,
                    theme.font_size,
                    FontWeight::BOLD,
                    false,
                );
                ry += 20.0;
                for line in &self.state.visual_summary_lines {
                    p.draw_text(
                        line,
                        rx + spacing,
                        ry,
                        theme.on_surface,
                        theme.font_size_small,
                        FontWeight::NORMAL,
                        false,
                    );
                    ry += 18.0;
                }
                if let Some(plan) = &self.state.visual_draw_plan {
                    let visual_rect = Rect::new(
                        rx + spacing,
                        ry + spacing,
                        size.width - spacing,
                        (ry + 150.0).min(size.height - spacing),
                    );
                    let surface =
                        VisualSurface::new(helpers::research_visual_surface_commands(plan))
                            .with_viewport(VisualSurfaceViewport {
                                zoom: f64::from(plan.viewport.zoom.max(0.2)),
                                pan: Vec2::new(
                                    f64::from(plan.viewport.pan_x),
                                    f64::from(plan.viewport.pan_y),
                                ),
                                timeline_position: 0.0,
                                reduced_motion: plan.reduced_motion,
                            })
                            .with_accessibility_summary(plan.accessibility_summary.clone());
                    surface.paint_in_rect(p, visual_rect, theme);
                }
            }
            4 => {
                // ── Outline heading ──────────────────────────────────────
                p.draw_text(
                    t("research.manuscript.outline"),
                    rx + spacing,
                    ry,
                    theme.on_background,
                    theme.font_size,
                    FontWeight::BOLD,
                    false,
                );
                ry += 22.0;

                // ── Summary lines (readiness dashboard) ────────────────
                for line in &self.state.manuscript_summary_lines {
                    p.draw_text(
                        line,
                        rx + spacing,
                        ry,
                        theme.on_surface,
                        theme.font_size_small,
                        FontWeight::NORMAL,
                        false,
                    );
                    ry += 16.0;
                }
                ry += 6.0;

                // ── Section list (outline) ─────────────────────────────
                if self.state.manuscript_sections.is_empty() {
                    p.draw_text(
                        t("research.manuscript.no_sections"),
                        rx + spacing,
                        ry,
                        theme.on_surface,
                        theme.font_size_small,
                        FontWeight::NORMAL,
                        true,
                    );
                    ry += 20.0;
                } else {
                    for (si, section) in self.state.manuscript_sections.iter().enumerate() {
                        let is_active = self.state.manuscript_active_section == Some(si);
                        let section_rect = Rect::new(
                            rx + spacing,
                            ry - 2.0,
                            size.width - spacing * 2.0,
                            ry + 18.0,
                        );
                        if is_active {
                            p.fill_rounded_rect(
                                section_rect,
                                Color::rgba8(
                                    theme.primary.r(),
                                    theme.primary.g(),
                                    theme.primary.b(),
                                    0x1F,
                                ),
                                theme.border_radius,
                            );
                        }
                        let bullet = if is_active { "▸ " } else { "▪ " };
                        let cite_count = section.citations.len();
                        let label = if cite_count > 0 {
                            format!("{}{} ({} cite)", bullet, section.title, cite_count)
                        } else {
                            format!("{}{}", bullet, section.title)
                        };
                        p.draw_text(
                            &label,
                            rx + spacing + 4.0,
                            ry,
                            if is_active {
                                theme.primary
                            } else {
                                theme.on_surface
                            },
                            theme.font_size_small,
                            if is_active {
                                FontWeight::MEDIUM
                            } else {
                                FontWeight::NORMAL
                            },
                            false,
                        );
                        ry += 20.0;

                        // Show content preview for active section
                        if is_active && !section.content.is_empty() {
                            let preview: String = section.content.chars().take(80).collect();
                            p.draw_text(
                                &preview,
                                rx + spacing + 16.0,
                                ry,
                                theme.on_surface,
                                theme.font_size_small,
                                FontWeight::NORMAL,
                                true,
                            );
                            ry += 16.0;
                        }
                    }
                }

                // ── Add section button ─────────────────────────────────
                ry += 4.0;
                let add_btn_rect = Rect::new(rx + spacing, ry, rx + spacing + 80.0, ry + 24.0);
                p.fill_rounded_rect(add_btn_rect, theme.surface, theme.border_radius);
                p.stroke_rounded_rect(add_btn_rect, theme.border, 1.0, theme.border_radius);
                p.draw_text(
                    t("research.manuscript.add_section"),
                    rx + spacing + 8.0,
                    ry + 6.0,
                    theme.primary,
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    false,
                );
                ry += 32.0;

                // ── Cite-while-you-write ───────────────────────────────
                if self.state.manuscript_active_section.is_some() {
                    let cite_search_y = ry;
                    let cite_search_rect = Rect::new(
                        rx + spacing,
                        cite_search_y,
                        size.width - spacing * 2.0,
                        cite_search_y + 24.0,
                    );
                    let cite_focused = self.state.focus == state::FocusTarget::ManuscriptCiteSearch;
                    p.fill_rounded_rect(cite_search_rect, theme.surface, theme.border_radius);
                    p.stroke_rounded_rect(
                        cite_search_rect,
                        if cite_focused {
                            theme.primary
                        } else {
                            theme.border
                        },
                        1.0,
                        theme.border_radius,
                    );
                    let cite_placeholder = t("research.manuscript.cite_search");
                    let cite_display = if self.state.manuscript_cite_search.is_empty() {
                        cite_placeholder.to_string()
                    } else {
                        self.state.manuscript_cite_search.clone()
                    };
                    p.draw_text(
                        &cite_display,
                        rx + spacing + 8.0,
                        cite_search_y + 6.0,
                        theme.on_surface,
                        theme.font_size_small,
                        FontWeight::NORMAL,
                        false,
                    );
                    if cite_focused {
                        let cursor_x = rx
                            + spacing
                            + 8.0
                            + self.state.manuscript_cite_search.len() as f64 * 7.0;
                        p.draw_line(
                            Point::new(cursor_x, cite_search_y + 4.0),
                            Point::new(cursor_x, cite_search_y + 20.0),
                            theme.primary,
                            1.0,
                        );
                    }
                    ry += 30.0;

                    // Cite search results
                    let cite_results = self.state.filtered_cite_results();
                    for (_paper_idx, paper_title) in &cite_results {
                        let result_rect =
                            Rect::new(rx + spacing, ry, size.width - spacing * 2.0, ry + 20.0);
                        p.fill_rounded_rect(result_rect, theme.surface, theme.border_radius);
                        let truncated: String = paper_title.chars().take(40).collect();
                        p.draw_text(
                            &truncated,
                            rx + spacing + 4.0,
                            ry + 4.0,
                            theme.on_surface,
                            theme.font_size_small,
                            FontWeight::NORMAL,
                            false,
                        );
                        // Insert button
                        let insert_x = size.width - spacing - 48.0;
                        p.draw_text(
                            t("research.manuscript.insert_cite"),
                            insert_x,
                            ry + 4.0,
                            theme.primary,
                            theme.font_size_small,
                            FontWeight::MEDIUM,
                            false,
                        );
                        ry += 24.0;
                    }
                }
            }
            5 => {
                p.draw_text(
                    t("research.inspector.citations"),
                    rx + spacing,
                    ry,
                    theme.on_background,
                    theme.font_size,
                    FontWeight::BOLD,
                    false,
                );
                ry += 20.0;

                // Copy as format buttons
                let formats = [
                    state::CitationExportFormat::BibTex,
                    state::CitationExportFormat::Ris,
                    state::CitationExportFormat::Apa,
                    state::CitationExportFormat::Chicago,
                    state::CitationExportFormat::Mla,
                ];
                let mut fmt_x = rx + spacing;
                for fmt in &formats {
                    let label = fmt.label();
                    let btn_w = label.len() as f64 * 7.0 + 12.0;
                    let btn_rect = Rect::new(fmt_x, ry - 8.0, fmt_x + btn_w, ry + 6.0);
                    let is_active = self.state.citation_export_format == *fmt;
                    let bg = if is_active {
                        theme.primary
                    } else {
                        theme.surface
                    };
                    let fg = if is_active {
                        theme.on_primary
                    } else {
                        theme.on_surface
                    };
                    p.fill_rounded_rect(btn_rect, bg, theme.border_radius);
                    if !is_active {
                        p.stroke_rounded_rect(btn_rect, theme.border, 1.0, theme.border_radius);
                    }
                    p.draw_text(
                        label,
                        fmt_x + 6.0,
                        ry,
                        fg,
                        theme.font_size_small,
                        FontWeight::MEDIUM,
                        false,
                    );
                    fmt_x += btn_w + 4.0;
                }
                ry += 24.0;

                // Reference list
                for reference in &paper.references {
                    p.draw_text(
                        reference,
                        rx + spacing,
                        ry,
                        theme.on_surface,
                        theme.font_size_small,
                        FontWeight::NORMAL,
                        false,
                    );
                    ry += 18.0;
                }

                // DOI / arXiv ID input
                ry += spacing;
                p.draw_text(
                    "DOI / arXiv ID",
                    rx + spacing,
                    ry,
                    theme.secondary,
                    theme.font_size_small,
                    FontWeight::BOLD,
                    false,
                );
                ry += 16.0;
                let fetch_x = rx + right_w - spacing - 44.0;
                let doi_rect = Rect::new(rx + spacing, ry - 8.0, fetch_x - 4.0, ry + 8.0);
                p.fill_rounded_rect(doi_rect, theme.background, theme.border_radius);
                p.stroke_rounded_rect(doi_rect, theme.border, 1.0, theme.border_radius);
                let doi_display = if self.state.doi_input.is_empty() {
                    "10.xxxx/..."
                } else {
                    &self.state.doi_input
                };
                p.draw_text(
                    doi_display,
                    rx + spacing + 6.0,
                    ry + 2.0,
                    if self.state.doi_input.is_empty() {
                        theme.disabled
                    } else {
                        theme.on_background
                    },
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );

                // Fetch button
                let fetch_rect = Rect::new(fetch_x, ry - 8.0, fetch_x + 44.0, ry + 8.0);
                p.fill_rounded_rect(fetch_rect, theme.primary, theme.border_radius);
                p.draw_text(
                    "Fetch",
                    fetch_x + 6.0,
                    ry + 2.0,
                    theme.on_primary,
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    false,
                );

                ry += 24.0;

                // Import BibTeX button
                let import_rect = Rect::new(rx + spacing, ry - 8.0, rx + spacing + 100.0, ry + 8.0);
                p.fill_rounded_rect(import_rect, theme.surface, theme.border_radius);
                p.stroke_rounded_rect(import_rect, theme.border, 1.0, theme.border_radius);
                p.draw_text(
                    "Import BibTeX",
                    rx + spacing + 8.0,
                    ry + 2.0,
                    theme.on_surface,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );

                ry += spacing_large;

                // Citation network visualization
                if let Some(plan) = &self.state.writing_visual_draw_plan {
                    let visual_rect = Rect::new(
                        rx + spacing,
                        ry,
                        size.width - spacing,
                        (ry + 150.0).min(size.height - spacing),
                    );
                    let surface =
                        VisualSurface::new(helpers::research_visual_surface_commands(plan))
                            .with_viewport(VisualSurfaceViewport {
                                zoom: f64::from(plan.viewport.zoom.max(0.2)),
                                pan: Vec2::new(
                                    f64::from(plan.viewport.pan_x),
                                    f64::from(plan.viewport.pan_y),
                                ),
                                timeline_position: 0.0,
                                reduced_motion: plan.reduced_motion,
                            })
                            .with_accessibility_summary(plan.accessibility_summary.clone());
                    surface.paint_in_rect(p, visual_rect, theme);
                }
            }
            _ => {}
        }
    }
}
