#[test]
fn study_root_reexports_core_workflows() {
    let catalog = tench_study_core::SubjectCatalog::production_scope();
    let curricula = tench_study_core::builtin_curricula();
    let templates = tench_study_core::study_ai_prompt_templates();

    assert!(!catalog.tracks.is_empty());
    assert!(!curricula.curricula.is_empty());
    assert!(!templates.is_empty());
}
