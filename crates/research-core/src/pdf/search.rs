use tench_search_core::normalize_search_text;
use unicode_segmentation::UnicodeSegmentation;

use super::*;

pub fn search_pdf_text(document: &PdfDocumentText, query: &str) -> PdfSearchState {
    search_pdf_text_with_limit(document, query, 200)
}

pub fn search_pdf_text_with_limit(
    document: &PdfDocumentText,
    query: &str,
    limit: usize,
) -> PdfSearchState {
    let normalized_query = normalize_pdf_query(query);
    let mut results = Vec::new();
    if !normalized_query.is_empty() && limit > 0 {
        let query_char_len = normalized_query.chars().count().max(1);
        for page in &document.pages {
            let (normalized_page, original_byte_map) = normalize_pdf_text_with_map(&page.text);
            let mut cursor = 0;
            while let Some(index) = normalized_page[cursor..].find(&normalized_query) {
                let absolute = cursor + index;
                let normalized_char_start = normalized_page[..absolute].chars().count();
                let original_byte_start = original_byte_map
                    .get(normalized_char_start)
                    .copied()
                    .unwrap_or(0);
                let original_char_start = page.text[..original_byte_start.min(page.text.len())]
                    .chars()
                    .count();
                let snippet = pdf_snippet(&page.text, original_char_start, query_char_len);
                results.push(PdfSearchResult {
                    id: PdfSearchResultId::from(format!(
                        "pdf-search-{}-{}",
                        page.page, normalized_char_start
                    )),
                    page: page.page,
                    rects: vec![PageRect {
                        x: (normalized_char_start % 80) as f32,
                        y: (normalized_char_start / 80) as f32,
                        width: query_char_len as f32,
                        height: 1.0,
                    }],
                    snippet,
                });
                if results.len() >= limit {
                    break;
                }
                cursor = absolute + normalized_query.len().max(1);
            }
            if results.len() >= limit {
                break;
            }
        }
    }
    PdfSearchState {
        query: query.to_string(),
        active_result: results.first().map(|result| result.id.clone()),
        results,
    }
}

pub fn copy_pdf_selection(selection: &PdfTextSelection) -> String {
    selection.text.clone()
}

pub fn copy_pdf_selection_text(state: &PdfReaderState) -> Option<String> {
    state.selected_text.as_ref().map(copy_pdf_selection)
}

fn normalize_pdf_query(value: &str) -> String {
    normalize_pdf_text_with_map(value).0
}

fn normalize_pdf_text_with_map(value: &str) -> (String, Vec<usize>) {
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

fn pdf_snippet(text: &str, match_start_char: usize, match_len_chars: usize) -> String {
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
    let take = match_len_chars.max(80);
    graphemes.into_iter().skip(start).take(take).collect()
}
