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

fn assert_absent(capture: &UiAutomationCapture, debug_id: &str) {
    assert!(
        find_node(tree(capture), &selector(debug_id)).is_none(),
        "unexpected selector {debug_id}"
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
    assert_absent(&main, "study.profile");
    main
}

fn open_authoring_panel(harness: &mut TestHarness) -> UiAutomationCapture {
    // Open authoring panel via keyboard shortcut Ctrl+Shift+A
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
fn authoring_title_field_types_and_deletes_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // Open authoring panel via keyboard shortcut
    let panel = open_authoring_panel(&mut harness);
    let panel_image = decode_png(&panel);
    assert_selector(&panel, "study.authoring.title");
    assert_selector(&panel, "study.authoring.body");
    assert_selector(&panel, "study.authoring.save_draft");

    // Click on title field to focus it
    let title_focus = click(&mut harness, "study.authoring.title");
    decode_png(&title_focus);

    // Type "Hello"
    let after_type = type_text(&mut harness, "Hello");
    let typed_image = decode_png(&after_type);
    assert!(
        panel_image.as_raw() != typed_image.as_raw(),
        "typing in title field should visibly update the rendered text"
    );

    // Verify the title node reflects the typed text
    let title_node =
        find_node(tree(&after_type), &selector("study.authoring.title")).expect("title node");
    assert_eq!(
        title_node.label.as_deref(),
        Some("Hello"),
        "title node label should reflect typed text"
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
        "backspace should visibly update the title text"
    );
    let title_after_bs = find_node(tree(&after_bs), &selector("study.authoring.title"))
        .expect("title node after backspace");
    assert_eq!(
        title_after_bs.label.as_deref(),
        Some("Hell"),
        "title should be 'Hell' after one backspace"
    );

    // Escape to defocus
    let after_escape = key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );
    decode_png(&after_escape);

    // Type more - should NOT change title since defocused
    let after_extra = type_text(&mut harness, "XY");
    let _extra_image = decode_png(&after_extra);
    // The title should still be "Hell" since focus was lost
    let title_final = find_node(tree(&after_extra), &selector("study.authoring.title"))
        .expect("title node after defocus");
    assert_eq!(
        title_final.label.as_deref(),
        Some("Hell"),
        "title should remain 'Hell' after defocus + extra typing"
    );
}

#[test]
fn authoring_title_field_ignores_keys_after_defocus_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let panel = open_authoring_panel(&mut harness);
    decode_png(&panel);

    // Focus title and type
    click(&mut harness, "study.authoring.title");
    type_text(&mut harness, "Test");

    // Defocus with Escape
    key(
        &mut harness,
        UiAutomationKey::Escape,
        UiAutomationModifiers::default(),
    );

    // Type more characters - should not affect title
    type_text(&mut harness, "NOPE");

    let final_capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    let title_node =
        find_node(tree(&final_capture), &selector("study.authoring.title")).expect("title node");
    assert_eq!(
        title_node.label.as_deref(),
        Some("Test"),
        "title should remain 'Test' after defocus"
    );
}

#[test]
fn authoring_title_field_backspace_to_empty_then_type_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    let panel = open_authoring_panel(&mut harness);
    decode_png(&panel);

    // Focus title and type
    click(&mut harness, "study.authoring.title");
    type_text(&mut harness, "AB");

    // Backspace to empty
    key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    let after_first_bs = key(
        &mut harness,
        UiAutomationKey::Backspace,
        UiAutomationModifiers::default(),
    );
    decode_png(&after_first_bs);

    // Verify empty
    let title_empty =
        find_node(tree(&after_first_bs), &selector("study.authoring.title")).expect("title node");
    assert_eq!(
        title_empty.label.as_deref(),
        Some(""),
        "title should be empty after backspacing all characters"
    );

    // Type again from empty
    let after_retype = type_text(&mut harness, "New");
    let retype_image = decode_png(&after_retype);
    let title_retype = find_node(tree(&after_retype), &selector("study.authoring.title"))
        .expect("title node after retype");
    assert_eq!(
        title_retype.label.as_deref(),
        Some("New"),
        "title should be 'New' after retyping from empty"
    );
    assert!(
        retype_image.as_raw() != decode_png(&after_first_bs).as_raw(),
        "retyping should cause visible change"
    );
}
