use super::super::{helpers, state, ResearchApp, ResearchRegions, HEADER_H};
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

impl ResearchApp {
    pub(super) fn paint_header(
        &self,
        p: &mut Painter<'_>,
        size: Size,
        theme: &Theme,
        regions: ResearchRegions,
    ) {
        let t = |key: &'static str| self.i18n.resolve(key).unwrap_or(key);
        let header_h = HEADER_H;
        let spacing = theme.spacing;
        // ── Header ──────────────────────────────────────────────────────
        p.fill_rect(regions.header, theme.surface);

        // Title
        p.draw_text(
            t("research.app.title"),
            spacing,
            14.0,
            theme.on_surface,
            theme.font_size_large,
            FontWeight::BOLD,
            false,
        );

        if size.width >= 700.0 {
            // Search input
            let search_x = 180.0;
            let search_w = (size.width - 540.0).clamp(180.0, 280.0);
            let search_rect =
                Rect::new(search_x, 8.0, search_x + search_w, 8.0 + theme.input_height);
            let is_focused = self.state.focus == state::FocusTarget::SearchBox;
            p.fill_rounded_rect(search_rect, theme.background, theme.border_radius);
            p.stroke_rounded_rect(
                search_rect,
                if is_focused {
                    theme.primary
                } else {
                    theme.border
                },
                if is_focused { 2.0 } else { 1.0 },
                theme.border_radius,
            );
            let query = if self.state.search_query.is_empty() {
                t("research.search.prompt")
            } else {
                &self.state.search_query
            };
            p.draw_text(
                query,
                search_x + 10.0,
                22.0,
                if self.state.search_query.is_empty() {
                    theme.disabled
                } else {
                    theme.on_background
                },
                theme.font_size,
                FontWeight::NORMAL,
                false,
            );

            // Advanced search toggle button indicator
            let adv_x = search_x + search_w - 24.0;
            p.draw_text(
                if self.state.show_advanced_search {
                    "\u{25BC}"
                } else {
                    "\u{25B6}"
                },
                adv_x + 4.0,
                22.0,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );

            // Action buttons
            let btn_x = search_x + search_w + spacing;
            let labels = [
                t("research.action.import"),
                t("research.action.export"),
                t("research.action.sync"),
            ];
            for (i, label) in labels.iter().enumerate() {
                let bx = btn_x + (i as f64) * (80.0 + spacing);
                let btn_rect = Rect::new(bx, 8.0, bx + 80.0, 8.0 + theme.button_height);
                p.fill_rounded_rect(btn_rect, theme.primary, theme.border_radius);
                p.draw_text(
                    label,
                    bx + 40.0,
                    22.0,
                    theme.on_primary,
                    theme.font_size,
                    FontWeight::MEDIUM,
                    true,
                );
            }
            if size.width >= 1040.0 {
                let import_status = helpers::localized_research_status(
                    &self.i18n,
                    self.state.import_status.label(),
                );
                let reader_mode = helpers::localized_research_reader_mode(
                    &self.i18n,
                    self.state.reader_mode.label(),
                );
                p.draw_text(
                    &format!(
                        "{} | {} | {} {}",
                        import_status,
                        reader_mode,
                        t("research.status.favorites"),
                        if self.state.favorites_only {
                            t("research.status.on")
                        } else {
                            t("research.status.off")
                        }
                    ),
                    btn_x + 270.0,
                    22.0,
                    theme.secondary,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
            }
        }

        // Header separator
        p.draw_line(
            Point::new(0.0, header_h),
            Point::new(size.width, header_h),
            theme.border,
            1.0,
        );

        // ── Advanced search panel ──────────────────────────────────────
        if self.state.show_advanced_search {
            if let Some(adv) = &self.state.advanced_search {
                let panel_h = 120.0;
                let panel_rect = Rect::new(180.0, header_h, size.width - 180.0, header_h + panel_h);
                p.fill_rect(panel_rect, theme.surface);
                p.stroke_rounded_rect(panel_rect, theme.border, 1.0, theme.border_radius);
                let mut field_y = header_h + spacing + 4.0;
                let fields = [
                    ("Title:", &adv.title_query),
                    ("Author:", &adv.author_query),
                    ("Venue:", &adv.venue_query),
                    ("Tag:", &adv.tag_query),
                ];
                for (label, value) in &fields {
                    p.draw_text(
                        label,
                        190.0,
                        field_y,
                        theme.secondary,
                        theme.font_size_small,
                        FontWeight::BOLD,
                        false,
                    );
                    let field_rect = Rect::new(240.0, field_y - 8.0, 420.0, field_y + 8.0);
                    p.fill_rounded_rect(field_rect, theme.background, theme.border_radius);
                    p.stroke_rounded_rect(field_rect, theme.border, 1.0, theme.border_radius);
                    let display = if value.is_empty() {
                        "..."
                    } else {
                        value.as_str()
                    };
                    p.draw_text(
                        display,
                        246.0,
                        field_y,
                        if value.is_empty() {
                            theme.disabled
                        } else {
                            theme.on_background
                        },
                        theme.font_size_small,
                        FontWeight::NORMAL,
                        false,
                    );
                    field_y += 24.0;
                }
                // Year range
                p.draw_text(
                    "Year:",
                    190.0,
                    field_y,
                    theme.secondary,
                    theme.font_size_small,
                    FontWeight::BOLD,
                    false,
                );
                let year_from = adv.year_from.map_or(String::new(), |y| y.to_string());
                let year_to = adv.year_to.map_or(String::new(), |y| y.to_string());
                let yr_rect = Rect::new(240.0, field_y - 8.0, 300.0, field_y + 8.0);
                p.fill_rounded_rect(yr_rect, theme.background, theme.border_radius);
                p.stroke_rounded_rect(yr_rect, theme.border, 1.0, theme.border_radius);
                p.draw_text(
                    &format!(
                        "{} - {}",
                        if year_from.is_empty() {
                            "..."
                        } else {
                            &year_from
                        },
                        if year_to.is_empty() { "..." } else { &year_to }
                    ),
                    246.0,
                    field_y,
                    theme.on_background,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );

                // Save search button
                let save_rect = Rect::new(440.0, field_y - 8.0, 540.0, field_y + 8.0);
                p.fill_rounded_rect(save_rect, theme.primary, theme.border_radius);
                p.draw_text(
                    "Save Search",
                    452.0,
                    field_y,
                    theme.on_primary,
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    false,
                );

                p.draw_line(
                    Point::new(180.0, header_h + panel_h),
                    Point::new(size.width, header_h + panel_h),
                    theme.border,
                    1.0,
                );
            }
        }
    }
}
