use tench_study_lib::ui::StudyApp;
use tench_ui_automation_core::{
    find_node, UiAutomationAction, UiAutomationCapture, UiAutomationCaptureRequest,
    UiAutomationKey, UiAutomationModifiers, UiAutomationNode, UiAutomationSelector,
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

fn key(
    harness: &mut TestHarness,
    key: UiAutomationKey,
    modifiers: UiAutomationModifiers,
) -> UiAutomationCapture {
    harness
        .automation_action(UiAutomationAction::KeyPress { key, modifiers })
        .expect("key press")
}

fn type_text(harness: &mut TestHarness, text: &str) -> UiAutomationCapture {
    let mut capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    for ch in text.chars() {
        capture = key(
            harness,
            UiAutomationKey::Character(ch.to_string()),
            UiAutomationModifiers::default(),
        );
    }
    capture
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

#[test]
fn learn_review_concept_switches_to_learn_stage_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // Start practice
    let practice = click(&mut harness, "study.learn.start_practice");
    decode_png(&practice);

    // Submit a wrong answer to get feedback
    type_text(&mut harness, "wrong");
    let submitted = click(&mut harness, "study.practice.submit");
    decode_png(&submitted);

    // Now the review_concept button should be available
    assert_selector(&submitted, "study.practice.review_concept");

    // Click Review Concept - should switch to Learn stage
    let after_review = click(&mut harness, "study.practice.review_concept");
    let _review_image = decode_png(&after_review);

    // Should now be in Learn stage (verify by checking for learn-specific nodes)
    assert_selector(&after_review, "study.learn.start_practice");
}

#[test]
fn learn_review_concept_button_disabled_without_feedback_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // Start practice
    let practice = click(&mut harness, "study.learn.start_practice");
    decode_png(&practice);

    // Without submitting an answer, the review_concept button should not be present
    // (it only appears when feedback is Some)
    let review_concept_node =
        find_node(tree(&practice), &selector("study.practice.review_concept"));
    assert!(
        review_concept_node.is_none(),
        "review_concept button should not appear without feedback"
    );
}
