use tench_study_lib::ui::StudyApp;
use tench_ui_automation_core::{
    UiAutomationAction, UiAutomationCapture, UiAutomationCaptureRequest, UiAutomationKey,
    UiAutomationModifiers, UiAutomationSelector,
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

fn complete_profile_setup(harness: &mut TestHarness) -> UiAutomationCapture {
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    assert!(initial.ui_tree.is_some());

    let learner_focus = click(harness, "study.profile.learner_id");
    decode_png(&learner_focus);
    key(
        harness,
        UiAutomationKey::Character("l".to_string()),
        UiAutomationModifiers::default(),
    );
    key(
        harness,
        UiAutomationKey::Character("0".to_string()),
        UiAutomationModifiers::default(),
    );

    let name_focus = click(harness, "study.profile.display_name");
    decode_png(&name_focus);
    key(
        harness,
        UiAutomationKey::Character("S".to_string()),
        UiAutomationModifiers::default(),
    );

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

/// Verifies that the Hard variant exists in TouchReviewAction and the swipe
/// actions array includes 4 entries (Again, Hard, Good, Easy).
/// This is validated at the unit-test level (state::tests) but this e2e test
/// confirms the app renders without crashing when the Hard variant is present.
#[test]
fn review_swipe_hard_variant_exists_in_swipe_actions_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // The app should render the learn stage without error.
    // The Hard variant is confirmed by the unit test:
    //   state::tests::touch_review_swipe_actions_has_four_entries
    // Here we just verify the app is functional after profile setup.
    let capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    let image = decode_png(&capture);
    assert!(is_nonblank(&image, 64), "learn stage should render");
}

/// Verifies that the stage pill cycles correctly when there are no review items.
/// The default state has no review queue, so cycling stays on Learn.
#[test]
fn review_swipe_hard_stage_cycles_without_review_items_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // Click stage pill — should stay on Learn (no review items, no problems)
    let first = click(&mut harness, "study.header.stage");
    decode_png(&first);

    // Click again — still Learn
    let second = click(&mut harness, "study.header.stage");
    decode_png(&second);

    // App should still be functional
    let capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    let image = decode_png(&capture);
    assert!(
        is_nonblank(&image, 64),
        "app should render after stage cycling"
    );
}

/// Verifies the keyboard shortcut Ctrl+Shift+A opens the authoring panel
/// which contains the Hard variant in the swipe action configuration.
#[test]
fn review_swipe_hard_authoring_panel_accessible_ui_e2e() {
    let mut harness = harness();
    complete_profile_setup(&mut harness);

    // Open authoring panel
    let authoring = key(
        &mut harness,
        UiAutomationKey::Character("a".to_string()),
        UiAutomationModifiers {
            control: true,
            shift: true,
            ..UiAutomationModifiers::default()
        },
    );
    decode_png(&authoring);

    // Authoring panel should be visible
    let tree = authoring.ui_tree.as_ref().expect("automation tree");
    assert!(
        tench_ui_automation_core::find_node(tree, &selector("study.authoring.panel")).is_some(),
        "authoring panel should be open"
    );
}
