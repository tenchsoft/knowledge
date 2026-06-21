#[tauri::command]
pub fn research_release_readiness_report(
    input: tench_research_core::ResearchReleaseReadinessInput,
) -> tench_research_core::ResearchReleaseReadinessReport {
    tench_research_core::research_release_readiness_report(&input)
}
