use std::collections::BTreeSet;

use super::builders::{
    default_accessibility_labels, default_achievements, default_goals, default_keyboard_shortcuts,
    default_selection, offline_asset_state, problems_from_builtin, units_from_builtin,
};
use super::*;

impl Default for DailyStudyDashboard {
    fn default() -> Self {
        Self {
            due_review_count: 0,
            new_lesson_count: 0,
            current_streak: 0,
            minutes_today: 0,
            accuracy_percent: 0,
            recommended_concept_id: None,
            offline_ready: true,
        }
    }
}

impl Default for StudyState {
    fn default() -> Self {
        let builtin = tench_study_core::builtin_curricula();
        let builtin_curriculum_count = builtin.curricula.len();
        let builtin_lesson_count = builtin
            .curricula
            .iter()
            .flat_map(|curriculum| curriculum.graph.nodes.iter())
            .filter(|node| matches!(node.kind, tench_study_core::CurriculumNodeKind::Lesson))
            .count();
        let visual_specs = tench_study_core::builtin_visual_specs_for_all();
        let builtin_visual_count = visual_specs.len();
        let builtin_glossary_count =
            tench_study_core::glossary_terms_from_all_curricula(&builtin.curricula).len();
        let units = units_from_builtin(&builtin);
        let problems = problems_from_builtin(&builtin);
        let selection = default_selection(&units);
        let offline_assets = offline_asset_state(&visual_specs);
        let accessibility_labels = default_accessibility_labels();
        let mut state = Self {
            active_subject: units
                .first()
                .map(|unit| unit.label.clone())
                .unwrap_or_else(|| "Mathematics".to_string()),
            units,
            problems,
            profile_setup: ProfileSetupState {
                learner_id: String::new(),
                display_name: String::new(),
                primary_locale: selection.locale.clone(),
                target_locales: vec![selection.locale.clone()],
                completed: false,
            },
            selection: selection.clone(),
            dashboard: DailyStudyDashboard::default(),
            viewport_class: StudyViewportClass::Desktop,
            touch_review: TouchReviewState {
                enabled: false,
                min_hit_size_px: 44,
                swipe_actions: vec![
                    TouchReviewAction::Again,
                    TouchReviewAction::Hard,
                    TouchReviewAction::Good,
                    TouchReviewAction::Easy,
                ],
            },
            batch_edit: BatchEditState::default(),
            keyboard_shortcuts: default_keyboard_shortcuts(),
            accessibility_labels,
            offline_assets,
            ux_audit: StudyUxAudit::default(),
            stage: Stage::Learn,
            active_unit_idx: 0,
            active_concept_idx: 0,
            streak: 0,
            elapsed_seconds: 0,
            problem_index: 1,
            review_index: 1,
            hint_level: 0,
            feedback: None,
            input_text: String::new(),
            input_cursor_pos: 0,
            session_results: Vec::new(),
            review_queue: Vec::new(),
            show_result_modal: false,
            show_stats_modal: false,
            builtin_curriculum_count,
            builtin_lesson_count,
            builtin_visual_count,
            builtin_glossary_count,
            visual_specs,
            // Phase 1: Profile setup wizard
            show_profile_setup_modal: true,
            profile_setup_step: ProfileSetupStep::Identity,
            wizard_learner_id: String::new(),
            wizard_display_name: String::new(),
            wizard_primary_locale: selection.locale.clone(),
            wizard_target_locales: vec![selection.locale.clone()],
            wizard_active_field: ProfileField::LearnerId,
            wizard_domain_idx: 0,
            wizard_level_idx: 0,
            wizard_locale_idx: 0,
            // Phase 2: Search/Notes/Bookmarks
            search_focused: false,
            search_query: String::new(),
            expanded_units: vec![true],
            outline_scroll_offset: 0.0,
            bookmarked_concept_ids: BTreeSet::new(),
            show_notes_panel: false,
            notes: Vec::new(),
            notes_input: String::new(),
            focus_target: StudyFocusTarget::None,
            // Phase 3: Practice improvements
            session_paused: false,
            show_math_palette: false,
            // Phase 4: Mobile/touch
            swipe_start: None,
            show_hamburger_menu: false,
            // Phase 5: Stats/progress
            daily_accuracy_history: vec![0; 30],
            streak_calendar: vec![false; 30],
            goals: default_goals(),
            achievements: default_achievements(),
            show_goal_modal: false,
            // Phase 6: Spaced repetition
            spaced_repetition_data: Vec::new(),
            pending_rating: None,
            // Phase 7: Tutor expansion
            tutor_chat_input: String::new(),
            tutor_chat_messages: Vec::new(),
            expanded_glossary_idx: None,
            glossary_search_focused: false,
            glossary_search_query: String::new(),
            // Phase 8: Visual interaction
            visual_playing: false,
            visual_autoplay: false,
            visual_timeline_position: 0.0,
            // Phase 9: Content authoring
            show_authoring_panel: false,
            authoring_title: String::new(),
            authoring_body: String::new(),
            authoring_problem_text: String::new(),
            authoring_problem_answer: String::new(),
            // Phase 10: Shortcuts/accessibility
            show_shortcut_help: false,
            focus_indicator: None,
            high_contrast_mode: false,
            expanded_achievement_idx: None,
        };
        state.refresh_daily_dashboard();
        state.refresh_ux_audit(&crate::i18n::study_i18n_catalog(
            crate::i18n::DEFAULT_LOCALE,
        ));
        state
    }
}
