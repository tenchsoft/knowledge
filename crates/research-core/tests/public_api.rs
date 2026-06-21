#[test]
fn research_root_reexports_core_workflows() {
    let snapshot = tench_research_core::example_snapshot();
    let templates = tench_research_core::research_ai_prompt_templates();
    let filter = tench_research_core::PaperSearchFilter {
        query: Some("attention".to_string()),
        collection_id: None,
        tag: None,
        status: None,
        favorites_only: false,
    };

    let papers = tench_research_core::search_papers(&snapshot.papers, &filter);
    assert!(!templates.is_empty());
    assert!(!papers.is_empty());
}
