use super::*;
use tench_search_core::{normalize_search_text, SearchDomain, SearchQuery};

pub fn to_shared_search_query(request: &ResearchSearchRequest) -> SearchQuery {
    SearchQuery {
        text: normalize_search_query(&request.query),
        domains: vec![SearchDomain::Documents, SearchDomain::Notes],
        tags: request
            .filters
            .iter()
            .filter(|filter| filter.field == SearchField::Tag)
            .map(|filter| filter.value.clone())
            .collect(),
        limit: request.limit.max(1),
    }
}

pub fn parse_research_search_request(query: &str, limit: u16) -> ResearchSearchRequest {
    let mut text_terms = Vec::new();
    let mut filters = Vec::new();
    let mut sort = SearchSort::default();
    for token in tokenize_query(query) {
        let Some((field, value)) = token.split_once(':') else {
            text_terms.push(token);
            continue;
        };
        if apply_search_control(field, value.trim_matches('"'), &mut sort) {
            continue;
        }
        let Some(field) = parse_search_field(field) else {
            text_terms.push(token);
            continue;
        };
        filters.push(SearchFilter {
            field,
            value: value.trim_matches('"').to_string(),
        });
    }
    ResearchSearchRequest {
        query: normalize_search_query(&text_terms.join(" ")),
        locale: None,
        filters,
        sort,
        limit,
    }
}

pub fn normalize_search_query(query: &str) -> String {
    normalize_search_text(query)
}

fn tokenize_query(query: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escaped = false;

    for ch in query.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quote = !in_quote;
            continue;
        }
        if ch.is_whitespace() && !in_quote {
            if !current.trim().is_empty() {
                tokens.push(current.trim().to_string());
                current.clear();
            }
            continue;
        }
        current.push(ch);
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    tokens
}

fn parse_search_field(value: &str) -> Option<SearchField> {
    match value.to_ascii_lowercase().as_str() {
        "author" => Some(SearchField::Author),
        "tag" => Some(SearchField::Tag),
        "year" => Some(SearchField::Year),
        "venue" => Some(SearchField::Venue),
        "status" => Some(SearchField::Status),
        "collection" => Some(SearchField::Collection),
        "has" | "has_attachment" => Some(SearchField::HasAttachment),
        "citekey" => Some(SearchField::Citekey),
        _ => None,
    }
}

fn apply_search_control(field: &str, value: &str, sort: &mut SearchSort) -> bool {
    match field.to_ascii_lowercase().as_str() {
        "sort" => {
            if let Some(parsed_sort) = parse_sort_value(value) {
                *sort = parsed_sort;
            }
            true
        }
        "order" | "direction" => {
            if let Some(direction) = parse_sort_direction(value) {
                sort.direction = direction;
            }
            true
        }
        _ => false,
    }
}

fn parse_sort_value(value: &str) -> Option<SearchSort> {
    let mut value = value.trim();
    let mut direction = None;
    if let Some(stripped) = value.strip_prefix('-') {
        value = stripped;
        direction = Some(SortDirection::Desc);
    } else if let Some(stripped) = value.strip_prefix('+') {
        value = stripped;
        direction = Some(SortDirection::Asc);
    }
    if let Some((field, suffix)) = value.rsplit_once(':') {
        value = field;
        direction = parse_sort_direction(suffix).or(direction);
    }
    if let Some(stripped) = value.strip_suffix("_asc") {
        value = stripped;
        direction = Some(SortDirection::Asc);
    } else if let Some(stripped) = value.strip_suffix("_desc") {
        value = stripped;
        direction = Some(SortDirection::Desc);
    }
    let field = match value.to_ascii_lowercase().as_str() {
        "relevance" | "score" => SearchSortField::Relevance,
        "added" | "added_date" | "created" | "created_at" => SearchSortField::AddedDate,
        "updated" | "updated_date" | "updated_at" => SearchSortField::UpdatedDate,
        "year" => SearchSortField::Year,
        "title" => SearchSortField::Title,
        _ => return None,
    };
    Some(SearchSort {
        field,
        direction: direction.unwrap_or(match field {
            SearchSortField::Relevance
            | SearchSortField::AddedDate
            | SearchSortField::UpdatedDate => SortDirection::Desc,
            SearchSortField::Year => SortDirection::Desc,
            SearchSortField::Title => SortDirection::Asc,
        }),
    })
}

fn parse_sort_direction(value: &str) -> Option<SortDirection> {
    match value.to_ascii_lowercase().as_str() {
        "asc" | "ascending" => Some(SortDirection::Asc),
        "desc" | "descending" => Some(SortDirection::Desc),
        _ => None,
    }
}
