use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use crate::ui::curriculum::StudyRegions;
use crate::ui::state::{StudyFocusTarget, StudyState, TutorChatRole};
use crate::ui::state::{
    ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_400, NEUTRAL_500, NEUTRAL_600, NEUTRAL_700,
    STATUS_WARNING,
};

pub fn paint_tutor_panel(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    let tutor = regions.tutor;
    if tutor.width() < 160.0 {
        return;
    }
    p.fill_rect(tutor, NEUTRAL_700);
    p.draw_line(
        Point::new(tutor.x0, tutor.y0),
        Point::new(tutor.x0, tutor.y1),
        NEUTRAL_600,
        1.0,
    );
    p.draw_text(
        t("study.tutor.title"),
        tutor.x0 + 16.0,
        tutor.y0 + 28.0,
        NEUTRAL_100,
        16.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        t("study.tutor.context"),
        tutor.x0 + 16.0,
        tutor.y0 + 66.0,
        NEUTRAL_400,
        11.0,
        FontWeight::BOLD,
        false,
    );
    for (index, chip) in [
        state.active_subject.as_str(),
        state.active_unit().label.as_str(),
        state.active_concept().label.as_str(),
    ]
    .iter()
    .enumerate()
    {
        let rect = Rect::new(
            tutor.x0 + 16.0,
            tutor.y0 + 78.0 + index as f64 * 28.0,
            tutor.x1 - 16.0,
            tutor.y0 + 102.0 + index as f64 * 28.0,
        );
        p.fill_rounded_rect(rect, NEUTRAL_600, 999.0);
        p.draw_text(
            chip,
            rect.x0 + 10.0,
            rect.y0 + 17.0,
            NEUTRAL_100,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }

    p.draw_text(
        t("study.tutor.hints"),
        tutor.x0 + 16.0,
        tutor.y0 + 178.0,
        NEUTRAL_400,
        11.0,
        FontWeight::BOLD,
        false,
    );
    for level in 1..=3 {
        let rect = hint_rect(tutor, level);
        let unlocked = state.hint_level >= level;
        p.fill_rounded_rect(rect, if unlocked { NEUTRAL_600 } else { NEUTRAL_700 }, 6.0);
        p.stroke_rounded_rect(
            rect,
            if unlocked { ACCENT_STUDY } else { NEUTRAL_500 },
            1.0,
            6.0,
        );
        p.draw_text(
            if unlocked {
                hint_text(level, i18n)
            } else {
                t("study.tutor.reveal_hint")
            },
            rect.x0 + 10.0,
            rect.y0 + 23.0,
            if unlocked { ACCENT_STUDY } else { NEUTRAL_100 },
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }

    p.draw_text(
        t("study.tutor.weak_points"),
        tutor.x0 + 16.0,
        tutor.y0 + 342.0,
        NEUTRAL_400,
        11.0,
        FontWeight::BOLD,
        false,
    );
    let weak_points = state.weak_points();
    if weak_points.is_empty() {
        p.draw_text(
            t("study.tutor.no_weak_points"),
            tutor.x0 + 16.0,
            tutor.y0 + 368.0,
            NEUTRAL_400,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    } else {
        for (index, weak) in weak_points.iter().enumerate() {
            p.draw_text(
                weak,
                tutor.x0 + 16.0,
                tutor.y0 + 368.0 + index as f64 * 22.0,
                STATUS_WARNING,
                12.0,
                FontWeight::NORMAL,
                false,
            );
        }
    }

    p.draw_text(
        t("study.tutor.glossary"),
        tutor.x0 + 16.0,
        tutor.y0 + 472.0,
        NEUTRAL_400,
        11.0,
        FontWeight::BOLD,
        false,
    );
    // Phase 7: Glossary search input
    if state.glossary_search_focused || !state.glossary_search_query.is_empty() {
        let search_rect = Rect::new(
            tutor.x0 + 16.0,
            tutor.y0 + 458.0,
            tutor.x1 - 16.0,
            tutor.y0 + 474.0,
        );
        p.fill_rounded_rect(search_rect, NEUTRAL_600, 4.0);
        let border = if state.glossary_search_focused {
            ACCENT_STUDY
        } else {
            NEUTRAL_500
        };
        p.stroke_rounded_rect(search_rect, border, 1.0, 4.0);
        if state.glossary_search_query.is_empty() {
            p.draw_text(
                t("study.tutor.glossary_search"),
                search_rect.x0 + 6.0,
                search_rect.y0 + 11.0,
                NEUTRAL_500,
                9.0,
                FontWeight::NORMAL,
                false,
            );
        } else {
            p.draw_text(
                &state.glossary_search_query,
                search_rect.x0 + 6.0,
                search_rect.y0 + 11.0,
                NEUTRAL_100,
                9.0,
                FontWeight::NORMAL,
                false,
            );
        }
    }
    let mut glossary_y = tutor.y0 + 498.0;
    for (index, term) in state.active_glossary_terms().iter().take(3).enumerate() {
        // Phase 7: Filter by glossary search query
        if !state.glossary_search_query.is_empty() {
            let query = state.glossary_search_query.to_lowercase();
            if !term.term.to_lowercase().contains(&query)
                && !term.definition.to_lowercase().contains(&query)
            {
                continue;
            }
        }
        let is_expanded = state.expanded_glossary_idx == Some(index);
        let definition = if is_expanded {
            &term.definition
        } else {
            // Phase 7: Show clipped text normally, full on expand
            let clipped = clipped_text(&term.definition, 36);
            // Need to allocate for the clipped string
            p.draw_text(
                &term.term,
                tutor.x0 + 16.0,
                glossary_y,
                ACCENT_STUDY,
                12.0,
                FontWeight::BOLD,
                false,
            );
            p.draw_text(
                &clipped,
                tutor.x0 + 16.0,
                glossary_y + 18.0,
                NEUTRAL_300,
                10.0,
                FontWeight::NORMAL,
                false,
            );
            if is_expanded {
                glossary_y += 18.0;
            }
            glossary_y += 42.0;
            continue;
        };
        p.draw_text(
            &term.term,
            tutor.x0 + 16.0,
            glossary_y,
            ACCENT_STUDY,
            12.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            definition,
            tutor.x0 + 16.0,
            glossary_y + 18.0,
            NEUTRAL_300,
            10.0,
            FontWeight::NORMAL,
            false,
        );
        // Phase 7: Expand indicator
        p.draw_text(
            if is_expanded { "-" } else { "+" },
            tutor.x1 - 28.0,
            glossary_y,
            NEUTRAL_400,
            10.0,
            FontWeight::NORMAL,
            false,
        );
        if is_expanded {
            glossary_y += 18.0;
        }
        glossary_y += 42.0;
    }

    p.draw_text(
        &format!("{}: {}%", t("study.stats.accuracy"), state.accuracy()),
        tutor.x0 + 16.0,
        tutor.y1 - 70.0,
        NEUTRAL_300,
        12.0,
        FontWeight::NORMAL,
        false,
    );
    p.draw_text(
        &format!("{}: {}", t("study.stats.reviews"), state.review_queue.len()),
        tutor.x0 + 16.0,
        tutor.y1 - 48.0,
        NEUTRAL_300,
        12.0,
        FontWeight::NORMAL,
        false,
    );

    // Phase 7: Tutor chat input and messages
    let chat_input_y = tutor.y1 - 44.0;
    let chat_input = Rect::new(
        tutor.x0 + 16.0,
        chat_input_y,
        tutor.x1 - 52.0,
        chat_input_y + 28.0,
    );
    let chat_border = if state.focus_target == StudyFocusTarget::TutorChat {
        ACCENT_STUDY
    } else {
        NEUTRAL_500
    };
    p.fill_rounded_rect(chat_input, NEUTRAL_600, 6.0);
    p.stroke_rounded_rect(chat_input, chat_border, 1.0, 6.0);
    if state.tutor_chat_input.is_empty() {
        p.draw_text(
            t("study.tutor.chat_placeholder"),
            chat_input.x0 + 8.0,
            chat_input.y0 + 19.0,
            NEUTRAL_500,
            10.0,
            FontWeight::NORMAL,
            false,
        );
    } else {
        p.draw_text(
            &state.tutor_chat_input,
            chat_input.x0 + 8.0,
            chat_input.y0 + 19.0,
            NEUTRAL_100,
            10.0,
            FontWeight::NORMAL,
            false,
        );
    }

    // Phase 7: Chat messages (show last 3 above input)
    let mut msg_y = chat_input_y - 8.0;
    for msg in state.tutor_chat_messages.iter().rev().take(3) {
        msg_y -= 28.0;
        let msg_color = match msg.role {
            TutorChatRole::User => ACCENT_STUDY,
            TutorChatRole::Assistant => NEUTRAL_300,
        };
        p.draw_text(
            &msg.text,
            tutor.x0 + 16.0,
            msg_y,
            msg_color,
            10.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

pub fn hint_rect(tutor: Rect, level: u8) -> Rect {
    Rect::new(
        tutor.x0 + 16.0,
        tutor.y0 + 194.0 + (level as f64 - 1.0) * 46.0,
        tutor.x1 - 16.0,
        tutor.y0 + 230.0 + (level as f64 - 1.0) * 46.0,
    )
}

fn hint_text(level: u8, i18n: &tench_app_core::I18nCatalog) -> &str {
    match level {
        1 => crate::i18n::resolve(i18n, "study.hint.one"),
        2 => crate::i18n::resolve(i18n, "study.hint.two"),
        _ => crate::i18n::resolve(i18n, "study.hint.three"),
    }
}

fn clipped_text(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut output = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    output.push_str("...");
    output
}
