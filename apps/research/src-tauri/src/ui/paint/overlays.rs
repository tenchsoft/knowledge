use super::super::{state, ResearchApp};
use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;

impl ResearchApp {
    pub(super) fn paint_overlays(&self, p: &mut Painter<'_>, size: Size, theme: &Theme) {
        let t = |key: &'static str| self.i18n.resolve(key).unwrap_or(key);
        // ── Toast overlay ─────────────────────────────────────────────
        if !self.state.toasts.is_empty() {
            let mut toast_y = size.height - 60.0;
            for toast in self.state.toasts.iter().rev().take(5) {
                let msg_w = toast.message.len() as f64 * 7.0 + 24.0;
                let toast_x = (size.width - msg_w) / 2.0;
                let toast_rect = Rect::new(toast_x, toast_y, toast_x + msg_w, toast_y + 28.0);
                let bg_color = match toast.kind {
                    state::ToastKind::Success => Color::rgba8(0x22, 0xC5, 0x5E, 0xE0),
                    state::ToastKind::Error => Color::rgba8(0xEF, 0x44, 0x44, 0xE0),
                    state::ToastKind::Warning => Color::rgba8(0xF5, 0x9E, 0x0B, 0xE0),
                    state::ToastKind::Info => Color::rgba8(0x3B, 0x82, 0xF6, 0xE0),
                };
                p.fill_rounded_rect(toast_rect, bg_color, theme.border_radius);
                p.draw_text(
                    &toast.message,
                    toast_x + 12.0,
                    toast_y + 8.0,
                    Color::WHITE,
                    theme.font_size_small,
                    FontWeight::MEDIUM,
                    false,
                );
                toast_y -= 36.0;
            }
        }

        // ── Welcome screen overlay ────────────────────────────────────
        if self.state.show_welcome {
            p.fill_rect(
                Rect::new(0.0, 0.0, size.width, size.height),
                Color::rgba8(0x00, 0x00, 0x00, 0xCC),
            );
            let cx = size.width / 2.0;
            let cy = size.height / 2.0;
            let card_w = 420.0_f64.min(size.width - 40.0);
            let card_rect = Rect::new(cx - card_w / 2.0, cy - 120.0, cx + card_w / 2.0, cy + 120.0);
            p.fill_rounded_rect(card_rect, theme.background, 12.0);
            p.stroke_rounded_rect(card_rect, theme.border, 1.0, 12.0);
            p.draw_text(
                t("research.welcome.title"),
                cx - 100.0,
                cy - 80.0,
                theme.on_background,
                theme.font_size + 4.0,
                FontWeight::BOLD,
                false,
            );
            p.draw_text(
                t("research.welcome.body1"),
                cx - 160.0,
                cy - 40.0,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );
            p.draw_text(
                t("research.welcome.body2"),
                cx - 160.0,
                cy - 20.0,
                theme.on_surface,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );

            // Get started button
            let btn_w = 160.0;
            let btn_rect = Rect::new(cx - btn_w / 2.0, cy + 20.0, cx + btn_w / 2.0, cy + 52.0);
            p.fill_rounded_rect(btn_rect, theme.primary, theme.border_radius);
            p.draw_text(
                t("research.welcome.get_started"),
                cx - 42.0,
                cy + 30.0,
                theme.on_primary,
                theme.font_size,
                FontWeight::MEDIUM,
                false,
            );

            // Import from file button
            let import_btn_rect =
                Rect::new(cx - btn_w / 2.0, cy + 64.0, cx + btn_w / 2.0, cy + 96.0);
            p.stroke_rounded_rect(import_btn_rect, theme.primary, 1.0, theme.border_radius);
            p.draw_text(
                t("research.welcome.import"),
                cx - 48.0,
                cy + 74.0,
                theme.primary,
                theme.font_size,
                FontWeight::MEDIUM,
                false,
            );
        }

        // ── Shortcut help modal ─────────────────────────────────────────
        if self.state.show_shortcut_help {
            let cx = size.width / 2.0;
            let cy = size.height / 2.0;
            let modal_w = 360.0_f64.min(size.width - 40.0);
            let modal_h = 280.0_f64.min(size.height - 40.0);
            let modal_rect = Rect::new(
                cx - modal_w / 2.0,
                cy - modal_h / 2.0,
                cx + modal_w / 2.0,
                cy + modal_h / 2.0,
            );
            // Backdrop
            p.fill_rect(
                Rect::new(0.0, 0.0, size.width, size.height),
                Color::rgba8(0x00, 0x00, 0x00, 0x99),
            );
            p.fill_rounded_rect(modal_rect, theme.background, 12.0);
            p.stroke_rounded_rect(modal_rect, theme.border, 1.0, 12.0);

            // Title
            p.draw_text(
                "Keyboard Shortcuts",
                modal_rect.x0 + 20.0,
                modal_rect.y0 + 28.0,
                theme.on_background,
                theme.font_size_large,
                FontWeight::BOLD,
                false,
            );

            // Shortcut list
            let shortcuts = [
                ("?", "Toggle this help"),
                ("Ctrl+F", "Focus search"),
                ("Ctrl+I", "Import"),
                ("Ctrl+E", "Export"),
                ("Escape", "Close modal / defocus"),
                ("Enter", "Confirm / next"),
                ("Arrow keys", "Navigate papers"),
                ("Delete", "Remove selected"),
            ];
            let mut sy = modal_rect.y0 + 56.0;
            for (key, desc) in &shortcuts {
                p.draw_text(
                    key,
                    modal_rect.x0 + 20.0,
                    sy,
                    theme.primary,
                    theme.font_size_small,
                    FontWeight::BOLD,
                    false,
                );
                p.draw_text(
                    desc,
                    modal_rect.x0 + 100.0,
                    sy,
                    theme.on_surface,
                    theme.font_size_small,
                    FontWeight::NORMAL,
                    false,
                );
                sy += 24.0;
            }

            // Close hint
            p.draw_text(
                "Press ? or Escape to close",
                modal_rect.x0 + 20.0,
                modal_rect.y1 - 24.0,
                theme.disabled,
                theme.font_size_small,
                FontWeight::NORMAL,
                false,
            );
        }
    }
}
