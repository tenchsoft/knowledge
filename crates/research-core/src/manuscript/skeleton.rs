use super::target::{section_label, section_slug};
use super::*;
use crate::{LocalizedField, ResearchLocale, Timestamp};
use tench_document_core::TenchDocument;

pub fn create_manuscript_skeleton(
    id: ManuscriptId,
    library_id: crate::LibraryId,
    title: LocalizedField,
    target: ManuscriptTarget,
    locale: ResearchLocale,
    now: Timestamp,
) -> ResearchManuscript {
    let required_sections = target
        .section_rules
        .iter()
        .filter(|rule| rule.required)
        .map(|rule| RequiredSection {
            kind: rule.kind,
            label: section_label(rule.kind).to_string(),
        })
        .collect::<Vec<_>>();
    let sections = required_sections
        .iter()
        .map(|required| OutlineSection {
            id: SectionId::new(format!("section-{}", section_slug(required.kind))),
            title: LocalizedField::plain(required.label.clone()),
            kind: required.kind,
            status: SectionStatus::Planned,
            source_notes: Vec::new(),
            cited_references: Vec::new(),
            target_words: target
                .section_rules
                .iter()
                .find(|rule| rule.kind == required.kind)
                .and_then(|rule| rule.word_limit),
            children: Vec::new(),
        })
        .collect();
    let document_title = title.value.clone();

    ResearchManuscript {
        id,
        library_id,
        title,
        subtitle: None,
        authors: Vec::new(),
        target: target.clone(),
        locale: locale.clone(),
        template_id: None,
        outline: ManuscriptOutline {
            sections,
            required_sections,
        },
        document: TenchDocument::new(&document_title),
        citation_state: ManuscriptCitationState {
            style_id: target
                .citation_style
                .clone()
                .unwrap_or_else(|| "apa".to_string()),
            locale,
            citations: Vec::new(),
            bibliography: BibliographySnapshot::default(),
            unresolved_citations: Vec::new(),
            citekey_map: Vec::new(),
        },
        assets: Vec::new(),
        cross_references: Vec::new(),
        checks: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    }
}

pub fn create_manuscript_from_template(
    id: ManuscriptId,
    library_id: crate::LibraryId,
    title: LocalizedField,
    template: ManuscriptTemplateKind,
    locale: ResearchLocale,
    now: Timestamp,
) -> ResearchManuscript {
    let target = manuscript_target_for_template(template, locale.clone());
    let mut manuscript =
        create_manuscript_skeleton(id, library_id, title, target, locale, now.clone());
    manuscript.template_id = Some(TemplateId::from(format!("{template:?}").to_lowercase()));
    manuscript
}
