use super::super::{helpers, paper_list, state, ResearchApp, ResearchRegions, HEADER_H};
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

impl ResearchApp {
    pub(super) fn paint_center_panel<'a>(
        &'a self,
        p: &mut Painter<'_>,
        theme: &Theme,
        regions: ResearchRegions,
        spacing: f64,
        spacing_large: f64,
    ) -> Option<&'a state::Paper> {
        let t = |key: &'static str| self.i18n.resolve(key).unwrap_or(key);
        let header_h = HEADER_H;
        // ── Center: Paper detail ───────────────────────────────────────
        let center_x = regions.center.x0;
        let center_right = regions.center.x1;
        let Some(paper) = self.state.selected_paper() else {
            p.draw_text(
                t("research.empty.title"),
                center_x + spacing_large,
                header_h + spacing_large,
                theme.on_background,
                theme.font_size_large,
                FontWeight::BOLD,
                false,
            );
            p.draw_text(
                t("research.empty.body"),
                center_x + spacing_large,
                header_h + spacing_large + 28.0,
                theme.secondary,
                theme.font_size,
                FontWeight::NORMAL,
                false,
            );
            return None;
        };
        let mut cy = header_h + spacing_large;

        // Title
        p.draw_text(
            &paper.title,
            center_x + spacing_large,
            cy,
            theme.on_background,
            theme.font_size_large,
            FontWeight::BOLD,
            false,
        );
        cy += 28.0;

        // Authors + year
        p.draw_text(
            &format!(
                "{} ({}) | {} | {} {} | {}",
                paper.authors,
                paper.year,
                paper.venue,
                paper.pages,
                t("research.unit.pages"),
                paper.file_name
            ),
            center_x + spacing_large,
            cy,
            theme.secondary,
            theme.font_size,
            FontWeight::NORMAL,
            false,
        );
        cy += 24.0;

        // Tags
        let mut tx = center_x + spacing_large;
        for tag in &paper.tags {
            let tw = 100.0;
            let tag_rect = Rect::new(tx, cy - 10.0, tx + tw, cy + 6.0);
            p.fill_rounded_rect(tag_rect, theme.primary, theme.border_radius);
            p.draw_text(
                tag,
                tx + 8.0,
                cy,
                theme.on_primary,
                theme.font_size_small,
                FontWeight::MEDIUM,
                false,
            );
            tx += tw + spacing;
        }
        cy += 24.0;

        // Status badge
        let status_rect = Rect::new(
            center_x + spacing_large,
            cy - 10.0,
            center_x + spacing_large + 80.0,
            cy + 6.0,
        );
        let status_color = paper_list::status_color(&paper.status);
        p.fill_rounded_rect(status_rect, status_color, theme.border_radius);
        let paper_status =
            helpers::localized_research_status(&self.i18n, state::status_label(&paper.status));
        p.draw_text(
            &paper_status,
            center_x + spacing_large + 8.0,
            cy,
            theme.on_primary,
            theme.font_size_small,
            FontWeight::MEDIUM,
            false,
        );
        cy += 24.0;

        // Abstract heading
        p.draw_text(
            t("research.section.abstract"),
            center_x + spacing_large,
            cy,
            theme.on_background,
            theme.font_size,
            FontWeight::BOLD,
            false,
        );
        cy += 20.0;

        // Abstract text (wrapped manually into lines)
        let abstract_lines = paper_list::wrap_text(&paper.abstract_text, 80);
        for line in &abstract_lines {
            p.draw_text(
                line,
                center_x + spacing_large,
                cy,
                theme.on_surface,
                theme.font_size,
                FontWeight::NORMAL,
                false,
            );
            cy += 18.0;
        }
        cy += spacing_large;

        // PDF search bar (only in PDF mode)
        if self.state.reader_mode == state::ReaderMode::Pdf {
            let search_bar_rect = Rect::new(
                center_x + spacing_large,
                cy,
                center_right - spacing_large,
                cy + 24.0,
            );
            p.fill_rounded_rect(search_bar_rect, theme.background, theme.border_radius);
            p.stroke_rounded_rect(search_bar_rect, theme.border, 1.0, theme.border_radius);
            let search_text = if self.state.pdf_search_query.is_empty() {
                "Search in PDF..."
            } else {
                &self.state.pdf_search_query
            };
            p.draw_text(
                search_text,
                center_x + spacing_large + 8.0,
                cy + 14.0,
                if self.state.pdf_search_query.is_empty() {
                    theme.disabled
                } else {
                    theme.on_background
                },
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );
            if !self.state.pdf_search_results.is_empty() {
                let count_label = format!(
                    "{}/{}",
                    self.state
                        .pdf_search_active_index
                        .map(|i| i + 1)
                        .unwrap_or(0),
                    self.state.pdf_search_results.len()
                );
                p.draw_text(
                    &count_label,
                    center_right - spacing_large - 60.0,
                    cy + 14.0,
                    theme.secondary,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
            }
            cy += 28.0;
        }

        // Visual analysis or PDF reader preview
        let fig_rect = Rect::new(
            center_x + spacing_large,
            cy,
            center_right - spacing_large,
            cy + 180.0,
        );
        if self.state.reader_mode == state::ReaderMode::Pdf {
            helpers::build_pdf_surface_for_paper(paper, &self.state)
                .paint_in_rect(p, fig_rect, theme);
        } else {
            p.fill_rounded_rect(fig_rect, theme.background, theme.border_radius);
            p.stroke_rounded_rect(fig_rect, theme.border, 1.0, theme.border_radius);
            p.draw_text(
                t("research.section.visual"),
                center_x + spacing_large + 10.0,
                cy + 24.0,
                theme.on_background,
                theme.font_size,
                FontWeight::BOLD,
                false,
            );
            for (index, line) in self.state.visual_summary_lines.iter().take(6).enumerate() {
                p.draw_text(
                    line,
                    center_x + spacing_large + 10.0,
                    cy + 50.0 + index as f64 * 18.0,
                    theme.secondary,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
            }
        }
        cy += 196.0;

        // PDF navigation controls (only in PDF mode)
        if self.state.reader_mode == state::ReaderMode::Pdf {
            let nav_y = cy;
            let nav_x = center_x + spacing_large;

            // Page navigation
            let page_label = format!(
                "Page {}/{}",
                self.state.pdf_current_page,
                paper.pages.max(1)
            );
            p.draw_text(
                &page_label,
                nav_x,
                nav_y,
                theme.on_background,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );

            // Prev/Next buttons
            let btn_w = 28.0;
            let btn_h = 20.0;
            let prev_x = nav_x + 100.0;
            let prev_rect = Rect::new(prev_x, nav_y - 12.0, prev_x + btn_w, nav_y - 12.0 + btn_h);
            p.fill_rounded_rect(prev_rect, theme.primary, theme.border_radius);
            p.draw_text(
                "<",
                prev_x + 9.0,
                nav_y - 2.0,
                theme.on_primary,
                theme.font_size_small,
                FontWeight::BOLD,
                true,
            );

            let next_x = prev_x + btn_w + spacing;
            let next_rect = Rect::new(next_x, nav_y - 12.0, next_x + btn_w, nav_y - 12.0 + btn_h);
            p.fill_rounded_rect(next_rect, theme.primary, theme.border_radius);
            p.draw_text(
                ">",
                next_x + 9.0,
                nav_y - 2.0,
                theme.on_primary,
                theme.font_size_small,
                FontWeight::BOLD,
                true,
            );

            // Zoom controls
            let zoom_label = format!("{:.0}%", self.state.pdf_zoom * 100.0);
            let zoom_x = next_x + btn_w + spacing_large;
            p.draw_text(
                &zoom_label,
                zoom_x,
                nav_y,
                theme.on_background,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );

            let zoom_out_x = zoom_x + 50.0;
            let zoom_out_rect = Rect::new(
                zoom_out_x,
                nav_y - 12.0,
                zoom_out_x + btn_w,
                nav_y - 12.0 + btn_h,
            );
            p.fill_rounded_rect(zoom_out_rect, theme.surface, theme.border_radius);
            p.stroke_rounded_rect(zoom_out_rect, theme.border, 1.0, theme.border_radius);
            p.draw_text(
                "-",
                zoom_out_x + 10.0,
                nav_y - 2.0,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::BOLD,
                true,
            );

            let zoom_in_x = zoom_out_x + btn_w + 4.0;
            let zoom_in_rect = Rect::new(
                zoom_in_x,
                nav_y - 12.0,
                zoom_in_x + btn_w,
                nav_y - 12.0 + btn_h,
            );
            p.fill_rounded_rect(zoom_in_rect, theme.surface, theme.border_radius);
            p.stroke_rounded_rect(zoom_in_rect, theme.border, 1.0, theme.border_radius);
            p.draw_text(
                "+",
                zoom_in_x + 8.0,
                nav_y - 2.0,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::BOLD,
                true,
            );

            // Rotate button
            let rot_x = zoom_in_x + btn_w + spacing;
            let rot_rect = Rect::new(rot_x, nav_y - 12.0, rot_x + 36.0, nav_y - 12.0 + btn_h);
            p.fill_rounded_rect(rot_rect, theme.surface, theme.border_radius);
            p.stroke_rounded_rect(rot_rect, theme.border, 1.0, theme.border_radius);
            p.draw_text(
                "Rot",
                rot_x + 6.0,
                nav_y - 2.0,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::NORMAL,
                true,
            );

            // Annotation tools
            let tool_x = rot_x + 44.0 + spacing;
            let tools = ["H", "U", "S", "N"];
            for (i, tool_label) in tools.iter().enumerate() {
                let tx = tool_x + (i as f64) * (24.0 + 4.0);
                let tool_rect = Rect::new(tx, nav_y - 12.0, tx + 24.0, nav_y - 12.0 + btn_h);
                let is_active = matches!(
                    (i, self.state.pdf_annotation_tool),
                    (0, state::PdfAnnotationTool::Highlight)
                        | (1, state::PdfAnnotationTool::Underline)
                        | (2, state::PdfAnnotationTool::Strikeout)
                        | (3, state::PdfAnnotationTool::StickyNote)
                );
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
                p.fill_rounded_rect(tool_rect, bg, theme.border_radius);
                if !is_active {
                    p.stroke_rounded_rect(tool_rect, theme.border, 1.0, theme.border_radius);
                }
                p.draw_text(
                    tool_label,
                    tx + 6.0,
                    nav_y - 2.0,
                    fg,
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    true,
                );
            }

            // Annotation list toggle button
            let ann_list_x = tool_x + tools.len() as f64 * (24.0 + 4.0) + spacing;
            let ann_list_rect = Rect::new(
                ann_list_x,
                nav_y - 12.0,
                ann_list_x + 32.0,
                nav_y - 12.0 + btn_h,
            );
            let ann_list_bg = if self.state.pdf_show_annotation_list {
                theme.primary
            } else {
                theme.surface
            };
            let ann_list_fg = if self.state.pdf_show_annotation_list {
                theme.on_primary
            } else {
                theme.on_surface
            };
            p.fill_rounded_rect(ann_list_rect, ann_list_bg, theme.border_radius);
            if !self.state.pdf_show_annotation_list {
                p.stroke_rounded_rect(ann_list_rect, theme.border, 1.0, theme.border_radius);
            }
            p.draw_text(
                "Ann",
                ann_list_x + 4.0,
                nav_y - 2.0,
                ann_list_fg,
                theme.font_size_small,
                FontWeight::MEDIUM,
                true,
            );

            cy += 28.0;
        }

        // References
        p.draw_text(
            t("research.section.references"),
            center_x + spacing_large,
            cy,
            theme.on_background,
            theme.font_size,
            FontWeight::BOLD,
            false,
        );
        cy += 20.0;
        for reference in &paper.references {
            p.draw_text(
                reference,
                center_x + spacing_large,
                cy,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );
            cy += 18.0;
        }

        // Annotation list sidebar (only in PDF mode when toggled)
        if self.state.reader_mode == state::ReaderMode::Pdf && self.state.pdf_show_annotation_list {
            cy += spacing_large;
            p.draw_text(
                "Annotations",
                center_x + spacing_large,
                cy,
                theme.on_background,
                theme.font_size,
                FontWeight::BOLD,
                false,
            );
            cy += 20.0;
            if self.state.pdf_annotations.is_empty() {
                p.draw_text(
                    "No annotations yet.",
                    center_x + spacing_large,
                    cy,
                    theme.disabled,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
            } else {
                for ann in &self.state.pdf_annotations {
                    let is_selected =
                        self.state.pdf_selected_annotation.as_deref() == Some(&ann.id);
                    let kind_label = match ann.kind {
                        state::PdfAnnotationTool::Highlight => "HL",
                        state::PdfAnnotationTool::Underline => "UL",
                        state::PdfAnnotationTool::Strikeout => "ST",
                        state::PdfAnnotationTool::StickyNote => "Note",
                        _ => "?",
                    };
                    let row_bg = if is_selected {
                        theme.primary
                    } else {
                        theme.surface
                    };
                    let row_fg = if is_selected {
                        theme.on_primary
                    } else {
                        theme.on_surface
                    };
                    let row_rect = Rect::new(
                        center_x + spacing_large,
                        cy - 10.0,
                        center_right - spacing_large,
                        cy + 8.0,
                    );
                    p.fill_rounded_rect(row_rect, row_bg, theme.border_radius);
                    p.draw_text(
                        kind_label,
                        center_x + spacing_large + 4.0,
                        cy,
                        ann.color,
                        theme.font_size_small,
                        FontWeight::BOLD,
                        false,
                    );
                    let label = ann.text.as_deref().unwrap_or("—");
                    let display = if label.len() > 40 {
                        format!("{}...", &label[..40])
                    } else {
                        label.to_string()
                    };
                    p.draw_text(
                        &format!("p{}: {}", ann.page, display),
                        center_x + spacing_large + 40.0,
                        cy,
                        row_fg,
                        theme.font_size_small,
                        FontWeight::NORMAL,
                        false,
                    );
                    cy += 22.0;
                }
            }
        }

        Some(paper)
    }
}
