use super::checks::run_non_ai_writing_checks;
use super::document::{
    document_contains_heading, ensure_section_heading, find_section_mut, mark_section_drafting,
    paragraph_block, push_unique, split_paragraphs,
};
use super::*;
use tench_document_core::{BlockNode, InlineNode, Marks, TableCell, TableRow};

pub fn insert_manuscript_text(
    mut manuscript: ResearchManuscript,
    insertion: ManuscriptTextInsertion,
) -> Result<ResearchManuscript, String> {
    let section = find_section_mut(&mut manuscript.outline.sections, &insertion.section_id)
        .ok_or_else(|| {
            format!(
                "unknown manuscript section {}",
                insertion.section_id.as_str()
            )
        })?;
    let text = insertion.text.trim();
    if text.is_empty() {
        return Err("manuscript text insertion is empty".to_string());
    }

    for reference_id in &insertion.cited_references {
        push_unique(&mut section.cited_references, reference_id.clone());
    }
    section.status = if section.cited_references.is_empty() {
        SectionStatus::Drafting
    } else {
        SectionStatus::NeedsRevision
    };
    let section_title = section.title.value.clone();

    if !document_contains_heading(&manuscript.document, &section_title) {
        manuscript.document.content.push(BlockNode::Heading {
            level: 2,
            content: vec![InlineNode::Text {
                text: section_title,
                marks: Marks::default(),
            }],
            attrs: Default::default(),
        });
    }
    for paragraph in split_paragraphs(text) {
        manuscript.document.content.push(BlockNode::Paragraph {
            content: vec![InlineNode::Text {
                text: paragraph.to_string(),
                marks: Marks::default(),
            }],
            attrs: Default::default(),
        });
    }

    manuscript.document.metadata.updated_at = Some(insertion.now.0.clone());
    manuscript.updated_at = insertion.now;
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

pub fn link_note_to_manuscript_section(
    mut manuscript: ResearchManuscript,
    link: ManuscriptSectionNoteLink,
) -> Result<ResearchManuscript, String> {
    let section = find_section_mut(&mut manuscript.outline.sections, &link.section_id)
        .ok_or_else(|| format!("unknown manuscript section {}", link.section_id.as_str()))?;
    push_unique(&mut section.source_notes, link.note_id);
    if section.status == SectionStatus::Planned {
        section.status = SectionStatus::Drafting;
    }
    manuscript.updated_at = link.now;
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

pub fn convert_annotation_to_manuscript_quote(
    mut manuscript: ResearchManuscript,
    insertion: ManuscriptAnnotationQuoteInsertion,
) -> Result<ResearchManuscript, String> {
    let quote = insertion
        .annotation
        .selected_text
        .as_deref()
        .or(insertion.annotation.note_markdown.as_deref())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| {
            format!(
                "annotation {} does not contain quote text",
                insertion.annotation.id.as_str()
            )
        })?
        .to_string();
    let section_title = {
        let section = find_section_mut(&mut manuscript.outline.sections, &insertion.section_id)
            .ok_or_else(|| {
                format!(
                    "unknown manuscript section {}",
                    insertion.section_id.as_str()
                )
            })?;
        if let Some(note_id) = insertion.note_id {
            push_unique(&mut section.source_notes, note_id);
        }
        push_unique(
            &mut section.cited_references,
            insertion.annotation.reference_id.clone(),
        );
        section.status = SectionStatus::NeedsRevision;
        section.title.value.clone()
    };

    ensure_section_heading(&mut manuscript.document, &section_title);
    manuscript.document.content.push(BlockNode::BlockQuote {
        content: vec![paragraph_block(format!(
            "{} (p. {})",
            quote, insertion.annotation.page
        ))],
    });
    manuscript.updated_at = insertion.now.clone();
    manuscript.document.metadata.updated_at = Some(insertion.now.0);
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

pub fn insert_manuscript_table(
    mut manuscript: ResearchManuscript,
    insertion: ManuscriptTableInsertion,
) -> Result<ResearchManuscript, String> {
    if insertion.rows.is_empty() || insertion.rows.iter().all(Vec::is_empty) {
        return Err("table insertion requires at least one cell".to_string());
    }
    let section_title = mark_section_drafting(
        &mut manuscript,
        &insertion.section_id,
        insertion.now.clone(),
    )?;
    ensure_section_heading(&mut manuscript.document, &section_title);
    if let Some(caption) = insertion.caption {
        if !caption.value.trim().is_empty() {
            manuscript
                .document
                .content
                .push(paragraph_block(format!("Table: {}", caption.value)));
        }
    }
    manuscript.document.content.push(BlockNode::Table {
        rows: insertion
            .rows
            .into_iter()
            .map(|row| TableRow {
                cells: row
                    .into_iter()
                    .map(|cell| TableCell {
                        content: vec![paragraph_block(cell)],
                        ..Default::default()
                    })
                    .collect(),
            })
            .collect(),
    });
    manuscript.document.metadata.updated_at = Some(insertion.now.0);
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}

pub fn insert_manuscript_equation(
    mut manuscript: ResearchManuscript,
    insertion: ManuscriptEquationInsertion,
) -> Result<ResearchManuscript, String> {
    let latex = insertion.latex.trim();
    if latex.is_empty() {
        return Err("equation insertion is empty".to_string());
    }
    let section_title = mark_section_drafting(
        &mut manuscript,
        &insertion.section_id,
        insertion.now.clone(),
    )?;
    ensure_section_heading(&mut manuscript.document, &section_title);
    if let Some(label) = insertion
        .label
        .as_deref()
        .map(str::trim)
        .filter(|label| !label.is_empty())
    {
        manuscript
            .document
            .content
            .push(paragraph_block(format!("Equation: {label}")));
    }
    manuscript.document.content.push(BlockNode::CodeBlock {
        language: Some("latex-math".to_string()),
        code: latex.to_string(),
    });
    manuscript.document.metadata.updated_at = Some(insertion.now.0);
    manuscript.checks = run_non_ai_writing_checks(&manuscript);
    Ok(manuscript)
}
