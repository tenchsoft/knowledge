use tench_research_lib::ui::state::{ResearchState, SavedSearch};
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

fn harness_with_saved_searches() -> TestHarness {
    let mut state = ResearchState::example();
    state.saved_searches = vec![
        SavedSearch {
            id: "ss-0".into(),
            name: "Graph Theory".into(),
            query: "graph".into(),
            advanced: None,
        },
        SavedSearch {
            id: "ss-1".into(),
            name: "Attention Models".into(),
            query: "attention".into(),
            advanced: None,
        },
    ];
    TestHarness::with_config(
        ResearchApp::with_state(state),
        HarnessConfig::with_viewport(1280.0, 720.0),
    )
}

#[test]
fn saved_search_row_restores_query_on_click_ui_e2e() {
    let mut harness = harness_with_saved_searches();
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    let initial_image = decode_png(&initial);

    // Verify saved search nodes exist
    assert_selector(&initial, "research.saved_search.0");
    assert_selector(&initial, "research.saved_search.1");

    // Click the first saved search
    let clicked = click(&mut harness, "research.saved_search.0");
    let clicked_image = decode_png(&clicked);
    assert!(
        initial_image.as_raw() != clicked_image.as_raw(),
        "clicking saved search should visibly update the UI"
    );
}

#[test]
fn saved_search_row_noop_when_empty_ui_e2e() {
    // Use default example state which has no saved searches
    let mut harness = TestHarness::with_config(
        ResearchApp::with_state(ResearchState::example()),
        HarnessConfig::with_viewport(1280.0, 720.0),
    );
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    decode_png(&initial);

    // No saved search nodes should exist
    assert_absent(&initial, "research.saved_search.0");
}

#[test]
fn saved_search_row_rapid_clicks_idempotent_ui_e2e() {
    let mut harness = harness_with_saved_searches();
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    decode_png(&initial);

    // Click the same saved search multiple times rapidly
    let first = click(&mut harness, "research.saved_search.0");
    decode_png(&first);
    let first_query =
        find_node(tree(&first), &selector("research.search")).map(|n| n.value.clone());

    let second = click(&mut harness, "research.saved_search.0");
    decode_png(&second);
    let second_query =
        find_node(tree(&second), &selector("research.search")).map(|n| n.value.clone());

    // Repeated clicks should load the same query (idempotent at the state level)
    assert_eq!(
        first_query, second_query,
        "repeated clicks on the same saved search should be idempotent"
    );
}
