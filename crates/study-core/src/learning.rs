use serde::{Deserialize, Serialize};

use crate::{
    build_learning_visual_draw_plan, ContentLocale, CurriculumNodeId, LearningVisualDrawCommand,
    LearningVisualDrawPlan, LearningVisualKind, LearningVisualSpec, LocalizedText, ReviewDate,
    ReviewQueuePolicy, ReviewRating, SkillId, SpacedRepetitionState, VisualRuntimeState,
};

crate::study_id_type!(LearnerId);
crate::study_id_type!(PracticeItemId);
crate::study_id_type!(AttemptId);
crate::study_id_type!(ReviewQueueItemId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LearnerProfile {
    pub id: LearnerId,
    pub display_name: String,
    pub primary_locale: ContentLocale,
    #[serde(default)]
    pub target_locales: Vec<ContentLocale>,
    #[serde(default)]
    pub accommodations: Vec<LearningAccommodation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningAccommodation {
    ReducedMotion,
    HighContrast,
    LargerText,
    Captions,
    ScreenReader,
    KeyboardOnly,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LearnerProgress {
    pub learner_id: LearnerId,
    pub node_id: CurriculumNodeId,
    pub mastery: MasteryState,
    #[serde(default)]
    pub attempts: Vec<AttemptRecord>,
    pub review_state: SpacedRepetitionState,
}

impl LearnerProgress {
    pub fn record_attempt(&mut self, attempt: AttemptRecord) {
        self.mastery.update(attempt.correct);
        self.attempts.push(attempt);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MasteryState {
    pub score: f32,
    pub attempts: u32,
    pub correct: u32,
}

impl Default for MasteryState {
    fn default() -> Self {
        Self {
            score: 0.0,
            attempts: 0,
            correct: 0,
        }
    }
}

impl MasteryState {
    pub fn update(&mut self, correct: bool) {
        self.attempts += 1;
        if correct {
            self.correct += 1;
        }
        self.score = self.correct as f32 / self.attempts as f32;
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PracticeItem {
    pub id: PracticeItemId,
    pub node_id: CurriculumNodeId,
    pub prompt: LocalizedText,
    pub kind: PracticeKind,
    pub answer_key: AnswerKey,
    pub explanation: LocalizedText,
    #[serde(default)]
    pub skills: Vec<SkillId>,
    #[serde(default)]
    pub difficulty: Option<f32>,
}

impl PracticeItem {
    pub fn grade(&self, submission: &AnswerSubmission) -> GradingResult {
        grade_answer(&self.answer_key, &submission.response)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PracticeKind {
    ExactAnswer,
    Numeric,
    MultipleChoice,
    Cloze,
    ProofStep,
    CodeTrace,
    CodeRun,
    SpokenResponse,
    FreeResponse,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum AnswerKey {
    Exact { value: String, case_sensitive: bool },
    Numeric { value: f64, tolerance: f64 },
    MultipleChoice { option_id: String },
    Cloze { accepted: Vec<String> },
    Rubric { rubric_id: String, max_score: f32 },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnswerSubmission {
    pub attempt_id: AttemptId,
    pub item_id: PracticeItemId,
    pub response: String,
    #[serde(default)]
    pub locale: Option<ContentLocale>,
    #[serde(default)]
    pub submitted_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GradingResult {
    pub correct: bool,
    pub score: f32,
    #[serde(default)]
    pub feedback_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttemptRecord {
    pub id: AttemptId,
    pub item_id: PracticeItemId,
    pub correct: bool,
    pub score: f32,
    pub response: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewQueueItem {
    pub id: ReviewQueueItemId,
    pub node_id: CurriculumNodeId,
    pub practice_item_id: PracticeItemId,
    pub wrong_answer: String,
    pub correct_answer: String,
    pub cause_tag: String,
    pub explanation: LocalizedText,
    pub rating: ReviewRating,
    pub reviewed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyDashboardSnapshot {
    pub learner_id: LearnerId,
    pub display_name: String,
    pub total_lesson_nodes: u32,
    pub completed_nodes: u32,
    pub in_progress_nodes: u32,
    pub average_mastery: f32,
    pub due_review_count: u32,
    pub overdue_review_count: u32,
    #[serde(default)]
    pub weak_nodes: Vec<CurriculumNodeId>,
    #[serde(default)]
    pub recommended_nodes: Vec<CurriculumNodeId>,
}

pub fn build_study_dashboard_snapshot(
    profile: &LearnerProfile,
    curricula: &[crate::Curriculum],
    progress: &[LearnerProgress],
    review_items: &[ReviewQueueItem],
    today: ReviewDate,
    daily_limit: u16,
) -> StudyDashboardSnapshot {
    let learner_progress = progress
        .iter()
        .filter(|progress| progress.learner_id == profile.id)
        .cloned()
        .collect::<Vec<_>>();
    let total_lesson_nodes = curricula
        .iter()
        .flat_map(|curriculum| curriculum.graph.nodes.iter())
        .filter(|node| matches!(node.kind, crate::CurriculumNodeKind::Lesson))
        .count() as u32;
    let completed_nodes = learner_progress
        .iter()
        .filter(|progress| progress.mastery.score >= 0.9)
        .count() as u32;
    let in_progress_nodes = learner_progress
        .iter()
        .filter(|progress| progress.mastery.attempts > 0 && progress.mastery.score < 0.9)
        .count() as u32;
    let average_mastery = if learner_progress.is_empty() {
        0.0
    } else {
        learner_progress
            .iter()
            .map(|progress| progress.mastery.score)
            .sum::<f32>()
            / learner_progress.len() as f32
    };
    let review_stats = crate::review_stats(&learner_progress, review_items, &today);
    let review_plan = crate::build_review_queue(
        &learner_progress,
        review_items,
        &ReviewQueuePolicy {
            today,
            daily_limit,
            include_suspended: false,
        },
    );
    let mut weak_nodes = learner_progress
        .iter()
        .filter(|progress| progress.mastery.attempts > 0 && progress.mastery.score < 0.6)
        .map(|progress| (progress.node_id.clone(), progress.mastery.score))
        .collect::<Vec<_>>();
    weak_nodes.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });
    let weak_nodes = weak_nodes
        .into_iter()
        .map(|(node_id, _)| node_id)
        .take(8)
        .collect::<Vec<_>>();
    let mut recommended_nodes = review_plan
        .due
        .iter()
        .map(|item| item.node_id.clone())
        .collect::<Vec<_>>();
    for node_id in &weak_nodes {
        if !recommended_nodes.contains(node_id) {
            recommended_nodes.push(node_id.clone());
        }
    }
    recommended_nodes.truncate(8);

    StudyDashboardSnapshot {
        learner_id: profile.id.clone(),
        display_name: profile.display_name.clone(),
        total_lesson_nodes,
        completed_nodes,
        in_progress_nodes,
        average_mastery,
        due_review_count: review_stats.due_count,
        overdue_review_count: review_stats.overdue_count,
        weak_nodes,
        recommended_nodes,
    }
}

pub fn grade_answer(key: &AnswerKey, response: &str) -> GradingResult {
    match key {
        AnswerKey::Exact {
            value,
            case_sensitive,
        } => {
            let correct = if *case_sensitive {
                normalize_space(response) == normalize_space(value)
            } else {
                normalize_space(response).eq_ignore_ascii_case(&normalize_space(value))
            };
            binary_result(correct)
        }
        AnswerKey::Numeric { value, tolerance } => {
            let parsed = response.trim().parse::<f64>();
            let correct = parsed
                .map(|candidate| (candidate - value).abs() <= tolerance.abs())
                .unwrap_or(false);
            binary_result(correct)
        }
        AnswerKey::MultipleChoice { option_id } => {
            binary_result(normalize_space(response) == normalize_space(option_id))
        }
        AnswerKey::Cloze { accepted } => {
            let response = normalize_space(response).to_ascii_lowercase();
            binary_result(
                accepted
                    .iter()
                    .any(|answer| normalize_space(answer).to_ascii_lowercase() == response),
            )
        }
        AnswerKey::Rubric { .. } => GradingResult {
            correct: false,
            score: 0.0,
            feedback_code: Some("rubric_required".to_string()),
        },
    }
}

pub fn build_practice_feedback_visual_draw_plan(
    visual: &LearningVisualSpec,
    practice: &PracticeItem,
    submission: &AnswerSubmission,
    grading: &GradingResult,
    state: &VisualRuntimeState,
    reduced_motion: bool,
) -> Result<LearningVisualDrawPlan, String> {
    if practice.id != submission.item_id {
        return Err(format!(
            "submission {} does not match practice item {}",
            submission.item_id.as_str(),
            practice.id.as_str()
        ));
    }
    if practice.node_id != visual.node_id {
        return Err(format!(
            "practice node {} does not match visual node {}",
            practice.node_id.as_str(),
            visual.node_id.as_str()
        ));
    }

    let mut plan = build_learning_visual_draw_plan(visual, state, reduced_motion)?;
    plan.commands
        .push(LearningVisualDrawCommand::ParameterControl {
            name: "answer_score".to_string(),
            value: grading.score as f64,
        });
    plan.commands.push(LearningVisualDrawCommand::Shape2d {
        id: "answer-feedback".to_string(),
        role: if grading.correct {
            "correct_answer_feedback".to_string()
        } else {
            "incorrect_answer_feedback".to_string()
        },
        progress: grading.score.clamp(0.0, 1.0),
        selected: true,
    });
    plan.commands.push(LearningVisualDrawCommand::TextLabel {
        id: "answer-feedback-label".to_string(),
        text: grading.feedback_code.clone().unwrap_or_else(|| {
            if grading.correct {
                "correct".to_string()
            } else {
                "needs_review".to_string()
            }
        }),
    });

    if is_math_feedback_visual(visual.kind) {
        append_math_answer_overlay(&mut plan, practice, submission, grading);
    }

    Ok(plan)
}

fn append_math_answer_overlay(
    plan: &mut LearningVisualDrawPlan,
    practice: &PracticeItem,
    submission: &AnswerSubmission,
    grading: &GradingResult,
) {
    match &practice.answer_key {
        AnswerKey::Numeric { value, tolerance } => {
            let response = submission.response.trim().parse::<f64>().ok();
            plan.commands.push(LearningVisualDrawCommand::Shape2d {
                id: "math-answer-target".to_string(),
                role: "target_value".to_string(),
                progress: normalize_numeric_visual_value(*value),
                selected: grading.correct,
            });
            if let Some(response) = response {
                plan.commands.push(LearningVisualDrawCommand::Shape2d {
                    id: "math-answer-response".to_string(),
                    role: "learner_value".to_string(),
                    progress: normalize_numeric_visual_value(response),
                    selected: !grading.correct,
                });
                plan.commands.push(LearningVisualDrawCommand::TextLabel {
                    id: "math-answer-delta".to_string(),
                    text: format!("delta {}", response - *value),
                });
            }
            plan.commands.push(LearningVisualDrawCommand::TextLabel {
                id: "math-answer-tolerance".to_string(),
                text: format!("tolerance {tolerance}"),
            });
        }
        AnswerKey::Exact { value, .. } => {
            plan.commands.push(LearningVisualDrawCommand::TextLabel {
                id: "math-answer-target".to_string(),
                text: value.clone(),
            });
            plan.commands.push(LearningVisualDrawCommand::TextLabel {
                id: "math-answer-response".to_string(),
                text: submission.response.clone(),
            });
        }
        AnswerKey::MultipleChoice { option_id } => {
            plan.commands.push(LearningVisualDrawCommand::TextLabel {
                id: "math-answer-target".to_string(),
                text: option_id.clone(),
            });
        }
        AnswerKey::Cloze { accepted } => {
            if let Some(value) = accepted.first() {
                plan.commands.push(LearningVisualDrawCommand::TextLabel {
                    id: "math-answer-target".to_string(),
                    text: value.clone(),
                });
            }
        }
        AnswerKey::Rubric {
            rubric_id,
            max_score,
        } => {
            plan.commands.push(LearningVisualDrawCommand::TextLabel {
                id: "math-answer-rubric".to_string(),
                text: format!("{rubric_id} / {max_score}"),
            });
        }
    }
}

fn is_math_feedback_visual(kind: LearningVisualKind) -> bool {
    matches!(
        kind,
        LearningVisualKind::NumberLine
            | LearningVisualKind::FunctionGraph
            | LearningVisualKind::GeometryConstruction
            | LearningVisualKind::MatrixTransformation
            | LearningVisualKind::ProbabilitySimulation
    )
}

fn normalize_numeric_visual_value(value: f64) -> f32 {
    ((value / (value.abs() + 1.0)) * 0.5 + 0.5).clamp(0.0, 1.0) as f32
}

fn binary_result(correct: bool) -> GradingResult {
    GradingResult {
        correct,
        score: if correct { 1.0 } else { 0.0 },
        feedback_code: None,
    }
}

fn normalize_space(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grades_numeric_answer_with_tolerance() {
        let result = grade_answer(
            &AnswerKey::Numeric {
                value: 3.15,
                tolerance: 0.01,
            },
            "3.141",
        );

        assert!(result.correct);
    }

    #[test]
    fn dashboard_snapshot_uses_profile_progress_reviews_and_curriculum() {
        let curricula = crate::builtin_curricula();
        let curriculum = curricula.curricula.first().expect("curriculum");
        let lesson_ids = curriculum
            .graph
            .nodes
            .iter()
            .filter(|node| matches!(node.kind, crate::CurriculumNodeKind::Lesson))
            .map(|node| node.id.clone())
            .take(2)
            .collect::<Vec<_>>();
        let profile = LearnerProfile {
            id: LearnerId::from("learner"),
            display_name: "Learner".to_string(),
            primary_locale: ContentLocale::parse("en-US").expect("locale"),
            target_locales: Vec::new(),
            accommodations: Vec::new(),
        };
        let progress = vec![
            LearnerProgress {
                learner_id: profile.id.clone(),
                node_id: lesson_ids[0].clone(),
                mastery: MasteryState {
                    score: 0.95,
                    attempts: 5,
                    correct: 5,
                },
                attempts: Vec::new(),
                review_state: SpacedRepetitionState {
                    due_on: Some(ReviewDate::parse("2026-05-10").unwrap()),
                    ..SpacedRepetitionState::default()
                },
            },
            LearnerProgress {
                learner_id: profile.id.clone(),
                node_id: lesson_ids[1].clone(),
                mastery: MasteryState {
                    score: 0.4,
                    attempts: 5,
                    correct: 2,
                },
                attempts: Vec::new(),
                review_state: SpacedRepetitionState {
                    due_on: Some(ReviewDate::parse("2026-05-01").unwrap()),
                    ..SpacedRepetitionState::default()
                },
            },
        ];
        let review_items = vec![ReviewQueueItem {
            id: ReviewQueueItemId::from("review-1"),
            node_id: lesson_ids[1].clone(),
            practice_item_id: PracticeItemId::from("practice-1"),
            wrong_answer: "wrong".to_string(),
            correct_answer: "correct".to_string(),
            cause_tag: "recall".to_string(),
            explanation: LocalizedText::plain("Review this concept."),
            rating: ReviewRating::Again,
            reviewed: false,
        }];

        let dashboard = build_study_dashboard_snapshot(
            &profile,
            &curricula.curricula,
            &progress,
            &review_items,
            ReviewDate::parse("2026-05-04").unwrap(),
            20,
        );

        assert_eq!(dashboard.learner_id, profile.id);
        assert!(dashboard.total_lesson_nodes >= lesson_ids.len() as u32);
        assert_eq!(dashboard.completed_nodes, 1);
        assert_eq!(dashboard.in_progress_nodes, 1);
        assert!(dashboard.average_mastery > 0.6);
        assert_eq!(dashboard.due_review_count, 1);
        assert_eq!(dashboard.overdue_review_count, 1);
        assert_eq!(dashboard.weak_nodes, vec![lesson_ids[1].clone()]);
        assert_eq!(dashboard.recommended_nodes[0], lesson_ids[1]);
    }

    #[test]
    fn math_practice_feedback_visual_overlays_target_and_response() {
        let visual = crate::LearningVisualSpec {
            id: crate::LearningVisualId::from("linear-graph"),
            node_id: CurriculumNodeId::from("algebra"),
            kind: crate::LearningVisualKind::FunctionGraph,
            title: LocalizedText::plain("Linear graph"),
            description: LocalizedText::plain("Graph feedback"),
            renderer: crate::VisualRenderer {
                engine: crate::VisualRendererEngine::Plot,
                spec_version: 1,
                scene_ref: "builtin://linear".to_string(),
            },
            playback: crate::VisualPlayback {
                animated: true,
                autoplay: false,
                duration_ms: Some(1000),
                timeline_position: 0.0,
                reduced_motion_fallback: true,
            },
            interactions: vec![crate::VisualInteraction::ScrubTimeline],
            accessibility: crate::VisualAccessibility {
                alt_text: "Linear graph".to_string(),
                transcript: Some("Graph feedback".to_string()),
                table_fallback_ref: Some("table://linear".to_string()),
                keyboard_model: vec!["tab".to_string()],
            },
            locale: None,
        };
        let practice = PracticeItem {
            id: PracticeItemId::from("slope"),
            node_id: CurriculumNodeId::from("algebra"),
            prompt: LocalizedText::plain("What is the slope?"),
            kind: PracticeKind::Numeric,
            answer_key: AnswerKey::Numeric {
                value: 2.0,
                tolerance: 0.1,
            },
            explanation: LocalizedText::plain("Slope is rise over run."),
            skills: Vec::new(),
            difficulty: Some(0.3),
        };
        let submission = AnswerSubmission {
            attempt_id: AttemptId::from("attempt"),
            item_id: PracticeItemId::from("slope"),
            response: "1.5".to_string(),
            locale: None,
            submitted_at: None,
        };
        let grading = practice.grade(&submission);
        let state = crate::VisualRuntimeState {
            visual_id: crate::LearningVisualId::from("linear-graph"),
            selected_id: None,
            active_layers: Vec::new(),
            parameter_values: Vec::new(),
            playback: crate::VisualPlayback {
                animated: true,
                autoplay: false,
                duration_ms: Some(1000),
                timeline_position: 0.4,
                reduced_motion_fallback: true,
            },
        };

        let plan = build_practice_feedback_visual_draw_plan(
            &visual,
            &practice,
            &submission,
            &grading,
            &state,
            false,
        )
        .expect("feedback plan");

        assert!(plan.commands.iter().any(|command| matches!(
            command,
            LearningVisualDrawCommand::Shape2d { id, role, .. }
                if id == "math-answer-target" && role == "target_value"
        )));
        assert!(plan.commands.iter().any(|command| matches!(
            command,
            LearningVisualDrawCommand::Shape2d { id, role, selected, .. }
                if id == "math-answer-response" && role == "learner_value" && *selected
        )));
        assert!(plan.commands.iter().any(|command| matches!(
            command,
            LearningVisualDrawCommand::ParameterControl { name, value }
                if name == "answer_score" && (*value - 0.0).abs() < f64::EPSILON
        )));
    }

    #[test]
    fn practice_feedback_visual_rejects_mismatched_submission() {
        let visual = crate::LearningVisualSpec {
            id: crate::LearningVisualId::from("number-line"),
            node_id: CurriculumNodeId::from("number"),
            kind: crate::LearningVisualKind::NumberLine,
            title: LocalizedText::plain("Number line"),
            description: LocalizedText::plain("Number line feedback"),
            renderer: crate::VisualRenderer {
                engine: crate::VisualRendererEngine::Tench2d,
                spec_version: 1,
                scene_ref: "builtin://number".to_string(),
            },
            playback: crate::VisualPlayback {
                animated: false,
                autoplay: false,
                duration_ms: None,
                timeline_position: 0.0,
                reduced_motion_fallback: true,
            },
            interactions: Vec::new(),
            accessibility: crate::VisualAccessibility {
                alt_text: "Number line".to_string(),
                transcript: None,
                table_fallback_ref: Some("table://number".to_string()),
                keyboard_model: vec!["tab".to_string()],
            },
            locale: None,
        };
        let practice = PracticeItem {
            id: PracticeItemId::from("expected"),
            node_id: CurriculumNodeId::from("number"),
            prompt: LocalizedText::plain("What is 1 + 1?"),
            kind: PracticeKind::Numeric,
            answer_key: AnswerKey::Numeric {
                value: 2.0,
                tolerance: 0.0,
            },
            explanation: LocalizedText::plain("Add one more."),
            skills: Vec::new(),
            difficulty: None,
        };
        let submission = AnswerSubmission {
            attempt_id: AttemptId::from("attempt"),
            item_id: PracticeItemId::from("other"),
            response: "2".to_string(),
            locale: None,
            submitted_at: None,
        };
        let state = crate::VisualRuntimeState {
            visual_id: crate::LearningVisualId::from("number-line"),
            selected_id: None,
            active_layers: Vec::new(),
            parameter_values: Vec::new(),
            playback: visual.playback.clone(),
        };
        let grading = GradingResult {
            correct: true,
            score: 1.0,
            feedback_code: None,
        };

        let result = build_practice_feedback_visual_draw_plan(
            &visual,
            &practice,
            &submission,
            &grading,
            &state,
            false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn mastery_updates_from_attempts() {
        let mut mastery = MasteryState::default();

        mastery.update(true);
        mastery.update(false);

        assert_eq!(mastery.attempts, 2);
        assert_eq!(mastery.correct, 1);
        assert_eq!(mastery.score, 0.5);
    }
}
