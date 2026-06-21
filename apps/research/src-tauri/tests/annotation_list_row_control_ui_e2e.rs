use tench_research_lib::ui::state::{PdfAnnotationState, PdfAnnotationTool, ResearchState};
use tench_research_lib::ui::ResearchApp;
use tench_ui::prelude::{Color, Rect};
use tench_ui_automation_core::{
    find_node, UiAutomationAction, UiAutomationCapture, UiAutomationKey, UiAutomationModifiers,
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

fn harness_with_annotations() -> TestHarness {
    let mut state = ResearchState::example();
    state.pdf_annotations = vec![
        PdfAnnotationState {
            id: "ann-0".into(),
            page: 1,
            rect: Rect::new(10.0, 20.0, 110.0, 40.0),
            kind: PdfAnnotationTool::Highlight,
            color: Color::rgba8(255, 255, 0, 255),
            text: Some("Important finding".into()),
        },
        PdfAnnotationState {
            id: "ann-1".into(),
            page: 2,
            rect: Rect::new(10.0, 50.0, 110.0, 70.0),
            kind: PdfAnnotationTool::StickyNote,
            color: Color::rgba8(0, 128, 0, 255),
            text: Some("Review this section".into()),
        },
    ];
    TestHarness::with_config(
        ResearchApp::with_state(state),
        HarnessConfig::with_viewport(1280.0, 720.0),
    )
}

fn enter_pdf_mode(harness: &mut TestHarness) -> UiAutomationCapture {
    // First select a paper
    let paper_selected = click(harness, "research.paper.0");
    decode_png(&paper_selected);

    // Switch to PDF mode with Alt+P
    harness
        .automation_action(UiAutomationAction::KeyPress {
            key: UiAutomationKey::Character("p".to_string()),
            modifiers: UiAutomationModifiers {
                alt: true,
                ..UiAutomationModifiers::default()
            },
        })
        .expect("Alt+P for PDF mode")
}

#[test]
fn annotation_list_row_selects_on_click_ui_e2e() {
    let mut harness = harness_with_annotations();
    let pdf_mode = enter_pdf_mode(&mut harness);
    let pdf_image = decode_png(&pdf_mode);
    assert_selector(&pdf_mode, "research.pdf.annotation_list_toggle");

    // Toggle annotation list
    let list_toggled = click(&mut harness, "research.pdf.annotation_list_toggle");
    let list_image = decode_png(&list_toggled);
    assert!(
        pdf_image.as_raw() != list_image.as_raw(),
        "toggling annotation list should visibly open the sidebar"
    );

    // Verify annotation nodes exist
    assert_selector(&list_toggled, "research.pdf.annotation.0");
    assert_selector(&list_toggled, "research.pdf.annotation.1");

    // Click the first annotation
    let clicked = click(&mut harness, "research.pdf.annotation.0");
    let clicked_image = decode_png(&clicked);
    assert!(
        list_image.as_raw() != clicked_image.as_raw(),
        "clicking an annotation should visibly update the selection state"
    );
}

#[test]
fn annotation_list_row_noop_when_no_annotations_ui_e2e() {
    // Use default example state which has no annotations
    let mut harness = TestHarness::with_config(
        ResearchApp::with_state(ResearchState::example()),
        HarnessConfig::with_viewport(1280.0, 720.0),
    );
    let pdf_mode = enter_pdf_mode(&mut harness);
    decode_png(&pdf_mode);

    // Toggle annotation list
    let list_toggled = click(&mut harness, "research.pdf.annotation_list_toggle");
    decode_png(&list_toggled);

    // No annotation row nodes should exist
    assert_absent(&list_toggled, "research.pdf.annotation.0");
}

#[test]
fn annotation_list_row_rapid_clicks_idempotent_ui_e2e() {
    let mut harness = harness_with_annotations();
    let pdf_mode = enter_pdf_mode(&mut harness);
    decode_png(&pdf_mode);

    // Toggle annotation list
    click(&mut harness, "research.pdf.annotation_list_toggle");

    // Click the same annotation multiple times
    let first = click(&mut harness, "research.pdf.annotation.0");
    decode_png(&first);

    // Verify the annotation node is still present and unchanged
    let first_node = find_node(tree(&first), &selector("research.pdf.annotation.0"))
        .expect("annotation node after first click");

    let second = click(&mut harness, "research.pdf.annotation.0");
    decode_png(&second);

    // Repeated clicks should keep the annotation node label unchanged (idempotent state)
    let second_node = find_node(tree(&second), &selector("research.pdf.annotation.0"))
        .expect("annotation node after second click");
    assert_eq!(
        first_node.label, second_node.label,
        "repeated clicks on the same annotation should keep label unchanged"
    );
}
