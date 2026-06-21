use serde::{Deserialize, Serialize};
use tench_search_core::normalize_search_text;

use crate::{
    ContentLocale, Curriculum, CurriculumNode, CurriculumNodeId, CurriculumNodeKind,
    LocalizedStringSet,
};

crate::study_id_type!(GlossaryTermId);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlossaryTerm {
    pub id: GlossaryTermId,
    pub node_id: CurriculumNodeId,
    pub term: LocalizedStringSet,
    pub definition: LocalizedStringSet,
    #[serde(default)]
    pub aliases: Vec<LocalizedStringSet>,
    #[serde(default)]
    pub related_term_ids: Vec<GlossaryTermId>,
    #[serde(default)]
    pub subject_tags: Vec<String>,
}

impl GlossaryTerm {
    pub fn validate_for_release(&self) -> Result<(), String> {
        if self.term.default.value.trim().is_empty() {
            return Err("glossary term is required".to_string());
        }
        if self.definition.default.value.trim().is_empty() {
            return Err("glossary definition is required".to_string());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GlossarySearchResult {
    pub term_id: GlossaryTermId,
    pub node_id: CurriculumNodeId,
    pub title: String,
    pub snippet: String,
    pub score: u16,
}

pub fn glossary_terms_from_curriculum(curriculum: &Curriculum) -> Vec<GlossaryTerm> {
    curriculum
        .graph
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, CurriculumNodeKind::Lesson))
        .map(|node| glossary_term_from_node(curriculum, node))
        .collect()
}

pub fn glossary_terms_from_all_curricula(curricula: &[Curriculum]) -> Vec<GlossaryTerm> {
    curricula
        .iter()
        .flat_map(glossary_terms_from_curriculum)
        .collect()
}

pub fn glossary_terms_for_node(
    terms: &[GlossaryTerm],
    node_id: &CurriculumNodeId,
) -> Vec<GlossaryTerm> {
    terms
        .iter()
        .filter(|term| &term.node_id == node_id)
        .cloned()
        .collect()
}

pub fn search_glossary_terms(
    terms: &[GlossaryTerm],
    query: &str,
    locale: Option<&ContentLocale>,
    limit: usize,
) -> Vec<GlossarySearchResult> {
    let query = normalize_search_text(query);
    if query.is_empty() || limit == 0 {
        return Vec::new();
    }

    let mut results = terms
        .iter()
        .filter_map(|term| glossary_search_match(term, &query, locale))
        .collect::<Vec<_>>();
    results.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.title.cmp(&right.title))
    });
    results.truncate(limit);
    results
}

fn glossary_term_from_node(curriculum: &Curriculum, node: &CurriculumNode) -> GlossaryTerm {
    GlossaryTerm {
        id: GlossaryTermId::from(format!("glossary-{}", node.id.as_str())),
        node_id: node.id.clone(),
        term: node.title.clone(),
        definition: node.summary.clone(),
        aliases: node
            .objectives
            .iter()
            .map(|objective| LocalizedStringSet {
                default: objective.statement.clone(),
                translations: Vec::new(),
            })
            .collect(),
        related_term_ids: Vec::new(),
        subject_tags: vec![format!("{:?}", curriculum.domain).to_ascii_lowercase()],
    }
}

fn glossary_search_match(
    term: &GlossaryTerm,
    query: &str,
    locale: Option<&ContentLocale>,
) -> Option<GlossarySearchResult> {
    let title = localized_string(&term.term, locale);
    let definition = localized_string(&term.definition, locale);
    let alias_text = term
        .aliases
        .iter()
        .map(|alias| localized_string(alias, locale))
        .collect::<Vec<_>>()
        .join(" ");
    let haystack = normalize_search_text(&format!("{title} {definition} {alias_text}"));
    if !haystack.contains(query) {
        return None;
    }

    let title_score = normalize_search_text(&title).contains(query) as u16 * 80;
    let alias_score = normalize_search_text(&alias_text).contains(query) as u16 * 40;
    Some(GlossarySearchResult {
        term_id: term.id.clone(),
        node_id: term.node_id.clone(),
        title,
        snippet: definition,
        score: 10 + title_score + alias_score,
    })
}

fn localized_string(value: &LocalizedStringSet, locale: Option<&ContentLocale>) -> String {
    locale
        .and_then(|locale| {
            value
                .translations
                .iter()
                .find(|translation| translation.locale.as_ref() == Some(locale))
        })
        .unwrap_or(&value.default)
        .value
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{builtin_curricula, SubjectDomain};

    #[test]
    fn curriculum_glossary_terms_cover_lessons_and_validate() {
        let curriculum = builtin_curricula()
            .by_domain(&SubjectDomain::Mathematics)
            .expect("math curriculum")
            .clone();

        let terms = glossary_terms_from_curriculum(&curriculum);

        assert!(!terms.is_empty());
        assert!(terms.iter().all(|term| term.validate_for_release().is_ok()));
    }

    #[test]
    fn glossary_search_is_locale_and_accent_aware() {
        let locale = ContentLocale::parse("es-ES").expect("locale");
        let term = GlossaryTerm {
            id: GlossaryTermId::from("term-1"),
            node_id: CurriculumNodeId::from("node-1"),
            term: LocalizedStringSet::plain("Function"),
            definition: LocalizedStringSet {
                default: crate::LocalizedText::plain("A mapping from inputs to outputs."),
                translations: vec![crate::LocalizedText::localized(
                    "Relación entre entradas y salidas.",
                    locale.clone(),
                )],
            },
            aliases: vec![LocalizedStringSet::plain("función")],
            related_term_ids: Vec::new(),
            subject_tags: vec!["mathematics".to_string()],
        };

        let results = search_glossary_terms(&[term], "funcion", Some(&locale), 5);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].snippet, "Relación entre entradas y salidas.");
    }
}
