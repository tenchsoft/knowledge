use serde::{Deserialize, Serialize};
use serde_json::json;
use tench_job_core::{JobDescriptor, JobState};
use tench_shared_types::{ChatCompletionParams, ChatMessage, EngineMethod, EngineRequest};

use crate::{ContentLocale, CurriculumNodeId, LearnerId};

crate::study_id_type!(StudyAiDraftId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyAiTaskKind {
    ExplainConcept,
    GenerateExtraPractice,
    GenerateReviewFromMistakes,
    TranslateDraftContent,
    PersonalizeStudyPlan,
}

impl StudyAiTaskKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExplainConcept => "explain_concept",
            Self::GenerateExtraPractice => "generate_extra_practice",
            Self::GenerateReviewFromMistakes => "generate_review_from_mistakes",
            Self::TranslateDraftContent => "translate_draft_content",
            Self::PersonalizeStudyPlan => "personalize_study_plan",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyEngineTransport {
    Ipc,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyAiPromptTemplate {
    pub task: StudyAiTaskKind,
    pub title: String,
    pub system_prompt: String,
    pub user_prompt: String,
    #[serde(default)]
    pub required_context: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StudyAiContext {
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub concept_title: Option<String>,
    #[serde(default)]
    pub lesson_summary: Option<String>,
    #[serde(default)]
    pub progress_summary: Option<String>,
    #[serde(default)]
    pub mistakes: Vec<String>,
    #[serde(default)]
    pub draft_content: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyAiRequest {
    pub task: StudyAiTaskKind,
    pub learner_id: LearnerId,
    pub node_id: CurriculumNodeId,
    #[serde(default)]
    pub locale: Option<ContentLocale>,
    pub prompt: String,
    pub context: StudyAiContext,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyAiEngineRequestPlan {
    pub request_id: String,
    pub draft_id: StudyAiDraftId,
    pub task: StudyAiTaskKind,
    pub learner_id: LearnerId,
    pub node_id: CurriculumNodeId,
    pub model: String,
    pub prompt_template: StudyAiPromptTemplate,
    pub context_preview: String,
    pub transport: StudyEngineTransport,
    pub requires_user_approval: bool,
    pub job: JobDescriptor,
    pub engine_request: EngineRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyAiDraftStatus {
    Proposed,
    Accepted,
    Rejected,
    Edited,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyAiDraft {
    pub id: StudyAiDraftId,
    pub task: StudyAiTaskKind,
    pub learner_id: LearnerId,
    pub node_id: CurriculumNodeId,
    pub content_markdown: String,
    pub created_at: String,
    pub status: StudyAiDraftStatus,
    pub requires_user_approval: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyAiCommitDestination {
    NoteDraft,
    PracticeDraft,
    ReviewDraft,
    TranslationDraft,
    StudyPlanDraft,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyAiCommitPlan {
    pub draft_id: StudyAiDraftId,
    pub task: StudyAiTaskKind,
    pub learner_id: LearnerId,
    pub node_id: CurriculumNodeId,
    pub destination: StudyAiCommitDestination,
    pub content_markdown: String,
    pub requires_user_confirmation: bool,
}

impl StudyAiDraft {
    pub fn can_commit_to_learner_data(&self) -> bool {
        matches!(
            self.status,
            StudyAiDraftStatus::Accepted | StudyAiDraftStatus::Edited
        )
    }
}

pub fn study_ai_prompt_templates() -> Vec<StudyAiPromptTemplate> {
    [
        StudyAiTaskKind::ExplainConcept,
        StudyAiTaskKind::GenerateExtraPractice,
        StudyAiTaskKind::GenerateReviewFromMistakes,
        StudyAiTaskKind::TranslateDraftContent,
        StudyAiTaskKind::PersonalizeStudyPlan,
    ]
    .into_iter()
    .map(study_ai_prompt_template)
    .collect()
}

pub fn build_study_ai_engine_request(
    request: &StudyAiRequest,
    model: Option<&str>,
    stream: bool,
) -> Result<StudyAiEngineRequestPlan, String> {
    if request.prompt.trim().is_empty() {
        return Err("study AI prompt is required".to_string());
    }
    let context = study_ai_context_text(request);
    if context.trim().is_empty() {
        return Err("study AI context is required".to_string());
    }

    let model = model
        .filter(|model| !model.trim().is_empty())
        .unwrap_or("tench/study")
        .to_string();
    let request_id = format!(
        "study-ai-{}",
        stable_id(&format!(
            "{}:{}:{}:{}",
            request.task.as_str(),
            request.learner_id.as_str(),
            request.node_id.as_str(),
            request.prompt
        ))
    );
    let draft_id = StudyAiDraftId::from(format!("draft-{request_id}"));
    let template = study_ai_prompt_template(request.task);
    let params = ChatCompletionParams {
        model: model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: template.system_prompt.clone(),
            },
            ChatMessage::user(format!(
                "{}\n\nTask: {}\nLearner: {}\nNode: {}\nLocale: {}\nUser request: {}\nContext:\n{}",
                template.user_prompt,
                request.task.as_str(),
                request.learner_id.as_str(),
                request.node_id.as_str(),
                request
                    .locale
                    .as_ref()
                    .map(ContentLocale::bcp47)
                    .unwrap_or_else(|| "not specified".to_string()),
                request.prompt,
                context
            )),
        ],
        stream,
    };
    let engine_request = EngineRequest::new(
        request_id.clone(),
        EngineMethod::ChatCompletionsCreate,
        serde_json::to_value(params).unwrap_or_else(|_| json!({})),
    );
    let job = JobDescriptor {
        id: request_id.clone(),
        product_id: "tench-study".to_string(),
        kind: format!("study.ai.{}", request.task.as_str()),
        state: JobState::Queued,
        progress: None,
        payload: json!({
            "learner_id": request.learner_id.as_str(),
            "node_id": request.node_id.as_str(),
            "task": request.task.as_str(),
            "model": model,
            "transport": "ipc",
            "requires_user_approval": true
        }),
    };

    Ok(StudyAiEngineRequestPlan {
        request_id,
        draft_id,
        task: request.task,
        learner_id: request.learner_id.clone(),
        node_id: request.node_id.clone(),
        model,
        prompt_template: template,
        context_preview: truncate_chars(&context, 320),
        transport: StudyEngineTransport::Ipc,
        requires_user_approval: true,
        job,
        engine_request,
    })
}

pub fn study_ai_draft_from_engine_output(
    plan: &StudyAiEngineRequestPlan,
    assistant_content: impl Into<String>,
    created_at: impl Into<String>,
) -> Result<StudyAiDraft, String> {
    let assistant_content = assistant_content.into();
    if assistant_content.trim().is_empty() {
        return Err("study AI draft content is required".to_string());
    }
    Ok(StudyAiDraft {
        id: plan.draft_id.clone(),
        task: plan.task,
        learner_id: plan.learner_id.clone(),
        node_id: plan.node_id.clone(),
        content_markdown: assistant_content,
        created_at: created_at.into(),
        status: StudyAiDraftStatus::Proposed,
        requires_user_approval: true,
    })
}

pub fn set_study_ai_draft_status(
    mut draft: StudyAiDraft,
    status: StudyAiDraftStatus,
    edited_content_markdown: Option<String>,
) -> Result<StudyAiDraft, String> {
    if let Some(content) = edited_content_markdown {
        if content.trim().is_empty() {
            return Err("edited study AI draft content is required".to_string());
        }
        draft.content_markdown = content;
        draft.status = StudyAiDraftStatus::Edited;
    } else {
        if status == StudyAiDraftStatus::Edited {
            return Err("edited study AI draft content is required".to_string());
        }
        draft.status = status;
    }
    Ok(draft)
}

pub fn default_study_ai_commit_destination(task: StudyAiTaskKind) -> StudyAiCommitDestination {
    match task {
        StudyAiTaskKind::ExplainConcept => StudyAiCommitDestination::NoteDraft,
        StudyAiTaskKind::GenerateExtraPractice => StudyAiCommitDestination::PracticeDraft,
        StudyAiTaskKind::GenerateReviewFromMistakes => StudyAiCommitDestination::ReviewDraft,
        StudyAiTaskKind::TranslateDraftContent => StudyAiCommitDestination::TranslationDraft,
        StudyAiTaskKind::PersonalizeStudyPlan => StudyAiCommitDestination::StudyPlanDraft,
    }
}

pub fn approve_study_ai_draft_for_commit(
    draft: StudyAiDraft,
    destination: Option<StudyAiCommitDestination>,
) -> Result<StudyAiCommitPlan, String> {
    if !draft.can_commit_to_learner_data() {
        return Err("study AI draft must be accepted or edited before commit".to_string());
    }
    if draft.content_markdown.trim().is_empty() {
        return Err("study AI draft content is required".to_string());
    }
    Ok(StudyAiCommitPlan {
        draft_id: draft.id,
        task: draft.task,
        learner_id: draft.learner_id,
        node_id: draft.node_id,
        destination: destination.unwrap_or_else(|| default_study_ai_commit_destination(draft.task)),
        content_markdown: draft.content_markdown,
        requires_user_confirmation: true,
    })
}

fn study_ai_prompt_template(task: StudyAiTaskKind) -> StudyAiPromptTemplate {
    let (title, system_prompt, user_prompt, required_context) = match task {
        StudyAiTaskKind::ExplainConcept => (
            "Explain concept",
            study_ai_system_prompt(task),
            "Explain the selected concept using the provided curriculum context.",
            vec!["concept_title", "lesson_summary"],
        ),
        StudyAiTaskKind::GenerateExtraPractice => (
            "Generate extra practice",
            study_ai_system_prompt(task),
            "Draft extra practice items with answer keys and rubrics for approval.",
            vec!["concept_title", "lesson_summary"],
        ),
        StudyAiTaskKind::GenerateReviewFromMistakes => (
            "Generate review from mistakes",
            study_ai_system_prompt(task),
            "Draft review guidance from the learner's supplied mistakes.",
            vec!["mistakes"],
        ),
        StudyAiTaskKind::TranslateDraftContent => (
            "Translate draft content",
            study_ai_system_prompt(task),
            "Translate the supplied draft content while preserving meaning and structure.",
            vec!["draft_content"],
        ),
        StudyAiTaskKind::PersonalizeStudyPlan => (
            "Personalize study plan",
            study_ai_system_prompt(task),
            "Draft a personalized study plan from progress and curriculum context.",
            vec!["progress_summary"],
        ),
    };
    StudyAiPromptTemplate {
        task,
        title: title.to_string(),
        system_prompt: system_prompt.to_string(),
        user_prompt: user_prompt.to_string(),
        required_context: required_context.into_iter().map(str::to_string).collect(),
    }
}

fn study_ai_system_prompt(task: StudyAiTaskKind) -> &'static str {
    match task {
        StudyAiTaskKind::ExplainConcept => {
            "You are Tench Study explanation support. Use only the supplied curriculum and learner context. Return concise Markdown. Do not claim that you changed progress, notes, cards, or curriculum; the user must approve any saved output."
        }
        StudyAiTaskKind::GenerateExtraPractice => {
            "You are Tench Study practice support. Generate draft practice ideas only from supplied context. Mark answers and rubrics clearly. Do not save or modify learner data."
        }
        StudyAiTaskKind::GenerateReviewFromMistakes => {
            "You are Tench Study review support. Use supplied mistakes to draft review guidance. Do not mutate review queues, progress, notes, or cards."
        }
        StudyAiTaskKind::TranslateDraftContent => {
            "You are Tench Study localization support. Translate only supplied draft content. Return editable draft text for human approval."
        }
        StudyAiTaskKind::PersonalizeStudyPlan => {
            "You are Tench Study planning support. Use supplied progress context to draft a study plan. Do not change schedules or progress directly."
        }
    }
}

fn study_ai_context_text(request: &StudyAiRequest) -> String {
    let mut lines = Vec::new();
    if let Some(subject) = &request.context.subject {
        lines.push(format!("Subject: {subject}"));
    }
    if let Some(level) = &request.context.level {
        lines.push(format!("Level: {level}"));
    }
    if let Some(title) = &request.context.concept_title {
        lines.push(format!("Concept: {title}"));
    }
    if let Some(summary) = &request.context.lesson_summary {
        lines.push(format!("Lesson summary: {summary}"));
    }
    if let Some(progress) = &request.context.progress_summary {
        lines.push(format!("Progress: {progress}"));
    }
    if !request.context.mistakes.is_empty() {
        lines.push(format!(
            "Mistakes:\n{}",
            request.context.mistakes.join("\n")
        ));
    }
    if let Some(draft) = &request.context.draft_content {
        lines.push(format!("Draft content:\n{draft}"));
    }
    lines.join("\n")
}

fn truncate_chars(value: &str, max_chars: usize) -> String {
    let mut truncated = value.chars().take(max_chars).collect::<String>();
    if value.chars().count() > max_chars {
        truncated.push_str("...");
    }
    truncated
}

fn stable_id(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn study_ai_request_uses_engine_contract_and_requires_approval() {
        let request = StudyAiRequest {
            task: StudyAiTaskKind::ExplainConcept,
            learner_id: LearnerId::from("learner-1"),
            node_id: CurriculumNodeId::from("math-algebra"),
            locale: ContentLocale::parse("ko-KR"),
            prompt: "Explain this with one visual analogy.".to_string(),
            context: StudyAiContext {
                subject: Some("Mathematics".to_string()),
                concept_title: Some("Linear equations".to_string()),
                lesson_summary: Some("Solving ax + b = c".to_string()),
                ..StudyAiContext::default()
            },
        };

        let plan = build_study_ai_engine_request(&request, None, false).expect("plan");

        assert_eq!(plan.model, "tench/study");
        assert_eq!(plan.job.product_id, "tench-study");
        assert_eq!(plan.job.state, JobState::Queued);
        assert_eq!(
            plan.engine_request.method,
            EngineMethod::ChatCompletionsCreate
        );
        assert_eq!(plan.transport, StudyEngineTransport::Ipc);
        assert_eq!(plan.job.payload["transport"], "ipc");
        assert_eq!(plan.prompt_template.task, StudyAiTaskKind::ExplainConcept);
        assert!(plan.requires_user_approval);
    }

    #[test]
    fn study_ai_prompt_templates_cover_phase13_tasks() {
        let templates = study_ai_prompt_templates();
        let tasks = templates
            .iter()
            .map(|template| template.task)
            .collect::<Vec<_>>();

        assert_eq!(templates.len(), 5);
        assert!(tasks.contains(&StudyAiTaskKind::ExplainConcept));
        assert!(tasks.contains(&StudyAiTaskKind::GenerateExtraPractice));
        assert!(tasks.contains(&StudyAiTaskKind::GenerateReviewFromMistakes));
        assert!(tasks.contains(&StudyAiTaskKind::TranslateDraftContent));
        assert!(tasks.contains(&StudyAiTaskKind::PersonalizeStudyPlan));
        assert!(templates
            .iter()
            .all(|template| !template.system_prompt.trim().is_empty()
                && !template.required_context.is_empty()));
    }

    #[test]
    fn engine_output_becomes_uncommitted_draft() {
        let request = StudyAiRequest {
            task: StudyAiTaskKind::GenerateReviewFromMistakes,
            learner_id: LearnerId::from("learner-1"),
            node_id: CurriculumNodeId::from("science-heart"),
            locale: None,
            prompt: "Create review guidance.".to_string(),
            context: StudyAiContext {
                mistakes: vec!["mixed up atrium and ventricle".to_string()],
                ..StudyAiContext::default()
            },
        };
        let plan = build_study_ai_engine_request(&request, Some("local-small"), true).unwrap();

        let draft = study_ai_draft_from_engine_output(
            &plan,
            "Review the chamber labels before flow direction.",
            "2026-05-04T00:00:00Z",
        )
        .expect("draft");

        assert_eq!(draft.status, StudyAiDraftStatus::Proposed);
        assert!(!draft.can_commit_to_learner_data());
        assert!(draft.requires_user_approval);
    }

    #[test]
    fn study_ai_draft_requires_approval_before_commit_plan() {
        let request = StudyAiRequest {
            task: StudyAiTaskKind::GenerateExtraPractice,
            learner_id: LearnerId::from("learner-1"),
            node_id: CurriculumNodeId::from("math-fractions"),
            locale: None,
            prompt: "Create extra fraction practice.".to_string(),
            context: StudyAiContext {
                concept_title: Some("Fractions".to_string()),
                lesson_summary: Some("Equivalent fractions and simplification".to_string()),
                ..StudyAiContext::default()
            },
        };
        let plan = build_study_ai_engine_request(&request, None, false).expect("plan");
        let draft = study_ai_draft_from_engine_output(
            &plan,
            "1. Simplify 2/4. Answer: 1/2.",
            "2026-05-04T00:00:00Z",
        )
        .expect("draft");

        assert!(approve_study_ai_draft_for_commit(draft.clone(), None).is_err());
        let accepted =
            set_study_ai_draft_status(draft, StudyAiDraftStatus::Accepted, None).expect("accept");
        let commit = approve_study_ai_draft_for_commit(accepted, None).expect("commit plan");

        assert_eq!(commit.destination, StudyAiCommitDestination::PracticeDraft);
        assert!(commit.requires_user_confirmation);
        assert!(commit.content_markdown.contains("Simplify"));
    }

    #[test]
    fn edited_translation_draft_commits_only_as_draft_payload() {
        let draft = StudyAiDraft {
            id: StudyAiDraftId::from("draft-translate"),
            task: StudyAiTaskKind::TranslateDraftContent,
            learner_id: LearnerId::from("learner-1"),
            node_id: CurriculumNodeId::from("language-grammar"),
            content_markdown: "Original".to_string(),
            created_at: "2026-05-04T00:00:00Z".to_string(),
            status: StudyAiDraftStatus::Proposed,
            requires_user_approval: true,
        };

        let edited = set_study_ai_draft_status(
            draft,
            StudyAiDraftStatus::Edited,
            Some("Approved translation draft".to_string()),
        )
        .expect("edit");
        let commit = approve_study_ai_draft_for_commit(edited, None).expect("commit");

        assert_eq!(
            commit.destination,
            StudyAiCommitDestination::TranslationDraft
        );
        assert_eq!(commit.content_markdown, "Approved translation draft");
    }
}
