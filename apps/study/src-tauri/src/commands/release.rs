#[tauri::command]
pub fn study_release_readiness_report(
    input: tench_study_core::StudyReleaseReadinessInput,
) -> tench_study_core::StudyReleaseReadinessReport {
    tench_study_core::study_release_readiness_report(&input)
}
