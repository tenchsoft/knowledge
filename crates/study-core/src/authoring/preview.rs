use std::collections::HashMap;

use crate::{
    ContentLocale, CurriculumEdgeKind, CurriculumNodeId, LocalizedStringSet, LocalizedText,
    SubjectDomain,
};

use super::*;

pub fn preview_curriculum_pack_draft(
    draft: &CurriculumPackDraft,
    locale: Option<ContentLocale>,
) -> CurriculumPackPreviewSnapshot {
    let locale = locale.unwrap_or_else(|| draft.curriculum.locale.clone());
    let localization = curriculum_pack_localization_report(draft);
    let mut child_counts = HashMap::<CurriculumNodeId, usize>::new();
    for edge in &draft.curriculum.graph.edges {
        if edge.relation == CurriculumEdgeKind::Contains {
            *child_counts.entry(edge.from.clone()).or_default() += 1;
        }
    }

    let mut practice_counts = HashMap::<CurriculumNodeId, usize>::new();
    for item in &draft.problems {
        *practice_counts.entry(item.node_id.clone()).or_default() += 1;
    }

    let mut visual_counts = HashMap::<CurriculumNodeId, usize>::new();
    for visual in &draft.visuals {
        *visual_counts.entry(visual.node_id.clone()).or_default() += 1;
    }

    let mut assessment_counts = HashMap::<CurriculumNodeId, usize>::new();
    for assessment in &draft.assessments {
        for node_id in &assessment.node_ids {
            *assessment_counts.entry(node_id.clone()).or_default() += 1;
        }
    }

    let mut glossary_counts = HashMap::<CurriculumNodeId, usize>::new();
    for term in &draft.glossary {
        *glossary_counts.entry(term.node_id.clone()).or_default() += 1;
    }

    let outline = draft
        .curriculum
        .graph
        .nodes
        .iter()
        .map(|node| CurriculumPackPreviewNode {
            node_id: node.id.clone(),
            title: localized_string_set_value(&node.title, &locale),
            kind: node.kind,
            level: node.level,
            child_count: *child_counts.get(&node.id).unwrap_or(&0),
            practice_count: *practice_counts.get(&node.id).unwrap_or(&0),
            visual_count: *visual_counts.get(&node.id).unwrap_or(&0),
            assessment_count: *assessment_counts.get(&node.id).unwrap_or(&0),
            glossary_count: *glossary_counts.get(&node.id).unwrap_or(&0),
        })
        .collect::<Vec<_>>();

    CurriculumPackPreviewSnapshot {
        draft_id: draft.id.clone(),
        title: localized_string_set_value(&draft.curriculum.title, &locale),
        domain_label: subject_domain_label(&draft.curriculum.domain),
        locale,
        node_count: draft.curriculum.graph.nodes.len(),
        edge_count: draft.curriculum.graph.edges.len(),
        lesson_count: draft.lessons.len(),
        practice_count: draft.problems.len(),
        visual_count: draft.visuals.len(),
        assessment_count: draft.assessments.len(),
        glossary_count: draft.glossary.len(),
        orphan_node_count: draft.curriculum.graph.orphan_nodes().len(),
        validation: draft.validate_for_distribution(),
        localization,
        outline,
    }
}

pub fn curriculum_pack_localization_report(
    draft: &CurriculumPackDraft,
) -> CurriculumPackLocalizationReport {
    let default_locale = draft.curriculum.locale.clone();
    let required_locales = draft_required_locales(draft);
    let mut report = CurriculumPackLocalizationReport {
        default_locale: default_locale.clone(),
        required_locales: required_locales.clone(),
        checked_field_count: 0,
        missing: Vec::new(),
        release_ready: true,
    };

    check_string_set_locales(
        &mut report,
        &required_locales,
        &default_locale,
        "curriculum",
        draft.curriculum.id.as_str(),
        "title",
        &draft.curriculum.title,
    );
    check_string_set_locales(
        &mut report,
        &required_locales,
        &default_locale,
        "curriculum",
        draft.curriculum.id.as_str(),
        "description",
        &draft.curriculum.description,
    );

    for node in &draft.curriculum.graph.nodes {
        check_string_set_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "curriculum_node",
            node.id.as_str(),
            "title",
            &node.title,
        );
        check_string_set_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "curriculum_node",
            node.id.as_str(),
            "summary",
            &node.summary,
        );
        for objective in &node.objectives {
            check_text_locales(
                &mut report,
                &required_locales,
                &default_locale,
                "learning_objective",
                objective.id.as_str(),
                "statement",
                &objective.statement,
            );
        }
    }

    for lesson in &draft.lessons {
        check_string_set_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "lesson",
            lesson.id.as_str(),
            "title",
            &lesson.title,
        );
        check_text_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "lesson",
            lesson.id.as_str(),
            "accessibility_summary",
            &lesson.accessibility_summary,
        );
        for (index, block) in lesson.blocks.iter().enumerate() {
            let block_id = format!("{}:block:{index}", lesson.id.as_str());
            match block {
                LessonBlock::Paragraph { text } => check_text_locales(
                    &mut report,
                    &required_locales,
                    &default_locale,
                    "lesson_block",
                    &block_id,
                    "text",
                    text,
                ),
                LessonBlock::WorkedExample { prompt, solution } => {
                    check_text_locales(
                        &mut report,
                        &required_locales,
                        &default_locale,
                        "lesson_block",
                        &block_id,
                        "prompt",
                        prompt,
                    );
                    check_text_locales(
                        &mut report,
                        &required_locales,
                        &default_locale,
                        "lesson_block",
                        &block_id,
                        "solution",
                        solution,
                    );
                }
                LessonBlock::Callout { label, body } => {
                    check_text_locales(
                        &mut report,
                        &required_locales,
                        &default_locale,
                        "lesson_block",
                        &block_id,
                        "label",
                        label,
                    );
                    check_text_locales(
                        &mut report,
                        &required_locales,
                        &default_locale,
                        "lesson_block",
                        &block_id,
                        "body",
                        body,
                    );
                }
                LessonBlock::Visual { .. }
                | LessonBlock::PracticeHook { .. }
                | LessonBlock::GlossaryLink { .. } => {}
            }
        }
    }

    for practice in &draft.problems {
        check_text_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "practice_item",
            practice.id.as_str(),
            "prompt",
            &practice.prompt,
        );
        check_text_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "practice_item",
            practice.id.as_str(),
            "explanation",
            &practice.explanation,
        );
    }

    for visual in &draft.visuals {
        check_text_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "learning_visual",
            visual.id.as_str(),
            "title",
            &visual.title,
        );
        check_text_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "learning_visual",
            visual.id.as_str(),
            "description",
            &visual.description,
        );
    }

    for assessment in &draft.assessments {
        check_string_set_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "assessment",
            assessment.id.as_str(),
            "title",
            &assessment.title,
        );
        check_text_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "assessment",
            assessment.id.as_str(),
            "report_template",
            &assessment.report_template,
        );
    }

    for term in &draft.glossary {
        check_string_set_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "glossary_term",
            term.id.as_str(),
            "term",
            &term.term,
        );
        check_string_set_locales(
            &mut report,
            &required_locales,
            &default_locale,
            "glossary_term",
            term.id.as_str(),
            "definition",
            &term.definition,
        );
        for (index, alias) in term.aliases.iter().enumerate() {
            check_string_set_locales(
                &mut report,
                &required_locales,
                &default_locale,
                "glossary_alias",
                &format!("{}:{index}", term.id.as_str()),
                "alias",
                alias,
            );
        }
    }

    report.release_ready = report.missing.is_empty();
    report
}

fn draft_required_locales(draft: &CurriculumPackDraft) -> Vec<ContentLocale> {
    let mut locales = if draft.manifest.required_locales.is_empty() {
        vec![draft.curriculum.locale.clone()]
    } else {
        draft.manifest.required_locales.clone()
    };
    if !locales.contains(&draft.curriculum.locale) {
        locales.push(draft.curriculum.locale.clone());
    }
    locales
}

fn check_string_set_locales(
    report: &mut CurriculumPackLocalizationReport,
    required_locales: &[ContentLocale],
    default_locale: &ContentLocale,
    item_kind: &str,
    item_id: &str,
    field: &str,
    value: &LocalizedStringSet,
) {
    for locale in required_locales {
        report.checked_field_count += 1;
        if !localized_string_set_has_locale(value, locale, default_locale) {
            report.missing.push(CurriculumPackLocalizationGap {
                item_kind: item_kind.to_string(),
                item_id: item_id.to_string(),
                field: field.to_string(),
                locale: locale.clone(),
            });
        }
    }
}

fn check_text_locales(
    report: &mut CurriculumPackLocalizationReport,
    required_locales: &[ContentLocale],
    default_locale: &ContentLocale,
    item_kind: &str,
    item_id: &str,
    field: &str,
    value: &LocalizedText,
) {
    for locale in required_locales {
        report.checked_field_count += 1;
        if !localized_text_has_locale(value, locale, default_locale) {
            report.missing.push(CurriculumPackLocalizationGap {
                item_kind: item_kind.to_string(),
                item_id: item_id.to_string(),
                field: field.to_string(),
                locale: locale.clone(),
            });
        }
    }
}

fn localized_string_set_has_locale(
    value: &LocalizedStringSet,
    locale: &ContentLocale,
    default_locale: &ContentLocale,
) -> bool {
    if locale == default_locale && !value.default.value.trim().is_empty() {
        return true;
    }
    value.translations.iter().any(|translation| {
        translation.locale.as_ref() == Some(locale) && !translation.value.trim().is_empty()
    })
}

fn localized_text_has_locale(
    value: &LocalizedText,
    locale: &ContentLocale,
    default_locale: &ContentLocale,
) -> bool {
    if locale == default_locale && !value.value.trim().is_empty() {
        return true;
    }
    value.locale.as_ref() == Some(locale) && !value.value.trim().is_empty()
}

fn localized_string_set_value(value: &LocalizedStringSet, locale: &ContentLocale) -> String {
    value
        .translations
        .iter()
        .find(|translation| translation.locale.as_ref() == Some(locale))
        .unwrap_or(&value.default)
        .value
        .clone()
}

fn subject_domain_label(domain: &SubjectDomain) -> String {
    match domain {
        SubjectDomain::Mathematics => "mathematics".to_string(),
        SubjectDomain::Science => "science".to_string(),
        SubjectDomain::Language => "language".to_string(),
        SubjectDomain::Programming => "programming".to_string(),
        SubjectDomain::Custom { label, .. } => label.clone(),
    }
}
