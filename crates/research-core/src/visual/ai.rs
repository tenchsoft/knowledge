use super::*;
use serde_json::json;
use tench_job_core::{JobDescriptor, JobState};
use tench_shared_types::{ChatCompletionParams, ChatMessage, EngineMethod, EngineRequest};

use crate::Timestamp;

pub fn build_ai_visual_engine_request(
    request: &AiVisualRequest,
    model: Option<&str>,
    stream: bool,
) -> Result<AiVisualEngineRequestPlan, String> {
    if request.prompt.trim().is_empty() {
        return Err("AI visual request prompt is required".to_string());
    }
    if request.library_id.trim().is_empty() {
        return Err("AI visual request requires a library id".to_string());
    }

    let model = model
        .filter(|model| !model.trim().is_empty())
        .unwrap_or("tench/research-visual")
        .to_string();
    let request_id = format!(
        "research-visual-{}",
        stable_visual_id(&format!(
            "{}:{}:{:?}",
            request.source_reference_id.as_str(),
            request.prompt,
            request.requested_kind
        ))
    );
    let draft_id = VisualDraftId::from(format!("draft-{request_id}"));
    let context = ai_visual_context(request);
    let params = ChatCompletionParams {
        model: model.clone(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: ai_visual_system_prompt().to_string(),
            },
            ChatMessage::user(context.clone()),
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
        kind: "research.ai.visual_draft".to_string(),
        state: JobState::Queued,
        progress: None,
        payload: json!({
            "library_id": request.library_id,
            "source_reference_id": request.source_reference_id.as_str(),
            "source_attachment_id": request.source_attachment_id.as_ref().map(|id| id.as_str()),
            "requested_kind": request.requested_kind,
            "source_range_count": request.source_ranges.len(),
            "requires_user_approval": true
        }),
    };

    Ok(AiVisualEngineRequestPlan {
        request_id,
        draft_id,
        source_reference_id: request.source_reference_id.clone(),
        source_attachment_id: request.source_attachment_id.clone(),
        source_ranges: request.source_ranges.clone(),
        requested_kind: request.requested_kind,
        model,
        context_preview: context.chars().take(320).collect(),
        requires_user_approval: true,
        job,
        engine_request,
    })
}

pub fn ai_visual_draft_from_engine_spec(
    plan: &AiVisualEngineRequestPlan,
    mut visual_spec: ResearchVisualSpec,
    confidence: Option<f32>,
    warnings: Vec<AiVisualWarning>,
    created_at: Timestamp,
) -> Result<AiVisualDraft, String> {
    visual_spec.validate_for_non_ai_release()?;
    visual_spec.source = VisualSource::LlmDerivedDraft;
    if visual_spec.kind != plan.requested_kind {
        return Err(format!(
            "AI visual draft kind {:?} does not match requested kind {:?}",
            visual_spec.kind, plan.requested_kind
        ));
    }
    Ok(AiVisualDraft {
        id: plan.draft_id.clone(),
        source_reference_id: plan.source_reference_id.clone(),
        source_attachment_id: plan.source_attachment_id.clone(),
        source_ranges: plan.source_ranges.clone(),
        visual_spec,
        confidence,
        warnings,
        created_by: EngineRunId::from(plan.request_id.clone()),
        status: DraftStatus::Proposed,
        created_at,
    })
}

fn ai_visual_system_prompt() -> &'static str {
    "You are Tench Research visual draft support. Use only supplied source context. Return a draft visual specification suggestion; do not claim the visual has been saved. User approval is required before canonical storage."
}

fn ai_visual_context(request: &AiVisualRequest) -> String {
    format!(
        concat!(
            "Library: {}\n",
            "Reference: {}\n",
            "Attachment: {}\n",
            "Requested visual kind: {:?}\n",
            "Source ranges: {}\n",
            "Prompt:\n{}"
        ),
        request.library_id,
        request.source_reference_id.as_str(),
        request
            .source_attachment_id
            .as_ref()
            .map(|id| id.as_str())
            .unwrap_or("none"),
        request.requested_kind,
        request
            .source_ranges
            .iter()
            .map(|range| format!(
                "{:?}:{}-{}",
                range.kind,
                range.start.unwrap_or(0),
                range.end.unwrap_or(0)
            ))
            .collect::<Vec<_>>()
            .join(", "),
        request.prompt
    )
}

fn stable_visual_id(value: &str) -> String {
    let mut hash: u64 = 14_695_981_039_346_656_037;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    format!("{hash:016x}")
}
