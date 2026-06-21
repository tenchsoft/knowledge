use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::layout::{modal_close_rect, modal_rect};
use crate::ui::state::StudyState;
use crate::ui::state::{ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_500, NEUTRAL_800};

pub fn paint_shortcut_help_modal(
    p: &mut Painter<'_>,
    state: &StudyState,
    size: Size,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 160),
    );
    let modal = modal_rect(size);
    p.fill_rounded_rect(modal, NEUTRAL_800, 8.0);
    p.stroke_rounded_rect(modal, NEUTRAL_500, 1.0, 8.0);

    p.draw_text(
        t("study.shortcuts.title"),
        modal.x0 + 18.0,
        modal.y0 + 34.0,
        NEUTRAL_100,
        18.0,
        FontWeight::BOLD,
        false,
    );

    let close = modal_close_rect(size);
    p.stroke_rounded_rect(close, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        t("study.modal.close"),
        close.x0 + 10.0,
        close.y0 + 21.0,
        NEUTRAL_300,
        14.0,
        FontWeight::BOLD,
        false,
    );

    for (idx, shortcut) in state.keyboard_shortcuts.iter().enumerate() {
        let y = modal.y0 + 68.0 + idx as f64 * 28.0;
        p.draw_text(
            &shortcut.key,
            modal.x0 + 18.0,
            y + 16.0,
            ACCENT_STUDY,
            12.0,
            FontWeight::BOLD,
            false,
        );
        let label = crate::i18n::resolve(i18n, &shortcut.label_key);
        p.draw_text(
            label,
            modal.x0 + 90.0,
            y + 16.0,
            NEUTRAL_300,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }

    // Extra shortcuts
    let extra_y = modal.y0 + 68.0 + state.keyboard_shortcuts.len() as f64 * 28.0;
    let extras: [(&str, &str); 5] = [
        ("?", "study.shortcuts.help"),
        ("1/2/3", "study.shortcuts.hint_levels"),
        ("Ctrl+S", "study.shortcuts.stats"),
        ("Ctrl+R", "study.shortcuts.review"),
        ("Space", "study.shortcuts.space_input"),
    ];
    for (idx, (key, label_key)) in extras.iter().enumerate() {
        let y = extra_y + idx as f64 * 28.0;
        p.draw_text(
            key,
            modal.x0 + 18.0,
            y + 16.0,
            ACCENT_STUDY,
            12.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            t(label_key),
            modal.x0 + 90.0,
            y + 16.0,
            NEUTRAL_300,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }
}
