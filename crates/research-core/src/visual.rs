mod ai;
mod draw;
mod library;
mod manual;
mod manuscript;
mod references;
mod table;
mod types;

pub use ai::{ai_visual_draft_from_engine_spec, build_ai_visual_engine_request};
pub use draw::{build_research_visual_draw_plan, research_visual_table_fallback};
pub use library::{
    aggregate_research_library_visuals, build_citation_warning_visual,
    build_library_overview_visual, build_pdf_annotation_heatmap_visual,
    build_reference_keyword_map_visual,
};
pub use manual::build_manual_paper_analysis_visual;
pub use manuscript::{
    build_manuscript_citation_density_visual, build_manuscript_claim_evidence_visual,
    build_manuscript_readiness_dashboard_visual,
};
pub use references::{build_reference_influence_graph_visual, build_reference_timeline_visual};
pub use types::*;

#[cfg(test)]
mod tests;
