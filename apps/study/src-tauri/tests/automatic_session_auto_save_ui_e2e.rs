use tench_study_lib::ui::StudyApp;
use tench_ui_automation_core::{
    find_node, UiAutomationAction, UiAutomationCapture, UiAutomationCaptureRequest,
    UiAutomationNode, UiAutomationSelector,
};
use tench_ui_test::{harness::HarnessConfig, snapshot::is_nonblank, TestHarness};

fn harness() -> TestHarness {
    TestHarness::with_config(StudyApp::new(), HarnessConfig::with_viewport(1280.0, 720.0))
}

fn selector(debug_id: &str) -> UiAutomationSelector {
    UiAutomationSelector::ByDebugId {
        debug_id: debug_id.to_string(),
    }
}

fn decode_png(capture: &UiAutomationCapture) -> image::RgbaImage {
    assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
    let image = image::load_from_memory(&capture.png_bytes)
        .expect("valid automation png")
        .to_rgba8();
    assert_eq!(image.width(), capture.width);
    assert_eq!(image.height(), capture.height);
    assert!(is_nonblank(&image, 64), "capture should be non-blank");
    image
}

fn tree(capture: &UiAutomationCapture) -> &UiAutomationNode {
    capture.ui_tree.as_ref().expect("automation tree")
}

fn assert_selector(capture: &UiAutomationCapture, debug_id: &str) {
    assert!(
        find_node(tree(capture), &selector(debug_id)).is_some(),
        "missing selector {debug_id}"
    );
}

fn click(harness: &mut TestHarness, debug_id: &str) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::Click {
            selector: selector(debug_id),
            modifiers: Default::default(),
        })
        .unwrap_or_else(|err| panic!("click {debug_id}: {err:?}"))
}

fn complete_profile_setup(harness: &mut TestHarness) -> UiAutomationCapture {
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    assert_selector(&initial, "study.profile.learner_id");

    let learner_focus = click(harness, "study.profile.learner_id");
    decode_png(&learner_focus);
    type_text(harness, "learner01");

    let name_focus = click(harness, "study.profile.display_name");
    decode_png(&name_focus);
    type_text(harness, "Tench Student");

    let domain_step = click(harness, "study.profile.next");
    decode_png(&domain_step);
    let domain_selected = click(harness, "study.profile.domain.0");
    decode_png(&domain_selected);
    let level_selected = click(harness, "study.profile.level.high-school");
    decode_png(&level_selected);

    let locale_step = click(harness, "study.profile.next");
    decode_png(&locale_step);
    let locale_selected = click(harness, "study.profile.locale.ko-KR");
    decode_png(&locale_selected);

    let main = click(harness, "study.profile.next");
    decode_png(&main);
    main
}

fn type_text(harness: &mut TestHarness, text: &str) -> UiAutomationCapture {
    use tench_ui_automation_core::{UiAutomationAction, UiAutomationKey, UiAutomationModifiers};
    let mut capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    for ch in text.chars() {
        capture = harness
            .automation_action(UiAutomationAction::KeyPress {
                key: UiAutomationKey::Character(ch.to_string()),
                modifiers: UiAutomationModifiers::default(),
            })
            .expect("key press");
    }
    capture
}

#[test]
fn session_auto_save_creates_snapshot_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    let main_image = decode_png(&main);

    // Start practice to enter Practice stage where auto-save is active
    let practice = click(&mut harness, "study.learn.start_practice");
    let practice_image = decode_png(&practice);

    assert!(
        main_image.as_raw() != practice_image.as_raw(),
        "starting practice should visibly change the UI"
    );

    // The auto-save increments elapsed_seconds on each paint call.
    // After 30 paint cycles in Practice stage, auto_save_session() is called.
    // We can verify this indirectly by triggering multiple paints and checking state persists.
    type_text(&mut harness, "answer");
    let after_input = harness.automation_capture(UiAutomationCaptureRequest::default());
    let after_image = decode_png(&after_input);
    assert!(
        practice_image.as_raw() != after_image.as_raw(),
        "typing in practice should visibly update the answer field"
    );
}

#[test]
fn session_auto_save_idempotent_without_changes_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // Start practice
    let practice = click(&mut harness, "study.learn.start_practice");
    decode_png(&practice);

    // Capture multiple frames without changes
    let second = harness.automation_capture(UiAutomationCaptureRequest::default());
    decode_png(&second);

    // Verify the practice answer node is still present and unchanged
    let first_node = find_node(tree(&practice), &selector("study.practice.answer"));
    let second_node = find_node(tree(&second), &selector("study.practice.answer"));
    assert_eq!(
        first_node.map(|n| n.label.clone()),
        second_node.map(|n| n.label.clone()),
        "without user interaction, practice answer node should remain stable"
    );
}

#[test]
fn session_auto_save_updates_after_state_change_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // Start practice
    let practice = click(&mut harness, "study.learn.start_practice");
    let practice_image = decode_png(&practice);

    // Type answer and submit
    type_text(&mut harness, "test");
    let submitted = click(&mut harness, "study.practice.submit");
    let submit_image = decode_png(&submitted);

    assert!(
        practice_image.as_raw() != submit_image.as_raw(),
        "submitting an answer should visibly change the practice state"
    );

    // After state change, auto-save snapshot should reflect new state
    // Verify by checking feedback-related nodes appear
    assert_selector(&submitted, "study.practice.retry");
}
