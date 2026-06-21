use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::layout::{modal_close_rect, modal_rect};
use crate::ui::state::StudyState;
use crate::ui::state::{
    ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_400, NEUTRAL_500, NEUTRAL_600, NEUTRAL_700,
    STATUS_WARNING,
};

pub fn paint_modals(
    p: &mut Painter<'_>,
    state: &StudyState,
    size: Size,
    i18n: &tench_app_core::I18nCatalog,
) {
    if !state.show_result_modal && !state.show_stats_modal {
        return;
    }
    let t = |key| crate::i18n::resolve(i18n, key);
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 140),
    );
    let modal = modal_rect(size);
    p.fill_rounded_rect(modal, NEUTRAL_700, 8.0);
    p.stroke_rounded_rect(modal, NEUTRAL_500, 1.0, 8.0);
    let title = if state.show_result_modal {
        t("study.modal.session_result")
    } else {
        t("study.modal.stats")
    };
    p.draw_text(
        title,
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

    if state.show_result_modal {
        p.draw_text(
            &format!("{}: {}%", t("study.stats.accuracy"), state.accuracy()),
            modal.x0 + 18.0,
            modal.y0 + 82.0,
            ACCENT_STUDY,
            16.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            &format!(
                "{}: {}",
                t("study.stats.solved"),
                state.session_results.len()
            ),
            modal.x0 + 18.0,
            modal.y0 + 116.0,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!(
                "{}: {}",
                t("study.stats.wrong"),
                state
                    .session_results
                    .iter()
                    .filter(|correct| !**correct)
                    .count()
            ),
            modal.x0 + 18.0,
            modal.y0 + 142.0,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );
    } else {
        p.draw_text(
            &format!("{}: {}", t("study.stats.total_sessions"), 1),
            modal.x0 + 18.0,
            modal.y0 + 82.0,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!(
                "{}: {}",
                t("study.stats.pending_reviews"),
                state.review_queue.len()
            ),
            modal.x0 + 18.0,
            modal.y0 + 108.0,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!("{}: {}", t("study.stats.streak"), state.streak),
            modal.x0 + 18.0,
            modal.y0 + 134.0,
            STATUS_WARNING,
            13.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            &format!(
                "{}: {}",
                t("study.stats.builtin_curricula"),
                state.builtin_curriculum_count
            ),
            modal.x0 + 18.0,
            modal.y0 + 160.0,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!(
                "{}: {} / {}",
                t("study.stats.lessons_visuals"),
                state.builtin_lesson_count,
                state.builtin_visual_count
            ),
            modal.x0 + 18.0,
            modal.y0 + 186.0,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!(
                "{}: {}",
                t("study.stats.glossary"),
                state.builtin_glossary_count
            ),
            modal.x0 + 18.0,
            modal.y0 + 212.0,
            NEUTRAL_100,
            13.0,
            FontWeight::NORMAL,
            false,
        );

        // Phase 5: Streak calendar visualization (last 14 days)
        let cal_y = modal.y0 + 240.0;
        p.draw_text(
            t("study.stats.streak_calendar"),
            modal.x0 + 18.0,
            cal_y,
            NEUTRAL_400,
            10.0,
            FontWeight::BOLD,
            false,
        );
        for (idx, &active) in state.streak_calendar.iter().take(14).enumerate() {
            let x = modal.x0 + 18.0 + idx as f64 * 18.0;
            let y = cal_y + 14.0;
            let cell = Rect::new(x, y, x + 14.0, y + 14.0);
            let color = if active { STATUS_WARNING } else { NEUTRAL_600 };
            p.fill_rounded_rect(cell, color, 2.0);
        }
    }
}
