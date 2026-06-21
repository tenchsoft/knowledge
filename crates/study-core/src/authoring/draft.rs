use crate::{
    Curriculum, CurriculumAuthority, CurriculumGraph, CurriculumNode, CurriculumNodeKind,
    GlossaryTerm, LearningObjective, LearningObjectiveId, LearningVisualSpec, LessonManifest,
    PackIntegrity, PracticeItem, SubjectDomain,
};

use super::validation::{ensure_draft_node_exists, validate_practice_item_for_authoring};
use super::*;

pub fn export_curriculum_pack_draft_json(
    draft: &CurriculumPackDraft,
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(draft)
}

pub fn import_curriculum_pack_draft_json(
    text: &str,
) -> Result<CurriculumPackDraft, serde_json::Error> {
    serde_json::from_str(text)
}

pub fn new_custom_curriculum_pack_draft(
    request: CustomCurriculumDraftRequest,
) -> CurriculumPackDraft {
    let required_locales = if request.required_locales.is_empty() {
        vec![request.default_locale.clone()]
    } else {
        request.required_locales.clone()
    };
    let mut provided_locales = required_locales.clone();
    if !provided_locales.contains(&request.default_locale) {
        provided_locales.push(request.default_locale.clone());
    }

    CurriculumPackDraft {
        id: request.draft_id,
        owner: request.owner.clone(),
        license: request.license,
        update_policy: request.update_policy,
        curriculum: Curriculum {
            id: request.curriculum_id.clone(),
            domain: SubjectDomain::Custom {
                id: request.domain_id,
                label: request.domain_label,
                owner: request.owner.name.clone(),
            },
            title: request.title.clone(),
            description: request.description,
            locale: request.default_locale.clone(),
            supported_locales: provided_locales.clone(),
            authority: CurriculumAuthority {
                owner: request.owner.name,
                source_url: None,
                version: Some(request.version.clone()),
                custom: true,
            },
            level_range: request.level_range,
            graph: CurriculumGraph::default(),
            standards: Vec::new(),
            metadata: crate::CurriculumMetadata {
                created_at: Some(request.created_at.clone()),
                updated_at: Some(request.created_at),
                source_notes: Vec::new(),
            },
        },
        manifest: ContentPackManifest {
            id: request.pack_id,
            title: request.title.default.value,
            curriculum_id: request.curriculum_id,
            version: request.version,
            default_locale: request.default_locale,
            required_locales,
            provided_locales,
            lessons: Vec::new(),
            assets: Vec::new(),
            visuals: Vec::new(),
            practice_items: Vec::new(),
            assessments: Vec::new(),
            glossary_terms: Vec::new(),
            integrity: PackIntegrity {
                algorithm: "sha256".to_string(),
                content_hash: request.integrity_hash,
            },
        },
        lessons: Vec::new(),
        problems: Vec::new(),
        visuals: Vec::new(),
        assessments: Vec::new(),
        glossary: Vec::new(),
    }
}

pub fn add_lesson_to_curriculum_pack_draft(
    mut draft: CurriculumPackDraft,
    input: LessonDraftInput,
) -> Result<CurriculumPackDraft, String> {
    if draft
        .lessons
        .iter()
        .any(|lesson| lesson.id == input.lesson_id)
    {
        return Err(format!("duplicate lesson id {}", input.lesson_id.as_str()));
    }
    if !draft.curriculum.level_range.contains(input.level) {
        return Err(format!(
            "lesson level {} is outside curriculum level range",
            input.level.label()
        ));
    }
    if input.blocks.is_empty() {
        return Err("lesson requires at least one content block".to_string());
    }
    if input.accessibility_summary.value.trim().is_empty() {
        return Err("lesson requires an accessibility summary".to_string());
    }

    let objective_id = LearningObjectiveId::from(format!("objective-{}", input.node_id.as_str()));
    let node = CurriculumNode {
        id: input.node_id.clone(),
        kind: CurriculumNodeKind::Lesson,
        title: input.title.clone(),
        summary: input.summary,
        level: input.level,
        strand: input.strand,
        objectives: vec![LearningObjective {
            id: objective_id,
            statement: input.objective,
            taxonomy: input.taxonomy,
            measurable: true,
        }],
        skills: Vec::new(),
        standards: Vec::new(),
        visuals: input.visual_ids.clone(),
        estimated_minutes: input.estimated_minutes,
    };
    draft.curriculum.graph.add_node(node)?;

    draft.manifest.lessons.push(LessonManifest {
        node_id: input.node_id.clone(),
        locale: draft.curriculum.locale.clone(),
        path: format!("lessons/{}.json", input.lesson_id.as_str()),
        estimated_minutes: input.estimated_minutes,
    });
    for visual_id in &input.visual_ids {
        if !draft.manifest.visuals.contains(visual_id) {
            draft.manifest.visuals.push(visual_id.clone());
        }
    }
    for term_id in &input.glossary_terms {
        if !draft.manifest.glossary_terms.contains(term_id) {
            draft.manifest.glossary_terms.push(term_id.clone());
        }
    }
    draft.lessons.push(LessonDraft {
        id: input.lesson_id,
        node_id: input.node_id,
        title: input.title,
        blocks: input.blocks,
        visual_ids: input.visual_ids,
        glossary_terms: input.glossary_terms,
        accessibility_summary: input.accessibility_summary,
        reading_level: input.reading_level,
    });
    draft.curriculum.metadata.updated_at = draft.curriculum.metadata.created_at.clone();
    Ok(draft)
}

pub fn add_practice_item_to_curriculum_pack_draft(
    mut draft: CurriculumPackDraft,
    item: PracticeItem,
) -> Result<CurriculumPackDraft, String> {
    if draft.problems.iter().any(|existing| existing.id == item.id) {
        return Err(format!("duplicate practice item id {}", item.id.as_str()));
    }
    ensure_draft_node_exists(&draft, &item.node_id)?;
    validate_practice_item_for_authoring(&item)?;

    if !draft.manifest.practice_items.contains(&item.id) {
        draft.manifest.practice_items.push(item.id.clone());
    }
    draft.problems.push(item);
    draft.curriculum.metadata.updated_at = draft.curriculum.metadata.created_at.clone();
    Ok(draft)
}

pub fn add_learning_visual_to_curriculum_pack_draft(
    mut draft: CurriculumPackDraft,
    visual: LearningVisualSpec,
) -> Result<CurriculumPackDraft, String> {
    if draft
        .visuals
        .iter()
        .any(|existing| existing.id == visual.id)
    {
        return Err(format!("duplicate visual id {}", visual.id.as_str()));
    }
    ensure_draft_node_exists(&draft, &visual.node_id)?;
    visual.validate_for_release()?;

    if let Some(node) = draft
        .curriculum
        .graph
        .nodes
        .iter_mut()
        .find(|node| node.id == visual.node_id)
    {
        if !node.visuals.contains(&visual.id) {
            node.visuals.push(visual.id.clone());
        }
    }
    if !draft.manifest.visuals.contains(&visual.id) {
        draft.manifest.visuals.push(visual.id.clone());
    }
    draft.visuals.push(visual);
    draft.curriculum.metadata.updated_at = draft.curriculum.metadata.created_at.clone();
    Ok(draft)
}

pub fn add_assessment_to_curriculum_pack_draft(
    mut draft: CurriculumPackDraft,
    assessment: AssessmentDraft,
) -> Result<CurriculumPackDraft, String> {
    if draft
        .assessments
        .iter()
        .any(|existing| existing.id == assessment.id)
    {
        return Err(format!(
            "duplicate assessment id {}",
            assessment.id.as_str()
        ));
    }
    if assessment.item_ids.is_empty() {
        return Err("assessment requires at least one practice item".to_string());
    }
    if assessment.node_ids.is_empty() {
        return Err("assessment requires at least one curriculum node".to_string());
    }
    if assessment.report_template.value.trim().is_empty() {
        return Err("assessment requires a report template".to_string());
    }
    for node_id in &assessment.node_ids {
        ensure_draft_node_exists(&draft, node_id)?;
    }
    for item_id in &assessment.item_ids {
        if !draft.problems.iter().any(|problem| &problem.id == item_id) {
            return Err(format!(
                "assessment references missing practice item {}",
                item_id.as_str()
            ));
        }
    }

    if !draft.manifest.assessments.contains(&assessment.id) {
        draft.manifest.assessments.push(assessment.id.clone());
    }
    draft.assessments.push(assessment);
    draft.curriculum.metadata.updated_at = draft.curriculum.metadata.created_at.clone();
    Ok(draft)
}

pub fn add_glossary_term_to_curriculum_pack_draft(
    mut draft: CurriculumPackDraft,
    term: GlossaryTerm,
) -> Result<CurriculumPackDraft, String> {
    if draft.glossary.iter().any(|existing| existing.id == term.id) {
        return Err(format!("duplicate glossary term id {}", term.id.as_str()));
    }
    ensure_draft_node_exists(&draft, &term.node_id)?;
    term.validate_for_release()?;

    if !draft.manifest.glossary_terms.contains(&term.id) {
        draft.manifest.glossary_terms.push(term.id.clone());
    }
    draft.glossary.push(term);
    draft.curriculum.metadata.updated_at = draft.curriculum.metadata.created_at.clone();
    Ok(draft)
}

pub fn publish_curriculum_pack_draft(
    draft: CurriculumPackDraft,
    published_at: String,
) -> Result<PublishedCurriculumPack, DraftValidationReport> {
    let report = draft.validate_for_distribution();
    if !report.is_valid() {
        return Err(report);
    }
    Ok(PublishedCurriculumPack {
        owner: draft.owner,
        license: draft.license,
        update_policy: draft.update_policy,
        curriculum: draft.curriculum,
        manifest: draft.manifest,
        lessons: draft.lessons,
        problems: draft.problems,
        visuals: draft.visuals,
        assessments: draft.assessments,
        glossary: draft.glossary,
        published_at,
    })
}
