use serde::{Deserialize, Serialize};
use tench_search_core::{normalize_search_text, SearchDomain, SearchQuery, SearchResult};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Curriculum, CurriculumNode, CurriculumNodeKind, StudyCard, StudyNote};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudySearchRequest {
    pub text: String,
    #[serde(default)]
    pub domains: Vec<StudySearchDomain>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_search_limit")]
    pub limit: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudySearchDomain {
    Curriculum,
    Lessons,
    Problems,
    Notes,
    Cards,
    Glossary,
}

pub fn study_to_shared_search_query(request: &StudySearchRequest) -> SearchQuery {
    let domains = if request.domains.is_empty() {
        vec![SearchDomain::Documents, SearchDomain::Notes]
    } else {
        request
            .domains
            .iter()
            .flat_map(|domain| match domain {
                StudySearchDomain::Curriculum
                | StudySearchDomain::Lessons
                | StudySearchDomain::Problems
                | StudySearchDomain::Glossary => vec![SearchDomain::Documents],
                StudySearchDomain::Notes | StudySearchDomain::Cards => vec![SearchDomain::Notes],
            })
            .fold(Vec::new(), |mut acc, domain| {
                if !acc.contains(&domain) {
                    acc.push(domain);
                }
                acc
            })
    };
    let mut tags = request.tags.clone();
    for domain in &request.domains {
        let tag = format!("study:{}", study_search_domain_tag(*domain));
        if !tags.contains(&tag) {
            tags.push(tag);
        }
    }

    SearchQuery {
        text: normalize_search_text(&request.text),
        domains,
        tags,
        limit: request.limit.clamp(1, 500),
    }
}

pub fn curriculum_search_results(
    curriculum: &Curriculum,
    request: &StudySearchRequest,
) -> Vec<SearchResult> {
    let query = normalize_search_text(&request.text);
    let limit = request.limit.clamp(1, 500) as usize;
    curriculum
        .graph
        .nodes
        .iter()
        .filter(|node| study_node_matches_domains(node, &request.domains))
        .filter(|node| {
            query.is_empty()
                || normalize_search_text(&node.title.default.value).contains(&query)
                || normalize_search_text(&node.summary.default.value).contains(&query)
                || node
                    .strand
                    .as_deref()
                    .unwrap_or_default()
                    .normalized_for_search()
                    .contains(&query)
        })
        .take(limit)
        .map(|node| SearchResult {
            id: node.id.as_str().to_string(),
            domain: SearchDomain::Documents,
            title: node.title.default.value.clone(),
            snippet: node.summary.default.value.clone(),
            score: study_node_score(node, &query),
            location: Some(format!(
                "study://curriculum/{}/{}",
                curriculum.id.as_str(),
                node.id.as_str()
            )),
        })
        .collect()
}

pub fn study_notes_cards_search_results(
    notes: &[StudyNote],
    cards: &[StudyCard],
    request: &StudySearchRequest,
) -> Vec<SearchResult> {
    let query = normalize_search_text(&request.text);
    let limit = request.limit.clamp(1, 500) as usize;
    let include_notes =
        request.domains.is_empty() || request.domains.contains(&StudySearchDomain::Notes);
    let include_cards =
        request.domains.is_empty() || request.domains.contains(&StudySearchDomain::Cards);
    let mut results = Vec::new();

    if include_notes {
        for note in notes {
            if !study_tags_match(&note.tags, &request.tags) {
                continue;
            }
            let body = note.document.to_plain_text();
            let haystack = normalize_search_text(&format!("{} {}", note.title.value, body));
            if query.is_empty() || haystack.contains(&query) {
                results.push(SearchResult {
                    id: format!("note:{}", note.id.as_str()),
                    domain: SearchDomain::Notes,
                    title: note.title.value.clone(),
                    snippet: study_snippet(&body, &request.text),
                    score: study_text_score(&note.title.value, &body, &request.text),
                    location: Some(format!(
                        "study://node/{}/note/{}",
                        note.node_id.as_str(),
                        note.id.as_str()
                    )),
                });
            }
        }
    }

    if include_cards {
        for card in cards {
            if !study_tags_match(&card.tags, &request.tags) {
                continue;
            }
            let body = format!("{} {}", card.front.value, card.back.value);
            let haystack = normalize_search_text(&body);
            if query.is_empty() || haystack.contains(&query) {
                results.push(SearchResult {
                    id: format!("card:{}", card.id.as_str()),
                    domain: SearchDomain::Notes,
                    title: card.front.value.clone(),
                    snippet: study_snippet(&card.back.value, &request.text),
                    score: study_text_score(&card.front.value, &card.back.value, &request.text),
                    location: Some(format!(
                        "study://node/{}/card/{}",
                        card.node_id.as_str(),
                        card.id.as_str()
                    )),
                });
            }
        }
    }

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.id.cmp(&b.id))
    });
    results.truncate(limit);
    results
}

fn study_node_matches_domains(node: &CurriculumNode, domains: &[StudySearchDomain]) -> bool {
    if domains.is_empty() {
        return true;
    }
    domains.iter().any(|domain| match domain {
        StudySearchDomain::Curriculum => matches!(
            node.kind,
            CurriculumNodeKind::Program
                | CurriculumNodeKind::Course
                | CurriculumNodeKind::Unit
                | CurriculumNodeKind::Module
        ),
        StudySearchDomain::Lessons => node.kind == CurriculumNodeKind::Lesson,
        StudySearchDomain::Problems => node.kind == CurriculumNodeKind::PracticeSet,
        StudySearchDomain::Glossary => node.strand.as_deref() == Some("glossary"),
        StudySearchDomain::Notes | StudySearchDomain::Cards => false,
    })
}

fn study_tags_match(item_tags: &[String], required_tags: &[String]) -> bool {
    required_tags.is_empty()
        || required_tags.iter().all(|required| {
            let required = normalize_search_text(required.trim_start_matches('#'));
            item_tags
                .iter()
                .any(|tag| normalize_search_text(tag.trim_start_matches('#')) == required)
        })
}

fn study_node_score(node: &CurriculumNode, query: &str) -> f32 {
    if query.is_empty() {
        return 0.5;
    }
    let title = normalize_search_text(&node.title.default.value);
    if title == query {
        1.0
    } else if title.contains(query) {
        0.85
    } else {
        0.6
    }
}

fn study_text_score(title: &str, body: &str, query: &str) -> f32 {
    let query = normalize_search_text(query);
    if query.is_empty() {
        return 0.5;
    }
    let title = normalize_search_text(title);
    if title == query {
        1.0
    } else if title.contains(&query) {
        0.85
    } else if normalize_search_text(body).contains(&query) {
        0.65
    } else {
        0.0
    }
}

fn study_snippet(body: &str, query: &str) -> String {
    let query = normalize_search_text(query);
    if query.is_empty() {
        return first_graphemes(body, 160);
    }
    let (normalized_body, original_byte_map) = normalize_text_with_map(body);
    let Some(index) = normalized_body.find(&query) else {
        return first_graphemes(body, 160);
    };
    let normalized_char_start = normalized_body[..index].chars().count();
    let original_byte_start = original_byte_map
        .get(normalized_char_start)
        .copied()
        .unwrap_or(0)
        .min(body.len());
    let original_char_start = body[..original_byte_start].chars().count();
    grapheme_snippet(body, original_char_start, query.chars().count().max(1))
}

fn normalize_text_with_map(value: &str) -> (String, Vec<usize>) {
    let mut normalized = String::new();
    let mut original_byte_map = Vec::new();
    let mut pending_space = false;

    for (byte_index, grapheme) in value.grapheme_indices(true) {
        if grapheme.chars().all(char::is_whitespace) {
            pending_space = !normalized.is_empty();
            continue;
        }

        let folded = normalize_search_text(grapheme);
        if folded.is_empty() {
            continue;
        }

        if pending_space {
            normalized.push(' ');
            original_byte_map.push(byte_index);
            pending_space = false;
        }

        for folded_char in folded.chars() {
            normalized.push(folded_char);
            original_byte_map.push(byte_index);
        }
    }

    (normalized, original_byte_map)
}

fn first_graphemes(value: &str, count: usize) -> String {
    value.graphemes(true).take(count).collect()
}

fn grapheme_snippet(text: &str, match_start_char: usize, match_len_chars: usize) -> String {
    let graphemes = text.graphemes(true).collect::<Vec<_>>();
    if graphemes.is_empty() {
        return String::new();
    }

    let mut char_cursor = 0usize;
    let mut match_grapheme = 0usize;
    for (index, grapheme) in graphemes.iter().enumerate() {
        let next = char_cursor + grapheme.chars().count();
        if match_start_char < next {
            match_grapheme = index;
            break;
        }
        char_cursor = next;
        match_grapheme = index
            .saturating_add(1)
            .min(graphemes.len().saturating_sub(1));
    }

    let start = match_grapheme.saturating_sub(40);
    let take = match_len_chars.max(160);
    graphemes.into_iter().skip(start).take(take).collect()
}

trait NormalizeSearchExt {
    fn normalized_for_search(&self) -> String;
}

impl NormalizeSearchExt for str {
    fn normalized_for_search(&self) -> String {
        normalize_search_text(self)
    }
}

fn study_search_domain_tag(domain: StudySearchDomain) -> &'static str {
    match domain {
        StudySearchDomain::Curriculum => "curriculum",
        StudySearchDomain::Lessons => "lessons",
        StudySearchDomain::Problems => "problems",
        StudySearchDomain::Notes => "notes",
        StudySearchDomain::Cards => "cards",
        StudySearchDomain::Glossary => "glossary",
    }
}

fn default_search_limit() -> u16 {
    50
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{builtin_curricula, SubjectDomain};

    #[test]
    fn shared_search_query_maps_study_domains_to_tags() {
        let query = study_to_shared_search_query(&StudySearchRequest {
            text: "fractions".to_string(),
            domains: vec![StudySearchDomain::Lessons, StudySearchDomain::Cards],
            tags: vec!["math".to_string()],
            limit: 1000,
        });

        assert_eq!(query.limit, 500);
        assert!(query.domains.contains(&SearchDomain::Documents));
        assert!(query.domains.contains(&SearchDomain::Notes));
        assert!(query.tags.contains(&"study:lessons".to_string()));
        assert!(query.tags.contains(&"study:cards".to_string()));
    }

    #[test]
    fn curriculum_search_finds_builtin_nodes() {
        let curricula = builtin_curricula();
        let math = curricula
            .curricula
            .iter()
            .find(|curriculum| curriculum.domain == SubjectDomain::Mathematics)
            .expect("math curriculum");
        let results = curriculum_search_results(
            math,
            &StudySearchRequest {
                text: "number".to_string(),
                domains: vec![StudySearchDomain::Lessons],
                tags: Vec::new(),
                limit: 5,
            },
        );

        assert!(!results.is_empty());
        assert!(results.iter().all(|result| result
            .location
            .as_deref()
            .unwrap_or("")
            .starts_with("study://")));
    }

    #[test]
    fn notes_and_cards_search_returns_locations() {
        let note = crate::create_study_note(
            crate::StudyNoteId::from("note_1"),
            crate::LearnerId::from("learner"),
            crate::CurriculumNodeId::from("node"),
            crate::LocalizedText::plain("Heart note"),
            "The heart has four chambers.",
            "2026-05-04T00:00:00Z",
        )
        .expect("note");
        let card = crate::create_card_from_note(
            crate::StudyCardId::from("card_1"),
            crate::StudyDeckId::from("deck"),
            &note,
            crate::StudyCardKind::Basic,
            crate::LocalizedText::plain("How many heart chambers?"),
            crate::LocalizedText::plain("Four"),
            "2026-05-04T00:01:00Z",
        )
        .expect("card");

        let results = study_notes_cards_search_results(
            &[note],
            &[card],
            &StudySearchRequest {
                text: "heart".to_string(),
                domains: vec![StudySearchDomain::Notes, StudySearchDomain::Cards],
                tags: Vec::new(),
                limit: 10,
            },
        );

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|result| result
            .location
            .as_deref()
            .unwrap_or("")
            .starts_with("study://node/")));
    }

    #[test]
    fn notes_and_cards_search_matches_accent_folded_text() {
        let note = crate::create_study_note(
            crate::StudyNoteId::from("note_1"),
            crate::LearnerId::from("learner"),
            crate::CurriculumNodeId::from("node"),
            crate::LocalizedText::plain("Café vocabulary"),
            "Practice pronunciation.",
            "2026-05-04T00:00:00Z",
        )
        .expect("note");

        let results = study_notes_cards_search_results(
            &[note],
            &[],
            &StudySearchRequest {
                text: "cafe".to_string(),
                domains: vec![StudySearchDomain::Notes],
                tags: Vec::new(),
                limit: 10,
            },
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "note:note_1");
    }

    #[test]
    fn notes_and_cards_search_applies_tag_filters() {
        let mut note = crate::create_study_note(
            crate::StudyNoteId::from("note_1"),
            crate::LearnerId::from("learner"),
            crate::CurriculumNodeId::from("node"),
            crate::LocalizedText::plain("Heart note"),
            "The heart has four chambers.",
            "2026-05-04T00:00:00Z",
        )
        .expect("note");
        note.tags = vec!["biology".to_string(), "diagram".to_string()];
        let mut card = crate::create_card_from_note(
            crate::StudyCardId::from("card_1"),
            crate::StudyDeckId::from("deck"),
            &note,
            crate::StudyCardKind::Basic,
            crate::LocalizedText::plain("Heart chamber count"),
            crate::LocalizedText::plain("Four"),
            "2026-05-04T00:01:00Z",
        )
        .expect("card");
        card.tags = vec!["review".to_string()];

        let biology_results = study_notes_cards_search_results(
            std::slice::from_ref(&note),
            std::slice::from_ref(&card),
            &StudySearchRequest {
                text: "heart".to_string(),
                domains: vec![StudySearchDomain::Notes, StudySearchDomain::Cards],
                tags: vec!["#biology".to_string()],
                limit: 10,
            },
        );
        let missing_results = study_notes_cards_search_results(
            &[note],
            &[card],
            &StudySearchRequest {
                text: "heart".to_string(),
                domains: vec![StudySearchDomain::Notes, StudySearchDomain::Cards],
                tags: vec!["chemistry".to_string()],
                limit: 10,
            },
        );

        assert_eq!(biology_results.len(), 1);
        assert_eq!(biology_results[0].id, "note:note_1");
        assert!(missing_results.is_empty());
    }

    #[test]
    fn notes_search_snippet_uses_normalized_grapheme_safe_offsets() {
        let note = crate::create_study_note(
            crate::StudyNoteId::from("note_1"),
            crate::LearnerId::from("learner"),
            crate::CurriculumNodeId::from("node"),
            crate::LocalizedText::plain("Lesson note"),
            format!(
                "{} Cafe\u{301} retrieval practice belongs near the result.",
                "background ".repeat(220)
            ),
            "2026-05-04T00:00:00Z",
        )
        .expect("note");

        let results = study_notes_cards_search_results(
            &[note],
            &[],
            &StudySearchRequest {
                text: "cafe retrieval".to_string(),
                domains: vec![StudySearchDomain::Notes],
                tags: Vec::new(),
                limit: 10,
            },
        );

        assert_eq!(results.len(), 1);
        assert!(results[0].snippet.contains("Cafe\u{301} retrieval"));
        assert!(!results[0].snippet.starts_with("background background"));
        assert!(results[0]
            .snippet
            .graphemes(true)
            .all(|grapheme| !grapheme.ends_with('\u{301}') || grapheme.chars().count() > 1));
    }
}
