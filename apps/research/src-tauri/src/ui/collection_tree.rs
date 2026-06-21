use super::state::ResearchState;

pub fn paper_list_start_y(
    state: &ResearchState,
    header_h: f64,
    _spacing: f64,
    spacing_large: f64,
) -> f64 {
    header_h
        + spacing_large
        + 20.0
        + state.collections.len() as f64 * 20.0
        + spacing_large
        + 18.0
        + state.smart_collections.len() as f64 * 20.0
        + spacing_large
        + 20.0
        + state.tags.len() as f64 * 24.0
        + spacing_large
        + 18.0
        + state.saved_searches.len() as f64 * 20.0
        + spacing_large
        + 20.0
        + state.statuses.len() as f64 * 20.0
        + spacing_large
        + 20.0
}

/// Returns the Y coordinate where the collections section starts painting rows.
pub fn collection_rows_start_y(header_h: f64, spacing_large: f64) -> f64 {
    header_h + spacing_large + 20.0
}

/// Returns the index of the collection row at the given y position, if any.
pub fn hit_test_collection_row(
    state: &ResearchState,
    y: f64,
    header_h: f64,
    spacing_large: f64,
) -> Option<usize> {
    let rows_y = collection_rows_start_y(header_h, spacing_large);
    if y < rows_y {
        return None;
    }
    let row = ((y - rows_y) / 20.0) as usize;
    if row < state.collections.len() {
        Some(row)
    } else {
        None
    }
}

/// Returns the Y coordinate where the smart collections section starts painting rows.
pub fn smart_collection_rows_start_y(
    state: &ResearchState,
    header_h: f64,
    spacing_large: f64,
) -> f64 {
    collection_rows_start_y(header_h, spacing_large)
        + state.collections.len() as f64 * 20.0
        + spacing_large
        + 18.0
}

/// Returns the index of the smart collection row at the given y position, if any.
pub fn hit_test_smart_collection_row(
    state: &ResearchState,
    y: f64,
    header_h: f64,
    spacing_large: f64,
) -> Option<usize> {
    let smart_y = smart_collection_rows_start_y(state, header_h, spacing_large);
    if y < smart_y {
        return None;
    }
    let row = ((y - smart_y) / 20.0) as usize;
    if row < state.smart_collections.len() {
        Some(row)
    } else {
        None
    }
}

/// Returns the Y coordinate where the tags section starts painting rows.
pub fn tag_rows_start_y(state: &ResearchState, header_h: f64, spacing_large: f64) -> f64 {
    smart_collection_rows_start_y(state, header_h, spacing_large)
        + state.smart_collections.len() as f64 * 20.0
        + spacing_large
        + 20.0
}

/// Returns the index of the tag row at the given y position, if any.
pub fn hit_test_tag_row(
    state: &ResearchState,
    y: f64,
    header_h: f64,
    spacing_large: f64,
) -> Option<usize> {
    let rows_y = tag_rows_start_y(state, header_h, spacing_large);
    if y < rows_y {
        return None;
    }
    let row = ((y - rows_y) / 24.0) as usize;
    if row < state.tags.len() {
        Some(row)
    } else {
        None
    }
}

/// Returns the Y coordinate where the saved searches section starts painting rows.
pub fn saved_search_rows_start_y(state: &ResearchState, header_h: f64, spacing_large: f64) -> f64 {
    tag_rows_start_y(state, header_h, spacing_large)
        + state.tags.len() as f64 * 24.0
        + spacing_large
        + 18.0
}

/// Returns the index of the saved search row at the given y position, if any.
pub fn hit_test_saved_search_row(
    state: &ResearchState,
    y: f64,
    header_h: f64,
    spacing_large: f64,
) -> Option<usize> {
    let rows_y = saved_search_rows_start_y(state, header_h, spacing_large);
    if y < rows_y {
        return None;
    }
    let row = ((y - rows_y) / 20.0) as usize;
    if row < state.saved_searches.len() {
        Some(row)
    } else {
        None
    }
}

/// Returns the Y coordinate where the status filters section starts painting rows.
pub fn status_rows_start_y(state: &ResearchState, header_h: f64, spacing_large: f64) -> f64 {
    saved_search_rows_start_y(state, header_h, spacing_large)
        + state.saved_searches.len() as f64 * 20.0
        + spacing_large
        + 20.0
}

/// Returns the index of the status filter row at the given y position, if any.
pub fn hit_test_status_row(
    state: &ResearchState,
    y: f64,
    header_h: f64,
    spacing_large: f64,
) -> Option<usize> {
    let rows_y = status_rows_start_y(state, header_h, spacing_large);
    if y < rows_y {
        return None;
    }
    let row = ((y - rows_y) / 20.0) as usize;
    if row < state.statuses.len() {
        Some(row)
    } else {
        None
    }
}
