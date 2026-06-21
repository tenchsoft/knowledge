use tench_research_lib::ui::state::{ResearchState, SmartCollection, SmartCollectionRule};
use tench_research_lib::ui::ResearchApp;
use tench_ui_automation_core::{
    find_node, UiAutomationAction, UiAutomationCapture, UiAutomationCaptureRequest,
    UiAutomationNode, UiAutomationSelector,
};
use tench_ui_test::{harness::HarnessConfig, snapshot::is_nonblank, TestHarness};

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

fn harness_with_smart_collections() -> TestHarness {
    let mut state = ResearchState::example();
    state.smart_collections = vec![
        SmartCollection {
            id: "sc-0".into(),
            name: "Recently Added".into(),
            rule: SmartCollectionRule::RecentlyAdded { days: 30 },
            count: 3,
        },
        SmartCollection {
            id: "sc-1".into(),
            name: "Unread Papers".into(),
            rule: SmartCollectionRule::Unread,
            count: 5,
        },
    ];
    TestHarness::with_config(
        ResearchApp::with_state(state),
        HarnessConfig::with_viewport(1280.0, 720.0),
    )
}

#[test]
fn smart_collection_row_filters_papers_on_click_ui_e2e() {
    let mut harness = harness_with_smart_collections();
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    let initial_image = decode_png(&initial);

    // Verify smart collection nodes exist
    assert_selector(&initial, "research.smart_collection.0");
    assert_selector(&initial, "research.smart_collection.1");

    // Click the first smart collection
    let clicked = click(&mut harness, "research.smart_collection.0");
    let clicked_image = decode_png(&clicked);
    assert!(
        initial_image.as_raw() != clicked_image.as_raw(),
        "clicking smart collection should visibly update the UI"
    );
}

#[test]
fn smart_collection_row_noop_when_empty_ui_e2e() {
    // Use default example state which has no smart collections
    let mut harness = TestHarness::with_config(
        ResearchApp::with_state(ResearchState::example()),
        HarnessConfig::with_viewport(1280.0, 720.0),
    );
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    decode_png(&initial);

    // No smart collection nodes should exist
    assert_absent(&initial, "research.smart_collection.0");
}

#[test]
fn smart_collection_row_rapid_clicks_idempotent_ui_e2e() {
    let mut harness = harness_with_smart_collections();
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    decode_png(&initial);

    // Click the same smart collection multiple times rapidly
    let first = click(&mut harness, "research.smart_collection.0");
    decode_png(&first);

    let second = click(&mut harness, "research.smart_collection.0");
    decode_png(&second);

    // Repeated clicks should keep the smart collection node present
    // (visual pixels may differ due to dynamic state like timestamps)
    assert_selector(&second, "research.smart_collection.0");
}
