use super::super::{helpers, ResearchApp, ResearchRegions, HEADER_H};
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

impl ResearchApp {
    pub(super) fn paint_left_panel(
        &self,
        p: &mut Painter<'_>,
        size: Size,
        theme: &Theme,
        regions: ResearchRegions,
        spacing: f64,
        spacing_large: f64,
    ) {
        let t = |key: &'static str| self.i18n.resolve(key).unwrap_or(key);
        let header_h = HEADER_H;
        let left_w = regions.left.width();
        // ── Left panel ─────────────────────────────────────────────────
        p.fill_rect(regions.left, theme.surface);

        let mut y = header_h + spacing_large;

        // Collections
        p.draw_text(
            t("research.collection.heading"),
            spacing,
            y,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        y += 20.0;
        for col in &self.state.collections {
            let is_selected = self.state.selected_collection.as_deref() == Some(&col.id);
            let bg = if is_selected {
                theme.primary
            } else {
                theme.surface
            };
            let row_rect = Rect::new(0.0, y - 2.0, left_w, y + 18.0);
            p.fill_rect(row_rect, bg);
            let fg = if is_selected {
                theme.on_primary
            } else {
                theme.on_surface
            };
            let expand_icon = if col.expanded { "\u{25BE}" } else { "\u{25B8}" };
            p.draw_text(
                expand_icon,
                spacing,
                y,
                fg,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );
            p.draw_text(
                &format!("{} ({})", col.name, col.count),
                spacing + spacing + 8.0,
                y,
                fg,
                theme.font_size,
                FontWeight::NORMAL,
                false,
            );
            y += 20.0;
        }

        // Smart collections
        if !self.state.smart_collections.is_empty() {
            y += spacing;
            p.draw_text(
                "Smart",
                spacing,
                y,
                theme.secondary,
                theme.font_size_small,
                FontWeight::BOLD,
                false,
            );
            y += 18.0;
            for sc in &self.state.smart_collections {
                let is_selected = self.state.selected_collection.as_deref() == Some(&sc.id);
                let bg = if is_selected {
                    theme.primary
                } else {
                    theme.surface
                };
                let row_rect = Rect::new(0.0, y - 2.0, left_w, y + 18.0);
                p.fill_rect(row_rect, bg);
                let fg = if is_selected {
                    theme.on_primary
                } else {
                    theme.on_surface
                };
                p.draw_text(
                    &format!("\u{2699} {} ({})", sc.name, sc.count),
                    spacing + spacing,
                    y,
                    fg,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
                y += 20.0;
            }
        }

        y += spacing_large;

        // Tags
        p.draw_text(
            t("research.tag.heading"),
            spacing,
            y,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        y += 20.0;
        for tag in &self.state.tags {
            let tag_rect = Rect::new(spacing, y - 12.0, spacing + 100.0, y + 4.0);
            p.fill_rounded_rect(tag_rect, theme.primary, theme.border_radius);
            p.draw_text(
                tag,
                spacing + 8.0,
                y - 2.0,
                theme.on_primary,
                theme.font_size_small,
                FontWeight::MEDIUM,
                false,
            );
            y += 24.0;
        }

        // Saved searches
        if !self.state.saved_searches.is_empty() {
            y += spacing;
            p.draw_text(
                "Saved Searches",
                spacing,
                y,
                theme.secondary,
                theme.font_size_small,
                FontWeight::BOLD,
                false,
            );
            y += 18.0;
            for saved in &self.state.saved_searches {
                let row_rect = Rect::new(0.0, y - 2.0, left_w, y + 18.0);
                p.fill_rect(row_rect, theme.surface);
                p.draw_text(
                    &format!("\u{1F50D} {} [{}]", saved.name, saved.query),
                    spacing + spacing,
                    y,
                    theme.on_surface,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
                y += 20.0;
            }
        }

        y += spacing_large;

        // Status filters
        p.draw_text(
            t("research.status.heading"),
            spacing,
            y,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        y += 20.0;
        for status in &self.state.statuses {
            let status_label = helpers::localized_research_status(&self.i18n, status.label());
            p.draw_text(
                &status_label,
                spacing + spacing,
                y,
                theme.on_surface,
                theme.font_size,
                FontWeight::NORMAL,
                false,
            );
            y += 20.0;
        }

        // Paper list
        y += spacing_large;
        p.draw_text(
            t("research.paper.heading"),
            spacing,
            y,
            theme.secondary,
            theme.font_size_small,
            FontWeight::BOLD,
            false,
        );
        // Sort indicator button
        let sort_label = self.state.sort_mode.label();
        let sort_btn_w = sort_label.len() as f64 * 6.0 + 12.0;
        let sort_btn_rect = Rect::new(
            left_w - sort_btn_w - spacing,
            y - 4.0,
            left_w - spacing,
            y + 10.0,
        );
        p.fill_rounded_rect(sort_btn_rect, theme.surface, theme.border_radius);
        p.stroke_rounded_rect(sort_btn_rect, theme.border, 1.0, theme.border_radius);
        p.draw_text(
            sort_label,
            left_w - sort_btn_w - spacing + 6.0,
            y,
            theme.on_surface,
            theme.font_size_small - 2.0,
            FontWeight::NORMAL,
            false,
        );
        y += 20.0;
        for (i, paper) in self.state.visible_papers() {
            let is_multi_selected = self.state.selected_papers.contains(&i);
            let multi_sel_bg = Color::rgba8(0x60, 0xA5, 0xFA, 40); // primary with low alpha
            let bg = if i == self.state.selected_paper {
                theme.primary
            } else if is_multi_selected {
                multi_sel_bg
            } else {
                theme.surface
            };
            let row_rect = Rect::new(0.0, y - 2.0, left_w, y + 18.0);
            p.fill_rect(row_rect, bg);
            let fg = if i == self.state.selected_paper {
                theme.on_primary
            } else {
                theme.on_surface
            };
            // Multi-select checkbox
            if is_multi_selected {
                p.draw_text(
                    "\u{2611}",
                    spacing,
                    y + 10.0,
                    theme.primary,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
            }
            let title_text = &format!("{}{}", if paper.favorite { "* " } else { "" }, paper.title);
            let text_x = spacing + spacing + if is_multi_selected { 14.0 } else { 0.0 };
            // Search result highlighting
            let query_lower = self.state.search_query.trim().to_lowercase();
            if !query_lower.is_empty() && paper.title.to_lowercase().contains(&query_lower) {
                let highlight_rect = Rect::new(
                    text_x - 2.0,
                    y + 2.0,
                    text_x + title_text.len() as f64 * 6.5,
                    y + 16.0,
                );
                p.fill_rounded_rect(highlight_rect, Color::rgba8(255, 235, 59, 40), 2.0);
            }
            p.draw_text(
                title_text,
                text_x,
                y + 10.0,
                fg,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );
            y += 22.0;
        }

        // Left separator
        p.draw_line(
            Point::new(left_w, header_h),
            Point::new(left_w, size.height),
            theme.border,
            1.0,
        );
    }
}
