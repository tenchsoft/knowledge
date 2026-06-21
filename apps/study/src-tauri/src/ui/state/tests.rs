use super::*;
use tench_ui::prelude::Size;

#[test]
fn selecting_concept_resets_practice_progress() {
    let mut state = StudyState::default();
    state.start_practice();
    state.input_text = "draft".into();
    state.feedback = Some(false);
    state.session_results.push(false);

    state.select_concept(0, 1);

    assert_eq!(state.stage, Stage::Learn);
    assert_eq!(state.active_unit_idx, 0);
    assert_eq!(state.active_concept_idx, 1);
    assert_eq!(state.problem_index, 1);
    assert!(state.input_text.is_empty());
    assert!(state.session_results.is_empty());
}

#[test]
fn wrong_practice_answer_adds_review_item() {
    let mut state = StudyState::default();
    let initial_review_count = state.review_queue.len();
    state.start_practice();
    state.input_text = "not the answer".into();

    state.submit_answer();

    assert_eq!(state.feedback, Some(false));
    assert_eq!(state.session_results, vec![false]);
    assert_eq!(state.review_queue.len(), initial_review_count + 1);
    assert_eq!(state.accuracy(), 0);
}

#[test]
#[allow(clippy::approx_constant)]
fn practice_submission_uses_core_numeric_tolerance_grading() {
    let mut state = StudyState {
        units: vec![Unit {
            label: "Mathematics".to_string(),
            domain: tench_study_core::SubjectDomain::Mathematics,
            concepts: vec![Concept {
                id: "numeric".to_string(),
                label: "Numeric".to_string(),
                summary: "Tolerance grading".to_string(),
                level: tench_study_core::EducationLevel::Kindergarten,
                status: ConceptStatus::Active,
                visual_count: 0,
                primary_visual_id: "numeric-visual".to_string(),
                glossary_terms: Vec::new(),
            }],
        }],
        problems: vec![Problem {
            concept_id: "numeric".to_string(),
            text: "Approximate pi.".to_string(),
            matrices: String::new(),
            answer_key: tench_study_core::AnswerKey::Numeric {
                value: 3.14,
                tolerance: 0.01,
            },
            answer: "3.14".to_string(),
            solution: "Within tolerance.".to_string(),
            cause_tag: "numeric precision".to_string(),
            related_concept: "Numeric".to_string(),
        }],
        active_unit_idx: 0,
        active_concept_idx: 0,
        stage: Stage::Practice,
        input_text: "3.141".to_string(),
        ..Default::default()
    };

    state.submit_answer();

    assert_eq!(state.feedback, Some(true));
    assert!(state.review_queue.is_empty());
    assert_eq!(state.accuracy(), 100);
}

#[test]
fn default_state_connects_builtin_curriculum_summary() {
    let state = StudyState::default();

    assert_eq!(state.builtin_curriculum_count, 4);
    assert!(!state.profile_setup.completed);
    assert_eq!(state.streak, 0);
    assert_eq!(state.active_concept_idx, 0);
    assert!(state.builtin_lesson_count >= 4 * tench_study_core::EducationLevel::all().len());
    assert!(state.builtin_visual_count >= state.builtin_lesson_count);
    assert!(state.builtin_glossary_count >= state.builtin_lesson_count);
    assert!(!state.active_glossary_terms().is_empty());
    assert!(state.review_queue.is_empty());
    assert!(!state.problems.is_empty());
    assert!(state
        .problems
        .iter()
        .all(|problem| problem.answer != "core idea"));
    assert!(state
        .problems
        .iter()
        .any(|problem| problem.matrices.contains("Visuals:")));
    assert!(state.mock_content_removed());
    assert!(state.ux_audit.missing_i18n_keys.is_empty());
    assert!(state.ux_audit.mock_content_removed);
}

#[test]
fn first_run_profile_setup_selects_domain_level_and_locale() {
    let mut state = StudyState::default();

    state
        .complete_profile_setup(
            "learner-1",
            "Yoon",
            "ko-KR",
            vec!["ko-KR".to_string(), "en-US".to_string()],
        )
        .expect("profile setup");
    state
        .select_domain_level_locale(
            tench_study_core::SubjectDomain::Science,
            tench_study_core::EducationLevel::HighSchool,
            "ko-KR",
        )
        .expect("selection");

    assert!(state.profile_setup.completed);
    assert_eq!(state.profile_setup.primary_locale, "ko-KR");
    assert_eq!(
        state.selection.domain,
        tench_study_core::SubjectDomain::Science
    );
    assert_eq!(
        state.selection.level,
        tench_study_core::EducationLevel::HighSchool
    );
    assert_eq!(
        state.active_unit().domain,
        tench_study_core::SubjectDomain::Science
    );
    assert_eq!(
        state.active_concept().level,
        tench_study_core::EducationLevel::HighSchool
    );
}

#[test]
fn daily_dashboard_touch_review_and_offline_assets_are_release_state() {
    let mut state = StudyState::default();

    state.update_viewport(Size::new(390.0, 820.0));
    state.refresh_offline_asset_state();
    state.start_practice();
    state.input_text = "wrong".to_string();
    state.submit_answer();

    assert_eq!(state.viewport_class, StudyViewportClass::Mobile);
    assert!(state.touch_review.enabled);
    assert!(state.touch_review.min_hit_size_px >= 48);
    assert_eq!(state.touch_review.swipe_actions.len(), 4);
    assert!(state.offline_assets.cache_ready);
    assert!(!state.offline_assets.required_scene_refs.is_empty());
    assert_eq!(state.dashboard.due_review_count, state.review_queue.len());
    assert_eq!(state.dashboard.current_streak, state.streak);
    assert!(state.dashboard.recommended_concept_id.is_some());
}

#[test]
fn desktop_batch_edit_shortcuts_and_accessibility_are_i18n_covered() {
    let mut state = StudyState::default();
    let concept_id = state.active_concept().id.clone();

    state.update_viewport(Size::new(1440.0, 900.0));
    state.toggle_batch_concept_selection(concept_id.clone());
    state.apply_batch_concept_status(ConceptStatus::Warning);
    state.refresh_ux_audit(&crate::i18n::study_i18n_catalog(
        crate::i18n::DEFAULT_LOCALE,
    ));

    assert_eq!(state.viewport_class, StudyViewportClass::Desktop);
    assert!(!state.touch_review.enabled);
    assert!(state.batch_edit.selected_concept_ids.contains(&concept_id));
    assert_eq!(state.active_concept().status, ConceptStatus::Warning);
    assert!(state
        .keyboard_shortcuts
        .iter()
        .any(|shortcut| shortcut.action == StudyShortcutAction::OpenReviewQueue));
    assert!(state.accessibility_labels.len() >= 7);
    assert_eq!(
        state.ux_audit.accessibility_label_count,
        state.accessibility_labels.len()
    );
    assert!(state.ux_audit.missing_i18n_keys.is_empty());
}

#[test]
fn active_visual_runtime_state_tracks_hint_level() {
    let mut state = StudyState::default();
    state.reveal_hint(2);

    let visual_state = state.active_visual_runtime_state();

    assert_eq!(visual_state.parameter_values[0].name, "hint_level");
    assert!(visual_state.playback.timeline_position > 0.6);
    assert!(visual_state.active_layers.contains(&"labels".to_string()));
}

#[test]
fn keyboard_navigation_moves_concepts_and_cycles_stage() {
    let mut state = StudyState::default();
    let initial_unit = state.active_unit_idx;
    let initial_concept = state.active_concept_idx;

    state.move_concept(1);
    assert_ne!(
        (state.active_unit_idx, state.active_concept_idx),
        (initial_unit, initial_concept)
    );
    assert_eq!(state.stage, Stage::Learn);

    state.cycle_stage(false);
    assert_eq!(state.stage, Stage::Practice);
    state.cycle_stage(true);
    assert_eq!(state.stage, Stage::Learn);
}

#[test]
fn keyboard_primary_action_starts_practice_and_advances_feedback() {
    let mut state = StudyState::default();
    state.activate_primary_keyboard_action();
    assert_eq!(state.stage, Stage::Practice);

    let answer = state.current_problem().expect("problem").answer.clone();
    state.input_text = answer;
    state.activate_primary_keyboard_action();
    assert_eq!(state.feedback, Some(true));

    state.activate_primary_keyboard_action();
    assert!(state.feedback.is_none() || state.show_result_modal);
}

#[test]
fn perfect_practice_session_increments_streak_on_completion() {
    let mut state = StudyState::default();
    let starting_streak = state.streak;
    state.start_practice();

    while let Some(problem) = state.current_problem() {
        state.input_text = problem.answer.clone();
        state.submit_answer();
        state.next_problem();
        if state.show_result_modal {
            break;
        }
    }

    assert_eq!(state.streak, starting_streak + 1);
    assert!(state.show_result_modal);
    assert_eq!(state.accuracy(), 100);
}
