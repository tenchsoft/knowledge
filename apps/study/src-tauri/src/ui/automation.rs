use super::state::{ProfileSetupStep, SpacedRepetitionRating, Stage, StudyState};
use super::{curriculum, learn, practice, tutor};
use tench_ui::prelude::*;

pub(crate) fn study_automation_nodes(
    study: &StudyState,
    size: Size,
    base_id: u64,
    i18n: &tench_app_core::I18nCatalog,
) -> Vec<UiAutomationNode> {
    if study.show_profile_setup_modal {
        return profile_setup_automation_nodes(study, size, base_id, i18n);
    }

    let regions = curriculum::regions(size);
    let mut nodes = Vec::new();
    let mut next_id = base_id.saturating_mul(1000);
    let t = |key| crate::i18n::resolve(i18n, key).to_string();

    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        t("study.header.stats"),
        "study.header.stats",
        curriculum::stats_rect(regions.header),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        study.stage.label(),
        "study.header.stage",
        curriculum::stage_rect(regions.header),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "shortcuts",
        "study.header.shortcuts",
        Rect::new(
            regions.header.x0 + 42.0,
            regions.header.y0 + 8.0,
            regions.header.x0 + 62.0,
            regions.header.y0 + 32.0,
        ),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "high contrast",
        "study.header.high_contrast",
        Rect::new(
            regions.header.x0 + 64.0,
            regions.header.y0 + 8.0,
            regions.header.x0 + 84.0,
            regions.header.y0 + 32.0,
        ),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "goals",
        "study.header.goals",
        Rect::new(
            regions.header.x1 - 140.0,
            regions.header.y0 + 7.0,
            regions.header.x1 - 100.0,
            regions.header.y0 + 33.0,
        ),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "text_input",
        t("study.curriculum.search"),
        "study.curriculum.search",
        curriculum::search_rect(regions.outline),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "bookmark",
        "study.curriculum.bookmark",
        Rect::new(
            regions.outline.x1 - 36.0,
            regions.outline.y0 + 8.0,
            regions.outline.x1 - 12.0,
            regions.outline.y0 + 36.0,
        ),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        "notes",
        "study.curriculum.notes",
        Rect::new(
            regions.outline.x1 - 60.0,
            regions.outline.y0 + 8.0,
            regions.outline.x1 - 40.0,
            regions.outline.y0 + 36.0,
        ),
    );
    push_node(
        &mut nodes,
        &mut next_id,
        "button",
        t("study.review.queue"),
        "study.review.queue",
        curriculum::review_queue_rect(regions.outline),
    );

    for (unit_idx, unit) in study.units.iter().enumerate() {
        let unit_y = curriculum::unit_header_y(unit_idx) - study.outline_scroll_offset;
        let unit_rect = Rect::new(
            regions.outline.x0,
            unit_y,
            regions.outline.x1,
            unit_y + 28.0,
        );
        if unit_rect.y1 < regions.outline.y0 + 44.0
            || unit_rect.y0 > curriculum::review_queue_rect(regions.outline).y0
        {
            continue;
        }
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            &unit.label,
            format!("study.unit.{unit_idx}"),
            unit_rect,
        );
        if !study.expanded_units.get(unit_idx).copied().unwrap_or(true) {
            continue;
        }
        for (concept_idx, concept) in unit.concepts.iter().enumerate() {
            let rect = curriculum::concept_rect(unit_idx, concept_idx, regions.outline);
            let adjusted = Rect::new(
                rect.x0,
                rect.y0 - study.outline_scroll_offset,
                rect.x1,
                rect.y1 - study.outline_scroll_offset,
            );
            if adjusted.y1 < regions.outline.y0 + 44.0
                || adjusted.y0 > curriculum::review_queue_rect(regions.outline).y0
            {
                continue;
            }
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                &concept.label,
                format!("study.concept.{}.{}", unit_idx, concept_idx),
                adjusted,
            );
        }
    }

    match study.stage {
        Stage::Learn => {
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                t("study.learn.start_practice"),
                "study.learn.start_practice",
                learn::start_practice_rect(regions.surface),
            );
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                "play visual",
                "study.visual.play_pause",
                Rect::new(
                    regions.surface.x0 + 32.0,
                    regions.surface.y0 + 562.0,
                    regions.surface.x0 + 64.0,
                    regions.surface.y0 + 582.0,
                ),
            );
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                "autoplay visual",
                "study.visual.autoplay",
                Rect::new(
                    regions.surface.x1 - 92.0,
                    regions.surface.y0 + 562.0,
                    regions.surface.x1 - 32.0,
                    regions.surface.y0 + 582.0,
                ),
            );
            // Phase 8: Visual scrubber
            push_node(
                &mut nodes,
                &mut next_id,
                "slider",
                format!("timeline: {:.0}%", study.visual_timeline_position * 100.0),
                "study.visual.scrubber",
                learn::scrubber_rect(regions.surface),
            );
            if study.show_notes_panel {
                push_node(
                    &mut nodes,
                    &mut next_id,
                    "panel",
                    "notes",
                    "study.notes.panel",
                    learn::notes_panel_rect(&regions),
                );
                push_node(
                    &mut nodes,
                    &mut next_id,
                    "text_input",
                    "note input",
                    "study.notes.input",
                    learn::notes_input_rect(&regions),
                );
                push_node(
                    &mut nodes,
                    &mut next_id,
                    "button",
                    "save note",
                    "study.notes.save",
                    learn::notes_save_rect(&regions),
                );
                for (idx, note) in study
                    .notes
                    .iter()
                    .filter(|note| note.concept_id == study.active_concept().id)
                    .enumerate()
                {
                    push_node(
                        &mut nodes,
                        &mut next_id,
                        "button",
                        &note.text,
                        format!("study.notes.row.{idx}"),
                        learn::note_row_rect(&regions, idx),
                    );
                }
            }
        }
        Stage::Practice => {
            // Phase 3: Practice answer input field
            let surface = regions.surface;
            let input = Rect::new(
                surface.x0 + 32.0,
                surface.y0 + 294.0,
                surface.x1 - 32.0,
                surface.y0 + 334.0,
            );
            push_node(
                &mut nodes,
                &mut next_id,
                "text_input",
                &study.input_text,
                "study.practice.answer",
                input,
            );
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                t("study.practice.submit"),
                "study.practice.submit",
                practice::submit_rect(regions.surface),
            );
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                t("study.practice.skip"),
                "study.practice.skip",
                practice::skip_rect(regions.surface),
            );
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                "pause",
                "study.practice.pause",
                practice::pause_rect(regions.surface),
            );
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                "math palette",
                "study.practice.math_palette",
                Rect::new(
                    regions.surface.x0 + 458.0,
                    regions.surface.y0 + 350.0,
                    regions.surface.x0 + 500.0,
                    regions.surface.y0 + 386.0,
                ),
            );
            if study.show_math_palette {
                for (idx, debug_id) in [
                    "study.practice.math_symbol.power",
                    "study.practice.math_symbol.sqrt",
                    "study.practice.math_symbol.fraction",
                    "study.practice.math_symbol.pi",
                    "study.practice.math_symbol.alpha",
                    "study.practice.math_symbol.beta",
                    "study.practice.math_symbol.infinity",
                    "study.practice.math_symbol.sum",
                ]
                .iter()
                .enumerate()
                {
                    let col = idx % 4;
                    let row = idx / 4;
                    let x = regions.surface.x0 + 32.0 + col as f64 * 44.0;
                    let y = regions.surface.y0 + 394.0 + row as f64 * 32.0;
                    push_node(
                        &mut nodes,
                        &mut next_id,
                        "button",
                        *debug_id,
                        *debug_id,
                        Rect::new(x, y, x + 40.0, y + 28.0),
                    );
                }
            }
            if study.feedback.is_some() {
                push_node(
                    &mut nodes,
                    &mut next_id,
                    "button",
                    "retry",
                    "study.practice.retry",
                    practice::retry_rect(regions.surface),
                );
                push_node(
                    &mut nodes,
                    &mut next_id,
                    "button",
                    "review concept",
                    "study.practice.review_concept",
                    practice::review_concept_rect(regions.surface),
                );
                push_node(
                    &mut nodes,
                    &mut next_id,
                    "button",
                    "next problem",
                    "study.practice.next",
                    practice::next_rect(regions.surface),
                );
            }
        }
        Stage::Review => {
            for rating in [
                SpacedRepetitionRating::Again,
                SpacedRepetitionRating::Hard,
                SpacedRepetitionRating::Good,
                SpacedRepetitionRating::Easy,
            ] {
                push_node(
                    &mut nodes,
                    &mut next_id,
                    "button",
                    format!("{rating:?}"),
                    format!("study.review.rating.{rating:?}").to_lowercase(),
                    practice::rating_rect(regions.surface, &rating),
                );
            }
        }
    }

    if regions.tutor.width() >= 160.0 {
        for level in 1..=3 {
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                format!("hint {level}"),
                format!("study.tutor.hint.{level}"),
                tutor::hint_rect(regions.tutor, level),
            );
        }
        push_node(
            &mut nodes,
            &mut next_id,
            "text_input",
            "chat",
            "study.tutor.chat.input",
            Rect::new(
                regions.tutor.x0 + 16.0,
                regions.tutor.y1 - 44.0,
                regions.tutor.x1 - 52.0,
                regions.tutor.y1 - 20.0,
            ),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "send",
            "study.tutor.chat.send",
            Rect::new(
                regions.tutor.x1 - 40.0,
                regions.tutor.y1 - 44.0,
                regions.tutor.x1 - 16.0,
                regions.tutor.y1 - 20.0,
            ),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "text_input",
            "glossary search",
            "study.glossary.search",
            Rect::new(
                regions.tutor.x0 + 16.0,
                regions.tutor.y0 + 458.0,
                regions.tutor.x1 - 16.0,
                regions.tutor.y0 + 474.0,
            ),
        );
        for idx in 0..3 {
            let y = regions.tutor.y0 + 498.0 + idx as f64 * 42.0;
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                format!("glossary term {idx}"),
                format!("study.glossary.term.{idx}"),
                Rect::new(
                    regions.tutor.x0 + 16.0,
                    y,
                    regions.tutor.x1 - 16.0,
                    y + 38.0,
                ),
            );
        }
    }

    if study.show_authoring_panel {
        let modal = tutor::modal_rect(size);
        push_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "authoring",
            "study.authoring.panel",
            modal,
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "close",
            "study.modal.close",
            tutor::modal_close_rect(size),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "text_input",
            &study.authoring_title,
            "study.authoring.title",
            Rect::new(
                modal.x0 + 18.0,
                modal.y0 + 86.0,
                modal.x1 - 18.0,
                modal.y0 + 114.0,
            ),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "text_input",
            &study.authoring_body,
            "study.authoring.body",
            Rect::new(
                modal.x0 + 18.0,
                modal.y0 + 144.0,
                modal.x1 - 18.0,
                modal.y1 - 96.0,
            ),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "new unit",
            "study.authoring.new_unit",
            Rect::new(
                modal.x0 + 18.0,
                modal.y1 - 88.0,
                modal.x0 + 120.0,
                modal.y1 - 60.0,
            ),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "new concept",
            "study.authoring.new_concept",
            Rect::new(
                modal.x0 + 126.0,
                modal.y1 - 88.0,
                modal.x0 + 248.0,
                modal.y1 - 60.0,
            ),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "new curriculum",
            "study.authoring.new_curriculum",
            Rect::new(
                modal.x0 + 18.0,
                modal.y1 - 56.0,
                modal.x0 + 150.0,
                modal.y1 - 28.0,
            ),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "save draft",
            "study.authoring.save_draft",
            Rect::new(
                modal.x1 - 120.0,
                modal.y1 - 56.0,
                modal.x1 - 18.0,
                modal.y1 - 28.0,
            ),
        );
        return nodes;
    }

    if study.show_stats_modal {
        push_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "stats",
            "study.modal.stats",
            tutor::modal_rect(size),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "close",
            "study.modal.close",
            tutor::modal_close_rect(size),
        );
    }
    if study.show_result_modal {
        push_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "result",
            "study.modal.result",
            tutor::modal_rect(size),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "close",
            "study.modal.close",
            tutor::modal_close_rect(size),
        );
    }
    if study.show_shortcut_help {
        push_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "shortcuts",
            "study.modal.shortcuts",
            tutor::modal_rect(size),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "close",
            "study.modal.close",
            tutor::modal_close_rect(size),
        );
    }
    if study.show_goal_modal {
        push_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "goals",
            "study.modal.goal",
            tutor::modal_rect(size),
        );
        push_node(
            &mut nodes,
            &mut next_id,
            "button",
            "close",
            "study.modal.close",
            tutor::modal_close_rect(size),
        );
        // Phase 5: Achievement badge nodes
        for (idx, achievement) in study.achievements.iter().enumerate() {
            let modal = tutor::modal_rect(size);
            let badge_y = modal.y0 + 80.0 + idx as f64 * 48.0;
            let badge_rect = Rect::new(modal.x0 + 24.0, badge_y, modal.x1 - 24.0, badge_y + 44.0);
            push_node(
                &mut nodes,
                &mut next_id,
                "button",
                if achievement.unlocked {
                    "unlocked"
                } else {
                    "locked"
                },
                format!("study.achievement.{idx}"),
                badge_rect,
            );
        }
    }

    // Phase 4: Hamburger menu row nodes
    if study.show_hamburger_menu {
        let menu = Rect::new(0.0, 0.0, 240.0, regions.outline.y1);
        let items = [
            (t("study.menu.learn"), "study.hamburger.learn"),
            (t("study.menu.practice"), "study.hamburger.practice"),
            (t("study.menu.review"), "study.hamburger.review"),
        ];
        for (idx, (label, debug_id)) in items.iter().enumerate() {
            let y = menu.y0 + 56.0 + idx as f64 * 48.0;
            let row = Rect::new(menu.x0 + 8.0, y, menu.x1 - 8.0, y + 44.0);
            push_node(&mut nodes, &mut next_id, "button", label, *debug_id, row);
        }
    }

    nodes
}

fn profile_setup_automation_nodes(
    study: &StudyState,
    size: Size,
    base_id: u64,
    i18n: &tench_app_core::I18nCatalog,
) -> Vec<UiAutomationNode> {
    let mut next_id = base_id.saturating_mul(1000);
    let mut dialog = automation_node(
        next_id,
        "dialog",
        Some("profile setup".to_string()),
        Some("study.profile".to_string()),
        profile_modal_rect(size),
    );
    let modal = profile_modal_rect(size);
    let t = |key| crate::i18n::resolve(i18n, key).to_string();

    match study.profile_setup_step {
        ProfileSetupStep::Identity => {
            push_child_node(
                &mut dialog,
                &mut next_id,
                "text_input",
                t("study.profile.learner_id"),
                "study.profile.learner_id",
                Rect::new(
                    modal.x0 + 24.0,
                    modal.y0 + 86.0,
                    modal.x1 - 24.0,
                    modal.y0 + 114.0,
                ),
            );
            push_child_node(
                &mut dialog,
                &mut next_id,
                "text_input",
                t("study.profile.display_name"),
                "study.profile.display_name",
                Rect::new(
                    modal.x0 + 24.0,
                    modal.y0 + 144.0,
                    modal.x1 - 24.0,
                    modal.y0 + 172.0,
                ),
            );
        }
        ProfileSetupStep::DomainLevel => {
            for (idx, domain) in study.units.iter().map(|unit| &unit.domain).enumerate() {
                push_child_node(
                    &mut dialog,
                    &mut next_id,
                    "button",
                    format!("{domain:?}"),
                    format!("study.profile.domain.{idx}"),
                    Rect::new(
                        modal.x0 + 24.0,
                        modal.y0 + 90.0 + idx as f64 * 36.0,
                        modal.x1 - 24.0,
                        modal.y0 + 122.0 + idx as f64 * 36.0,
                    ),
                );
            }
            let level_y_start = modal.y0 + 90.0 + study.units.len() as f64 * 36.0 + 16.0;
            for (idx, level) in tench_study_core::EducationLevel::all().iter().enumerate() {
                push_child_node(
                    &mut dialog,
                    &mut next_id,
                    "button",
                    level.label(),
                    format!("study.profile.level.{}", level.stable_id()),
                    Rect::new(
                        modal.x0 + 24.0,
                        level_y_start + 18.0 + idx as f64 * 30.0,
                        modal.x1 - 24.0,
                        level_y_start + 44.0 + idx as f64 * 30.0,
                    ),
                );
            }
        }
        ProfileSetupStep::Locale => {
            for (idx, locale) in ["en-US", "ko-KR", "ja-JP", "zh-CN"].iter().enumerate() {
                push_child_node(
                    &mut dialog,
                    &mut next_id,
                    "button",
                    *locale,
                    format!("study.profile.locale.{locale}"),
                    Rect::new(
                        modal.x0 + 24.0,
                        modal.y0 + 90.0 + idx as f64 * 36.0,
                        modal.x1 - 24.0,
                        modal.y0 + 122.0 + idx as f64 * 36.0,
                    ),
                );
            }
        }
        ProfileSetupStep::Done => {}
    }

    push_child_node(
        &mut dialog,
        &mut next_id,
        "button",
        match study.profile_setup_step {
            ProfileSetupStep::Done => t("study.profile.start"),
            _ => t("study.profile.next"),
        },
        "study.profile.next",
        Rect::new(
            modal.x1 - 120.0,
            modal.y1 - 52.0,
            modal.x1 - 24.0,
            modal.y1 - 24.0,
        ),
    );
    if study.profile_setup_step != ProfileSetupStep::Identity {
        push_child_node(
            &mut dialog,
            &mut next_id,
            "button",
            t("study.profile.back"),
            "study.profile.back",
            Rect::new(
                modal.x0 + 24.0,
                modal.y1 - 52.0,
                modal.x0 + 100.0,
                modal.y1 - 24.0,
            ),
        );
    }

    vec![dialog]
}

fn profile_modal_rect(size: Size) -> Rect {
    let w = 420.0_f64.min(size.width - 48.0).max(280.0);
    let h = 260.0_f64.min(size.height - 48.0).max(200.0);
    let x = (size.width - w) / 2.0;
    let y = (size.height - h) / 2.0;
    Rect::new(x, y, x + w, y + h)
}

fn push_node(
    nodes: &mut Vec<UiAutomationNode>,
    next_id: &mut u64,
    role: &str,
    label: impl Into<String>,
    debug_id: impl Into<String>,
    rect: Rect,
) {
    *next_id = next_id.saturating_add(1);
    nodes.push(automation_node(
        *next_id,
        role,
        Some(label.into()),
        Some(debug_id.into()),
        rect,
    ));
}

fn push_child_node(
    parent: &mut UiAutomationNode,
    next_id: &mut u64,
    role: &str,
    label: impl Into<String>,
    debug_id: impl Into<String>,
    rect: Rect,
) {
    *next_id = next_id.saturating_add(1);
    parent.children.push(automation_node(
        *next_id,
        role,
        Some(label.into()),
        Some(debug_id.into()),
        rect,
    ));
}

fn automation_node(
    id: u64,
    role: &str,
    label: Option<String>,
    debug_id: Option<String>,
    rect: Rect,
) -> UiAutomationNode {
    UiAutomationNode {
        id,
        debug_id,
        role: role.to_string(),
        label,
        value: None,
        bounds: UiAutomationRect {
            x: rect.x0,
            y: rect.y0,
            width: rect.width(),
            height: rect.height(),
        },
        enabled: true,
        focused: false,
        hovered: false,
        children: Vec::new(),
    }
}

#[cfg(test)]
mod automation_tests {
    use super::super::StudyApp;
    use super::*;
    use tench_ui_automation_core::{find_node, UiAutomationAction, UiAutomationSelector};
    use tench_ui_test::{harness::HarnessConfig, TestHarness};

    #[test]
    fn study_profile_setup_exposes_selector_nodes_ui_automation() {
        let app = StudyApp::new();
        let nodes = study_automation_nodes(&app.state, Size::new(800.0, 600.0), 1, &app.i18n);
        let root = UiAutomationNode {
            id: 1,
            debug_id: Some("study.root".to_string()),
            role: "window".to_string(),
            label: Some("Tench Study".to_string()),
            value: None,
            bounds: UiAutomationRect {
                x: 0.0,
                y: 0.0,
                width: 800.0,
                height: 600.0,
            },
            enabled: true,
            focused: false,
            hovered: false,
            children: nodes,
        };

        assert!(find_node(
            &root,
            &UiAutomationSelector::ByDebugId {
                debug_id: "study.profile.next".to_string()
            }
        )
        .is_some());
    }

    #[test]
    fn study_profile_next_click_advances_wizard_ui_automation() {
        let mut harness =
            TestHarness::with_config(StudyApp::new(), HarnessConfig::with_viewport(800.0, 600.0));

        let capture = harness
            .automation_action(UiAutomationAction::Click {
                selector: UiAutomationSelector::ByDebugId {
                    debug_id: "study.profile.next".to_string(),
                },
                modifiers: Default::default(),
            })
            .expect("profile next click");
        assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));

        let tree = harness.automation_tree();
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: "study.profile.domain.0".to_string()
                }
            )
            .is_some(),
            "profile wizard should expose domain choices after clicking next"
        );
    }
}
