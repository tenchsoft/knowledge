#[tauri::command]
pub fn apply_study_visual_action(
    state: tench_study_core::VisualRuntimeState,
    action: tench_study_core::VisualRuntimeAction,
) -> tench_study_core::VisualRuntimeState {
    state.apply_action(action)
}

#[tauri::command]
pub fn build_study_visual_draw_plan(
    visual: tench_study_core::LearningVisualSpec,
    state: tench_study_core::VisualRuntimeState,
    reduced_motion: bool,
) -> Result<tench_study_core::LearningVisualDrawPlan, String> {
    tench_study_core::build_learning_visual_draw_plan(&visual, &state, reduced_motion)
}

#[tauri::command]
pub fn build_study_practice_feedback_visual_draw_plan(
    visual: tench_study_core::LearningVisualSpec,
    practice: tench_study_core::PracticeItem,
    submission: tench_study_core::AnswerSubmission,
    grading: tench_study_core::GradingResult,
    state: tench_study_core::VisualRuntimeState,
    reduced_motion: bool,
) -> Result<tench_study_core::LearningVisualDrawPlan, String> {
    tench_study_core::build_practice_feedback_visual_draw_plan(
        &visual,
        &practice,
        &submission,
        &grading,
        &state,
        reduced_motion,
    )
}

#[tauri::command]
pub fn build_study_code_trace_visual_draw_plan(
    trace: tench_study_core::AlgorithmTrace,
    grading: Option<tench_study_core::CodeGradingResult>,
    state: tench_study_core::VisualRuntimeState,
    reduced_motion: bool,
) -> Result<tench_study_core::LearningVisualDrawPlan, String> {
    tench_study_core::build_code_trace_visual_draw_plan(
        &trace,
        grading.as_ref(),
        &state,
        reduced_motion,
    )
}
