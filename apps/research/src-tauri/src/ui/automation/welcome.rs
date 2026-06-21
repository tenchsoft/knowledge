use super::{push_research_child_node, research_automation_node};
use tench_ui::prelude::*;

pub(super) fn research_welcome_automation_nodes(
    size: Size,
    base_id: u64,
    i18n: &tench_app_core::I18nCatalog,
) -> Vec<UiAutomationNode> {
    let mut next_id = base_id.saturating_mul(1000);
    let mut dialog = research_automation_node(
        next_id,
        "dialog",
        Some(
            i18n.resolve("research.welcome.title")
                .unwrap_or("research.welcome.title")
                .to_string(),
        ),
        Some("research.welcome".to_string()),
        research_welcome_card_rect(size),
    );
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let btn_w = 160.0;
    push_research_child_node(
        &mut dialog,
        &mut next_id,
        "button",
        i18n.resolve("research.welcome.get_started")
            .unwrap_or("research.welcome.get_started")
            .to_string(),
        "research.welcome.get_started",
        Rect::new(cx - btn_w / 2.0, cy + 20.0, cx + btn_w / 2.0, cy + 52.0),
    );
    push_research_child_node(
        &mut dialog,
        &mut next_id,
        "button",
        i18n.resolve("research.welcome.import")
            .unwrap_or("research.welcome.import")
            .to_string(),
        "research.welcome.import",
        Rect::new(cx - btn_w / 2.0, cy + 64.0, cx + btn_w / 2.0, cy + 96.0),
    );
    vec![dialog]
}

fn research_welcome_card_rect(size: Size) -> Rect {
    let cx = size.width / 2.0;
    let cy = size.height / 2.0;
    let card_w = 420.0_f64.min(size.width - 40.0);
    Rect::new(cx - card_w / 2.0, cy - 120.0, cx + card_w / 2.0, cy + 120.0)
}
