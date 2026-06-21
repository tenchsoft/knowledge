use super::*;

#[test]
fn code_grading_hides_hidden_failures_and_scores_visible_results() {
    let result = CodeExecutionResult {
        id: CodeExecutionId::from("run"),
        state: CodeExecutionState::Failed,
        stdout: String::new(),
        stderr: String::new(),
        exit_code: Some(1),
        duration_ms: 10,
        test_results: vec![
            TestCaseResult {
                test_case_id: TestCaseId::from("visible"),
                passed: false,
                expected_stdout: "4".to_string(),
                actual_stdout: "5".to_string(),
                hidden: false,
            },
            TestCaseResult {
                test_case_id: TestCaseId::from("hidden"),
                passed: false,
                expected_stdout: "8".to_string(),
                actual_stdout: "9".to_string(),
                hidden: true,
            },
        ],
    };

    let grading = grade_code_execution(&result);

    assert_eq!(
        grading.classification,
        CodeFeedbackClassification::WrongAnswer
    );
    assert_eq!(grading.visible_failures.len(), 1);
}

#[test]
fn code_trace_visual_plan_links_trace_steps_and_grading() {
    let trace = AlgorithmTrace {
        source_code: "let mut sum = 0;\nsum += 1;\nprintln!(\"{sum}\");".to_string(),
        input_state: "n=1".to_string(),
        steps: vec![
            AlgorithmTraceStep {
                index: 0,
                line: Some(1),
                variables: vec![("sum".to_string(), "0".to_string())],
                call_stack: vec!["main".to_string()],
                heap_or_data_structure: "stack: sum".to_string(),
                output: String::new(),
            },
            AlgorithmTraceStep {
                index: 1,
                line: Some(2),
                variables: vec![("sum".to_string(), "1".to_string())],
                call_stack: vec!["main".to_string()],
                heap_or_data_structure: "stack: sum=1".to_string(),
                output: "1".to_string(),
            },
        ],
        assessment_hooks: vec!["sum-updated".to_string()],
    };
    let grading = CodeGradingResult {
        passed: false,
        score: 0.5,
        visible_failures: vec![TestCaseResult {
            test_case_id: TestCaseId::from("case-1"),
            passed: false,
            expected_stdout: "2".to_string(),
            actual_stdout: "1".to_string(),
            hidden: false,
        }],
        classification: CodeFeedbackClassification::WrongAnswer,
    };
    let state = crate::VisualRuntimeState {
        visual_id: crate::LearningVisualId::from("trace"),
        selected_id: Some("trace-step-1".to_string()),
        active_layers: Vec::new(),
        parameter_values: Vec::new(),
        playback: crate::VisualPlayback {
            animated: true,
            autoplay: false,
            duration_ms: Some(1000),
            timeline_position: 0.0,
            reduced_motion_fallback: true,
        },
    };

    let plan =
        build_code_trace_visual_draw_plan(&trace, Some(&grading), &state, false).expect("plan");

    assert_eq!(plan.renderer.engine, crate::VisualRendererEngine::CodeTrace);
    assert_eq!(plan.frame.selected_id.as_deref(), Some("trace-step-1"));
    assert!(plan.commands.iter().any(|command| matches!(
        command,
        LearningVisualDrawCommand::Shape2d {
            id,
            selected: true,
            ..
        } if id == "trace-step-1"
    )));
    assert!(plan.commands.iter().any(|command| matches!(
        command,
        LearningVisualDrawCommand::ParameterControl { name, value }
            if name == "score" && (*value - 0.5).abs() < f64::EPSILON
    )));
    assert!(plan.commands.iter().any(|command| matches!(
        command,
        LearningVisualDrawCommand::TextLabel { id, text }
            if id == "trace-failure-case-1" && text.contains("expected 2")
    )));
}

#[test]
fn reduced_motion_code_trace_uses_final_step() {
    let trace = AlgorithmTrace {
        source_code: "print(1)".to_string(),
        input_state: String::new(),
        steps: vec![
            AlgorithmTraceStep {
                index: 0,
                line: Some(1),
                variables: Vec::new(),
                call_stack: Vec::new(),
                heap_or_data_structure: String::new(),
                output: String::new(),
            },
            AlgorithmTraceStep {
                index: 2,
                line: Some(1),
                variables: Vec::new(),
                call_stack: Vec::new(),
                heap_or_data_structure: String::new(),
                output: "1".to_string(),
            },
        ],
        assessment_hooks: Vec::new(),
    };
    let state = crate::VisualRuntimeState {
        visual_id: crate::LearningVisualId::from("trace"),
        selected_id: None,
        active_layers: Vec::new(),
        parameter_values: Vec::new(),
        playback: crate::VisualPlayback {
            animated: true,
            autoplay: false,
            duration_ms: Some(1000),
            timeline_position: 0.0,
            reduced_motion_fallback: true,
        },
    };

    let plan = build_code_trace_visual_draw_plan(&trace, None, &state, true).expect("plan");

    assert_eq!(plan.frame.timeline_position, 1.0);
    assert_eq!(plan.frame.selected_id.as_deref(), Some("trace-step-2"));
}

#[test]
fn sandbox_validation_rejects_network_and_unbounded_limits() {
    let request = CodeExecutionRequest {
        id: CodeExecutionId::from("run"),
        practice_item_id: PracticeItemId::from("item"),
        language_id: "python".to_string(),
        source_code: "print(1)".to_string(),
        stdin: String::new(),
        sandbox: SandboxPolicy {
            local_only: true,
            network_disabled: false,
            cancellable: true,
            timeout_ms: 0,
            memory_limit_mb: 2048,
            file_access: SandboxFileAccess::None,
        },
        test_cases: Vec::new(),
    };

    let report = validate_code_execution_request(&request);

    assert!(!report.is_valid());
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == "network_disabled_required"));
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == "timeout_out_of_range"));
}

#[test]
fn code_execution_job_is_cancellable_and_uses_job_contract() {
    let request = code_request();

    let job = queue_code_execution_job(request).expect("queue");
    let running = start_code_execution_job(job);
    let cancelled = cancel_code_execution_job(running);

    assert_eq!(cancelled.descriptor.kind, StudyJobKind::CodeGrade.as_str());
    assert_eq!(cancelled.descriptor.state, JobState::Cancelled);
}

#[test]
fn test_outputs_are_evaluated_without_exposing_hidden_failures() {
    let request = code_request();
    let result = code_execution_result_from_test_outputs(
        &request,
        &[CodeTestOutput {
            test_case_id: TestCaseId::from("visible"),
            stdout: "4\n".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
        }],
        20,
    );

    let grading = grade_code_execution(&result);

    assert_eq!(result.state, CodeExecutionState::Failed);
    assert_eq!(grading.visible_failures.len(), 0);
    assert!(grading.score > 0.0);
}

fn code_request() -> CodeExecutionRequest {
    CodeExecutionRequest {
        id: CodeExecutionId::from("run"),
        practice_item_id: PracticeItemId::from("item"),
        language_id: "python".to_string(),
        source_code: "print(2 + 2)".to_string(),
        stdin: String::new(),
        sandbox: SandboxPolicy::default(),
        test_cases: vec![
            TestCase {
                id: TestCaseId::from("visible"),
                name: "visible".to_string(),
                stdin: String::new(),
                expected_stdout: "4".to_string(),
                hidden: false,
            },
            TestCase {
                id: TestCaseId::from("hidden"),
                name: "hidden".to_string(),
                stdin: String::new(),
                expected_stdout: "8".to_string(),
                hidden: true,
            },
        ],
    }
}
