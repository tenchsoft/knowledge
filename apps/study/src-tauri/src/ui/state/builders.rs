use super::*;

pub(super) fn units_from_builtin(builtin: &tench_study_core::BuiltinCurriculumSet) -> Vec<Unit> {
    builtin
        .curricula
        .iter()
        .map(|curriculum| {
            let glossary_terms = tench_study_core::glossary_terms_from_curriculum(curriculum);
            let mut concepts = curriculum
                .graph
                .nodes
                .iter()
                .filter(|node| matches!(node.kind, tench_study_core::CurriculumNodeKind::Course))
                .map(|node| Concept {
                    id: node.id.as_str().to_string(),
                    label: node.level.label().to_string(),
                    summary: node.summary.default.value.clone(),
                    level: node.level,
                    status: status_for_level(node.level),
                    visual_count: curriculum
                        .graph
                        .nodes
                        .iter()
                        .filter(|lesson| {
                            lesson.level == node.level
                                && matches!(
                                    lesson.kind,
                                    tench_study_core::CurriculumNodeKind::Lesson
                                )
                        })
                        .map(|lesson| lesson.visuals.len())
                        .sum(),
                    primary_visual_id: curriculum
                        .graph
                        .nodes
                        .iter()
                        .find(|lesson| {
                            lesson.level == node.level
                                && matches!(
                                    lesson.kind,
                                    tench_study_core::CurriculumNodeKind::Lesson
                                )
                        })
                        .and_then(|lesson| lesson.visuals.first())
                        .map(|visual_id| visual_id.as_str().to_string())
                        .unwrap_or_else(|| format!("visual-{}", node.id.as_str())),
                    glossary_terms: curriculum
                        .graph
                        .nodes
                        .iter()
                        .filter(|lesson| {
                            lesson.level == node.level
                                && matches!(
                                    lesson.kind,
                                    tench_study_core::CurriculumNodeKind::Lesson
                                )
                        })
                        .flat_map(|lesson| {
                            glossary_terms
                                .iter()
                                .filter(move |term| term.node_id == lesson.id)
                                .map(|term| GlossaryPreview {
                                    term: term.term.default.value.clone(),
                                    definition: term.definition.default.value.clone(),
                                })
                        })
                        .collect(),
                })
                .collect::<Vec<_>>();
            concepts.sort_by_key(|concept| level_rank(&concept.id));
            Unit {
                label: curriculum.title.default.value.clone(),
                domain: curriculum.domain.clone(),
                concepts,
            }
        })
        .collect()
}

pub(super) fn problems_from_builtin(
    builtin: &tench_study_core::BuiltinCurriculumSet,
) -> Vec<Problem> {
    builtin
        .curricula
        .iter()
        .flat_map(|curriculum| {
            let items = tench_study_core::builtin_practice_items(curriculum);
            items.into_iter().map(|item| {
                let lesson = curriculum
                    .graph
                    .nodes
                    .iter()
                    .find(|node| node.id == item.node_id);
                let course_id = parent_course_id(curriculum, &item.node_id)
                    .unwrap_or_else(|| item.node_id.as_str().to_string());
                let lesson_title = lesson
                    .map(|node| node.title.default.value.clone())
                    .unwrap_or_else(|| item.node_id.as_str().to_string());
                let visual_count = lesson.map(|node| node.visuals.len()).unwrap_or_default();
                Problem {
                    concept_id: course_id,
                    text: item.prompt.value,
                    matrices: format!(
                        "Curriculum: {}\nLesson: {}\nVisuals: {}",
                        curriculum.title.default.value, lesson_title, visual_count
                    ),
                    answer_key: item.answer_key.clone(),
                    answer: answer_key_display(&item.answer_key),
                    solution: item.explanation.value,
                    cause_tag: "objective recall".to_string(),
                    related_concept: lesson_title,
                }
            })
        })
        .collect()
}

pub(super) fn parent_course_id(
    curriculum: &tench_study_core::Curriculum,
    lesson_id: &tench_study_core::CurriculumNodeId,
) -> Option<String> {
    curriculum
        .graph
        .edges
        .iter()
        .find(|edge| {
            edge.to == *lesson_id
                && matches!(
                    edge.relation,
                    tench_study_core::CurriculumEdgeKind::Contains
                )
                && curriculum.graph.nodes.iter().any(|node| {
                    node.id == edge.from
                        && matches!(node.kind, tench_study_core::CurriculumNodeKind::Course)
                })
        })
        .map(|edge| edge.from.as_str().to_string())
}

pub(super) fn answer_key_display(answer_key: &tench_study_core::AnswerKey) -> String {
    match answer_key {
        tench_study_core::AnswerKey::Exact { value, .. } => value.clone(),
        tench_study_core::AnswerKey::Numeric { value, .. } => value.to_string(),
        tench_study_core::AnswerKey::MultipleChoice { option_id } => option_id.clone(),
        tench_study_core::AnswerKey::Cloze { accepted } => {
            accepted.first().cloned().unwrap_or_default()
        }
        tench_study_core::AnswerKey::Rubric { rubric_id, .. } => rubric_id.clone(),
    }
}

pub(super) fn default_selection(units: &[Unit]) -> StudySelectionState {
    let domain = units
        .first()
        .map(|unit| unit.domain.clone())
        .unwrap_or(tench_study_core::SubjectDomain::Mathematics);
    let level = units
        .first()
        .and_then(|unit| unit.concepts.first())
        .map(|concept| concept.level)
        .unwrap_or(tench_study_core::EducationLevel::Kindergarten);
    StudySelectionState {
        domain,
        level,
        locale: crate::i18n::DEFAULT_LOCALE.to_string(),
    }
}

pub(super) fn offline_asset_state(
    visual_specs: &[tench_study_core::LearningVisualSpec],
) -> OfflineAssetState {
    let mut required_scene_refs = visual_specs
        .iter()
        .map(|visual| visual.renderer.scene_ref.trim().to_string())
        .filter(|scene_ref| !scene_ref.is_empty())
        .collect::<Vec<_>>();
    required_scene_refs.sort();
    required_scene_refs.dedup();
    let missing_scene_refs = visual_specs
        .iter()
        .filter(|visual| visual.renderer.scene_ref.trim().is_empty())
        .map(|visual| visual.id.as_str().to_string())
        .collect::<Vec<_>>();
    OfflineAssetState {
        required_scene_refs,
        cache_ready: missing_scene_refs.is_empty(),
        missing_scene_refs,
    }
}

pub(super) fn default_keyboard_shortcuts() -> Vec<StudyKeyboardShortcut> {
    vec![
        StudyKeyboardShortcut {
            action: StudyShortcutAction::StartOrSubmit,
            key: "Enter".to_string(),
            label_key: "study.shortcut.start_or_submit".to_string(),
        },
        StudyKeyboardShortcut {
            action: StudyShortcutAction::CycleStage,
            key: "Tab".to_string(),
            label_key: "study.shortcut.cycle_stage".to_string(),
        },
        StudyKeyboardShortcut {
            action: StudyShortcutAction::PreviousConcept,
            key: "ArrowUp".to_string(),
            label_key: "study.shortcut.previous_concept".to_string(),
        },
        StudyKeyboardShortcut {
            action: StudyShortcutAction::NextConcept,
            key: "ArrowDown".to_string(),
            label_key: "study.shortcut.next_concept".to_string(),
        },
        StudyKeyboardShortcut {
            action: StudyShortcutAction::OpenStats,
            key: "S".to_string(),
            label_key: "study.shortcut.open_stats".to_string(),
        },
        StudyKeyboardShortcut {
            action: StudyShortcutAction::OpenReviewQueue,
            key: "R".to_string(),
            label_key: "study.shortcut.open_review".to_string(),
        },
        StudyKeyboardShortcut {
            action: StudyShortcutAction::CloseModal,
            key: "Escape".to_string(),
            label_key: "study.shortcut.close".to_string(),
        },
    ]
}

pub(super) fn default_accessibility_labels() -> Vec<StudyAccessibilityLabel> {
    vec![
        StudyAccessibilityLabel {
            target: StudyAccessibilityTarget::Header,
            label_key: "study.a11y.header".to_string(),
        },
        StudyAccessibilityLabel {
            target: StudyAccessibilityTarget::Curriculum,
            label_key: "study.a11y.curriculum".to_string(),
        },
        StudyAccessibilityLabel {
            target: StudyAccessibilityTarget::LearnSurface,
            label_key: "study.a11y.learn_surface".to_string(),
        },
        StudyAccessibilityLabel {
            target: StudyAccessibilityTarget::PracticeSurface,
            label_key: "study.a11y.practice_surface".to_string(),
        },
        StudyAccessibilityLabel {
            target: StudyAccessibilityTarget::ReviewSurface,
            label_key: "study.a11y.review_surface".to_string(),
        },
        StudyAccessibilityLabel {
            target: StudyAccessibilityTarget::TutorPanel,
            label_key: "study.a11y.tutor_panel".to_string(),
        },
        StudyAccessibilityLabel {
            target: StudyAccessibilityTarget::StatsModal,
            label_key: "study.a11y.stats_modal".to_string(),
        },
    ]
}

pub(super) fn status_for_level(level: tench_study_core::EducationLevel) -> ConceptStatus {
    match level {
        tench_study_core::EducationLevel::Kindergarten
        | tench_study_core::EducationLevel::ElementaryLower
        | tench_study_core::EducationLevel::ElementaryUpper => ConceptStatus::Completed,
        tench_study_core::EducationLevel::MiddleSchool
        | tench_study_core::EducationLevel::HighSchool => ConceptStatus::Active,
        tench_study_core::EducationLevel::UndergraduateLower
        | tench_study_core::EducationLevel::UndergraduateUpper => ConceptStatus::InProgress,
        tench_study_core::EducationLevel::GraduateMasters
        | tench_study_core::EducationLevel::GraduateDoctoral => ConceptStatus::Warning,
    }
}

pub(super) fn level_rank(id: &str) -> u8 {
    tench_study_core::EducationLevel::all()
        .iter()
        .position(|level| id.ends_with(level.stable_id()))
        .unwrap_or(usize::MAX) as u8
}

pub(super) fn default_goals() -> Vec<StudyGoal> {
    vec![
        StudyGoal {
            id: "daily-problems".to_string(),
            label_key: "study.goal.daily_problems".to_string(),
            target: 10,
            current: 0,
            unit: "problems".to_string(),
        },
        StudyGoal {
            id: "daily-minutes".to_string(),
            label_key: "study.goal.daily_minutes".to_string(),
            target: 30,
            current: 0,
            unit: "minutes".to_string(),
        },
        StudyGoal {
            id: "daily-accuracy".to_string(),
            label_key: "study.goal.daily_accuracy".to_string(),
            target: 80,
            current: 0,
            unit: "%".to_string(),
        },
    ]
}

pub(super) fn default_achievements() -> Vec<StudyAchievement> {
    vec![
        StudyAchievement {
            id: "first-session".to_string(),
            label_key: "study.achievement.first_session".to_string(),
            description_key: "study.achievement.first_session_desc".to_string(),
            unlocked: false,
            progress: 0.0,
        },
        StudyAchievement {
            id: "streak-10".to_string(),
            label_key: "study.achievement.streak_10".to_string(),
            description_key: "study.achievement.streak_10_desc".to_string(),
            unlocked: false,
            progress: 0.0,
        },
        StudyAchievement {
            id: "problems-100".to_string(),
            label_key: "study.achievement.problems_100".to_string(),
            description_key: "study.achievement.problems_100_desc".to_string(),
            unlocked: false,
            progress: 0.0,
        },
    ]
}
