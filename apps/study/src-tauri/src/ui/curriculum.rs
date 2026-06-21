use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::state::{
    ConceptStatus, SpacedRepetitionRating, Stage, StudyHit, StudyState, ACCENT_STUDY, NEUTRAL_100,
    NEUTRAL_300, NEUTRAL_400, NEUTRAL_500, NEUTRAL_600, NEUTRAL_700, NEUTRAL_800, NEUTRAL_900,
    STATUS_WARNING,
};

const HEADER_H: f64 = 40.0;
const NAV_W: f64 = 180.0;
const PANEL_W: f64 = 260.0;

pub struct StudyRegions {
    pub header: Rect,
    pub outline: Rect,
    pub surface: Rect,
    pub tutor: Rect,
}

pub fn regions(size: Size) -> StudyRegions {
    let outline_w = if size.width < 520.0 {
        (size.width * 0.30).clamp(88.0, NAV_W)
    } else {
        NAV_W
    };
    let tutor_w = if size.width < 900.0 { 0.0 } else { PANEL_W };
    let surface_x0 = outline_w.min(size.width);
    let surface_x1 = (size.width - tutor_w).max(surface_x0);
    StudyRegions {
        header: Rect::new(0.0, 0.0, size.width, HEADER_H),
        outline: Rect::new(0.0, HEADER_H, surface_x0, size.height),
        surface: Rect::new(surface_x0, HEADER_H, surface_x1, size.height),
        tutor: Rect::new(surface_x1, HEADER_H, size.width, size.height),
    }
}

pub fn paint_shell(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);

    p.fill_rect(
        Rect::new(0.0, 0.0, regions.header.x1, regions.tutor.y1),
        NEUTRAL_900,
    );
    p.fill_rect(regions.header, NEUTRAL_800);
    p.draw_line(
        Point::new(regions.header.x0, regions.header.y1 - 1.0),
        Point::new(regions.header.x1, regions.header.y1 - 1.0),
        NEUTRAL_600,
        1.0,
    );
    p.draw_text(
        &state.active_subject,
        16.0,
        25.0,
        NEUTRAL_300,
        12.0,
        FontWeight::BOLD,
        false,
    );
    if regions.header.width() >= 560.0 {
        p.draw_text(
            "/",
            116.0,
            25.0,
            NEUTRAL_500,
            12.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &state.active_unit().label,
            132.0,
            25.0,
            NEUTRAL_300,
            12.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            "/",
            210.0,
            25.0,
            NEUTRAL_500,
            12.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &state.active_concept().label,
            226.0,
            25.0,
            NEUTRAL_100,
            12.0,
            FontWeight::BOLD,
            false,
        );
    }

    // Phase 10: High contrast toggle button in header
    let hc_btn = Rect::new(
        regions.header.x0 + 64.0,
        regions.header.y0 + 8.0,
        regions.header.x0 + 84.0,
        regions.header.y0 + 32.0,
    );
    p.stroke_rounded_rect(hc_btn, NEUTRAL_500, 1.0, 4.0);
    p.draw_text(
        if state.high_contrast_mode { "HC" } else { "hc" },
        hc_btn.x0 + 6.0,
        hc_btn.y0 + 15.0,
        if state.high_contrast_mode {
            ACCENT_STUDY
        } else {
            NEUTRAL_400
        },
        10.0,
        FontWeight::BOLD,
        false,
    );

    let stage = stage_rect(regions.header);
    p.fill_rounded_rect(stage, NEUTRAL_700, 999.0);
    let stage_label = match state.stage {
        Stage::Learn => t("study.stage.learn"),
        Stage::Practice => t("study.stage.practice"),
        Stage::Review => t("study.stage.review"),
    };
    p.draw_text(
        stage_label,
        stage.x0 + 12.0,
        stage.y0 + 15.0,
        state.stage.color(),
        11.0,
        FontWeight::BOLD,
        false,
    );

    if regions.header.width() >= 560.0 {
        p.draw_text(
            &format!("{} {}", t("study.header.streak"), state.streak),
            regions.header.x1 - 220.0,
            25.0,
            STATUS_WARNING,
            12.0,
            FontWeight::BOLD,
            false,
        );
        // Phase 11: HH:MM:SS timer format
        p.draw_text(
            &format!(
                "{:02}:{:02}:{:02}",
                state.elapsed_seconds / 3600,
                (state.elapsed_seconds % 3600) / 60,
                state.elapsed_seconds % 60
            ),
            regions.header.x1 - 142.0,
            25.0,
            NEUTRAL_400,
            11.0,
            FontWeight::NORMAL,
            false,
        );
    }
    p.stroke_rounded_rect(stats_rect(regions.header), NEUTRAL_500, 1.0, 6.0);
    p.draw_text(
        t("study.header.stats"),
        regions.header.x1 - 66.0,
        25.0,
        NEUTRAL_100,
        12.0,
        FontWeight::NORMAL,
        false,
    );

    // Phase 5: Daily dashboard mini-widget in header
    if regions.header.width() >= 700.0 {
        let dash_x = regions.header.x1 - 320.0;
        p.draw_text(
            &format!(
                "{}:{}",
                t("study.stats.due"),
                state.dashboard.due_review_count
            ),
            dash_x,
            25.0,
            STATUS_WARNING,
            10.0,
            FontWeight::BOLD,
            false,
        );
        p.draw_text(
            &format!(
                "{}:{}",
                t("study.stats.new"),
                state.dashboard.new_lesson_count
            ),
            dash_x + 60.0,
            25.0,
            NEUTRAL_400,
            10.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &format!(
                "{}:{}%",
                t("study.stats.accuracy"),
                state.dashboard.accuracy_percent
            ),
            dash_x + 120.0,
            25.0,
            NEUTRAL_400,
            10.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

pub fn paint_outline(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);

    p.fill_rect(regions.outline, NEUTRAL_700);
    p.draw_line(
        Point::new(regions.outline.x1 - 1.0, regions.outline.y0),
        Point::new(regions.outline.x1 - 1.0, regions.outline.y1),
        NEUTRAL_600,
        1.0,
    );

    // Phase 2: Active search box with text input
    let search = search_rect(regions.outline);
    let search_border = if state.search_focused {
        ACCENT_STUDY
    } else {
        NEUTRAL_600
    };
    p.fill_rounded_rect(search, NEUTRAL_800, 4.0);
    p.stroke_rounded_rect(search, search_border, 1.0, 4.0);
    if state.search_query.is_empty() {
        p.draw_text(
            t("study.curriculum.search"),
            search.x0 + 8.0,
            search.y0 + 19.0,
            NEUTRAL_500,
            12.0,
            FontWeight::NORMAL,
            false,
        );
        // Phase 2: Search cursor when empty and focused
        if state.search_focused {
            let cursor_x = search.x0 + 8.0;
            p.fill_rect(
                Rect::new(cursor_x, search.y0 + 6.0, cursor_x + 1.0, search.y1 - 6.0),
                NEUTRAL_100,
            );
        }
    } else {
        p.draw_text(
            &state.search_query,
            search.x0 + 8.0,
            search.y0 + 19.0,
            NEUTRAL_100,
            12.0,
            FontWeight::NORMAL,
            false,
        );
        // Phase 2: Search cursor at end of text
        if state.search_focused {
            let text_width = state.search_query.len() as f64 * 7.0;
            let cursor_x = search.x0 + 8.0 + text_width;
            if cursor_x < search.x1 - 4.0 {
                p.fill_rect(
                    Rect::new(cursor_x, search.y0 + 6.0, cursor_x + 1.0, search.y1 - 6.0),
                    NEUTRAL_100,
                );
            }
        }
    }

    // Phase 2: Search match count indicator
    if !state.search_query.is_empty() {
        let matches = state.search_matches(&state.search_query);
        let match_text = format!("{}", matches.len());
        p.draw_text(
            &match_text,
            regions.outline.x1 - 84.0,
            regions.outline.y0 + 19.0,
            ACCENT_STUDY,
            10.0,
            FontWeight::BOLD,
            false,
        );
    }

    // Phase 2: Bookmark toggle button next to search
    let bookmark_rect = Rect::new(
        regions.outline.x1 - 36.0,
        regions.outline.y0 + 8.0,
        regions.outline.x1 - 12.0,
        regions.outline.y0 + 36.0,
    );
    let is_bookmarked = state
        .bookmarked_concept_ids
        .contains(&state.active_concept().id);
    p.draw_text(
        if is_bookmarked {
            "\u{2605}"
        } else {
            "\u{2606}"
        },
        bookmark_rect.x0 + 4.0,
        bookmark_rect.y0 + 19.0,
        if is_bookmarked {
            STATUS_WARNING
        } else {
            NEUTRAL_400
        },
        14.0,
        FontWeight::NORMAL,
        false,
    );

    // Phase 2: Notes toggle button
    let notes_btn = Rect::new(
        regions.outline.x1 - 60.0,
        regions.outline.y0 + 8.0,
        regions.outline.x1 - 40.0,
        regions.outline.y0 + 36.0,
    );
    p.draw_text(
        "N",
        notes_btn.x0 + 2.0,
        notes_btn.y0 + 19.0,
        if state.show_notes_panel {
            ACCENT_STUDY
        } else {
            NEUTRAL_400
        },
        12.0,
        FontWeight::NORMAL,
        false,
    );

    // Phase 2: Virtual scroll - compute visible range instead of breaking
    let scroll_y = state.outline_scroll_offset;
    let visible_top = regions.outline.y0 + 44.0;
    let visible_bottom = review_queue_rect(regions.outline).y0;

    // Phase 2: Unit expand/collapse support
    for (unit_idx, unit) in state.units.iter().enumerate() {
        let unit_y = unit_header_y(unit_idx) - scroll_y;
        if unit_y + 36.0 < visible_top {
            continue;
        }
        if unit_y > visible_bottom {
            break;
        }
        let expanded = state.expanded_units.get(unit_idx).copied().unwrap_or(true);
        p.draw_text(
            if expanded { "v" } else { ">" },
            regions.outline.x0 + 12.0,
            unit_y + 21.0,
            NEUTRAL_400,
            10.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &unit.label,
            regions.outline.x0 + 30.0,
            unit_y + 22.0,
            NEUTRAL_100,
            14.0,
            FontWeight::BOLD,
            false,
        );
        // Phase 11: Concept progress display per unit
        let completed = unit
            .concepts
            .iter()
            .filter(|c| c.status == ConceptStatus::Completed)
            .count();
        let total = unit.concepts.len();
        if total > 0 {
            p.draw_text(
                &format!("{}/{}", completed, total),
                regions.outline.x1 - 50.0,
                unit_y + 22.0,
                NEUTRAL_400,
                10.0,
                FontWeight::NORMAL,
                false,
            );
        }

        if !expanded {
            continue;
        }

        for (concept_idx, concept) in unit.concepts.iter().enumerate() {
            let rect = concept_rect(unit_idx, concept_idx, regions.outline);
            let adjusted_rect = Rect::new(rect.x0, rect.y0 - scroll_y, rect.x1, rect.y1 - scroll_y);
            if adjusted_rect.y1 < visible_top || adjusted_rect.y0 > visible_bottom {
                continue;
            }
            let active =
                state.active_unit_idx == unit_idx && state.active_concept_idx == concept_idx;
            if active {
                p.fill_rect(adjusted_rect, NEUTRAL_500);
                p.fill_rect(
                    Rect::new(
                        adjusted_rect.x0,
                        adjusted_rect.y0 + 4.0,
                        adjusted_rect.x0 + 2.0,
                        adjusted_rect.y1 - 4.0,
                    ),
                    ACCENT_STUDY,
                );
            }
            let icon = match concept.status {
                ConceptStatus::Completed => "\u{2713}",
                ConceptStatus::Active => "\u{2022}",
                ConceptStatus::InProgress => "\u{25CB}",
                ConceptStatus::Warning => "!",
            };
            p.draw_text(
                icon,
                adjusted_rect.x0 + 26.0,
                adjusted_rect.y0 + 22.0,
                status_color(concept.status),
                11.0,
                FontWeight::BOLD,
                false,
            );
            // Phase 2: Bookmark indicator
            let is_bookmarked = state.bookmarked_concept_ids.contains(&concept.id);
            if is_bookmarked {
                p.draw_text(
                    "\u{2605}",
                    adjusted_rect.x1 - 18.0,
                    adjusted_rect.y0 + 22.0,
                    STATUS_WARNING,
                    10.0,
                    FontWeight::NORMAL,
                    false,
                );
            }
            p.draw_text(
                &concept.label,
                adjusted_rect.x0 + 44.0,
                adjusted_rect.y0 + 22.0,
                if active { ACCENT_STUDY } else { NEUTRAL_300 },
                13.0,
                if active {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                },
                false,
            );
        }
    }

    let review = review_queue_rect(regions.outline);
    p.fill_rect(review, NEUTRAL_800);
    p.draw_line(
        Point::new(review.x0, review.y0),
        Point::new(review.x1, review.y0),
        NEUTRAL_600,
        1.0,
    );
    p.draw_text(
        t("study.review.queue"),
        review.x0 + 38.0,
        review.y0 + 25.0,
        STATUS_WARNING,
        12.0,
        FontWeight::BOLD,
        false,
    );
    p.fill_rounded_rect(
        Rect::new(
            review.x1 - 42.0,
            review.y0 + 11.0,
            review.x1 - 22.0,
            review.y0 + 29.0,
        ),
        STATUS_WARNING,
        999.0,
    );
    p.draw_text(
        &state.review_queue.len().to_string(),
        review.x1 - 36.0,
        review.y0 + 24.0,
        NEUTRAL_900,
        10.0,
        FontWeight::BOLD,
        false,
    );
}

pub fn hit_test(regions: &StudyRegions, state: &StudyState, pos: Point) -> Option<StudyHit> {
    if state.show_notes_panel && state.stage == Stage::Learn {
        if super::learn::notes_input_rect(regions).contains(pos) {
            return Some(StudyHit::NotesInput);
        }
        if super::learn::notes_save_rect(regions).contains(pos) {
            return Some(StudyHit::NotesSave);
        }
    }

    if stats_rect(regions.header).contains(pos) {
        return Some(StudyHit::Stats);
    }
    if review_queue_rect(regions.outline).contains(pos) {
        return Some(StudyHit::ReviewQueue);
    }

    // Phase 11: Stage pill click cycles stage
    if stage_rect(regions.header).contains(pos) {
        return Some(StudyHit::StageClick);
    }

    // Phase 10: High contrast toggle button
    let hc_btn = Rect::new(
        regions.header.x0 + 64.0,
        regions.header.y0 + 8.0,
        regions.header.x0 + 84.0,
        regions.header.y0 + 32.0,
    );
    if hc_btn.contains(pos) {
        return Some(StudyHit::HighContrastToggle);
    }

    // Phase 2: Bookmark toggle button
    let bookmark_rect = Rect::new(
        regions.outline.x1 - 36.0,
        regions.outline.y0 + 8.0,
        regions.outline.x1 - 12.0,
        regions.outline.y0 + 36.0,
    );
    if bookmark_rect.contains(pos) {
        return Some(StudyHit::BookmarkConcept);
    }

    // Phase 2: Notes toggle button
    let notes_btn = Rect::new(
        regions.outline.x1 - 60.0,
        regions.outline.y0 + 8.0,
        regions.outline.x1 - 40.0,
        regions.outline.y0 + 36.0,
    );
    if notes_btn.contains(pos) {
        return Some(StudyHit::NotesToggle);
    }

    // Phase 2: Search box
    let search = search_rect(regions.outline);
    if search.contains(pos) {
        return Some(StudyHit::SearchBox);
    }

    // Phase 2: Unit expand/collapse headers
    for (unit_idx, unit) in state.units.iter().enumerate() {
        let expanded = state.expanded_units.get(unit_idx).copied().unwrap_or(true);
        let unit_y = unit_header_y(unit_idx) - state.outline_scroll_offset;
        let header_rect = Rect::new(
            regions.outline.x0,
            unit_y,
            regions.outline.x1,
            unit_y + 28.0,
        );
        if header_rect.contains(pos) {
            return Some(StudyHit::ToggleUnit(unit_idx));
        }
        if !expanded {
            continue;
        }
        for concept_idx in 0..unit.concepts.len() {
            if concept_rect(unit_idx, concept_idx, regions.outline).contains(pos) {
                return Some(StudyHit::Concept(unit_idx, concept_idx));
            }
        }
    }

    if super::learn::start_practice_rect(regions.surface).contains(pos) {
        return Some(StudyHit::StartPractice);
    }
    if super::learn::review_concept_rect(regions.surface).contains(pos) {
        return Some(StudyHit::ReviewConcept);
    }
    if super::practice::submit_rect(regions.surface).contains(pos) {
        return Some(StudyHit::SubmitAnswer);
    }
    if super::practice::next_rect(regions.surface).contains(pos) {
        return Some(StudyHit::NextProblem);
    }
    if super::practice::retry_rect(regions.surface).contains(pos) {
        return Some(StudyHit::RetryAnswer);
    }
    if super::practice::review_concept_rect(regions.surface).contains(pos) {
        return Some(StudyHit::ReviewConcept);
    }

    // Phase 3: Skip problem button
    if super::practice::skip_rect(regions.surface).contains(pos) {
        return Some(StudyHit::SkipProblem);
    }
    // Phase 3: Pause/resume button
    if super::practice::pause_rect(regions.surface).contains(pos) {
        return Some(StudyHit::PauseResume);
    }

    // Phase 6: Spaced repetition rating buttons
    for rating in [
        SpacedRepetitionRating::Again,
        SpacedRepetitionRating::Hard,
        SpacedRepetitionRating::Good,
        SpacedRepetitionRating::Easy,
    ] {
        if super::practice::rating_rect(regions.surface, &rating).contains(pos) {
            return Some(StudyHit::RatingButton(rating));
        }
    }

    for level in 1..=3 {
        if super::tutor::hint_rect(regions.tutor, level).contains(pos) {
            return Some(StudyHit::RevealHint(level));
        }
    }
    if (state.show_result_modal
        || state.show_stats_modal
        || state.show_shortcut_help
        || state.show_goal_modal)
        && super::tutor::modal_close_rect(size_from_regions(regions)).contains(pos)
    {
        return Some(StudyHit::CloseModal);
    }

    // Phase 4: Hamburger menu button (mobile only)
    if state.viewport_class == super::state::StudyViewportClass::Mobile {
        let hamburger = Rect::new(4.0, 4.0, 40.0, 36.0);
        if hamburger.contains(pos) {
            return Some(StudyHit::HamburgerMenu);
        }
    }

    // Phase 4: Hamburger menu row items (when menu is open)
    if state.show_hamburger_menu {
        let menu = Rect::new(0.0, 0.0, 240.0, regions.outline.y1);
        if menu.contains(pos) {
            let items = [Stage::Learn, Stage::Practice, Stage::Review];
            for (idx, stage) in items.iter().enumerate() {
                let y = menu.y0 + 56.0 + idx as f64 * 48.0;
                let row = Rect::new(menu.x0 + 8.0, y, menu.x1 - 8.0, y + 44.0);
                if row.contains(pos) {
                    return Some(StudyHit::HamburgerMenuRow(*stage));
                }
            }
            // Click inside menu but not on a row: consume event (no-op)
            return None;
        }
    }

    // Phase 5: Goal setting button in header
    let goal_btn = Rect::new(
        regions.header.x1 - 140.0,
        regions.header.y0 + 7.0,
        regions.header.x1 - 100.0,
        regions.header.y0 + 33.0,
    );
    if goal_btn.contains(pos) {
        return Some(StudyHit::GoalSetting);
    }

    // Phase 7: Tutor chat send button
    if regions.tutor.width() >= 160.0 {
        let chat_send = Rect::new(
            regions.tutor.x1 - 40.0,
            regions.tutor.y1 - 44.0,
            regions.tutor.x1 - 16.0,
            regions.tutor.y1 - 20.0,
        );
        let chat_input = Rect::new(
            regions.tutor.x0 + 16.0,
            regions.tutor.y1 - 44.0,
            regions.tutor.x1 - 52.0,
            regions.tutor.y1 - 20.0,
        );
        if chat_input.contains(pos) {
            return Some(StudyHit::TutorChatInput);
        }
        if chat_send.contains(pos) {
            return Some(StudyHit::TutorChatSend);
        }
        // Phase 7: Glossary expand
        for idx in 0..3 {
            let y = regions.tutor.y0 + 498.0 + idx as f64 * 42.0;
            let term_rect = Rect::new(
                regions.tutor.x0 + 16.0,
                y,
                regions.tutor.x1 - 16.0,
                y + 38.0,
            );
            if term_rect.contains(pos) {
                return Some(StudyHit::GlossaryExpand(idx));
            }
        }
        // Phase 7: Glossary search toggle
        let glossary_search = Rect::new(
            regions.tutor.x0 + 16.0,
            regions.tutor.y0 + 458.0,
            regions.tutor.x1 - 16.0,
            regions.tutor.y0 + 474.0,
        );
        if glossary_search.contains(pos) {
            return Some(StudyHit::GlossarySearchToggle);
        }
    }

    // Phase 8: Visual play/pause
    if state.stage == Stage::Learn {
        let play_btn = Rect::new(
            regions.surface.x0 + 32.0,
            regions.surface.y0 + 562.0,
            regions.surface.x0 + 64.0,
            regions.surface.y0 + 582.0,
        );
        if play_btn.contains(pos) {
            return Some(StudyHit::VisualPlayPause);
        }
        let autoplay_btn = Rect::new(
            regions.surface.x1 - 92.0,
            regions.surface.y0 + 562.0,
            regions.surface.x1 - 32.0,
            regions.surface.y0 + 582.0,
        );
        if autoplay_btn.contains(pos) {
            return Some(StudyHit::VisualAutoplay);
        }
        // Phase 8: Visual scrubber
        let scrubber = super::learn::scrubber_rect(regions.surface);
        if scrubber.contains(pos) {
            let position = ((pos.x - scrubber.x0) / scrubber.width()).clamp(0.0, 1.0);
            return Some(StudyHit::VisualScrubber(position));
        }
    }

    // Phase 10: Shortcut help button in header
    let shortcut_btn = Rect::new(
        regions.header.x0 + 42.0,
        regions.header.y0 + 8.0,
        regions.header.x0 + 62.0,
        regions.header.y0 + 32.0,
    );
    if shortcut_btn.contains(pos) {
        return Some(StudyHit::ShortcutHelp);
    }

    // Phase 3: Math palette toggle
    let math_btn = Rect::new(
        regions.surface.x0 + 458.0,
        regions.surface.y0 + 350.0,
        regions.surface.x0 + 500.0,
        regions.surface.y0 + 386.0,
    );
    if math_btn.contains(pos) && state.stage == Stage::Practice {
        return Some(StudyHit::MathPaletteToggle);
    }

    // Phase 3: Math palette symbol buttons
    if state.show_math_palette && state.stage == Stage::Practice {
        let math_symbols = 8;
        let btn_w = 40.0;
        let gap = 4.0;
        let palette_y = regions.surface.y0 + 394.0;
        for idx in 0..math_symbols {
            let col = idx % 4;
            let row = idx / 4;
            let x = regions.surface.x0 + 32.0 + col as f64 * (btn_w + gap);
            let y = palette_y + row as f64 * (28.0 + gap);
            let btn = Rect::new(x, y, x + btn_w, y + 28.0);
            if btn.contains(pos) {
                return Some(StudyHit::MathSymbol(idx));
            }
        }
    }

    None
}

fn status_color(status: ConceptStatus) -> Color {
    match status {
        ConceptStatus::Completed => NEUTRAL_400,
        ConceptStatus::Active => ACCENT_STUDY,
        ConceptStatus::InProgress => NEUTRAL_300,
        ConceptStatus::Warning => STATUS_WARNING,
    }
}

pub(crate) fn search_rect(outline: Rect) -> Rect {
    Rect::new(
        outline.x0 + 12.0,
        outline.y0 + 8.0,
        outline.x1 - 12.0,
        outline.y0 + 36.0,
    )
}

pub(crate) fn unit_header_y(unit_idx: usize) -> f64 {
    88.0 + unit_idx as f64 * 336.0
}

pub(crate) fn concept_rect(unit_idx: usize, concept_idx: usize, outline: Rect) -> Rect {
    let y = unit_header_y(unit_idx) + 32.0 + concept_idx as f64 * 32.0;
    Rect::new(outline.x0, y, outline.x1, y + 32.0)
}

pub(crate) fn review_queue_rect(outline: Rect) -> Rect {
    Rect::new(outline.x0, outline.y1 - 40.0, outline.x1, outline.y1)
}

pub(crate) fn stats_rect(header: Rect) -> Rect {
    Rect::new(
        header.x1 - 78.0,
        header.y0 + 7.0,
        header.x1 - 16.0,
        header.y0 + 33.0,
    )
}

pub(crate) fn stage_rect(header: Rect) -> Rect {
    let stage_x = if header.width() < 560.0 {
        (header.x1 - 170.0).max(132.0)
    } else {
        420.0_f64.min((header.x1 - 300.0).max(260.0))
    };
    Rect::new(stage_x, 10.0, stage_x + 80.0, 30.0)
}

fn size_from_regions(regions: &StudyRegions) -> Size {
    Size::new(regions.header.x1, regions.outline.y1)
}

// Phase 4: Mobile hamburger menu

pub fn paint_hamburger_menu(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    // Overlay
    p.fill_rect(
        Rect::new(0.0, 0.0, regions.header.x1, regions.outline.y1),
        Color::rgba8(0, 0, 0, 160),
    );
    // Menu panel (slides from left)
    let menu = Rect::new(0.0, 0.0, 240.0, regions.outline.y1);
    p.fill_rect(menu, NEUTRAL_800);
    p.draw_line(
        Point::new(menu.x1, menu.y0),
        Point::new(menu.x1, menu.y1),
        NEUTRAL_600,
        1.0,
    );

    p.draw_text(
        t("study.menu.title"),
        menu.x0 + 16.0,
        menu.y0 + 28.0,
        NEUTRAL_100,
        16.0,
        FontWeight::BOLD,
        false,
    );

    let items = [
        (t("study.menu.learn"), Stage::Learn),
        (t("study.menu.practice"), Stage::Practice),
        (t("study.menu.review"), Stage::Review),
    ];
    for (idx, (label, stage)) in items.iter().enumerate() {
        let y = menu.y0 + 56.0 + idx as f64 * 48.0;
        let rect = Rect::new(menu.x0 + 8.0, y, menu.x1 - 8.0, y + 44.0);
        let active = state.stage == *stage;
        if active {
            p.fill_rounded_rect(rect, NEUTRAL_600, 6.0);
        }
        p.draw_text(
            label,
            rect.x0 + 16.0,
            rect.y0 + 28.0,
            if active { ACCENT_STUDY } else { NEUTRAL_300 },
            14.0,
            if active {
                FontWeight::BOLD
            } else {
                FontWeight::NORMAL
            },
            false,
        );
    }

    // Stats and settings
    let stats_y = menu.y0 + 56.0 + 3.0 * 48.0 + 16.0;
    p.draw_text(
        t("study.menu.stats"),
        menu.x0 + 24.0,
        stats_y,
        NEUTRAL_400,
        13.0,
        FontWeight::NORMAL,
        false,
    );
    p.draw_text(
        t("study.menu.settings"),
        menu.x0 + 24.0,
        stats_y + 28.0,
        NEUTRAL_400,
        13.0,
        FontWeight::NORMAL,
        false,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_regions_do_not_overlap_across_core_viewports() {
        for size in [
            Size::new(390.0, 844.0),
            Size::new(768.0, 1024.0),
            Size::new(1440.0, 900.0),
        ] {
            let regions = regions(size);

            assert_rect_valid(regions.header);
            assert_rect_valid(regions.outline);
            assert_rect_valid(regions.surface);
            assert_rect_valid(regions.tutor);
            assert_eq!(regions.outline.x1, regions.surface.x0);
            assert_eq!(regions.surface.x1, regions.tutor.x0);
            assert_eq!(regions.tutor.x1, size.width);
            assert!(regions.surface.width() >= 220.0);
        }
    }

    #[test]
    fn compact_header_keeps_stage_clear_of_stats_button() {
        for width in [390.0, 560.0, 768.0, 1440.0] {
            let header = Rect::new(0.0, 0.0, width, HEADER_H);
            let stage = stage_rect(header);
            let stats = stats_rect(header);

            assert_rect_valid(stage);
            assert_rect_valid(stats);
            assert!(
                stage.x1 <= stats.x0 || stats.x1 <= stage.x0,
                "stage {:?} overlaps stats {:?} at width {}",
                stage,
                stats,
                width
            );
        }
    }

    fn assert_rect_valid(rect: Rect) {
        assert!(rect.x0 <= rect.x1, "{rect:?}");
        assert!(rect.y0 <= rect.y1, "{rect:?}");
        assert!(rect.width() >= 0.0, "{rect:?}");
        assert!(rect.height() >= 0.0, "{rect:?}");
    }
}
