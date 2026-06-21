use std::io::Write;
use std::path::Path;

use tench_study_lib::ui::StudyApp;
use tench_ui_automation_core::UiAutomationCaptureRequest;
use tench_ui_test::harness::HarnessConfig;
use tench_ui_test::TestHarness;

fn save_png(harness: &mut TestHarness, name: &str) {
    let capture = harness.automation_capture(UiAutomationCaptureRequest::default());
    let dir = Path::new("../../plans/ui");
    std::fs::create_dir_all(dir).unwrap();
    let path = dir.join(format!("study_{name}.png"));
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(&capture.png_bytes).unwrap();
    eprintln!("saved {}", path.display());
}

#[test]
#[ignore]
fn capture_study_screenshots() {
    let mut harness =
        TestHarness::with_config(StudyApp::new(), HarnessConfig::with_viewport(1280.0, 720.0));

    // Profile setup state
    save_png(&mut harness, "profile_setup");
}
