//! Research app UI — Mendeley-style research paper management tool.

mod automation;
pub mod collection_tree;
mod events;
pub mod helpers;
pub mod inspector;
mod paint;
pub mod paper_list;
pub mod state;

use tench_ui::prelude::*;

use state::ResearchState;

const HEADER_H: f64 = 48.0;
const LEFT_W: f64 = 240.0;
const RIGHT_W: f64 = 280.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResearchRegions {
    pub header: Rect,
    pub left: Rect,
    pub center: Rect,
    pub right: Rect,
}

pub fn research_regions(size: Size) -> ResearchRegions {
    let left_w = if size.width < 560.0 {
        (size.width * 0.30).clamp(96.0, LEFT_W)
    } else {
        LEFT_W
    };
    let right_w = if size.width < 980.0 { 0.0 } else { RIGHT_W };
    let center_x0 = left_w.min(size.width);
    let center_x1 = (size.width - right_w).max(center_x0);
    ResearchRegions {
        header: Rect::new(0.0, 0.0, size.width, HEADER_H),
        left: Rect::new(0.0, HEADER_H, center_x0, size.height),
        center: Rect::new(center_x0, HEADER_H, center_x1, size.height),
        right: Rect::new(center_x1, HEADER_H, size.width, size.height),
    }
}

// ---------------------------------------------------------------------------
// ResearchApp widget
// ---------------------------------------------------------------------------

pub struct ResearchApp {
    state: ResearchState,
    i18n: tench_app_core::I18nCatalog,
}

impl Default for ResearchApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ResearchApp {
    pub fn new() -> Self {
        Self {
            state: ResearchState::empty(),
            i18n: crate::i18n::research_i18n_catalog(crate::i18n::DEFAULT_LOCALE),
        }
    }

    pub fn with_state(state: ResearchState) -> Self {
        Self {
            state,
            i18n: crate::i18n::research_i18n_catalog(crate::i18n::DEFAULT_LOCALE),
        }
    }
}

impl Widget for ResearchApp {
    fn measure(&mut self, _ctx: &mut MeasureCtx, axis: Axis, available: f64) -> f64 {
        match axis {
            Axis::Horizontal => available,
            Axis::Vertical => available,
        }
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _size: Size) {}

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        self.paint_impl(ctx, scene);
    }

    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        self.on_pointer_event_impl(ctx, event);
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        self.on_text_event_impl(ctx, event);
    }

    fn accessibility_tree(&self, state: &WidgetState) -> AccessibilityNode {
        AccessibilityNode {
            role: AccessRole::Window,
            label: Some("Tench Research".to_string()),
            value: Some(self.state.reader_mode.label().to_string()),
            focused: state.is_focused,
            disabled: state.is_disabled,
            children: Vec::new(),
        }
    }

    fn automation_children(&self, state: &WidgetState) -> Vec<UiAutomationNode> {
        automation::research_automation_nodes(
            &self.state,
            state.size,
            state.id.to_raw(),
            &self.i18n,
        )
    }

    fn debug_id(&self) -> Option<&str> {
        Some("research.root")
    }
}

#[cfg(test)]
use automation::research_automation_nodes;

#[cfg(test)]
mod tests {
    use super::*;
    use tench_ui_automation_core::{find_node, UiAutomationAction, UiAutomationSelector};
    use tench_ui_test::{harness::HarnessConfig, TestHarness};

    #[test]
    fn responsive_regions_do_not_overlap_across_core_viewports() {
        for size in [
            Size::new(390.0, 844.0),
            Size::new(768.0, 1024.0),
            Size::new(1440.0, 900.0),
        ] {
            let regions = research_regions(size);

            assert_rect_valid(regions.header);
            assert_rect_valid(regions.left);
            assert_rect_valid(regions.center);
            assert_rect_valid(regions.right);
            assert_eq!(regions.left.x1, regions.center.x0);
            assert_eq!(regions.center.x1, regions.right.x0);
            assert_eq!(regions.right.x1, size.width);
            assert!(regions.center.width() >= 260.0);
        }
    }

    #[test]
    fn research_header_and_paper_rows_expose_selector_nodes_ui_automation() {
        let state = ResearchState::example();
        let i18n = crate::i18n::research_i18n_catalog(crate::i18n::DEFAULT_LOCALE);
        let nodes = research_automation_nodes(&state, Size::new(1440.0, 900.0), 1, &i18n);
        let root = UiAutomationNode {
            id: 1,
            debug_id: Some("research.root".to_string()),
            role: "window".to_string(),
            label: Some("Tench Research".to_string()),
            value: None,
            bounds: UiAutomationRect {
                x: 0.0,
                y: 0.0,
                width: 1440.0,
                height: 900.0,
            },
            enabled: true,
            focused: false,
            hovered: false,
            children: nodes,
        };

        for debug_id in ["research.header.import", "research.paper.0"] {
            assert!(
                find_node(
                    &root,
                    &UiAutomationSelector::ByDebugId {
                        debug_id: debug_id.to_string()
                    }
                )
                .is_some(),
                "missing automation node {debug_id}"
            );
        }
    }

    #[test]
    fn research_advanced_search_click_updates_tree_ui_automation() {
        let mut harness = TestHarness::with_config(
            ResearchApp::new(),
            HarnessConfig::with_viewport(1440.0, 900.0),
        );

        let capture = harness
            .automation_action(UiAutomationAction::Click {
                selector: UiAutomationSelector::ByDebugId {
                    debug_id: "research.header.advanced_search".to_string(),
                },
                modifiers: Default::default(),
            })
            .expect("advanced search click");
        assert!(capture.png_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));

        let tree = harness.automation_tree();
        assert!(
            find_node(
                &tree,
                &UiAutomationSelector::ByDebugId {
                    debug_id: "research.advanced.title".to_string()
                }
            )
            .is_some(),
            "advanced search panel should expose title field after click"
        );
    }

    fn assert_rect_valid(rect: Rect) {
        assert!(rect.x0 <= rect.x1, "{rect:?}");
        assert!(rect.y0 <= rect.y1, "{rect:?}");
        assert!(rect.width() >= 0.0, "{rect:?}");
        assert!(rect.height() >= 0.0, "{rect:?}");
    }
}
