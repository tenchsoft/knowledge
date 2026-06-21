use super::checks::run_non_ai_writing_checks;
use super::document::{find_section_mut, push_unique};
use super::*;
use crate::{
    generate_citekey, render_citation_preview, CitationOutputFormat, CitationRenderMode,
    CitationRenderRequest, CitationStyleId, ReferenceId, ReferenceItem, ResearchLocale, Timestamp,
};
use tench_document_core::{BlockNode, InlineNode, Marks};

pub fn insert_manuscript_citation(
    mut manuscript: ResearchManuscript,
    insertion: ManuscriptCitationInsertion,
    references: &[ReferenceItem],
) -> Result<ResearchManuscript, String> {
    if insertion.reference_ids.is_empty() {
        return Err("citation requires at least one reference".to_string());
    }
    if manuscript
        .citation_state
        .citations
        .iter()
        .any(|citation| citation.id == insertion.id)
    {
        return Err(format!("duplicate citation id {}", insertion.id.as_str()));
    }

    if let Some(section_id) = &insertion.section_id {
        let section = find_section_mut(&mut manuscript.outline.sections, section_id)
            .ok_or_else(|| format!("unknown manuscript section {}", section_id.as_str()))?;
        for reference_id in &insertion.reference_ids {
            push_unique(&mut section.cited_references, reference_id.clone());
        }
        section.status = SectionStatus::NeedsRevision;
    }

    for reference_id in &insertion.reference_ids {
        if let Some(reference) = references
            .iter()
            .find(|reference| &reference.id == reference_id)
        {
            let citekey = reference
                .citekey
                .clone()
                .unwrap_or_else(|| generate_citekey(reference));
            if !manuscript
                .citation_state
                .citekey_map
                .iter()
                .any(|binding| binding.reference_id == *reference_id)
            {
                manuscript.citation_state.citekey_map.push(CitekeyBinding {
                    reference_id: reference_id.clone(),
                    citekey,
                    locked: false,
                });
            }
        }
    }

    let citation = InlineCitation {
        id: insertion.id.clone(),
        reference_ids: insertion.reference_ids.clone(),
        section_id: insertion.section_id.clone(),
        mode: insertion.mode,
    };
    let marker = render_citation_marker(&manuscript, &citation, references);
    manuscript.citation_state.citations.push(citation);
    manuscript.document.content.push(BlockNode::Paragraph {
        content: vec![InlineNode::Text {
            text: marker,
            marks: Marks::default(),
        }],
        attrs: Default::default(),
    });
    manuscript = refresh_manuscript_bibliography(manuscript, references, insertion.now);
    Ok(manuscript)
}

pub fn refresh_manuscript_bibliography(
    mut manuscript: ResearchManuscript,
    references: &[ReferenceItem],
    now: Timestamp,
) -> ResearchManuscript {
    let mut ordered_reference_ids = Vec::new();
    let mut issues = Vec::new();
    for citation in &manuscript.citation_state.citations {
        for reference_id in &citation.reference_ids {
            if !ordered_reference_ids.contains(reference_id) {
                ordered_reference_ids.push(reference_id.clone());
            }
            if !references
                .iter()
                .any(|reference| reference.id == *reference_id)
            {
                issues.push(CitationIssue {
                    code: "missing_reference".to_string(),
                    message: format!("Citation references missing item {}", reference_id.as_str()),
                    reference_id: Some(reference_id.clone()),
                });
            }
        }
    }
    let bibliography_references = ordered_reference_ids
        .iter()
        .filter_map(|reference_id| {
            references
                .iter()
                .find(|reference| reference.id == *reference_id)
                .cloned()
        })
        .collect::<Vec<_>>();
    let citation_request = manuscript_citation_request(
        &manuscript,
        CitationRenderMode::InText,
        true,
        CitationOutputFormat::PlainText,
    );
    let citation_output = render_citation_preview(&citation_request, &bibliography_references);
    for warning in citation_output.warnings {
        issues.push(CitationIssue {
            code: "citation_metadata_warning".to_string(),
            message: warning,
            reference_id: None,
        });
    }
    append_duplicate_citekey_issues(&manuscript, &mut issues);
    manuscript.citation_state.bibliography = BibliographySnapshot {
        rendered: citation_output.bibliography.unwrap_or_default(),
        reference_ids: ordered_reference_ids,
        generated_at: Some(now.clone()),
    };
    manuscript.citation_state.unresolved_citations = issues;
    manuscript.updated_at = now;
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    manuscript
}

fn render_citation_marker(
    manuscript: &ResearchManuscript,
    citation: &InlineCitation,
    references: &[ReferenceItem],
) -> String {
    let citation_references = citation
        .reference_ids
        .iter()
        .filter_map(|reference_id| {
            references
                .iter()
                .find(|reference| reference.id == *reference_id)
                .cloned()
        })
        .collect::<Vec<_>>();
    if citation_references.len() == citation.reference_ids.len() {
        let request = manuscript_citation_request(
            manuscript,
            citation_render_mode(citation.mode),
            false,
            CitationOutputFormat::PlainText,
        );
        let output = render_citation_preview(&request, &citation_references);
        if !output.inline_citations.is_empty() {
            return output
                .inline_citations
                .iter()
                .map(|preview| preview.rendered.clone())
                .collect::<Vec<_>>()
                .join("; ");
        }
    }

    let citekeys = citation
        .reference_ids
        .iter()
        .map(|reference_id| {
            manuscript
                .citation_state
                .citekey_map
                .iter()
                .find(|binding| binding.reference_id == *reference_id)
                .map(|binding| format!("@{}", binding.citekey.as_str()))
                .unwrap_or_else(|| format!("@{}", reference_id.as_str()))
        })
        .collect::<Vec<_>>()
        .join("; ");
    match citation.mode {
        CitationMode::InText => format!("({citekeys})"),
        CitationMode::Footnote => format!("[^{citekeys}]"),
        CitationMode::Endnote => format!("[endnote: {citekeys}]"),
    }
}

fn manuscript_citation_request(
    manuscript: &ResearchManuscript,
    mode: CitationRenderMode,
    include_bibliography: bool,
    output_format: CitationOutputFormat,
) -> CitationRenderRequest {
    CitationRenderRequest {
        style_id: CitationStyleId::from(manuscript.citation_state.style_id.clone()),
        locale: manuscript.citation_state.locale.clone(),
        fallback_locale: ResearchLocale::parse("en-US"),
        mode,
        output_format,
        include_bibliography,
    }
}

fn citation_render_mode(mode: CitationMode) -> CitationRenderMode {
    match mode {
        CitationMode::InText => CitationRenderMode::InText,
        CitationMode::Footnote => CitationRenderMode::Footnote,
        CitationMode::Endnote => CitationRenderMode::Endnote,
    }
}

fn append_duplicate_citekey_issues(
    manuscript: &ResearchManuscript,
    issues: &mut Vec<CitationIssue>,
) {
    let mut seen = std::collections::BTreeMap::<String, ReferenceId>::new();
    for binding in &manuscript.citation_state.citekey_map {
        let citekey = binding.citekey.as_str().to_string();
        if let Some(first_reference_id) = seen.get(&citekey) {
            issues.push(CitationIssue {
                code: "duplicate_citekey".to_string(),
                message: format!(
                    "Duplicate citekey '{}' is used by {} and {}",
                    citekey,
                    first_reference_id.as_str(),
                    binding.reference_id.as_str()
                ),
                reference_id: Some(binding.reference_id.clone()),
            });
        } else {
            seen.insert(citekey, binding.reference_id.clone());
        }
    }
}
