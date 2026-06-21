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

fn open_authoring_panel(harness: &mut TestHarness) -> UiAutomationCapture {
    key(
        harness,
        UiAutomationKey::Character("a".to_string()),
        UiAutomationModifiers {
            control: true,
            shift: true,
            ..UiAutomationModifiers::default()
        },
    )
}

#[test]
fn new_unit_authoring_button_appends_unit_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let panel = open_authoring_panel(&mut harness);
    decode_png(&panel);
    assert_selector(&panel, "study.authoring.new_unit");

    // Type something in the title first
    click(&mut harness, "study.authoring.title");
    type_text(&mut harness, "First Title");

    // Click New Unit button
    let after_new_unit = click(&mut harness, "study.authoring.new_unit");
    decode_png(&after_new_unit);

    // Title should be reset to empty
    let title_node =
        find_node(tree(&after_new_unit), &selector("study.authoring.title")).expect("title node");
    assert_eq!(
        title_node.label.as_deref(),
        Some(""),
        "title should be reset to empty after new unit"
    );

    // Panel should still be open
    assert_selector(&after_new_unit, "study.authoring.panel");
}

#[test]
fn new_unit_authoring_button_repeated_clicks_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let panel = open_authoring_panel(&mut harness);
    decode_png(&panel);

    // Click New Unit multiple times
    let first = click(&mut harness, "study.authoring.new_unit");
    decode_png(&first);
    assert_selector(&first, "study.authoring.panel");

    // Type something
    click(&mut harness, "study.authoring.title");
    type_text(&mut harness, "X");

    // Click again - should clear again
    let second = click(&mut harness, "study.authoring.new_unit");
    decode_png(&second);
    let title_node =
        find_node(tree(&second), &selector("study.authoring.title")).expect("title node");
    assert_eq!(
        title_node.label.as_deref(),
        Some(""),
        "title should be reset again after second new unit click"
    );

    // Panel should still be open
    assert_selector(&second, "study.authoring.panel");
}
