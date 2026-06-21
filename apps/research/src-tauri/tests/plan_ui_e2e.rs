use tench_research_lib::ui::{state::ResearchState, ResearchApp};
use tench_ui_automation_core::{
    find_node, UiAutomationAction, UiAutomationCapture, UiAutomationCaptureRequest,
    UiAutomationKey, UiAutomationModifiers, UiAutomationNode, UiAutomationRect,
    UiAutomationSelector,
};
use tench_ui_test::{harness::HarnessConfig, snapshot::is_nonblank, TestHarness};

fn harness() -> TestHarness {
    TestHarness::with_config(
        ResearchApp::with_state(ResearchState::example()),
        HarnessConfig::with_viewport(1280.0, 720.0),
    )
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
    assert!(
        is_nonblank(&image, 64),
        "capture should be visually non-blank"
    );
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

fn assert_bounds_inside(capture: &UiAutomationCapture, debug_id: &str) {
    let node = find_node(tree(capture), &selector(debug_id)).expect("selector node");
    assert_rect_inside(
        &node.bounds,
        capture.width as f64,
        capture.height as f64,
        debug_id,
    );
}

fn assert_rect_inside(rect: &UiAutomationRect, width: f64, height: f64, label: &str) {
    assert!(
        rect.width > 0.0,
        "{label} width should be positive: {rect:?}"
    );
    assert!(
        rect.height > 0.0,
        "{label} height should be positive: {rect:?}"
    );
    assert!(rect.x >= 0.0, "{label} x should be on-screen: {rect:?}");
    assert!(rect.y >= 0.0, "{label} y should be on-screen: {rect:?}");
    assert!(
        rect.x + rect.width <= width,
        "{label} should fit viewport width: {rect:?} / {width}"
    );
    assert!(
        rect.y + rect.height <= height,
        "{label} should fit viewport height: {rect:?} / {height}"
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

#[test]
fn research_plan_header_list_and_advanced_search_use_real_events_ui_e2e() {
    let mut harness = harness();
    let initial = harness.automation_capture(UiAutomationCaptureRequest::default());
    let initial_image = decode_png(&initial);

    for debug_id in [
        "research.header.search",
        "research.header.advanced_search",
        "research.header.import",
        "research.header.export",
        "research.header.sync",
        "research.collection.0",
        "research.status.0",
        "research.paper.sort",
        "research.paper.0",
        "research.inspector.tab.0",
        "research.inspector.tab.5",
    ] {
        assert_selector(&initial, debug_id);
        assert_bounds_inside(&initial, debug_id);
    }

    let advanced = click(&mut harness, "research.header.advanced_search");
    let advanced_image = decode_png(&advanced);
    assert!(
        initial_image.as_raw() != advanced_image.as_raw(),
        "advanced search click should visibly open the panel"
    );
    for debug_id in [
        "research.advanced.title",
        "research.advanced.author",
        "research.advanced.venue",
        "research.advanced.tag",
    ] {
        assert_selector(&advanced, debug_id);
        assert_bounds_inside(&advanced, debug_id);
    }

    let import = click(&mut harness, "research.header.import");
    let import_image = decode_png(&import);
    assert!(
        advanced_image.as_raw() != import_image.as_raw(),
        "import button should show visible queued/import feedback"
    );

    let search_focus = click(&mut harness, "research.header.search");
    decode_png(&search_focus);
    let after_search_text = type_text(&mut harness, "graph");
    let search_image = decode_png(&after_search_text);
    assert!(
        import_image.as_raw() != search_image.as_raw(),
        "typing in header search should visibly update filtering/search text"
    );

    let sorted = click(&mut harness, "research.paper.sort");
    let sorted_image = decode_png(&sorted);
    assert!(
        search_image.as_raw() != sorted_image.as_raw(),
        "sort button should visibly update list ordering or sort state"
    );
    assert_selector(&sorted, "research.paper.sort");
}

#[test]
fn research_plan_inspector_pdf_and_write_flows_use_real_events_ui_e2e() {
    let mut harness = harness();
    let paper_selected = click(&mut harness, "research.paper.0");
    decode_png(&paper_selected);

    let qa_tab = click(&mut harness, "research.inspector.tab.2");
    let qa_image = decode_png(&qa_tab);
    assert_selector(&qa_tab, "research.qa.input");
    assert_selector(&qa_tab, "research.qa.send");
    assert_selector(&qa_tab, "research.qa.quick.summarize");
    assert_selector(&qa_tab, "research.qa.quick.key_points");
    assert_selector(&qa_tab, "research.qa.quick.limitations");
    assert_bounds_inside(&qa_tab, "research.qa.input");

    let qa_focus = click(&mut harness, "research.qa.input");
    decode_png(&qa_focus);
    let qa_text = type_text(&mut harness, "summarize");
    decode_png(&qa_text);
    let qa_sent = click(&mut harness, "research.qa.send");
    let qa_sent_image = decode_png(&qa_sent);
    assert!(
        qa_image.as_raw() != qa_sent_image.as_raw(),
        "Q&A send button should add visible conversation output"
    );

    let quick_points = click(&mut harness, "research.qa.quick.key_points");
    let quick_points_image = decode_png(&quick_points);
    assert!(
        qa_sent_image.as_raw() != quick_points_image.as_raw(),
        "Q&A quick action should dispatch through the same visible message flow"
    );

    let write_tab = click(&mut harness, "research.inspector.tab.4");
    let write_image = decode_png(&write_tab);
    assert_selector(&write_tab, "research.manuscript.add_section");
    let section_added = click(&mut harness, "research.manuscript.add_section");
    let section_image = decode_png(&section_added);
    assert!(
        write_image.as_raw() != section_image.as_raw(),
        "add manuscript section button should visibly add a section row"
    );
    assert_selector(&section_added, "research.manuscript.section.0");
    assert_selector(&section_added, "research.manuscript.cite_search");
    let cite_search = click(&mut harness, "research.manuscript.cite_search");
    decode_png(&cite_search);
    let cite_text = type_text(&mut harness, "graph");
    let cite_text_image = decode_png(&cite_text);
    assert!(
        section_image.as_raw() != cite_text_image.as_raw(),
        "typing in manuscript cite search should visibly filter citation results"
    );

    let pdf_mode = key(
        &mut harness,
        UiAutomationKey::Character("p".to_string()),
        UiAutomationModifiers {
            alt: true,
            ..UiAutomationModifiers::default()
        },
    );
    let pdf_image = decode_png(&pdf_mode);
    assert!(
        section_image.as_raw() != pdf_image.as_raw(),
        "Alt+P should switch the reader into visible PDF mode"
    );
    assert_selector(&pdf_mode, "research.pdf.next");
    assert_selector(&pdf_mode, "research.pdf.search");
    assert_selector(&pdf_mode, "research.pdf.surface");
    assert_selector(&pdf_mode, "research.pdf.zoom_out");
    assert_selector(&pdf_mode, "research.pdf.zoom_in");
    assert_selector(&pdf_mode, "research.pdf.rotate");
    assert_selector(&pdf_mode, "research.pdf.tool.highlight");
    assert_selector(&pdf_mode, "research.pdf.annotation_list_toggle");
    assert_bounds_inside(&pdf_mode, "research.pdf.next");

    let pdf_search_focus = click(&mut harness, "research.pdf.search");
    decode_png(&pdf_search_focus);
    let pdf_search_text = type_text(&mut harness, "method");
    let pdf_search_text_image = decode_png(&pdf_search_text);
    assert!(
        pdf_image.as_raw() != pdf_search_text_image.as_raw(),
        "PDF search input click and typing should visibly update the search field"
    );

    let pdf_zoomed = click(&mut harness, "research.pdf.zoom_in");
    let zoomed_image = decode_png(&pdf_zoomed);
    assert!(
        pdf_search_text_image.as_raw() != zoomed_image.as_raw(),
        "PDF zoom button should visibly update PDF surface rendering"
    );

    let rotated = click(&mut harness, "research.pdf.rotate");
    let rotated_image = decode_png(&rotated);
    assert!(
        zoomed_image.as_raw() != rotated_image.as_raw(),
        "PDF rotate button should visibly update PDF surface rendering"
    );

    let highlight = click(&mut harness, "research.pdf.tool.highlight");
    decode_png(&highlight);
    let annotated = click(&mut harness, "research.pdf.surface");
    let annotated_image = decode_png(&annotated);
    assert!(
        rotated_image.as_raw() != annotated_image.as_raw(),
        "PDF annotation tool and surface click should visibly place an annotation"
    );

    let annotations = click(&mut harness, "research.pdf.annotation_list_toggle");
    let annotations_image = decode_png(&annotations);
    assert!(
        annotated_image.as_raw() != annotations_image.as_raw(),
        "annotation list toggle should visibly open the annotation sidebar"
    );
}

#[test]
fn research_plan_citation_controls_use_real_events_ui_e2e() {
    let mut harness = harness();
    let paper_selected = click(&mut harness, "research.paper.0");
    decode_png(&paper_selected);

    let cite_tab = click(&mut harness, "research.inspector.tab.5");
    let cite_image = decode_png(&cite_tab);
    for debug_id in [
        "research.citation.format.bibtex",
        "research.citation.format.ris",
        "research.citation.format.apa",
        "research.citation.format.chicago",
        "research.citation.format.mla",
        "research.citation.doi",
        "research.citation.fetch",
        "research.citation.import_bibtex",
    ] {
        assert_selector(&cite_tab, debug_id);
        assert_bounds_inside(&cite_tab, debug_id);
    }

    let ris = click(&mut harness, "research.citation.format.ris");
    let ris_image = decode_png(&ris);
    assert!(
        cite_image.as_raw() != ris_image.as_raw(),
        "citation format button should visibly update active format state"
    );

    let doi_focus = click(&mut harness, "research.citation.doi");
    decode_png(&doi_focus);
    let doi_typed = type_text(&mut harness, "10.1234/example");
    let doi_image = decode_png(&doi_typed);
    assert!(
        ris_image.as_raw() != doi_image.as_raw(),
        "DOI input should visibly render typed text"
    );

    let fetched = click(&mut harness, "research.citation.fetch");
    let fetched_image = decode_png(&fetched);
    assert!(
        doi_image.as_raw() != fetched_image.as_raw(),
        "Fetch DOI should visibly add queued metadata feedback"
    );

    let imported = click(&mut harness, "research.citation.import_bibtex");
    let imported_image = decode_png(&imported);
    assert!(
        fetched_image.as_raw() != imported_image.as_raw(),
        "Import BibTeX should visibly add import feedback"
    );
}
