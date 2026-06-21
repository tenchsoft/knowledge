use super::*;
use std::cmp::Ordering;
use tench_search_core::{normalize_search_text, SearchResult};
use unicode_segmentation::UnicodeSegmentation;

use crate::{PdfDocumentText, ResearchSnapshotV2};

pub fn search_research_snapshot(
    snapshot: &ResearchSnapshotV2,
    request: &ResearchSearchRequest,
) -> Vec<SearchResult> {
    search_research_snapshot_with_hits(snapshot, &[], request)
        .into_iter()
        .map(|item| item.result)
        .collect()
}

pub fn search_research_snapshot_with_pdf_text(
    snapshot: &ResearchSnapshotV2,
    pdf_texts: &[PdfDocumentText],
    request: &ResearchSearchRequest,
) -> Vec<SearchResult> {
    search_research_snapshot_with_hits(snapshot, pdf_texts, request)
        .into_iter()
        .map(|item| item.result)
        .collect()
}

pub fn search_research_snapshot_with_hits(
    snapshot: &ResearchSnapshotV2,
    pdf_texts: &[PdfDocumentText],
    request: &ResearchSearchRequest,
) -> Vec<ResearchSearchResult> {
    let documents = build_research_index_documents_with_pdf_text(snapshot, pdf_texts);
    search_research_documents(snapshot, documents, request)
}

pub(super) fn search_research_documents(
    snapshot: &ResearchSnapshotV2,
    documents: Vec<ResearchIndexDocument>,
    request: &ResearchSearchRequest,
) -> Vec<ResearchSearchResult> {
    let query = normalize_for_match(&request.query);
    let limit = request.limit.clamp(1, 500) as usize;
    let mut ranked = documents
        .into_iter()
        .filter(|document| document_matches_filters(snapshot, document, &request.filters))
        .filter_map(|document| {
            let hits = document_hits(&document, &query);
            let haystack = normalize_for_match(&format!("{} {}", document.title, document.body));
            if !query.is_empty() && hits.is_empty() && !haystack.contains(&query) {
                return None;
            }
            let reference = document_reference(snapshot, &document);
            let result = SearchResult {
                id: document.id.clone(),
                domain: document.domain,
                title: document.title.clone(),
                snippet: hits
                    .first()
                    .map(|hit| hit.range.snippet.clone())
                    .unwrap_or_else(|| make_snippet(&document.body, &request.query)),
                score: search_score(&document.title, &document.body, &request.query),
                location: document.location.clone(),
            };
            Some(RankedResearchSearchResult {
                title_sort: normalize_for_match(&document.title),
                year: reference.and_then(|reference| reference.issued.year),
                added_at: reference
                    .map(|reference| reference.created_at.0.clone())
                    .unwrap_or_default(),
                updated_at: reference
                    .map(|reference| reference.updated_at.0.clone())
                    .unwrap_or_default(),
                score: result.score,
                output: ResearchSearchResult { result, hits },
            })
        })
        .collect::<Vec<_>>();
    sort_ranked_results(&mut ranked, &request.sort);
    ranked
        .into_iter()
        .take(limit)
        .map(|item| item.output)
        .collect()
}

fn document_matches_filters(
    snapshot: &ResearchSnapshotV2,
    document: &ResearchIndexDocument,
    filters: &[SearchFilter],
) -> bool {
    let reference = document.reference_id.as_ref().and_then(|reference_id| {
        snapshot
            .references
            .iter()
            .find(|reference| &reference.id == reference_id)
    });
    filters.iter().all(|filter| match filter.field {
        SearchField::Tag => {
            research_filter_values_match(&document.tags, &filter.value)
                || reference.is_some_and(|reference| {
                    let tags = reference
                        .tags
                        .iter()
                        .map(|tag| tag.as_str().to_string())
                        .collect::<Vec<_>>();
                    research_filter_values_match(&tags, &filter.value)
                })
        }
        SearchField::Author => reference.is_some_and(|reference| {
            reference.creators.iter().any(|creator| {
                normalize_for_match(&creator.sort_name())
                    .contains(&normalize_for_match(&filter.value))
            })
        }),
        SearchField::Year => reference.is_some_and(|reference| {
            reference
                .issued
                .year
                .map(|year| year.to_string() == filter.value)
                .unwrap_or(false)
        }),
        SearchField::Venue => reference.is_some_and(|reference| {
            reference
                .venue
                .as_ref()
                .map(|venue| {
                    normalize_for_match(&venue.name.value)
                        .contains(&normalize_for_match(&filter.value))
                })
                .unwrap_or(false)
        }),
        SearchField::Status => reference.is_some_and(|reference| {
            format!("{:?}", reference.status).eq_ignore_ascii_case(&filter.value)
        }),
        SearchField::Collection => reference.is_some_and(|reference| {
            let normalized_value = normalize_filter_value(&filter.value);
            reference.collections.iter().any(|collection| {
                normalize_filter_value(collection.as_str()) == normalized_value
                    || snapshot.collections.iter().any(|candidate| {
                        candidate.id == *collection
                            && normalize_filter_value(&candidate.name) == normalized_value
                    })
            })
        }),
        SearchField::HasAttachment => reference.is_some_and(|reference| {
            let has_attachment = snapshot
                .attachments
                .iter()
                .any(|attachment| attachment.reference_id == reference.id);
            match filter.value.to_ascii_lowercase().as_str() {
                "true" | "yes" | "pdf" | "attachment" => has_attachment,
                "false" | "no" => !has_attachment,
                _ => has_attachment,
            }
        }),
        SearchField::Citekey => reference.is_some_and(|reference| {
            reference
                .citekey
                .as_ref()
                .map(|citekey| citekey.as_str() == filter.value)
                .unwrap_or(false)
        }),
    })
}

fn research_filter_values_match(values: &[String], filter_value: &str) -> bool {
    let normalized_value = normalize_filter_value(filter_value);
    values
        .iter()
        .any(|value| normalize_filter_value(value) == normalized_value)
}

fn normalize_filter_value(value: &str) -> String {
    normalize_for_match(value.trim().trim_start_matches('#'))
}

fn document_hits(
    document: &ResearchIndexDocument,
    normalized_query: &str,
) -> Vec<ResearchSearchHit> {
    if normalized_query.is_empty() {
        return Vec::new();
    }
    let mut hits = Vec::new();
    if let Some(range) = find_text_hit_range(&document.title, normalized_query) {
        hits.push(ResearchSearchHit {
            field: ResearchSearchHitField::Title,
            range,
            location: document.location.clone(),
            reference_id: document.reference_id.clone(),
            attachment_id: document.attachment_id(),
            page: document.page(),
        });
    }
    if let Some(range) = find_text_hit_range(&document.body, normalized_query) {
        hits.push(ResearchSearchHit {
            field: document_body_hit_field(document),
            range,
            location: document.location.clone(),
            reference_id: document.reference_id.clone(),
            attachment_id: document.attachment_id(),
            page: document.page(),
        });
    }
    hits
}

fn document_body_hit_field(document: &ResearchIndexDocument) -> ResearchSearchHitField {
    if document.id.starts_with("pdf:") {
        ResearchSearchHitField::PdfPage
    } else if document.id.starts_with("note:") {
        ResearchSearchHitField::Note
    } else if document.id.starts_with("annotation:") {
        ResearchSearchHitField::Annotation
    } else {
        ResearchSearchHitField::Metadata
    }
}

fn find_text_hit_range(value: &str, normalized_query: &str) -> Option<ResearchTextRange> {
    let (normalized_text, original_byte_map) = normalize_text_with_map(value);
    let byte_index = normalized_text.find(normalized_query)?;
    let normalized_char_start = normalized_text[..byte_index].chars().count();
    let original_byte_start = original_byte_map
        .get(normalized_char_start)
        .copied()
        .unwrap_or(0)
        .min(value.len());
    let start_grapheme = value[..original_byte_start].graphemes(true).count();
    let match_len = normalized_query.chars().count().max(1);
    Some(ResearchTextRange {
        start_grapheme: start_grapheme.try_into().unwrap_or(u32::MAX),
        end_grapheme: start_grapheme
            .saturating_add(match_len)
            .try_into()
            .unwrap_or(u32::MAX),
        snippet: grapheme_snippet(
            value,
            value[..original_byte_start].chars().count(),
            match_len,
        ),
    })
}

fn document_reference<'a>(
    snapshot: &'a ResearchSnapshotV2,
    document: &ResearchIndexDocument,
) -> Option<&'a crate::ReferenceItem> {
    let reference_id = document.reference_id.as_ref()?;
    snapshot
        .references
        .iter()
        .find(|reference| reference.id == *reference_id)
}

struct RankedResearchSearchResult {
    output: ResearchSearchResult,
    score: f32,
    title_sort: String,
    year: Option<u16>,
    added_at: String,
    updated_at: String,
}

fn sort_ranked_results(results: &mut [RankedResearchSearchResult], sort: &SearchSort) {
    results.sort_by(|left, right| {
        let ordering = match sort.field {
            SearchSortField::Relevance => left
                .score
                .partial_cmp(&right.score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.title_sort.cmp(&right.title_sort)),
            SearchSortField::AddedDate => left.added_at.cmp(&right.added_at),
            SearchSortField::UpdatedDate => left.updated_at.cmp(&right.updated_at),
            SearchSortField::Year => left.year.cmp(&right.year),
            SearchSortField::Title => left.title_sort.cmp(&right.title_sort),
        };
        match sort.direction {
            SortDirection::Asc => ordering,
            SortDirection::Desc => ordering.reverse(),
        }
    });
}

fn normalize_for_match(value: &str) -> String {
    normalize_search_text(value)
}

fn make_snippet(body: &str, query: &str) -> String {
    let body = body.trim();
    if body.is_empty() {
        return String::new();
    }
    let normalized_query = normalize_for_match(query);
    if normalized_query.is_empty() {
        return first_graphemes(body, 180);
    }
    let (normalized_body, original_byte_map) = normalize_text_with_map(body);
    let Some(byte_index) = normalized_body.find(&normalized_query) else {
        return first_graphemes(body, 180);
    };
    let normalized_char_start = normalized_body[..byte_index].chars().count();
    let original_byte_start = original_byte_map
        .get(normalized_char_start)
        .copied()
        .unwrap_or(0)
        .min(body.len());
    let original_char_start = body[..original_byte_start].chars().count();
    grapheme_snippet(
        body,
        original_char_start,
        normalized_query.chars().count().max(1),
    )
}

fn search_score(title: &str, body: &str, query: &str) -> f32 {
    let query = normalize_for_match(query);
    if query.is_empty() {
        return 0.5;
    }
    let title = normalize_for_match(title);
    if title == query {
        1.0
    } else if title.contains(&query) {
        0.9
    } else if normalize_for_match(body).contains(&query) {
        0.7
    } else {
        0.0
    }
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

pub(super) fn first_graphemes(value: &str, count: usize) -> String {
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

    let start = match_grapheme.saturating_sub(50);
    let take = match_len_chars.max(180);
    graphemes.into_iter().skip(start).take(take).collect()
}
