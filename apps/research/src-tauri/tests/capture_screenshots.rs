use std::io::Write;
use std::path::Path;

use tench_research_lib::ui::{state::ResearchState, ResearchApp};
use tench_ui_automation_core::UiAutomationCaptureRequest;
use tench_ui_test::harness::HarnessConfig;
use tench_ui_test::TestHarness;

fn save_png(harness: &mut TestHarness, name: &str) {
    let capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    let dir = Path::new("../../plans/ui");
    std::fs::create_dir_all(dir).unwrap();
    let path = dir.join(format!("research_{name}.png"));
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(&capture.png_bytes).unwrap();
    eprintln!("saved {}", path.display());
}

#[test]
#[ignore]
fn capture_research_screenshots() {
    let mut harness = TestHarness::with_config(
        ResearchApp::with_state(ResearchState::example()),
        HarnessConfig::with_viewport(1280.0, 720.0),
    );

    // Default state
    save_png(&mut harness, "default");

    // Advanced search open
    let _ = harness.automation_click_debug_id("research.header.advanced_search");
    save_png(&mut harness, "advanced_search");
}
