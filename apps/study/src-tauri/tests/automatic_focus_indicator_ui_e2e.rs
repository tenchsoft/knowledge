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
fn focus_indicator_appears_on_focused_element_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    let main_image = decode_png(&main);

    // Click on search box to focus it
    let search_focus = click(&mut harness, "study.curriculum.search");
    let focused_image = decode_png(&search_focus);
    assert!(
        main_image.as_raw() != focused_image.as_raw(),
        "focusing the search box should visibly update the UI (focus indicator)"
    );
}

#[test]
fn focus_indicator_absent_when_no_focus_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    decode_png(&main);

    // Focus search box
    let search_focus = click(&mut harness, "study.curriculum.search");
    let focused_image = decode_png(&search_focus);

    // Press Escape to defocus
    use tench_ui_automation_core::{UiAutomationAction, UiAutomationKey, UiAutomationModifiers};
    let defocused = harness
        .automation_action(UiAutomationAction::KeyPress {
            key: UiAutomationKey::Escape,
            modifiers: UiAutomationModifiers::default(),
        })
        .expect("escape key");
    let defocused_image = decode_png(&defocused);
    assert!(
        focused_image.as_raw() != defocused_image.as_raw(),
        "pressing Escape should visibly remove the focus indicator"
    );
}

#[test]
fn focus_indicator_moves_on_focus_change_ui_e2e() {
    let mut harness = harness();
    let main = complete_profile_setup(&mut harness);
    let main_image = decode_png(&main);

    // Focus search box
    let search_focus = click(&mut harness, "study.curriculum.search");
    let search_image = decode_png(&search_focus);

    // Focus notes input (different element)
    let notes_click = click(&mut harness, "study.curriculum.notes");
    decode_png(&notes_click);
    let notes_input_click = click(&mut harness, "study.notes.input");
    let notes_image = decode_png(&notes_input_click);

    assert!(
        search_image.as_raw() != notes_image.as_raw(),
        "focus indicator should move to different element"
    );
    assert!(
        main_image.as_raw() != notes_image.as_raw(),
        "focus indicator should be visible on notes input"
    );
}
