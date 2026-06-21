fn main() {
    tench_ui::run_native_with_config(
        tench_ui::NativeConfig {
            title: "Tench Research".into(),
            width: 1360.0,
            height: 860.0,
            resizable: true,
        },
        |backend| backend.set_root(tench_research_lib::ui::ResearchApp::new()),
    );
}
