use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::layout::{modal_close_rect, modal_rect};
use crate::ui::state::{StudyHit, StudyState};
use crate::ui::state::{
    ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_500, NEUTRAL_700, NEUTRAL_800, NEUTRAL_900,
    STATUS_READY,
};

pub fn paint_authoring_panel(
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
        t("study.authoring.title"),
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

    // Title field
    p.draw_text(
        t("study.authoring.curriculum_title"),
        modal.x0 + 18.0,
        modal.y0 + 72.0,
        NEUTRAL_300,
        12.0,
        FontWeight::BOLD,
        false,
    );
    let title_field = Rect::new(
        modal.x0 + 18.0,
        modal.y0 + 86.0,
        modal.x1 - 18.0,
        modal.y0 + 114.0,
    );
    p.fill_rounded_rect(title_field, NEUTRAL_700, 6.0);
    p.stroke_rounded_rect(title_field, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        if state.authoring_title.is_empty() {
            t("study.authoring.title_placeholder")
        } else {
            &state.authoring_title
        },
        title_field.x0 + 10.0,
        title_field.y0 + 19.0,
        if state.authoring_title.is_empty() {
            NEUTRAL_500
        } else {
            NEUTRAL_100
        },
        12.0,
        FontWeight::NORMAL,
        false,
    );

    // Body field
    p.draw_text(
        t("study.authoring.body"),
        modal.x0 + 18.0,
        modal.y0 + 130.0,
        NEUTRAL_300,
        12.0,
        FontWeight::BOLD,
        false,
    );
    let body_field = Rect::new(
        modal.x0 + 18.0,
        modal.y0 + 144.0,
        modal.x1 - 18.0,
        modal.y1 - 96.0,
    );
    p.fill_rounded_rect(body_field, NEUTRAL_700, 6.0);
    p.stroke_rounded_rect(body_field, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        if state.authoring_body.is_empty() {
            t("study.authoring.body_placeholder")
        } else {
            &state.authoring_body
        },
        body_field.x0 + 10.0,
        body_field.y0 + 19.0,
        if state.authoring_body.is_empty() {
            NEUTRAL_500
        } else {
            NEUTRAL_100
        },
        12.0,
        FontWeight::NORMAL,
        false,
    );

    // New Unit button
    let new_unit_btn = Rect::new(
        modal.x0 + 18.0,
        modal.y1 - 88.0,
        modal.x0 + 120.0,
        modal.y1 - 60.0,
    );
    p.fill_rounded_rect(new_unit_btn, NEUTRAL_700, 6.0);
    p.stroke_rounded_rect(new_unit_btn, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        t("study.authoring.new_unit"),
        new_unit_btn.x0 + 10.0,
        new_unit_btn.y0 + 19.0,
        NEUTRAL_100,
        11.0,
        FontWeight::BOLD,
        false,
    );

    // New Concept button
    let new_concept_btn = Rect::new(
        modal.x0 + 126.0,
        modal.y1 - 88.0,
        modal.x0 + 248.0,
        modal.y1 - 60.0,
    );
    p.fill_rounded_rect(new_concept_btn, NEUTRAL_700, 6.0);
    p.stroke_rounded_rect(new_concept_btn, NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        t("study.authoring.new_concept"),
        new_concept_btn.x0 + 10.0,
        new_concept_btn.y0 + 19.0,
        NEUTRAL_100,
        11.0,
        FontWeight::BOLD,
        false,
    );

    // Action buttons
    let new_curriculum_btn = Rect::new(
        modal.x0 + 18.0,
        modal.y1 - 56.0,
        modal.x0 + 150.0,
        modal.y1 - 28.0,
    );
    p.fill_rounded_rect(new_curriculum_btn, ACCENT_STUDY, 6.0);
    p.draw_text(
        t("study.authoring.new_curriculum"),
        new_curriculum_btn.x0 + 10.0,
        new_curriculum_btn.y0 + 19.0,
        NEUTRAL_900,
        11.0,
        FontWeight::BOLD,
        false,
    );

    let save_btn = Rect::new(
        modal.x1 - 120.0,
        modal.y1 - 56.0,
        modal.x1 - 18.0,
        modal.y1 - 28.0,
    );
    p.fill_rounded_rect(save_btn, STATUS_READY, 6.0);
    p.draw_text(
        t("study.authoring.save_draft"),
        save_btn.x0 + 14.0,
        save_btn.y0 + 19.0,
        NEUTRAL_900,
        11.0,
        FontWeight::BOLD,
        false,
    );
}

pub fn hit_test_authoring(size: Size, _state: &StudyState, pos: Point) -> Option<StudyHit> {
    let modal = modal_rect(size);

    // Close button
    let close = modal_close_rect(size);
    if close.contains(pos) {
        return Some(StudyHit::CloseModal);
    }

    // Buttons first (they sit below the body field in the layout, but we
    // check them before the body field so that clicks on buttons are not
    // swallowed by the overlapping body field hit area).
    // New Unit button
    let new_unit_btn = Rect::new(
        modal.x0 + 18.0,
        modal.y1 - 88.0,
        modal.x0 + 120.0,
        modal.y1 - 60.0,
    );
    if new_unit_btn.contains(pos) {
        return Some(StudyHit::AuthoringNewUnit);
    }

    // New Concept button
    let new_concept_btn = Rect::new(
        modal.x0 + 126.0,
        modal.y1 - 88.0,
        modal.x0 + 248.0,
        modal.y1 - 60.0,
    );
    if new_concept_btn.contains(pos) {
        return Some(StudyHit::AuthoringNewConcept);
    }

    // New Curriculum button
    let new_curriculum_btn = Rect::new(
        modal.x0 + 18.0,
        modal.y1 - 56.0,
        modal.x0 + 150.0,
        modal.y1 - 28.0,
    );
    if new_curriculum_btn.contains(pos) {
        return Some(StudyHit::AuthoringNewCurriculum);
    }

    // Save Draft button
    let save_btn = Rect::new(
        modal.x1 - 120.0,
        modal.y1 - 56.0,
        modal.x1 - 18.0,
        modal.y1 - 28.0,
    );
    if save_btn.contains(pos) {
        return Some(StudyHit::AuthoringSaveDraft);
    }

    // Title field
    let title_field = Rect::new(
        modal.x0 + 18.0,
        modal.y0 + 86.0,
        modal.x1 - 18.0,
        modal.y0 + 114.0,
    );
    if title_field.contains(pos) {
        return Some(StudyHit::AuthoringTitleFocus);
    }

    // Body field (clamped to stop above the button row)
    let body_field = Rect::new(
        modal.x0 + 18.0,
        modal.y0 + 144.0,
        modal.x1 - 18.0,
        modal.y1 - 96.0,
    );
    if body_field.contains(pos) {
        return Some(StudyHit::AuthoringBodyFocus);
    }

    None
}
