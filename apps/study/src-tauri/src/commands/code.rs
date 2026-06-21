#[tauri::command]
pub fn grade_study_code_execution(
    result: tench_study_core::CodeExecutionResult,
) -> tench_study_core::CodeGradingResult {
    tench_study_core::grade_code_execution(&result)
}

#[tauri::command]
pub fn validate_study_code_execution_request(
    request: tench_study_core::CodeExecutionRequest,
) -> tench_study_core::CodeSandboxValidationReport {
    tench_study_core::validate_code_execution_request(&request)
}

#[tauri::command]
pub fn queue_study_code_execution_job(
    request: tench_study_core::CodeExecutionRequest,
) -> Result<tench_study_core::CodeSandboxJob, String> {
    tench_study_core::queue_code_execution_job(request)
}

#[tauri::command]
pub fn start_study_code_execution_job(
    job: tench_study_core::CodeSandboxJob,
) -> tench_study_core::CodeSandboxJob {
    tench_study_core::start_code_execution_job(job)
}

#[tauri::command]
pub fn cancel_study_code_execution_job(
    job: tench_study_core::CodeSandboxJob,
) -> tench_study_core::CodeSandboxJob {
    tench_study_core::cancel_code_execution_job(job)
}

#[tauri::command]
pub fn study_code_execution_result_from_test_outputs(
    request: tench_study_core::CodeExecutionRequest,
    outputs: Vec<tench_study_core::CodeTestOutput>,
    duration_ms: u32,
) -> tench_study_core::CodeExecutionResult {
    tench_study_core::code_execution_result_from_test_outputs(&request, &outputs, duration_ms)
}
