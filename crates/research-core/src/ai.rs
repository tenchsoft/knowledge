use serde::{Deserialize, Serialize};
use serde_json::json;
use tench_job_core::{JobDescriptor, JobState};
use tench_shared_types::{ChatCompletionParams, ChatMessage, EngineMethod, EngineRequest};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub paper_id: String,
    pub prompt: String,
    pub page: Option<u16>,
    pub selection: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisEngineRequestPlan {
    pub request_id: String,
    pub thread_id: String,
    pub paper_id: String,
    pub page: Option<u16>,
    pub selection: Option<String>,
    pub model: String,
    pub context_preview: String,
    pub requires_user_approval: bool,
    pub job: JobDescriptor,
    pub engine_request: EngineRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchAiFeatureKind {
    SummarizePaper,
    SummarizeSelection,
    ExtractClaimsEvidence,
    SuggestTags,
    NoteFromAnnotations,
    DraftClaimEvidenceGraph,
    DraftMethodFlow,
    DraftExperimentTimeline,
    DraftResultComparison,
    DraftFigureTableExplanation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchEngineTransport {
    Ipc,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchAiPromptTemplate {
    pub kind: ResearchAiFeatureKind,
    pub title: String,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub output_contract: String,
    pub requires_user_approval: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchAiWorkflowRequest {
    pub kind: ResearchAiFeatureKind,
    pub library_id: String,
    pub reference_id: ReferenceId,
    pub title: String,
    pub context: String,
    #[serde(default)]
    pub selection: Option<String>,
    #[serde(default)]
    pub annotations: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub locale: Option<ResearchLocale>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchAiOutputDestination {
    AnalysisThread,
    DraftNote,
    DraftTags,
    DraftVisual,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchAiWorkflowPlan {
    pub request_id: String,
    pub kind: ResearchAiFeatureKind,
    pub reference_id: ReferenceId,
    pub library_id: String,
    pub model: String,
    pub prompt_template: ResearchAiPromptTemplate,
    pub context_preview: String,
    pub output_destination: ResearchAiOutputDestination,
    #[serde(default)]
    pub visual_kind: Option<ResearchVisualKind>,
    pub transport: ResearchEngineTransport,
    pub requires_user_approval: bool,
    pub job: JobDescriptor,
    pub engine_request: EngineRequest,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchAiDraftOutput {
    pub id: String,
    pub request_id: String,
    pub kind: ResearchAiFeatureKind,
    pub reference_id: ReferenceId,
    pub library_id: String,
    pub title: String,
    pub body_markdown: String,
    #[serde(default)]
    pub suggested_tags: Vec<String>,
    #[serde(default)]
    pub visual_kind: Option<ResearchVisualKind>,
    pub status: DraftStatus,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchError {
    pub code: String,
    pub message: String,
}

impl ResearchError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ResearchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ResearchError {}

pub fn build_analysis_engine_request(
    request: &AnalysisRequest,
    papers: &[Paper],
    model: Option<&str>,
    stream: bool,
) -> Result<AnalysisEngineRequestPlan, ResearchError> {
    let Some(paper) = papers.iter().find(|paper| paper.id == request.paper_id) else {
        return Err(ResearchError::new(
            "paper_not_found",
            format!("Unknown paper id: {}", request.paper_id),
        ));
    };

    let context = request
        .selection
        .as_deref()
        .filter(|selection| !selection.trim().is_empty())
        .unwrap_or(&paper.abstract_text);
    let model = model
        .filter(|model| !model.trim().is_empty())
        .unwrap_or("tench/research")
        .to_string();
    let thread_id = format!("analysis-{}-{}", paper.id, stable_id(&request.prompt));
    let request_id = format!(
        "research-analysis-{}",
        stable_id(&format!("{thread_id}:{model}"))
    );
    let user_message = build_analysis_user_message(request, paper, context);
    let params = ChatCompletionParams {
        model: model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: research_analysis_system_prompt().to_string(),
            },
            ChatMessage::user(user_message),
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
        product_id: "tench-research".to_string(),
        kind: "research.ai.analysis".to_string(),
        state: JobState::Queued,
        progress: None,
        payload: json!({
            "paper_id": &paper.id,
            "thread_id": &thread_id,
            "model": &model,
            "page": request.page,
            "has_selection": request.selection.as_ref().is_some_and(|selection| !selection.trim().is_empty()),
            "requires_user_approval": true
        }),
    };

    Ok(AnalysisEngineRequestPlan {
        request_id,
        thread_id,
        paper_id: paper.id.clone(),
        page: request.page,
        selection: request.selection.clone(),
        model,
        context_preview: truncate_chars(context, 320),
        requires_user_approval: true,
        job,
        engine_request,
    })
}

pub fn analysis_thread_from_engine_output(
    plan: &AnalysisEngineRequestPlan,
    prompt: impl Into<String>,
    assistant_content: impl Into<String>,
    created_at: Timestamp,
) -> Result<AnalysisThread, ResearchError> {
    let prompt = prompt.into();
    let assistant_content = assistant_content.into();
    if prompt.trim().is_empty() {
        return Err(ResearchError::new(
            "empty_analysis_prompt",
            "Analysis prompt cannot be empty.",
        ));
    }
    if assistant_content.trim().is_empty() {
        return Err(ResearchError::new(
            "empty_analysis_output",
            "Engine analysis output cannot be empty.",
        ));
    }

    Ok(AnalysisThread {
        id: plan.thread_id.clone(),
        paper_id: plan.paper_id.clone(),
        page: plan.page,
        selection: plan.selection.clone(),
        messages: vec![
            AnalysisMessage {
                id: format!("msg-user-{}", stable_id(&prompt)),
                role: AnalysisRole::User,
                content: prompt,
                created_at: created_at.0.clone(),
            },
            AnalysisMessage {
                id: format!("msg-assistant-{}", stable_id(&assistant_content)),
                role: AnalysisRole::Assistant,
                content: assistant_content,
                created_at: created_at.0,
            },
        ],
    })
}

pub fn append_analysis_to_note(note: &Note, thread: &AnalysisThread) -> Note {
    let mut next = note.clone();
    let assistant_text = thread
        .messages
        .iter()
        .rev()
        .find(|message| message.role == AnalysisRole::Assistant)
        .map(|message| message.content.as_str())
        .unwrap_or("");
    if !assistant_text.is_empty() {
        next.content_markdown = format!(
            "{}\n\n## Research analysis\n\n{}\n",
            next.content_markdown.trim_end(),
            assistant_text
        );
        next.word_count = count_words(&next.content_markdown);
        next.updated_at = "2026-04-28T10:00:05Z".to_string();
    }
    next
}

pub fn research_ai_prompt_templates() -> Vec<ResearchAiPromptTemplate> {
    phase12_ai_feature_kinds()
        .into_iter()
        .map(research_ai_prompt_template)
        .collect()
}

pub fn build_research_ai_workflow_engine_request(
    request: &ResearchAiWorkflowRequest,
    model: Option<&str>,
    stream: bool,
) -> Result<ResearchAiWorkflowPlan, ResearchError> {
    if request.library_id.trim().is_empty() {
        return Err(ResearchError::new(
            "missing_library_id",
            "AI workflow requires a library id.",
        ));
    }
    if request.title.trim().is_empty() {
        return Err(ResearchError::new(
            "missing_reference_title",
            "AI workflow requires a reference title.",
        ));
    }
    if request.context.trim().is_empty() {
        return Err(ResearchError::new(
            "missing_ai_context",
            "AI workflow requires local paper, selection, note, or annotation context.",
        ));
    }
    if request.kind == ResearchAiFeatureKind::SummarizeSelection
        && request
            .selection
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
    {
        return Err(ResearchError::new(
            "missing_selection",
            "Selection summary requires selected text.",
        ));
    }

    let template = research_ai_prompt_template(request.kind);
    let model = model
        .filter(|model| !model.trim().is_empty())
        .unwrap_or("tench/research-ai")
        .to_string();
    let request_id = format!(
        "research-ai-{}-{}",
        research_ai_feature_label(request.kind),
        stable_id(&format!(
            "{}:{}:{}",
            request.library_id,
            request.reference_id.as_str(),
            request.context
        ))
    );
    let user_message = build_research_ai_user_message(request, &template);
    let params = ChatCompletionParams {
        model: model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: template.system_prompt.clone(),
            },
            ChatMessage::user(user_message),
        ],
        stream,
    };
    let engine_request = EngineRequest::new(
        request_id.clone(),
        EngineMethod::ChatCompletionsCreate,
        serde_json::to_value(params).unwrap_or_else(|_| json!({})),
    );
    let destination = research_ai_output_destination(request.kind);
    let visual_kind = research_ai_visual_kind(request.kind);
    let job = JobDescriptor {
        id: request_id.clone(),
        product_id: "tench-research".to_string(),
        kind: "research.ai.workflow".to_string(),
        state: JobState::Queued,
        progress: None,
        payload: json!({
            "library_id": request.library_id,
            "reference_id": request.reference_id.as_str(),
            "feature": request.kind,
            "destination": destination,
            "visual_kind": visual_kind,
            "transport": "ipc",
            "requires_user_approval": true,
            "annotation_count": request.annotations.len(),
            "note_count": request.notes.len(),
        }),
    };

    Ok(ResearchAiWorkflowPlan {
        request_id,
        kind: request.kind,
        reference_id: request.reference_id.clone(),
        library_id: request.library_id.clone(),
        model,
        prompt_template: template,
        context_preview: truncate_chars(&request.context, 320),
        output_destination: destination,
        visual_kind,
        transport: ResearchEngineTransport::Ipc,
        requires_user_approval: true,
        job,
        engine_request,
    })
}

pub fn complete_research_ai_workflow_text_draft(
    plan: &ResearchAiWorkflowPlan,
    engine_output_markdown: impl Into<String>,
    created_at: Timestamp,
) -> Result<ResearchAiDraftOutput, ResearchError> {
    let body_markdown = engine_output_markdown.into();
    if body_markdown.trim().is_empty() {
        return Err(ResearchError::new(
            "empty_ai_output",
            "Engine AI workflow output cannot be empty.",
        ));
    }
    let suggested_tags = if plan.kind == ResearchAiFeatureKind::SuggestTags {
        parse_suggested_tags(&body_markdown)
    } else {
        Vec::new()
    };
    Ok(ResearchAiDraftOutput {
        id: format!("draft-{}", plan.request_id),
        request_id: plan.request_id.clone(),
        kind: plan.kind,
        reference_id: plan.reference_id.clone(),
        library_id: plan.library_id.clone(),
        title: format!("{} draft", plan.prompt_template.title),
        body_markdown,
        suggested_tags,
        visual_kind: plan.visual_kind,
        status: DraftStatus::Proposed,
        created_at,
    })
}

pub fn set_research_ai_draft_status(
    mut draft: ResearchAiDraftOutput,
    status: DraftStatus,
) -> ResearchAiDraftOutput {
    draft.status = status;
    draft
}

pub fn can_save_research_ai_draft_as_note(draft: &ResearchAiDraftOutput) -> bool {
    matches!(draft.status, DraftStatus::Accepted | DraftStatus::Edited)
        && !draft.body_markdown.trim().is_empty()
}

pub fn approve_research_ai_draft_as_note(
    draft: &ResearchAiDraftOutput,
    note_id: ResearchNoteId,
    title: Option<String>,
    now: Timestamp,
) -> Result<ResearchNote, ResearchError> {
    if !can_save_research_ai_draft_as_note(draft) {
        return Err(ResearchError::new(
            "ai_draft_not_approved",
            "AI draft must be accepted or edited before saving as a user note.",
        ));
    }
    Ok(ResearchNote {
        id: note_id,
        reference_id: Some(draft.reference_id.clone()),
        annotation_id: None,
        title: title
            .filter(|title| !title.trim().is_empty())
            .unwrap_or_else(|| draft.title.clone()),
        body_markdown: draft.body_markdown.clone(),
        tags: Vec::new(),
        backlinks: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    })
}

fn research_analysis_system_prompt() -> &'static str {
    "You are Tench Research analysis support. Use only the supplied paper context. Return concise Markdown with sections for summary, evidence to inspect, and follow-up questions. Do not claim that you changed notes, references, visuals, or manuscripts; the user must approve any saved output."
}

pub(crate) fn phase12_ai_feature_kinds() -> Vec<ResearchAiFeatureKind> {
    vec![
        ResearchAiFeatureKind::SummarizePaper,
        ResearchAiFeatureKind::SummarizeSelection,
        ResearchAiFeatureKind::ExtractClaimsEvidence,
        ResearchAiFeatureKind::SuggestTags,
        ResearchAiFeatureKind::NoteFromAnnotations,
        ResearchAiFeatureKind::DraftClaimEvidenceGraph,
        ResearchAiFeatureKind::DraftMethodFlow,
        ResearchAiFeatureKind::DraftExperimentTimeline,
        ResearchAiFeatureKind::DraftResultComparison,
        ResearchAiFeatureKind::DraftFigureTableExplanation,
    ]
}

fn research_ai_prompt_template(kind: ResearchAiFeatureKind) -> ResearchAiPromptTemplate {
    let title = research_ai_feature_title(kind);
    ResearchAiPromptTemplate {
        kind,
        title: title.to_string(),
        system_prompt: concat!(
            "You are Tench Research AI support. Use only the supplied local context. ",
            "Never claim that notes, tags, visuals, references, or manuscripts were saved. ",
            "Return an editable draft for user review. The product will save only after explicit user approval."
        )
        .to_string(),
        user_prompt_template: research_ai_user_prompt_template(kind).to_string(),
        output_contract: research_ai_output_contract(kind).to_string(),
        requires_user_approval: true,
    }
}

fn research_ai_feature_title(kind: ResearchAiFeatureKind) -> &'static str {
    match kind {
        ResearchAiFeatureKind::SummarizePaper => "Paper summary",
        ResearchAiFeatureKind::SummarizeSelection => "Selection summary",
        ResearchAiFeatureKind::ExtractClaimsEvidence => "Claims and evidence extraction",
        ResearchAiFeatureKind::SuggestTags => "Tag suggestions",
        ResearchAiFeatureKind::NoteFromAnnotations => "Annotation note draft",
        ResearchAiFeatureKind::DraftClaimEvidenceGraph => "Claim-evidence graph draft",
        ResearchAiFeatureKind::DraftMethodFlow => "Method flow draft",
        ResearchAiFeatureKind::DraftExperimentTimeline => "Experiment timeline draft",
        ResearchAiFeatureKind::DraftResultComparison => "Result comparison draft",
        ResearchAiFeatureKind::DraftFigureTableExplanation => "Figure and table explanation draft",
    }
}

fn research_ai_feature_label(kind: ResearchAiFeatureKind) -> &'static str {
    match kind {
        ResearchAiFeatureKind::SummarizePaper => "summarize-paper",
        ResearchAiFeatureKind::SummarizeSelection => "summarize-selection",
        ResearchAiFeatureKind::ExtractClaimsEvidence => "extract-claims-evidence",
        ResearchAiFeatureKind::SuggestTags => "suggest-tags",
        ResearchAiFeatureKind::NoteFromAnnotations => "note-from-annotations",
        ResearchAiFeatureKind::DraftClaimEvidenceGraph => "draft-claim-evidence-graph",
        ResearchAiFeatureKind::DraftMethodFlow => "draft-method-flow",
        ResearchAiFeatureKind::DraftExperimentTimeline => "draft-experiment-timeline",
        ResearchAiFeatureKind::DraftResultComparison => "draft-result-comparison",
        ResearchAiFeatureKind::DraftFigureTableExplanation => "draft-figure-table-explanation",
    }
}

fn research_ai_user_prompt_template(kind: ResearchAiFeatureKind) -> &'static str {
    match kind {
        ResearchAiFeatureKind::SummarizePaper => {
            "Summarize the paper with sections for contribution, method, evidence, limits, and follow-up reading."
        }
        ResearchAiFeatureKind::SummarizeSelection => {
            "Summarize the selected passage and explain how it relates to the paper."
        }
        ResearchAiFeatureKind::ExtractClaimsEvidence => {
            "Extract claims, evidence, methods, datasets, and limitations as a Markdown table."
        }
        ResearchAiFeatureKind::SuggestTags => {
            "Suggest concise tags. Return one tag per line and avoid duplicates."
        }
        ResearchAiFeatureKind::NoteFromAnnotations => {
            "Turn the supplied annotations into an editable reading note with quotes, page references, and open questions."
        }
        ResearchAiFeatureKind::DraftClaimEvidenceGraph => {
            "Draft a claim-evidence graph schema with node ids, labels, evidence links, and uncertainty warnings."
        }
        ResearchAiFeatureKind::DraftMethodFlow => {
            "Draft a method flow schema with ordered steps, inputs, outputs, and linked source ranges."
        }
        ResearchAiFeatureKind::DraftExperimentTimeline => {
            "Draft an experiment timeline with conditions, datasets, baselines, metrics, and source ranges."
        }
        ResearchAiFeatureKind::DraftResultComparison => {
            "Draft a result comparison matrix with metrics, baselines, reported values, and caveats."
        }
        ResearchAiFeatureKind::DraftFigureTableExplanation => {
            "Draft an explanation card for the supplied figure/table with caption, takeaway, variables, and caveats."
        }
    }
}

fn research_ai_output_contract(kind: ResearchAiFeatureKind) -> &'static str {
    match kind {
        ResearchAiFeatureKind::SuggestTags => {
            "Plain text tags, one per line. The product treats them as suggestions only."
        }
        ResearchAiFeatureKind::DraftClaimEvidenceGraph
        | ResearchAiFeatureKind::DraftMethodFlow
        | ResearchAiFeatureKind::DraftExperimentTimeline
        | ResearchAiFeatureKind::DraftResultComparison
        | ResearchAiFeatureKind::DraftFigureTableExplanation => {
            "Markdown plus a compact JSON-like draft schema. The product stores it as an AI draft until accepted."
        }
        _ => "Markdown draft. The product stores it as a user note only after approval.",
    }
}

fn research_ai_output_destination(kind: ResearchAiFeatureKind) -> ResearchAiOutputDestination {
    match kind {
        ResearchAiFeatureKind::SummarizePaper
        | ResearchAiFeatureKind::SummarizeSelection
        | ResearchAiFeatureKind::ExtractClaimsEvidence => {
            ResearchAiOutputDestination::AnalysisThread
        }
        ResearchAiFeatureKind::SuggestTags => ResearchAiOutputDestination::DraftTags,
        ResearchAiFeatureKind::NoteFromAnnotations => ResearchAiOutputDestination::DraftNote,
        ResearchAiFeatureKind::DraftClaimEvidenceGraph
        | ResearchAiFeatureKind::DraftMethodFlow
        | ResearchAiFeatureKind::DraftExperimentTimeline
        | ResearchAiFeatureKind::DraftResultComparison
        | ResearchAiFeatureKind::DraftFigureTableExplanation => {
            ResearchAiOutputDestination::DraftVisual
        }
    }
}

fn research_ai_visual_kind(kind: ResearchAiFeatureKind) -> Option<ResearchVisualKind> {
    match kind {
        ResearchAiFeatureKind::DraftClaimEvidenceGraph => {
            Some(ResearchVisualKind::ClaimEvidenceGraph)
        }
        ResearchAiFeatureKind::DraftMethodFlow => Some(ResearchVisualKind::MethodFlow),
        ResearchAiFeatureKind::DraftExperimentTimeline => {
            Some(ResearchVisualKind::ExperimentTimeline)
        }
        ResearchAiFeatureKind::DraftResultComparison => {
            Some(ResearchVisualKind::ResultComparisonChart)
        }
        ResearchAiFeatureKind::DraftFigureTableExplanation => {
            Some(ResearchVisualKind::PaperAnalysisMap)
        }
        _ => None,
    }
}

fn build_research_ai_user_message(
    request: &ResearchAiWorkflowRequest,
    template: &ResearchAiPromptTemplate,
) -> String {
    format!(
        concat!(
            "Feature: {:?}\n",
            "Library: {}\n",
            "Reference: {}\n",
            "Title: {}\n",
            "Locale: {}\n",
            "Instruction: {}\n",
            "Output contract: {}\n",
            "Selection:\n{}\n\n",
            "Annotations:\n{}\n\n",
            "Notes:\n{}\n\n",
            "Context:\n{}"
        ),
        request.kind,
        request.library_id,
        request.reference_id.as_str(),
        request.title,
        request
            .locale
            .as_ref()
            .map(ResearchLocale::bcp47)
            .unwrap_or_else(|| "und".to_string()),
        template.user_prompt_template,
        template.output_contract,
        request.selection.as_deref().unwrap_or(""),
        request.annotations.join("\n---\n"),
        request.notes.join("\n---\n"),
        request.context
    )
}

fn parse_suggested_tags(output: &str) -> Vec<String> {
    let mut tags = Vec::new();
    for line in output.lines() {
        let tag = line
            .trim()
            .trim_start_matches(['-', '*', '#', ' '])
            .trim_start_matches(|ch: char| ch.is_ascii_digit() || ch == '.' || ch == ')')
            .trim()
            .to_ascii_lowercase();
        if tag.is_empty() || tags.contains(&tag) {
            continue;
        }
        tags.push(tag);
        if tags.len() >= 20 {
            break;
        }
    }
    tags
}

fn build_analysis_user_message(request: &AnalysisRequest, paper: &Paper, context: &str) -> String {
    format!(
        "Paper title: {}\nAuthors: {}\nVenue: {}\nYear: {}\nPage: {}\nUser request: {}\nContext:\n{}",
        paper.title,
        paper.authors.join(", "),
        paper.venue,
        paper.year,
        request
            .page
            .map(|page| page.to_string())
            .unwrap_or_else(|| "not specified".to_string()),
        request.prompt,
        context
    )
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

fn count_words(value: &str) -> usize {
    value.split_whitespace().count()
}
