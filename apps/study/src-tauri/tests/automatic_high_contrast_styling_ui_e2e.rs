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
fn high_contrast_applies_styling_when_toggled_on_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    let main_image = decode_png(&main);

    // Click high contrast toggle
    let hc_on = click(&mut harness, "study.header.high_contrast");
    let hc_on_image = decode_png(&hc_on);
    assert!(
        main_image.as_raw() != hc_on_image.as_raw(),
        "toggling high contrast ON should visibly change the UI palette"
    );
}

#[test]
fn high_contrast_removes_styling_when_toggled_off_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    let _main_image = decode_png(&main);

    // Toggle ON
    let hc_on = click(&mut harness, "study.header.high_contrast");
    let hc_on_image = decode_png(&hc_on);

    // Toggle OFF
    let hc_off = click(&mut harness, "study.header.high_contrast");
    let hc_off_image = decode_png(&hc_off);
    assert!(
        hc_on_image.as_raw() != hc_off_image.as_raw(),
        "toggling high contrast OFF should visibly change the UI palette back"
    );
}

#[test]
fn high_contrast_toggle_roundtrip_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    let main_image = decode_png(&main);

    // Toggle ON then OFF
    let hc_on = click(&mut harness, "study.header.high_contrast");
    decode_png(&hc_on);
    let hc_off = click(&mut harness, "study.header.high_contrast");
    let hc_off_image = decode_png(&hc_off);

    // Toggle ON again
    let hc_on_again = click(&mut harness, "study.header.high_contrast");
    let hc_on_again_image = decode_png(&hc_on_again);

    assert!(
        hc_off_image.as_raw() != hc_on_again_image.as_raw(),
        "toggling HC ON again should produce different visual than OFF state"
    );
    assert!(
        main_image.as_raw() != hc_on_again_image.as_raw(),
        "initial state should differ from HC ON state"
    );
}
