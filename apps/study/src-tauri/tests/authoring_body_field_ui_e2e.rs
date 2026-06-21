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
fn authoring_body_field_types_and_deletes_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let panel = open_authoring_panel(&mut harness);
    let panel_image = decode_png(&panel);
    assert_selector(&panel, "study.authoring.body");

    // Click on body field to focus it
    let body_focus = click(&mut harness, "study.authoring.body");
    decode_png(&body_focus);

    // Type "Line1", Enter, "Line2"
    let _after_line1 = type_text(&mut harness, "Line1");
    let _after_enter = key(
        &mut harness,
        UiAutomationKey::Enter,
        UiAutomationModifiers::default(),
    );
    let after_line2 = type_text(&mut harness, "Line2");
    let typed_image = decode_png(&after_line2);
    assert!(
        panel_image.as_raw() != typed_image.as_raw(),
        "typing in body field should visibly update the rendered text"
    );

    // Verify the body node reflects the typed text
    let body_node =
        find_node(tree(&after_line2), &selector("study.authoring.body")).expect("body node");
    assert_eq!(
        body_node.label.as_deref(),
        Some("Line1\nLine2"),
        "body node label should reflect multiline typed text"
    );

    // Backspace
    let after_bs = key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    let bs_image = decode_png(&after_bs);
    assert!(
        typed_image.as_raw() != bs_image.as_raw(),
        "backspace should visibly update the body text"
    );
    let body_after_bs = find_node(tree(&after_bs), &selector("study.authoring.body"))
        .expect("body node after backspace");
    assert_eq!(
        body_after_bs.label.as_deref(),
        Some("Line1\nLine"),
        "body should be 'Line1\\nLine' after one backspace"
    );
}

#[test]
fn authoring_body_field_ignores_keys_after_defocus_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let panel = open_authoring_panel(&mut harness);
    decode_png(&panel);

    // Focus body and type
    click(&mut harness, "study.authoring.body");
    type_text(&mut harness, "Content");

    // Defocus with Escape
    key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );

    // Type more characters - should not affect body
    type_text(&mut harness, "NOPE");

    let final_capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    let body_node =
        find_node(tree(&final_capture), &selector("study.authoring.body")).expect("body node");
    assert_eq!(
        body_node.label.as_deref(),
        Some("Content"),
        "body should remain 'Content' after defocus"
    );
}

#[test]
fn authoring_body_field_multiline_backspace_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let panel = open_authoring_panel(&mut harness);
    decode_png(&panel);

    // Focus body, type "AB", Enter, "C"
    click(&mut harness, "study.authoring.body");
    type_text(&mut harness, "AB");
    key(
        &mut harness,
        UiAutomationKey::Enter,
        UiAutomationModifiers::default(),
    );
    type_text(&mut harness, "C");

    // Backspace removes 'C'
    key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    // Backspace removes '\n' (joins lines)
    let after_join = key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    decode_png(&after_join);

    let body_node =
        find_node(tree(&after_join), &selector("study.authoring.body")).expect("body node");
    assert_eq!(
        body_node.label.as_deref(),
        Some("AB"),
        "body should be 'AB' after backspacing newline"
    );
}
