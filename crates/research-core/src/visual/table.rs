use super::*;

pub(super) fn table_cell(key: impl Into<String>, value: impl ToString) -> ResearchVisualTableCell {
    ResearchVisualTableCell {
        key: key.into(),
        value: value.to_string(),
    }
}
