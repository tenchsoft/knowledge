use serde::{Deserialize, Serialize};
use tench_job_core::{JobDescriptor, JobProgress, JobState};

use crate::{
    learning_visual_table_fallback, study_job_descriptor, CurriculumNodeId,
    LearningVisualDrawCommand, LearningVisualDrawPlan, LocalizedText, PracticeItemId, StudyJobKind,
    VisualAccessibility, VisualFrame, VisualRenderer, VisualRendererEngine, VisualRuntimeState,
};

crate::study_id_type!(ProgrammingConceptId);
crate::study_id_type!(LanguageImplementationId);
crate::study_id_type!(CodeExecutionId);
crate::study_id_type!(TestCaseId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgrammingConcept {
    pub id: ProgrammingConceptId,
    pub node_id: CurriculumNodeId,
    pub language_independent_explanation: LocalizedText,
    #[serde(default)]
    pub implementations: Vec<LanguageImplementation>,
    pub runtime_requirements: RuntimeRequirements,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LanguageImplementation {
    pub id: LanguageImplementationId,
    pub language_id: String,
    pub source_code: String,
    #[serde(default)]
    pub trace: Option<AlgorithmTrace>,
    pub runnable: bool,
    pub sandbox: SandboxPolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeRequirements {
    pub execution_required_for_release: bool,
    pub trace_required_for_release: bool,
    pub mobile_low_power_fallback: RuntimeFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeFallback {
    SimulatedTrace,
    ReadingOnly,
    ServerDelegated,
    NotAvailable,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SandboxPolicy {
    pub local_only: bool,
    pub network_disabled: bool,
    pub cancellable: bool,
    pub timeout_ms: u32,
    pub memory_limit_mb: u32,
    #[serde(default)]
    pub file_access: SandboxFileAccess,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            local_only: true,
            network_disabled: true,
            cancellable: true,
            timeout_ms: 3000,
            memory_limit_mb: 128,
            file_access: SandboxFileAccess::None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxFileAccess {
    #[default]
    None,
    ReadFixtureOnly,
    TempWorkspace,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeExecutionRequest {
    pub id: CodeExecutionId,
    pub practice_item_id: PracticeItemId,
    pub language_id: String,
    pub source_code: String,
    pub stdin: String,
    pub sandbox: SandboxPolicy,
    #[serde(default)]
    pub test_cases: Vec<TestCase>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    pub id: TestCaseId,
    pub name: String,
    pub stdin: String,
    pub expected_stdout: String,
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeExecutionResult {
    pub id: CodeExecutionId,
    pub state: CodeExecutionState,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub duration_ms: u32,
    #[serde(default)]
    pub test_results: Vec<TestCaseResult>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeSandboxJob {
    pub request: CodeExecutionRequest,
    pub descriptor: JobDescriptor,
    pub validation: CodeSandboxValidationReport,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeSandboxValidationReport {
    #[serde(default)]
    pub issues: Vec<CodeSandboxIssue>,
}

impl CodeSandboxValidationReport {
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity == CodeSandboxIssueSeverity::Error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeSandboxIssue {
    pub severity: CodeSandboxIssueSeverity,
    pub code: String,
    pub message: String,
}

impl CodeSandboxIssue {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: CodeSandboxIssueSeverity::Error,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: CodeSandboxIssueSeverity::Warning,
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeSandboxIssueSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeTestOutput {
    pub test_case_id: TestCaseId,
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub exit_code: Option<i32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeExecutionState {
    Queued,
    Running,
    Passed,
    Failed,
    CompileError,
    RuntimeError,
    TimedOut,
    Cancelled,
    SandboxRejected,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestCaseResult {
    pub test_case_id: TestCaseId,
    pub passed: bool,
    pub expected_stdout: String,
    pub actual_stdout: String,
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeGradingResult {
    pub passed: bool,
    pub score: f32,
    pub visible_failures: Vec<TestCaseResult>,
    pub classification: CodeFeedbackClassification,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeFeedbackClassification {
    Correct,
    WrongAnswer,
    CompileError,
    RuntimeError,
    Timeout,
    Cancelled,
    SandboxRejected,
}

pub fn validate_code_execution_request(
    request: &CodeExecutionRequest,
) -> CodeSandboxValidationReport {
    let mut issues = Vec::new();
    if request.language_id.trim().is_empty() {
        issues.push(CodeSandboxIssue::error(
            "language_required",
            "code execution requires a language id",
        ));
    }
    if request.source_code.trim().is_empty() {
        issues.push(CodeSandboxIssue::error(
            "source_required",
            "code execution requires source code",
        ));
    }
    if !request.sandbox.local_only {
        issues.push(CodeSandboxIssue::error(
            "local_only_required",
            "study code execution must run inside a local sandbox boundary",
        ));
    }
    if !request.sandbox.network_disabled {
        issues.push(CodeSandboxIssue::error(
            "network_disabled_required",
            "study code execution must not expose local network services",
        ));
    }
    if !request.sandbox.cancellable {
        issues.push(CodeSandboxIssue::warning(
            "cancellation_recommended",
            "release-ready code execution should be cancellable",
        ));
    }
    if request.sandbox.timeout_ms == 0 || request.sandbox.timeout_ms > 30_000 {
        issues.push(CodeSandboxIssue::error(
            "timeout_out_of_range",
            "sandbox timeout must be between 1ms and 30000ms",
        ));
    }
    if request.sandbox.memory_limit_mb == 0 || request.sandbox.memory_limit_mb > 1024 {
        issues.push(CodeSandboxIssue::error(
            "memory_limit_out_of_range",
            "sandbox memory limit must be between 1MB and 1024MB",
        ));
    }
    if request.test_cases.is_empty() {
        issues.push(CodeSandboxIssue::warning(
            "test_cases_recommended",
            "code execution has no test cases",
        ));
    }
    CodeSandboxValidationReport { issues }
}

pub fn queue_code_execution_job(request: CodeExecutionRequest) -> Result<CodeSandboxJob, String> {
    let validation = validate_code_execution_request(&request);
    if !validation.is_valid() {
        return Err(validation
            .issues
            .iter()
            .filter(|issue| issue.severity == CodeSandboxIssueSeverity::Error)
            .map(|issue| issue.code.as_str())
            .collect::<Vec<_>>()
            .join(", "));
    }
    let mut descriptor = study_job_descriptor(
        format!("code-grade-{}", request.id.as_str()),
        StudyJobKind::CodeGrade,
        JobState::Queued,
        request.practice_item_id.as_str(),
    );
    descriptor.progress = Some(JobProgress {
        current: 0,
        total: Some(request.test_cases.len() as u64),
        message: Some("Queued code grading sandbox job".to_string()),
    });
    Ok(CodeSandboxJob {
        request,
        descriptor,
        validation,
    })
}

pub fn start_code_execution_job(mut job: CodeSandboxJob) -> CodeSandboxJob {
    job.descriptor.state = JobState::Running;
    if let Some(progress) = &mut job.descriptor.progress {
        progress.message = Some("Running inside sandbox".to_string());
    }
    job
}

pub fn cancel_code_execution_job(mut job: CodeSandboxJob) -> CodeSandboxJob {
    if job.request.sandbox.cancellable {
        job.descriptor.state = JobState::Cancelled;
        if let Some(progress) = &mut job.descriptor.progress {
            progress.message = Some("Code grading cancelled".to_string());
        }
    }
    job
}

pub fn code_execution_result_from_test_outputs(
    request: &CodeExecutionRequest,
    outputs: &[CodeTestOutput],
    duration_ms: u32,
) -> CodeExecutionResult {
    if duration_ms > request.sandbox.timeout_ms {
        return CodeExecutionResult {
            id: request.id.clone(),
            state: CodeExecutionState::TimedOut,
            stdout: String::new(),
            stderr: "sandbox timeout exceeded".to_string(),
            exit_code: None,
            duration_ms,
            test_results: Vec::new(),
        };
    }

    let test_results = request
        .test_cases
        .iter()
        .map(|test_case| {
            let output = outputs
                .iter()
                .find(|output| output.test_case_id == test_case.id);
            let actual_stdout = output
                .map(|output| output.stdout.trim().to_string())
                .unwrap_or_default();
            TestCaseResult {
                test_case_id: test_case.id.clone(),
                passed: normalize_output(&actual_stdout)
                    == normalize_output(&test_case.expected_stdout),
                expected_stdout: test_case.expected_stdout.clone(),
                actual_stdout,
                hidden: test_case.hidden,
            }
        })
        .collect::<Vec<_>>();
    let passed = test_results.iter().all(|result| result.passed);
    CodeExecutionResult {
        id: request.id.clone(),
        state: if passed {
            CodeExecutionState::Passed
        } else {
            CodeExecutionState::Failed
        },
        stdout: outputs
            .iter()
            .map(|output| output.stdout.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        stderr: outputs
            .iter()
            .map(|output| output.stderr.as_str())
            .filter(|stderr| !stderr.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n"),
        exit_code: outputs.iter().find_map(|output| output.exit_code),
        duration_ms,
        test_results,
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlgorithmTrace {
    pub source_code: String,
    pub input_state: String,
    #[serde(default)]
    pub steps: Vec<AlgorithmTraceStep>,
    #[serde(default)]
    pub assessment_hooks: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlgorithmTraceStep {
    pub index: u32,
    pub line: Option<u32>,
    pub variables: Vec<(String, String)>,
    pub call_stack: Vec<String>,
    pub heap_or_data_structure: String,
    pub output: String,
}

pub fn build_code_trace_visual_draw_plan(
    trace: &AlgorithmTrace,
    grading: Option<&CodeGradingResult>,
    state: &VisualRuntimeState,
    reduced_motion: bool,
) -> Result<LearningVisualDrawPlan, String> {
    if trace.steps.is_empty() {
        return Err("code trace visual requires at least one trace step".to_string());
    }

    let position = if reduced_motion {
        1.0
    } else {
        state.playback.timeline_position.clamp(0.0, 1.0)
    };
    let active_index = active_trace_step_index(trace, state.selected_id.as_deref(), position);
    let active_step = trace
        .steps
        .iter()
        .find(|step| step.index == active_index)
        .unwrap_or(&trace.steps[0]);
    let active_step_id = trace_step_id(active_step.index);
    let frame_selected_id = state
        .selected_id
        .clone()
        .unwrap_or_else(|| active_step_id.clone());
    let grading_role = grading
        .map(|grading| format!("{:?}", grading.classification).to_ascii_lowercase())
        .unwrap_or_else(|| "ungraded".to_string());

    let mut commands = vec![
        LearningVisualDrawCommand::TimelineCursor { position },
        LearningVisualDrawCommand::TextLabel {
            id: "trace-source".to_string(),
            text: format!(
                "{} source lines, {} trace steps",
                trace.source_code.lines().count(),
                trace.steps.len()
            ),
        },
    ];

    let final_step_index = trace
        .steps
        .last()
        .map(|step| step.index)
        .unwrap_or(0)
        .max(1) as f32;
    for step in &trace.steps {
        let id = trace_step_id(step.index);
        commands.push(LearningVisualDrawCommand::Shape2d {
            id: id.clone(),
            role: format!("trace_step_{grading_role}"),
            progress: step.index as f32 / final_step_index,
            selected: id == frame_selected_id,
        });
    }

    commands.push(LearningVisualDrawCommand::TextLabel {
        id: "trace-active-step".to_string(),
        text: format!("step {}", active_step.index),
    });
    if let Some(line) = active_step.line {
        commands.push(LearningVisualDrawCommand::TextLabel {
            id: "trace-active-line".to_string(),
            text: format!("line {line}"),
        });
    }
    if !active_step.variables.is_empty() {
        commands.push(LearningVisualDrawCommand::TextLabel {
            id: "trace-variables".to_string(),
            text: active_step
                .variables
                .iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join(", "),
        });
    }
    if !active_step.call_stack.is_empty() {
        commands.push(LearningVisualDrawCommand::TextLabel {
            id: "trace-call-stack".to_string(),
            text: active_step.call_stack.join(" -> "),
        });
    }
    if !active_step.heap_or_data_structure.trim().is_empty() {
        commands.push(LearningVisualDrawCommand::Shape2d {
            id: "trace-data-structure".to_string(),
            role: "data_structure_snapshot".to_string(),
            progress: position,
            selected: state.selected_id.as_deref() == Some("trace-data-structure"),
        });
        commands.push(LearningVisualDrawCommand::TextLabel {
            id: "trace-data-structure-label".to_string(),
            text: active_step.heap_or_data_structure.clone(),
        });
    }
    if !active_step.output.trim().is_empty() {
        commands.push(LearningVisualDrawCommand::TextLabel {
            id: "trace-output".to_string(),
            text: active_step.output.clone(),
        });
    }

    if let Some(grading) = grading {
        commands.push(LearningVisualDrawCommand::ParameterControl {
            name: "score".to_string(),
            value: grading.score as f64,
        });
        commands.push(LearningVisualDrawCommand::Shape2d {
            id: "trace-grade-score".to_string(),
            role: "code_grade".to_string(),
            progress: grading.score.clamp(0.0, 1.0),
            selected: grading.passed,
        });
        commands.push(LearningVisualDrawCommand::TextLabel {
            id: "trace-grade-classification".to_string(),
            text: grading_role,
        });
        for failure in &grading.visible_failures {
            commands.push(LearningVisualDrawCommand::TextLabel {
                id: format!("trace-failure-{}", failure.test_case_id.as_str()),
                text: format!(
                    "{} expected {}, actual {}",
                    failure.test_case_id.as_str(),
                    failure.expected_stdout,
                    failure.actual_stdout
                ),
            });
        }
    }

    let table_fallback = learning_visual_table_fallback(&commands);

    Ok(LearningVisualDrawPlan {
        visual_id: state.visual_id.clone(),
        renderer: VisualRenderer {
            engine: VisualRendererEngine::CodeTrace,
            spec_version: 1,
            scene_ref: format!("code-trace://{}", state.visual_id.as_str()),
        },
        frame: VisualFrame {
            timeline_position: position,
            selected_id: Some(frame_selected_id),
            active_layers: state.active_layers.clone(),
        },
        commands,
        accessibility: VisualAccessibility {
            alt_text: format!("Code trace with {} steps", trace.steps.len()),
            transcript: Some(trace_transcript(trace)),
            table_fallback_ref: Some("trace://variables".to_string()),
            keyboard_model: vec![
                "arrow-left/right".to_string(),
                "space".to_string(),
                "enter".to_string(),
            ],
        },
        table_fallback,
        reduced_motion,
    })
}

fn active_trace_step_index(
    trace: &AlgorithmTrace,
    selected_id: Option<&str>,
    position: f32,
) -> u32 {
    if let Some(selected_index) = selected_id.and_then(parse_trace_step_id) {
        if trace.steps.iter().any(|step| step.index == selected_index) {
            return selected_index;
        }
    }

    let last_position = trace.steps.len().saturating_sub(1);
    let positional_index = ((last_position as f32) * position.clamp(0.0, 1.0)).round() as usize;
    trace
        .steps
        .get(positional_index.min(last_position))
        .map(|step| step.index)
        .unwrap_or_default()
}

fn trace_step_id(index: u32) -> String {
    format!("trace-step-{index}")
}

fn parse_trace_step_id(id: &str) -> Option<u32> {
    id.strip_prefix("trace-step-")?.parse().ok()
}

fn trace_transcript(trace: &AlgorithmTrace) -> String {
    trace
        .steps
        .iter()
        .map(|step| {
            let line = step
                .line
                .map(|line| format!("line {line}"))
                .unwrap_or_else(|| "line unknown".to_string());
            let variables = if step.variables.is_empty() {
                "no variables".to_string()
            } else {
                step.variables
                    .iter()
                    .map(|(name, value)| format!("{name}={value}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            format!("step {}: {line}; {variables}", step.index)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn grade_code_execution(result: &CodeExecutionResult) -> CodeGradingResult {
    let classification = match result.state {
        CodeExecutionState::Passed => CodeFeedbackClassification::Correct,
        CodeExecutionState::Failed => CodeFeedbackClassification::WrongAnswer,
        CodeExecutionState::CompileError => CodeFeedbackClassification::CompileError,
        CodeExecutionState::RuntimeError => CodeFeedbackClassification::RuntimeError,
        CodeExecutionState::TimedOut => CodeFeedbackClassification::Timeout,
        CodeExecutionState::Cancelled => CodeFeedbackClassification::Cancelled,
        CodeExecutionState::SandboxRejected => CodeFeedbackClassification::SandboxRejected,
        CodeExecutionState::Queued | CodeExecutionState::Running => {
            CodeFeedbackClassification::WrongAnswer
        }
    };
    let total = result.test_results.len().max(1) as f32;
    let passed = result
        .test_results
        .iter()
        .filter(|test| test.passed)
        .count() as f32;
    let score = if result.state == CodeExecutionState::Passed {
        1.0
    } else {
        passed / total
    };
    CodeGradingResult {
        passed: classification == CodeFeedbackClassification::Correct,
        score,
        visible_failures: result
            .test_results
            .iter()
            .filter(|test| !test.passed && !test.hidden)
            .cloned()
            .collect(),
        classification,
    }
}

fn normalize_output(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests;
