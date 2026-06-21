use std::collections::HashSet;

use crate::{
    AnswerKey, CurriculumIssueSeverity, CurriculumNodeId, GlossaryTermId, LearningVisualId,
    PackValidationSeverity, PracticeItem, PracticeItemId, SubjectDomain,
};

use super::*;

impl CurriculumPackDraft {
    pub fn validate_for_distribution(&self) -> DraftValidationReport {
        let mut issues = Vec::new();

        if self.owner.name.trim().is_empty() {
            issues.push(DraftValidationIssue::error(
                "owner_required",
                "custom pack requires an owner name",
            ));
        }
        if self.license.name.trim().is_empty() {
            issues.push(DraftValidationIssue::error(
                "license_required",
                "custom pack requires a license",
            ));
        }
        if self.update_policy.channel == PackUpdateChannel::Unspecified {
            issues.push(DraftValidationIssue::error(
                "update_policy_required",
                "custom pack requires an update policy",
            ));
        }
        if self.lessons.is_empty() {
            issues.push(DraftValidationIssue::error(
                "lesson_required",
                "curriculum pack requires at least one lesson",
            ));
        }
        if self.problems.is_empty() {
            issues.push(DraftValidationIssue::warning(
                "problem_bank_empty",
                "curriculum pack has no practice items",
            ));
        }
        if matches!(self.curriculum.domain, SubjectDomain::Custom { .. })
            && !self.curriculum.authority.custom
        {
            issues.push(DraftValidationIssue::error(
                "custom_authority_required",
                "custom domains must be marked as custom authority",
            ));
        }

        let node_ids = self
            .curriculum
            .graph
            .nodes
            .iter()
            .map(|node| node.id.clone())
            .collect::<HashSet<_>>();
        let visual_ids = self
            .visuals
            .iter()
            .map(|visual| visual.id.clone())
            .collect::<HashSet<_>>();
        let practice_ids = self
            .problems
            .iter()
            .map(|problem| problem.id.clone())
            .collect::<HashSet<_>>();
        let assessment_ids = self
            .assessments
            .iter()
            .map(|assessment| assessment.id.clone())
            .collect::<HashSet<_>>();
        let glossary_ids = self
            .glossary
            .iter()
            .map(|term| term.id.clone())
            .collect::<HashSet<_>>();

        push_duplicate_issue(
            &mut issues,
            "duplicate_lesson_id",
            "lesson",
            self.lessons
                .iter()
                .map(|lesson| lesson.id.as_str().to_string()),
        );
        push_duplicate_issue(
            &mut issues,
            "duplicate_practice_item_id",
            "practice item",
            self.problems
                .iter()
                .map(|problem| problem.id.as_str().to_string()),
        );
        push_duplicate_issue(
            &mut issues,
            "duplicate_visual_id",
            "visual",
            self.visuals
                .iter()
                .map(|visual| visual.id.as_str().to_string()),
        );
        push_duplicate_issue(
            &mut issues,
            "duplicate_assessment_id",
            "assessment",
            self.assessments
                .iter()
                .map(|assessment| assessment.id.as_str().to_string()),
        );
        push_duplicate_issue(
            &mut issues,
            "duplicate_glossary_term_id",
            "glossary term",
            self.glossary
                .iter()
                .map(|term| term.id.as_str().to_string()),
        );

        for issue in self.curriculum.validate().issues {
            let severity = match issue.severity {
                CurriculumIssueSeverity::Info => DraftValidationSeverity::Info,
                CurriculumIssueSeverity::Warning => DraftValidationSeverity::Warning,
                CurriculumIssueSeverity::Error => DraftValidationSeverity::Error,
            };
            issues.push(DraftValidationIssue {
                severity,
                code: issue.code,
                message: issue.message,
            });
        }

        let pack_report = self.manifest.validate_for_release();
        for issue in pack_report.issues {
            let severity = match issue.severity {
                PackValidationSeverity::Info => DraftValidationSeverity::Info,
                PackValidationSeverity::Warning => DraftValidationSeverity::Warning,
                PackValidationSeverity::Error => DraftValidationSeverity::Error,
            };
            issues.push(DraftValidationIssue {
                severity,
                code: issue.code,
                message: issue.message,
            });
        }
        for gap in curriculum_pack_localization_report(self).missing {
            issues.push(DraftValidationIssue::error(
                "localization_missing",
                format!(
                    "{} {} field {} is missing locale {}",
                    gap.item_kind,
                    gap.item_id,
                    gap.field,
                    gap.locale.bcp47()
                ),
            ));
        }

        for lesson in &self.lessons {
            if !node_ids.contains(&lesson.node_id) {
                issues.push(DraftValidationIssue::error(
                    "lesson_node_missing",
                    format!(
                        "lesson {} references missing node {}",
                        lesson.id.as_str(),
                        lesson.node_id.as_str()
                    ),
                ));
            }
            if lesson.blocks.is_empty() {
                issues.push(DraftValidationIssue::error(
                    "lesson_blocks_required",
                    format!("lesson {} requires content blocks", lesson.id.as_str()),
                ));
            }
            if lesson.accessibility_summary.value.trim().is_empty() {
                issues.push(DraftValidationIssue::error(
                    "lesson_accessibility_required",
                    format!(
                        "lesson {} requires accessibility summary",
                        lesson.id.as_str()
                    ),
                ));
            }
            for visual_id in &lesson.visual_ids {
                if !visual_ids.contains(visual_id) {
                    issues.push(DraftValidationIssue::error(
                        "lesson_visual_missing",
                        format!(
                            "lesson {} references missing visual {}",
                            lesson.id.as_str(),
                            visual_id.as_str()
                        ),
                    ));
                }
            }
            for term_id in &lesson.glossary_terms {
                if !glossary_ids.contains(term_id) {
                    issues.push(DraftValidationIssue::error(
                        "lesson_glossary_missing",
                        format!(
                            "lesson {} references missing glossary term {}",
                            lesson.id.as_str(),
                            term_id.as_str()
                        ),
                    ));
                }
            }
            validate_lesson_block_refs(
                lesson,
                &visual_ids,
                &practice_ids,
                &glossary_ids,
                &mut issues,
            );
        }

        for visual in &self.visuals {
            if let Err(message) = visual.validate_for_release() {
                issues.push(DraftValidationIssue::error("visual_invalid", message));
            }
            if !node_ids.contains(&visual.node_id) {
                issues.push(DraftValidationIssue::error(
                    "visual_node_missing",
                    format!(
                        "visual {} references missing node {}",
                        visual.id.as_str(),
                        visual.node_id.as_str()
                    ),
                ));
            }
        }

        for problem in &self.problems {
            if let Err(message) = validate_practice_item_for_authoring(problem) {
                issues.push(DraftValidationIssue::error(
                    "practice_item_invalid",
                    message,
                ));
            }
            if !node_ids.contains(&problem.node_id) {
                issues.push(DraftValidationIssue::error(
                    "practice_node_missing",
                    format!(
                        "practice item {} references missing node {}",
                        problem.id.as_str(),
                        problem.node_id.as_str()
                    ),
                ));
            }
        }

        for assessment in &self.assessments {
            if assessment.item_ids.is_empty() {
                issues.push(DraftValidationIssue::error(
                    "assessment_items_required",
                    format!(
                        "assessment {} requires practice items",
                        assessment.id.as_str()
                    ),
                ));
            }
            for node_id in &assessment.node_ids {
                if !node_ids.contains(node_id) {
                    issues.push(DraftValidationIssue::error(
                        "assessment_node_missing",
                        format!(
                            "assessment {} references missing node {}",
                            assessment.id.as_str(),
                            node_id.as_str()
                        ),
                    ));
                }
            }
            for item_id in &assessment.item_ids {
                if !practice_ids.contains(item_id) {
                    issues.push(DraftValidationIssue::error(
                        "assessment_practice_item_missing",
                        format!(
                            "assessment {} references missing practice item {}",
                            assessment.id.as_str(),
                            item_id.as_str()
                        ),
                    ));
                }
            }
            if assessment.coverage_constraints.is_empty() {
                issues.push(DraftValidationIssue::warning(
                    "assessment_coverage_missing",
                    format!(
                        "assessment {} should declare objective coverage",
                        assessment.id.as_str()
                    ),
                ));
            }
        }

        for term in &self.glossary {
            if let Err(message) = term.validate_for_release() {
                issues.push(DraftValidationIssue::error("glossary_invalid", message));
            }
            if !node_ids.contains(&term.node_id) {
                issues.push(DraftValidationIssue::error(
                    "glossary_node_missing",
                    format!(
                        "glossary term {} references missing node {}",
                        term.id.as_str(),
                        term.node_id.as_str()
                    ),
                ));
            }
        }

        for lesson in &self.manifest.lessons {
            if !node_ids.contains(&lesson.node_id) {
                issues.push(DraftValidationIssue::error(
                    "manifest_lesson_node_missing",
                    format!(
                        "manifest lesson path {} references missing node {}",
                        lesson.path,
                        lesson.node_id.as_str()
                    ),
                ));
            }
        }
        for visual_id in &self.manifest.visuals {
            if !visual_ids.contains(visual_id) {
                issues.push(DraftValidationIssue::error(
                    "manifest_visual_missing",
                    format!("manifest references missing visual {}", visual_id.as_str()),
                ));
            }
        }
        for item_id in &self.manifest.practice_items {
            if !practice_ids.contains(item_id) {
                issues.push(DraftValidationIssue::error(
                    "manifest_practice_item_missing",
                    format!(
                        "manifest references missing practice item {}",
                        item_id.as_str()
                    ),
                ));
            }
        }
        for assessment_id in &self.manifest.assessments {
            if !assessment_ids.contains(assessment_id) {
                issues.push(DraftValidationIssue::error(
                    "manifest_assessment_missing",
                    format!(
                        "manifest references missing assessment {}",
                        assessment_id.as_str()
                    ),
                ));
            }
        }
        for term_id in &self.manifest.glossary_terms {
            if !glossary_ids.contains(term_id) {
                issues.push(DraftValidationIssue::error(
                    "manifest_glossary_missing",
                    format!(
                        "manifest references missing glossary term {}",
                        term_id.as_str()
                    ),
                ));
            }
        }

        DraftValidationReport { issues }
    }
}

pub(super) fn ensure_draft_node_exists(
    draft: &CurriculumPackDraft,
    node_id: &CurriculumNodeId,
) -> Result<(), String> {
    if draft
        .curriculum
        .graph
        .nodes
        .iter()
        .any(|node| &node.id == node_id)
    {
        Ok(())
    } else {
        Err(format!(
            "curriculum node {} does not exist",
            node_id.as_str()
        ))
    }
}

pub(super) fn validate_practice_item_for_authoring(item: &PracticeItem) -> Result<(), String> {
    if item.prompt.value.trim().is_empty() {
        return Err(format!(
            "practice item {} requires a prompt",
            item.id.as_str()
        ));
    }
    if item.explanation.value.trim().is_empty() {
        return Err(format!(
            "practice item {} requires an explanation",
            item.id.as_str()
        ));
    }
    if let Some(difficulty) = item.difficulty {
        if !difficulty.is_finite() || !(0.0..=1.0).contains(&difficulty) {
            return Err(format!(
                "practice item {} difficulty must be between 0 and 1",
                item.id.as_str()
            ));
        }
    }
    match &item.answer_key {
        AnswerKey::Exact { value, .. } => {
            if value.trim().is_empty() {
                return Err(format!(
                    "practice item {} requires a non-empty exact answer",
                    item.id.as_str()
                ));
            }
        }
        AnswerKey::Numeric { value, tolerance } => {
            if !value.is_finite() || !tolerance.is_finite() || *tolerance < 0.0 {
                return Err(format!(
                    "practice item {} requires a finite numeric answer and non-negative tolerance",
                    item.id.as_str()
                ));
            }
        }
        AnswerKey::MultipleChoice { option_id } => {
            if option_id.trim().is_empty() {
                return Err(format!(
                    "practice item {} requires a selected option id",
                    item.id.as_str()
                ));
            }
        }
        AnswerKey::Cloze { accepted } => {
            if accepted.iter().all(|answer| answer.trim().is_empty()) {
                return Err(format!(
                    "practice item {} requires at least one accepted cloze answer",
                    item.id.as_str()
                ));
            }
        }
        AnswerKey::Rubric {
            rubric_id,
            max_score,
        } => {
            if rubric_id.trim().is_empty() || !max_score.is_finite() || *max_score <= 0.0 {
                return Err(format!(
                    "practice item {} requires a rubric id and positive max score",
                    item.id.as_str()
                ));
            }
        }
    }
    Ok(())
}

fn push_duplicate_issue(
    issues: &mut Vec<DraftValidationIssue>,
    code: &'static str,
    label: &'static str,
    ids: impl IntoIterator<Item = String>,
) {
    let mut seen = HashSet::new();
    for id in ids {
        if !seen.insert(id.clone()) {
            issues.push(DraftValidationIssue::error(
                code,
                format!("duplicate {label} id {id}"),
            ));
        }
    }
}

fn validate_lesson_block_refs(
    lesson: &LessonDraft,
    visual_ids: &HashSet<LearningVisualId>,
    practice_ids: &HashSet<PracticeItemId>,
    glossary_ids: &HashSet<GlossaryTermId>,
    issues: &mut Vec<DraftValidationIssue>,
) {
    for block in &lesson.blocks {
        match block {
            LessonBlock::Visual { visual_id } => {
                if !visual_ids.contains(visual_id) {
                    issues.push(DraftValidationIssue::error(
                        "lesson_block_visual_missing",
                        format!(
                            "lesson {} visual block references missing visual {}",
                            lesson.id.as_str(),
                            visual_id.as_str()
                        ),
                    ));
                }
            }
            LessonBlock::PracticeHook { item_ids } => {
                for item_id in item_ids {
                    if !practice_ids.contains(item_id) {
                        issues.push(DraftValidationIssue::error(
                            "lesson_block_practice_item_missing",
                            format!(
                                "lesson {} practice hook references missing item {}",
                                lesson.id.as_str(),
                                item_id.as_str()
                            ),
                        ));
                    }
                }
            }
            LessonBlock::GlossaryLink { term_id } => {
                if !glossary_ids.contains(term_id) {
                    issues.push(DraftValidationIssue::error(
                        "lesson_block_glossary_missing",
                        format!(
                            "lesson {} glossary link references missing term {}",
                            lesson.id.as_str(),
                            term_id.as_str()
                        ),
                    ));
                }
            }
            LessonBlock::Paragraph { .. }
            | LessonBlock::WorkedExample { .. }
            | LessonBlock::Callout { .. } => {}
        }
    }
}
