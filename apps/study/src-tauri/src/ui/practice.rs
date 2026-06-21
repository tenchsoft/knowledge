use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::curriculum::StudyRegions;
use super::state::{
    SpacedRepetitionRating, StudyState, ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_400,
    NEUTRAL_500, NEUTRAL_600, NEUTRAL_700, NEUTRAL_800, STATUS_ERROR, STATUS_READY,
};

pub fn paint_practice_surface(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    let surface = regions.surface;
    p.fill_rect(surface, NEUTRAL_800);
    let x = surface.x0 + 32.0;
    let y = surface.y0 + 34.0;
    p.draw_text(
        &format!(
            "{} {} / {}",
            t("study.practice.problem"),
            state.problem_index,
            state.total_problems().max(1)
        ),
        x,
        y,
        NEUTRAL_300,
        14.0,
        FontWeight::BOLD,
        false,
    );

    if let Some(problem) = state.current_problem() {
        let card = Rect::new(x, y + 24.0, surface.x1 - 32.0, y + 204.0);
        p.fill_rounded_rect(card, NEUTRAL_600, 8.0);
        p.stroke_rounded_rect(card, NEUTRAL_500, 1.0, 8.0);
        p.draw_text(
            &problem.text,
            card.x0 + 20.0,
            card.y0 + 34.0,
            NEUTRAL_100,
            16.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &problem.matrices,
            card.x0 + 20.0,
            card.y0 + 76.0,
            NEUTRAL_100,
            14.0,
            FontWeight::NORMAL,
            false,
        );
    }

    p.draw_text(
        t("study.practice.answer"),
        x,
        y + 248.0,
        NEUTRAL_300,
        12.0,
        FontWeight::BOLD,
        false,
    );
    let input = Rect::new(x, y + 260.0, surface.x1 - 32.0, y + 300.0);
    p.fill_rounded_rect(input, NEUTRAL_800, 6.0);
    p.stroke_rounded_rect(input, ACCENT_STUDY, 1.0, 6.0);
    p.draw_text(
        if state.input_text.is_empty() {
            t("study.practice.answer_prompt")
        } else {
            &state.input_text
        },
        input.x0 + 12.0,
        input.y0 + 25.0,
        if state.input_text.is_empty() {
            NEUTRAL_500
        } else {
            NEUTRAL_100
        },
        13.0,
        FontWeight::NORMAL,
        false,
    );

    let submit = submit_rect(surface);
    p.fill_rounded_rect(submit, ACCENT_STUDY, 6.0);
    p.draw_text(
        t("study.practice.submit"),
        submit.x0 + 20.0,
        submit.y0 + 24.0,
        NEUTRAL_800,
        13.0,
        FontWeight::BOLD,
        false,
    );

    // Phase 3: Math palette buttons
    if state.show_math_palette {
        let math_symbols = ["^", "sqrt", "frac", "pi", "alpha", "beta", "inf", "sum"];
        let btn_w = 40.0;
        let gap = 4.0;
        let palette_y = surface.y0 + 394.0;
        for (idx, symbol) in math_symbols.iter().enumerate() {
            let col = idx % 4;
            let row = idx / 4;
            let x = surface.x0 + 32.0 + col as f64 * (btn_w + gap);
            let y = palette_y + row as f64 * (28.0 + gap);
            let btn = Rect::new(x, y, x + btn_w, y + 28.0);
            p.fill_rounded_rect(btn, NEUTRAL_600, 4.0);
            p.stroke_rounded_rect(btn, NEUTRAL_500, 1.0, 4.0);
            p.draw_text(
                symbol,
                btn.x0 + btn_w / 2.0 - symbol.len() as f64 * 3.0,
                btn.y0 + 19.0,
                NEUTRAL_100,
                10.0,
                FontWeight::NORMAL,
                false,
            );
        }
    }

    // Phase 3: Skip button
    let skip = skip_rect(surface);
    p.stroke_rounded_rect(skip, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        t("study.practice.skip"),
        skip.x0 + 10.0,
        skip.y0 + 19.0,
        NEUTRAL_300,
        11.0,
        FontWeight::NORMAL,
        false,
    );

    // Phase 3: Pause/resume button
    let pause = pause_rect(surface);
    let pause_label = if state.session_paused {
        t("study.practice.resume")
    } else {
        t("study.practice.pause")
    };
    p.stroke_rounded_rect(pause, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        pause_label,
        pause.x0 + 10.0,
        pause.y0 + 19.0,
        NEUTRAL_300,
        11.0,
        FontWeight::NORMAL,
        false,
    );

    // Phase 3: Math palette toggle
    let math_btn = Rect::new(
        surface.x0 + 458.0,
        surface.y0 + 350.0,
        surface.x0 + 500.0,
        surface.y0 + 386.0,
    );
    p.stroke_rounded_rect(math_btn, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        "fx",
        math_btn.x0 + 10.0,
        math_btn.y0 + 24.0,
        if state.show_math_palette {
            ACCENT_STUDY
        } else {
            NEUTRAL_300
        },
        12.0,
        FontWeight::BOLD,
        false,
    );

    if let Some(correct) = state.feedback {
        let feedback = Rect::new(x, y + 354.0, surface.x1 - 32.0, y + 462.0);
        p.fill_rounded_rect(feedback, NEUTRAL_700, 8.0);
        p.stroke_rounded_rect(
            feedback,
            if correct { STATUS_READY } else { STATUS_ERROR },
            1.0,
            8.0,
        );
        p.draw_text(
            if correct {
                t("study.practice.correct")
            } else {
                t("study.practice.needs_review")
            },
            feedback.x0 + 16.0,
            feedback.y0 + 28.0,
            if correct { STATUS_READY } else { STATUS_ERROR },
            14.0,
            FontWeight::BOLD,
            false,
        );
        if let Some(problem) = state.current_problem() {
            p.draw_text(
                &problem.solution,
                feedback.x0 + 16.0,
                feedback.y0 + 62.0,
                NEUTRAL_100,
                12.0,
                FontWeight::NORMAL,
                false,
            );
            // Phase 3: Show cause_tag in practice feedback
            p.draw_text(
                &format!("{}: {}", t("study.practice.cause"), problem.cause_tag),
                feedback.x0 + 16.0,
                feedback.y0 + 82.0,
                NEUTRAL_400,
                10.0,
                FontWeight::NORMAL,
                false,
            );
        }
        paint_outline_button(p, retry_rect(surface), t("study.practice.retry"));
        paint_outline_button(
            p,
            review_concept_rect(surface),
            t("study.learn.review_concept"),
        );
        let next = next_rect(surface);
        p.fill_rounded_rect(next, ACCENT_STUDY, 6.0);
        p.draw_text(
            t("study.practice.next"),
            next.x0 + 24.0,
            next.y0 + 24.0,
            NEUTRAL_800,
            13.0,
            FontWeight::BOLD,
            false,
        );
    }
}

pub fn paint_review_surface(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    let surface = regions.surface;
    p.fill_rect(surface, NEUTRAL_800);
    let x = surface.x0 + 32.0;
    let y = surface.y0 + 34.0;
    p.draw_text(
        &format!(
            "{} {} / {}",
            t("study.review.title"),
            state.review_index,
            state.review_queue.len().max(1)
        ),
        x,
        y,
        NEUTRAL_100,
        18.0,
        FontWeight::BOLD,
        false,
    );
    if let Some(item) = state.current_review() {
        let card = Rect::new(x, y + 40.0, surface.x1 - 32.0, y + 260.0);
        p.fill_rounded_rect(card, NEUTRAL_600, 8.0);
        p.stroke_rounded_rect(card, NEUTRAL_500, 1.0, 8.0);
        p.draw_text(
            &item.problem_text,
            card.x0 + 16.0,
            card.y0 + 30.0,
            NEUTRAL_100,
            15.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            &format!("{}: {}", t("study.review.wrong"), item.wrong_answer),
            card.x0 + 16.0,
            card.y0 + 72.0,
            STATUS_ERROR,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!("{}: {}", t("study.review.correct"), item.correct_answer),
            card.x0 + 16.0,
            card.y0 + 104.0,
            STATUS_READY,
            13.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &item.solution,
            card.x0 + 16.0,
            card.y0 + 146.0,
            NEUTRAL_100,
            12.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!(
                "{}: {} / {}: {}",
                t("study.review.cause"),
                item.cause_tag,
                t("study.review.related"),
                item.related_concept
            ),
            card.x0 + 16.0,
            card.y0 + 198.0,
            NEUTRAL_300,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

pub fn submit_rect(surface: Rect) -> Rect {
    Rect::new(
        surface.x0 + 32.0,
        surface.y0 + 350.0,
        surface.x0 + 118.0,
        surface.y0 + 386.0,
    )
}

pub fn retry_rect(surface: Rect) -> Rect {
    Rect::new(
        surface.x0 + 32.0,
        surface.y0 + 514.0,
        surface.x0 + 100.0,
        surface.y0 + 550.0,
    )
}

pub fn review_concept_rect(surface: Rect) -> Rect {
    Rect::new(
        surface.x0 + 110.0,
        surface.y0 + 514.0,
        surface.x0 + 234.0,
        surface.y0 + 550.0,
    )
}

pub fn next_rect(surface: Rect) -> Rect {
    Rect::new(
        surface.x0 + 244.0,
        surface.y0 + 514.0,
        surface.x0 + 312.0,
        surface.y0 + 550.0,
    )
}

fn paint_outline_button(p: &mut Painter<'_>, rect: Rect, label: &str) {
    p.stroke_rounded_rect(rect, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        label,
        rect.x0 + 12.0,
        rect.y0 + 24.0,
        NEUTRAL_100,
        13.0,
        FontWeight::NORMAL,
        false,
    );
}

// Phase 3: Skip problem button rect
pub fn skip_rect(surface: Rect) -> Rect {
    Rect::new(
        surface.x0 + 320.0,
        surface.y0 + 350.0,
        surface.x0 + 384.0,
        surface.y0 + 386.0,
    )
}

// Phase 3: Pause/resume button rect
pub fn pause_rect(surface: Rect) -> Rect {
    Rect::new(
        surface.x0 + 394.0,
        surface.y0 + 350.0,
        surface.x0 + 448.0,
        surface.y0 + 386.0,
    )
}

// Phase 6: Spaced repetition rating button rect
pub fn rating_rect(surface: Rect, rating: &SpacedRepetitionRating) -> Rect {
    let idx = match rating {
        SpacedRepetitionRating::Again => 0,
        SpacedRepetitionRating::Hard => 1,
        SpacedRepetitionRating::Good => 2,
        SpacedRepetitionRating::Easy => 3,
    };
    let btn_w = 64.0;
    let gap = 8.0;
    let start_x = surface.x0 + 32.0;
    let x = start_x + idx as f64 * (btn_w + gap);
    Rect::new(x, surface.y0 + 470.0, x + btn_w, surface.y0 + 506.0)
}
