mod assets;
mod checks;
mod citations;
mod document;
mod editing;
mod export;
mod output;
mod skeleton;
mod target;
mod types;

pub use assets::{
    build_manuscript_asset_numbering, create_manuscript_asset, insert_manuscript_asset_placement,
    insert_manuscript_cross_reference, update_manuscript_asset,
};
pub use checks::run_non_ai_writing_checks;
pub use citations::{insert_manuscript_citation, refresh_manuscript_bibliography};
pub use editing::{
    convert_annotation_to_manuscript_quote, insert_manuscript_equation, insert_manuscript_table,
    insert_manuscript_text, link_note_to_manuscript_section,
};
pub use output::{compare_manuscript_snapshots, create_manuscript_snapshot, export_manuscript};
pub use skeleton::{create_manuscript_from_template, create_manuscript_skeleton};
pub use target::manuscript_target_for_template;
pub use types::*;

#[cfg(test)]
mod tests;
