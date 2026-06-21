use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::layout::modal_rect;
use crate::ui::state::{ProfileField, ProfileSetupStep, StudyHit, StudyState};
use crate::ui::state::{
    ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_400, NEUTRAL_500, NEUTRAL_600, NEUTRAL_700,
    NEUTRAL_800, NEUTRAL_900,
};

pub fn paint_profile_setup_wizard(
    p: &mut Painter<'_>,
    state: &StudyState,
    size: Size,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    // Dim background
    p.fill_rect(
        Rect::new(0.0, 0.0, size.width, size.height),
        Color::rgba8(0, 0, 0, 180),
    );
    let modal = modal_rect(size);
    p.fill_rounded_rect(modal, NEUTRAL_800, 12.0);
    p.stroke_rounded_rect(modal, NEUTRAL_600, 1.0, 12.0);

    let title = match state.profile_setup_step {
        ProfileSetupStep::Identity => t("study.profile.identity_title"),
        ProfileSetupStep::DomainLevel => t("study.profile.domain_level_title"),
        ProfileSetupStep::Locale => t("study.profile.locale_title"),
        ProfileSetupStep::Done => t("study.profile.done_title"),
    };
    p.draw_text(
        title,
        modal.x0 + 24.0,
        modal.y0 + 36.0,
        NEUTRAL_100,
        18.0,
        FontWeight::BOLD,
        false,
    );

    match state.profile_setup_step {
        ProfileSetupStep::Identity => {
            // Learner ID field
            p.draw_text(
                t("study.profile.learner_id"),
                modal.x0 + 24.0,
                modal.y0 + 72.0,
                NEUTRAL_300,
                12.0,
                FontWeight::BOLD,
                false,
            );
            let id_field = Rect::new(
                modal.x0 + 24.0,
                modal.y0 + 86.0,
                modal.x1 - 24.0,
                modal.y0 + 114.0,
            );
            let id_border = if state.wizard_active_field == ProfileField::LearnerId {
                ACCENT_STUDY
            } else {
                NEUTRAL_500
            };
            p.fill_rounded_rect(id_field, NEUTRAL_700, 6.0);
            p.stroke_rounded_rect(id_field, id_border, 1.0, 6.0);
            if state.wizard_learner_id.is_empty() {
                p.draw_text(
                    t("study.profile.learner_id_placeholder"),
                    id_field.x0 + 10.0,
                    id_field.y0 + 19.0,
                    NEUTRAL_500,
                    12.0,
                    FontWeight::NORMAL,
                    false,
                );
            } else {
                p.draw_text(
                    &state.wizard_learner_id,
                    id_field.x0 + 10.0,
                    id_field.y0 + 19.0,
                    NEUTRAL_100,
                    12.0,
                    FontWeight::NORMAL,
                    false,
                );
            }

            // Display name field
            p.draw_text(
                t("study.profile.display_name"),
                modal.x0 + 24.0,
                modal.y0 + 130.0,
                NEUTRAL_300,
                12.0,
                FontWeight::BOLD,
                false,
            );
            let name_field = Rect::new(
                modal.x0 + 24.0,
                modal.y0 + 144.0,
                modal.x1 - 24.0,
                modal.y0 + 172.0,
            );
            let name_border = if state.wizard_active_field == ProfileField::DisplayName {
                ACCENT_STUDY
            } else {
                NEUTRAL_500
            };
            p.fill_rounded_rect(name_field, NEUTRAL_700, 6.0);
            p.stroke_rounded_rect(name_field, name_border, 1.0, 6.0);
            if state.wizard_display_name.is_empty() {
                p.draw_text(
                    t("study.profile.display_name_placeholder"),
                    name_field.x0 + 10.0,
                    name_field.y0 + 19.0,
                    NEUTRAL_500,
                    12.0,
                    FontWeight::NORMAL,
                    false,
                );
            } else {
                p.draw_text(
                    &state.wizard_display_name,
                    name_field.x0 + 10.0,
                    name_field.y0 + 19.0,
                    NEUTRAL_100,
                    12.0,
                    FontWeight::NORMAL,
                    false,
                );
            }
        }
        ProfileSetupStep::DomainLevel => {
            // Domain picker
            p.draw_text(
                t("study.profile.select_domain"),
                modal.x0 + 24.0,
                modal.y0 + 72.0,
                NEUTRAL_300,
                12.0,
                FontWeight::BOLD,
                false,
            );
            let domains: Vec<_> = state.units.iter().map(|u| u.domain.clone()).collect();
            for (idx, domain) in domains.iter().enumerate() {
                let y = modal.y0 + 90.0 + idx as f64 * 36.0;
                let rect = Rect::new(modal.x0 + 24.0, y, modal.x1 - 24.0, y + 32.0);
                let selected = state.wizard_domain_idx == idx;
                p.fill_rounded_rect(rect, if selected { NEUTRAL_600 } else { NEUTRAL_700 }, 6.0);
                if selected {
                    p.stroke_rounded_rect(rect, ACCENT_STUDY, 1.0, 6.0);
                }
                p.draw_text(
                    domain_label(domain),
                    rect.x0 + 12.0,
                    rect.y0 + 21.0,
                    if selected { ACCENT_STUDY } else { NEUTRAL_300 },
                    12.0,
                    FontWeight::NORMAL,
                    false,
                );
            }

            // Level picker
            let levels = tench_study_core::EducationLevel::all();
            let level_y_start = modal.y0 + 90.0 + domains.len() as f64 * 36.0 + 16.0;
            p.draw_text(
                t("study.profile.select_level"),
                modal.x0 + 24.0,
                level_y_start,
                NEUTRAL_300,
                12.0,
                FontWeight::BOLD,
                false,
            );
            for (idx, level) in levels.iter().enumerate() {
                let y = level_y_start + 18.0 + idx as f64 * 30.0;
                let rect = Rect::new(modal.x0 + 24.0, y, modal.x1 - 24.0, y + 26.0);
                let selected = state.wizard_level_idx == idx;
                p.fill_rounded_rect(rect, if selected { NEUTRAL_600 } else { NEUTRAL_700 }, 4.0);
                if selected {
                    p.stroke_rounded_rect(rect, ACCENT_STUDY, 1.0, 4.0);
                }
                p.draw_text(
                    level.label(),
                    rect.x0 + 12.0,
                    rect.y0 + 18.0,
                    if selected { ACCENT_STUDY } else { NEUTRAL_400 },
                    11.0,
                    FontWeight::NORMAL,
                    false,
                );
            }
        }
        ProfileSetupStep::Locale => {
            // Locale picker
            p.draw_text(
                t("study.profile.select_locale"),
                modal.x0 + 24.0,
                modal.y0 + 72.0,
                NEUTRAL_300,
                12.0,
                FontWeight::BOLD,
                false,
            );
            let locales = ["en-US", "ko-KR", "ja-JP", "zh-CN"];
            for (idx, locale) in locales.iter().enumerate() {
                let y = modal.y0 + 90.0 + idx as f64 * 36.0;
                let rect = Rect::new(modal.x0 + 24.0, y, modal.x1 - 24.0, y + 32.0);
                let selected = state.wizard_locale_idx == idx;
                p.fill_rounded_rect(rect, if selected { NEUTRAL_600 } else { NEUTRAL_700 }, 6.0);
                if selected {
                    p.stroke_rounded_rect(rect, ACCENT_STUDY, 1.0, 6.0);
                }
                p.draw_text(
                    locale,
                    rect.x0 + 12.0,
                    rect.y0 + 21.0,
                    if selected { ACCENT_STUDY } else { NEUTRAL_300 },
                    12.0,
                    FontWeight::NORMAL,
                    false,
                );
            }
        }
        ProfileSetupStep::Done => {
            p.draw_text(
                t("study.profile.setup_complete"),
                modal.x0 + 24.0,
                modal.y0 + 80.0,
                ACCENT_STUDY,
                14.0,
                FontWeight::BOLD,
                false,
            );
        }
    }

    // Navigation buttons
    let next_btn = Rect::new(
        modal.x1 - 120.0,
        modal.y1 - 52.0,
        modal.x1 - 24.0,
        modal.y1 - 24.0,
    );
    p.fill_rounded_rect(next_btn, ACCENT_STUDY, 6.0);
    let next_label = match state.profile_setup_step {
        ProfileSetupStep::Done => t("study.profile.start"),
        _ => t("study.profile.next"),
    };
    p.draw_text(
        next_label,
        next_btn.x0 + 20.0,
        next_btn.y0 + 19.0,
        NEUTRAL_900,
        12.0,
        FontWeight::BOLD,
        false,
    );

    if state.profile_setup_step != ProfileSetupStep::Identity {
        let back_btn = Rect::new(
            modal.x0 + 24.0,
            modal.y1 - 52.0,
            modal.x0 + 100.0,
            modal.y1 - 24.0,
        );
        p.stroke_rounded_rect(back_btn, NEUTRAL_500, 1.0, 6.0);
        p.draw_text(
            t("study.profile.back"),
            back_btn.x0 + 16.0,
            back_btn.y0 + 19.0,
            NEUTRAL_300,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

pub fn hit_test_profile_setup(step: ProfileSetupStep, size: Size, pos: Point) -> Option<StudyHit> {
    let modal = modal_rect(size);

    // Next button
    let next_btn = Rect::new(
        modal.x1 - 120.0,
        modal.y1 - 52.0,
        modal.x1 - 24.0,
        modal.y1 - 24.0,
    );
    if next_btn.contains(pos) {
        return Some(StudyHit::ProfileSetupNext);
    }

    // Back button (not on first step)
    if step != ProfileSetupStep::Identity {
        let back_btn = Rect::new(
            modal.x0 + 24.0,
            modal.y1 - 52.0,
            modal.x0 + 100.0,
            modal.y1 - 24.0,
        );
        if back_btn.contains(pos) {
            return Some(StudyHit::ProfileSetupBack);
        }
    }

    // Phase 1: Click on learner_id or display_name field to focus
    if step == ProfileSetupStep::Identity {
        let id_field = Rect::new(
            modal.x0 + 24.0,
            modal.y0 + 86.0,
            modal.x1 - 24.0,
            modal.y0 + 114.0,
        );
        if id_field.contains(pos) {
            return Some(StudyHit::ProfileSetupFocusLearnerId);
        }
        let name_field = Rect::new(
            modal.x0 + 24.0,
            modal.y0 + 144.0,
            modal.x1 - 24.0,
            modal.y0 + 172.0,
        );
        if name_field.contains(pos) {
            return Some(StudyHit::ProfileSetupFocusDisplayName);
        }
    }

    match step {
        ProfileSetupStep::DomainLevel => {
            let domains_count = 4; // We have 4 domains typically
            for idx in 0..domains_count {
                let y = modal.y0 + 90.0 + idx as f64 * 36.0;
                let rect = Rect::new(modal.x0 + 24.0, y, modal.x1 - 24.0, y + 32.0);
                if rect.contains(pos) {
                    return Some(StudyHit::ProfileDomainSelect(idx));
                }
            }
            let levels = tench_study_core::EducationLevel::all();
            let level_y_start = modal.y0 + 90.0 + domains_count as f64 * 36.0 + 16.0;
            for idx in 0..levels.len() {
                let y = level_y_start + 18.0 + idx as f64 * 30.0;
                let rect = Rect::new(modal.x0 + 24.0, y, modal.x1 - 24.0, y + 26.0);
                if rect.contains(pos) {
                    return Some(StudyHit::ProfileLevelSelect(idx));
                }
            }
        }
        ProfileSetupStep::Locale => {
            let locales_count = 4;
            for idx in 0..locales_count {
                let y = modal.y0 + 90.0 + idx as f64 * 36.0;
                let rect = Rect::new(modal.x0 + 24.0, y, modal.x1 - 24.0, y + 32.0);
                if rect.contains(pos) {
                    return Some(StudyHit::ProfileLocaleSelect(idx));
                }
            }
        }
        ProfileSetupStep::Identity | ProfileSetupStep::Done => {}
    }

    None
}

fn domain_label(domain: &tench_study_core::SubjectDomain) -> &'static str {
    match domain {
        tench_study_core::SubjectDomain::Mathematics => "Mathematics",
        tench_study_core::SubjectDomain::Science => "Science",
        tench_study_core::SubjectDomain::Language => "Language",
        tench_study_core::SubjectDomain::Programming => "Programming",
        tench_study_core::SubjectDomain::Custom { label, .. } => {
            // Return a static fallback; custom labels are dynamic but we need
            // a static str for draw_text. The caller should handle this.
            let _ = label;
            "Custom"
        }
    }
}
