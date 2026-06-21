use super::document::contains_section_id;
use super::export::{block_to_plain_text, inline_nodes_to_text};
use super::*;
use std::collections::BTreeSet;
use tench_document_core::BlockNode;

pub fn run_non_ai_writing_checks(manuscript: &ResearchManuscript) -> Vec<WritingCheckResult> {
    let mut results = Vec::new();
    for required in &manuscript.outline.required_sections {
        if !contains_section_kind(&manuscript.outline.sections, required.kind) {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Error,
                code: "missing_required_section".to_string(),
                message: format!("Missing required section: {}", required.label),
                section_id: None,
                reference_id: None,
                export_blocker: true,
            });
        }
    }
    if let Some(limit) = manuscript.target.word_limit {
        let word_count = manuscript_word_count(manuscript);
        if word_count > limit {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Error,
                code: "word_limit_exceeded".to_string(),
                message: format!("Manuscript has {word_count} words; target limit is {limit}"),
                section_id: None,
                reference_id: None,
                export_blocker: true,
            });
        }
    }
    append_section_writing_checks(manuscript, &manuscript.outline.sections, &mut results);
    append_citation_writing_checks(manuscript, &mut results);
    for asset in &manuscript.assets {
        if asset.caption.value.trim().is_empty() {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Warning,
                code: "missing_asset_caption".to_string(),
                message: format!("Missing caption for {}", asset.label),
                section_id: None,
                reference_id: None,
                export_blocker: false,
            });
        }
        if manuscript
            .target
            .figure_table_rules
            .iter()
            .any(|rule| rule.kind == asset.kind && rule.alt_text_required)
            && asset
                .alt_text
                .as_deref()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
        {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Warning,
                code: "missing_asset_alt_text".to_string(),
                message: format!("Missing alt text for {}", asset.label),
                section_id: None,
                reference_id: asset.linked_references.first().cloned(),
                export_blocker: false,
            });
        }
    }
    for cross_reference in &manuscript.cross_references {
        if !contains_section_id(&manuscript.outline.sections, &cross_reference.section_id) {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Error,
                code: "broken_cross_reference_section".to_string(),
                message: format!(
                    "Cross-reference {} points to missing section {}",
                    cross_reference.id.as_str(),
                    cross_reference.section_id.as_str()
                ),
                section_id: Some(cross_reference.section_id.clone()),
                reference_id: None,
                export_blocker: true,
            });
        }
        match &cross_reference.target {
            ManuscriptCrossReferenceTarget::Asset { asset_id } => {
                if !manuscript.assets.iter().any(|asset| asset.id == *asset_id) {
                    results.push(WritingCheckResult {
                        severity: WritingCheckSeverity::Error,
                        code: "broken_cross_reference_asset".to_string(),
                        message: format!(
                            "Cross-reference {} points to missing manuscript asset {}",
                            cross_reference.id.as_str(),
                            asset_id.as_str()
                        ),
                        section_id: Some(cross_reference.section_id.clone()),
                        reference_id: None,
                        export_blocker: true,
                    });
                }
            }
        }
    }
    for issue in &manuscript.citation_state.unresolved_citations {
        results.push(WritingCheckResult {
            severity: if issue.code == "missing_reference" {
                WritingCheckSeverity::Error
            } else {
                WritingCheckSeverity::Warning
            },
            code: issue.code.clone(),
            message: issue.message.clone(),
            section_id: None,
            reference_id: issue.reference_id.clone(),
            export_blocker: issue.code == "missing_reference",
        });
    }
    results
}

fn append_section_writing_checks(
    manuscript: &ResearchManuscript,
    sections: &[OutlineSection],
    results: &mut Vec<WritingCheckResult>,
) {
    for section in sections {
        let section_text = section_text(manuscript, section);
        let word_count = section_text.split_whitespace().count() as u32;
        if section.status == SectionStatus::NeedsCitation
            || (!section_text.trim().is_empty()
                && section.cited_references.is_empty()
                && requires_section_citation(section.kind))
        {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Warning,
                code: "missing_section_citation".to_string(),
                message: format!(
                    "Section '{}' has text without cited sources",
                    section.title.value
                ),
                section_id: Some(section.id.clone()),
                reference_id: None,
                export_blocker: false,
            });
        }
        if let Some(limit) = section
            .target_words
            .or_else(|| abstract_limit_for_section(manuscript, section))
        {
            if word_count > limit {
                results.push(WritingCheckResult {
                    severity: if section.kind == SectionKind::Abstract {
                        WritingCheckSeverity::Error
                    } else {
                        WritingCheckSeverity::Warning
                    },
                    code: if section.kind == SectionKind::Abstract {
                        "abstract_limit_exceeded"
                    } else {
                        "section_word_limit_exceeded"
                    }
                    .to_string(),
                    message: format!(
                        "Section '{}' has {word_count} words; target limit is {limit}",
                        section.title.value
                    ),
                    section_id: Some(section.id.clone()),
                    reference_id: None,
                    export_blocker: section.kind == SectionKind::Abstract,
                });
            }
        }
        if contains_unresolved_todo(&section_text) {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Warning,
                code: "unresolved_todo".to_string(),
                message: format!(
                    "Section '{}' contains TODO/comment markers",
                    section.title.value
                ),
                section_id: Some(section.id.clone()),
                reference_id: None,
                export_blocker: false,
            });
        }
        append_section_writing_checks(manuscript, &section.children, results);
    }
}

fn append_citation_writing_checks(
    manuscript: &ResearchManuscript,
    results: &mut Vec<WritingCheckResult>,
) {
    let mut cited_references = BTreeSet::new();
    for citation in &manuscript.citation_state.citations {
        let mut citation_seen = BTreeSet::new();
        for reference_id in &citation.reference_ids {
            cited_references.insert(reference_id.clone());
            if !citation_seen.insert(reference_id.clone()) {
                results.push(WritingCheckResult {
                    severity: WritingCheckSeverity::Warning,
                    code: "duplicate_citation_reference".to_string(),
                    message: format!(
                        "Citation {} contains duplicate reference {}",
                        citation.id.as_str(),
                        reference_id.as_str()
                    ),
                    section_id: citation.section_id.clone(),
                    reference_id: Some(reference_id.clone()),
                    export_blocker: false,
                });
            }
        }
    }
    for reference_id in &manuscript.citation_state.bibliography.reference_ids {
        if !cited_references.contains(reference_id) {
            results.push(WritingCheckResult {
                severity: WritingCheckSeverity::Warning,
                code: "uncited_bibliography_reference".to_string(),
                message: format!(
                    "Bibliography contains uncited reference {}",
                    reference_id.as_str()
                ),
                section_id: None,
                reference_id: Some(reference_id.clone()),
                export_blocker: false,
            });
        }
    }
}

fn manuscript_word_count(manuscript: &ResearchManuscript) -> u32 {
    manuscript
        .document
        .to_plain_text()
        .split_whitespace()
        .count()
        .try_into()
        .unwrap_or(u32::MAX)
}

fn section_text(manuscript: &ResearchManuscript, section: &OutlineSection) -> String {
    let mut active = false;
    let mut out = String::new();
    for block in &manuscript.document.content {
        if let BlockNode::Heading { content, .. } = block {
            let heading = inline_nodes_to_text(content);
            if active {
                break;
            }
            active = heading.trim() == section.title.value.trim();
            continue;
        }
        if active {
            out.push_str(&block_to_plain_text(block));
            out.push('\n');
        }
    }
    out
}

fn requires_section_citation(kind: SectionKind) -> bool {
    !matches!(
        kind,
        SectionKind::Title
            | SectionKind::Abstract
            | SectionKind::References
            | SectionKind::Appendix
    )
}

fn abstract_limit_for_section(
    manuscript: &ResearchManuscript,
    section: &OutlineSection,
) -> Option<u32> {
    if section.kind == SectionKind::Abstract {
        manuscript.target.abstract_limit
    } else {
        None
    }
}

fn contains_unresolved_todo(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("todo") || lower.contains("[comment:") || lower.contains("fixme")
}

fn contains_section_kind(sections: &[OutlineSection], kind: SectionKind) -> bool {
    sections
        .iter()
        .any(|section| section.kind == kind || contains_section_kind(&section.children, kind))
}
