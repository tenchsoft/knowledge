use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::layout::{modal_close_rect, modal_rect};
use crate::ui::state::StudyState;
use crate::ui::state::{
    ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_400, NEUTRAL_500, NEUTRAL_700, NEUTRAL_800,
    STATUS_WARNING,
};

pub fn paint_goal_modal(
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
        t("study.goals.title"),
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

    for (idx, goal) in state.goals.iter().enumerate() {
        let y = modal.y0 + 68.0 + idx as f64 * 52.0;
        let goal_label = crate::i18n::resolve(i18n, &goal.label_key);
        p.draw_text(
            goal_label,
            modal.x0 + 18.0,
            y,
            NEUTRAL_100,
            13.0,
            FontWeight::BOLD,
            false,
        );
        // Progress bar
        let bar = Rect::new(modal.x0 + 18.0, y + 18.0, modal.x1 - 18.0, y + 30.0);
        p.fill_rounded_rect(bar, NEUTRAL_700, 4.0);
        let progress = if goal.target > 0 {
            (goal.current as f64 / goal.target as f64).min(1.0)
        } else {
            0.0
        };
        let filled = Rect::new(bar.x0, bar.y0, bar.x0 + bar.width() * progress, bar.y1);
        if progress > 0.0 {
            p.fill_rounded_rect(filled, ACCENT_STUDY, 4.0);
        }
        let progress_text = format!("{} / {} {}", goal.current, goal.target, goal.unit);
        p.draw_text(
            &progress_text,
            modal.x0 + 24.0,
            y + 42.0,
            NEUTRAL_400,
            11.0,
            FontWeight::NORMAL,
            false,
        );
    }

    // Achievements section
    let achievements_y = modal.y0 + 68.0 + state.goals.len() as f64 * 52.0 + 16.0;
    p.draw_text(
        t("study.achievements.title"),
        modal.x0 + 18.0,
        achievements_y,
        NEUTRAL_100,
        13.0,
        FontWeight::BOLD,
        false,
    );
    for (idx, achievement) in state.achievements.iter().enumerate() {
        let y = achievements_y + 22.0 + idx as f64 * 24.0;
        let icon = if achievement.unlocked {
            "\u{2605}"
        } else {
            "\u{2606}"
        };
        p.draw_text(
            icon,
            modal.x0 + 18.0,
            y + 14.0,
            if achievement.unlocked {
                STATUS_WARNING
            } else {
                NEUTRAL_500
            },
            14.0,
            FontWeight::NORMAL,
            false,
        );
        let ach_label = crate::i18n::resolve(i18n, &achievement.label_key);
        p.draw_text(
            ach_label,
            modal.x0 + 40.0,
            y + 14.0,
            if achievement.unlocked {
                NEUTRAL_100
            } else {
                NEUTRAL_400
            },
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }
}
