use super::export::inline_nodes_to_text;
use super::*;
use crate::Timestamp;
use tench_document_core::{BlockNode, InlineNode, Marks, TenchDocument};

pub(super) fn mark_section_drafting(
    manuscript: &mut ResearchManuscript,
    section_id: &SectionId,
    now: Timestamp,
) -> Result<String, String> {
    let section = find_section_mut(&mut manuscript.outline.sections, section_id)
        .ok_or_else(|| format!("unknown manuscript section {}", section_id.as_str()))?;
    if section.status == SectionStatus::Planned {
        section.status = SectionStatus::Drafting;
    }
    let title = section.title.value.clone();
    manuscript.updated_at = now;
    Ok(title)
}

pub(super) fn ensure_section_heading(document: &mut TenchDocument, section_title: &str) {
    if !document_contains_heading(document, section_title) {
        document.content.push(BlockNode::Heading {
            level: 2,
            content: vec![InlineNode::Text {
                text: section_title.to_string(),
                marks: Marks::default(),
            }],
            attrs: Default::default(),
        });
    }
}

pub(super) fn paragraph_block(text: impl Into<String>) -> BlockNode {
    BlockNode::Paragraph {
        content: vec![InlineNode::Text {
            text: text.into(),
            marks: Marks::default(),
        }],
        attrs: Default::default(),
    }
}

pub(super) fn find_section_mut<'a>(
    sections: &'a mut [OutlineSection],
    section_id: &SectionId,
) -> Option<&'a mut OutlineSection> {
    for section in sections {
        if section.id == *section_id {
            return Some(section);
        }
        if let Some(child) = find_section_mut(&mut section.children, section_id) {
            return Some(child);
        }
    }
    None
}

pub(super) fn contains_section_id(sections: &[OutlineSection], section_id: &SectionId) -> bool {
    sections.iter().any(|section| {
        section.id == *section_id || contains_section_id(&section.children, section_id)
    })
}

pub(super) fn push_unique<T: Eq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}

pub(super) fn split_paragraphs(text: &str) -> impl Iterator<Item = &str> {
    text.lines().map(str::trim).filter(|line| !line.is_empty())
}

pub(super) fn document_contains_heading(document: &TenchDocument, heading: &str) -> bool {
    document.content.iter().any(|block| match block {
        BlockNode::Heading { content, .. } => inline_nodes_to_text(content).trim() == heading,
        _ => false,
    })
}
