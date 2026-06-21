mod inspector;
mod pdf;
mod welcome;

use super::state::ResearchState;
use super::{collection_tree, helpers, research_regions, HEADER_H};
use tench_ui::prelude::*;

pub(super) fn research_automation_nodes(
    research: &ResearchState,
    size: Size,
    base_id: u64,
    i18n: &tench_app_core::I18nCatalog,
) -> Vec<UiAutomationNode> {
    if research.show_welcome {
        return welcome::research_welcome_automation_nodes(size, base_id, i18n);
    }

    let regions = research_regions(size);
    let mut nodes = Vec::new();
    let mut next_id = base_id.saturating_mul(1000);
    let spacing = 8.0;
    let spacing_large = 16.0;
    let t = |key: &'static str| i18n.resolve(key).unwrap_or(key).to_string();

    if size.width >= 700.0 {
        let search_x = 180.0;
        let search_w = (size.width - 540.0).clamp(180.0, 280.0);
        let search_rect = Rect::new(search_x, 8.0, search_x + search_w, 40.0);
        push_research_node(
            &mut nodes,
            &mut next_id,
            "text_input",
            t("research.search.prompt"),
            "research.header.search",
            search_rect,
        );
        push_research_node(
            &mut nodes,
            &mut next_id,
            "button",
            "advanced search",
            "research.header.advanced_search",
            Rect::new(
                search_x + search_w - 24.0,
                10.0,
                search_x + search_w - 4.0,
                30.0,
            ),
        );

        let btn_x = search_x + search_w + spacing;
        for (index, (label, debug_id)) in [
            (t("research.action.import"), "research.header.import"),
            (t("research.action.export"), "research.header.export"),
            (t("research.action.sync"), "research.header.sync"),
        ]
        .into_iter()
        .enumerate()
        {
            let bx = btn_x + index as f64 * (80.0 + spacing);
            push_research_node(
                &mut nodes,
                &mut next_id,
                "button",
                label,
                debug_id,
                Rect::new(bx, 8.0, bx + 80.0, 40.0),
            );
        }
    }

    if research.show_advanced_search {
        let panel_y = HEADER_H + spacing + 4.0;
        for (index, (label, debug_id)) in [
            ("Title", "research.advanced.title"),
            ("Author", "research.advanced.author"),
            ("Venue", "research.advanced.venue"),
            ("Tag", "research.advanced.tag"),
        ]
        .into_iter()
        .enumerate()
        {
            let y = panel_y + index as f64 * 20.0;
            push_research_node(
                &mut nodes,
                &mut next_id,
                "text_input",
                label,
                debug_id,
                Rect::new(240.0, y - 8.0, 420.0, y + 8.0),
            );
        }
        // Year range fields
        let year_y = panel_y + 4.0 * 20.0;
        push_research_node(
            &mut nodes,
            &mut next_id,
            "text_input",
            "Year from",
            "research.advanced.year_from",
            Rect::new(240.0, year_y - 8.0, 270.0, year_y + 8.0),
        );
        push_research_node(
            &mut nodes,
            &mut next_id,
            "text_input",
            "Year to",
            "research.advanced.year_to",
            Rect::new(274.0, year_y - 8.0, 300.0, year_y + 8.0),
        );
    }

    let collection_y = collection_tree::collection_rows_start_y(HEADER_H, spacing_large);
    for (index, collection) in research.collections.iter().enumerate() {
        let row_y = collection_y + index as f64 * 20.0;
        // Expand/collapse toggle icon
        push_research_node(
            &mut nodes,
            &mut next_id,
            "button",
            if collection.expanded {
                "collapse"
            } else {
                "expand"
            },
            format!("research.collection.{index}.expand"),
            Rect::new(
                regions.left.x0,
                row_y,
                regions.left.x0 + spacing + 10.0,
                row_y + 20.0,
            ),
        );
        // Collection row (click to select)
        push_research_node(
            &mut nodes,
            &mut next_id,
            "button",
            &collection.name,
            format!("research.collection.{index}"),
            Rect::new(
                regions.left.x0 + spacing + 10.0,
                row_y,
                regions.left.x1,
                row_y + 20.0,
            ),
        );
    }

    // Smart collections automation nodes
    let smart_y = collection_y + research.collections.len() as f64 * 20.0 + spacing_large + 18.0;
    for (index, sc) in research.smart_collections.iter().enumerate() {
        push_research_node(
            &mut nodes,
            &mut next_id,
            "button",
            format!("\u{2699} {} ({})", sc.name, sc.count),
            format!("research.smart_collection.{index}"),
            Rect::new(
                regions.left.x0,
                smart_y + index as f64 * 20.0,
                regions.left.x1,
                smart_y + 20.0 + index as f64 * 20.0,
            ),
        );
    }

    let tag_y = collection_tree::tag_rows_start_y(research, HEADER_H, spacing_large);
    for (index, tag) in research.tags.iter().enumerate() {
        push_research_node(
            &mut nodes,
            &mut next_id,
            "button",
            tag,
            format!("research.tag.{index}"),
            Rect::new(
                regions.left.x0,
                tag_y + index as f64 * 24.0,
                regions.left.x1,
                tag_y + 24.0 + index as f64 * 24.0,
            ),
        );
    }

    // Saved searches automation nodes
    let saved_y = collection_tree::saved_search_rows_start_y(research, HEADER_H, spacing_large);
    for (index, saved) in research.saved_searches.iter().enumerate() {
        push_research_node(
            &mut nodes,
            &mut next_id,
            "button",
            &saved.name,
            format!("research.saved_search.{index}"),
            Rect::new(
                regions.left.x0,
                saved_y + index as f64 * 20.0,
                regions.left.x1,
                saved_y + 20.0 + index as f64 * 20.0,
            ),
        );
    }

    let status_y = collection_tree::status_rows_start_y(research, HEADER_H, spacing_large);
    for (index, status) in research.statuses.iter().enumerate() {
        push_research_node(
            &mut nodes,
            &mut next_id,
            "button",
            helpers::localized_research_status(i18n, status.label()),
            format!("research.status.{index}"),
            Rect::new(
                regions.left.x0,
                status_y + index as f64 * 20.0,
                regions.left.x1,
                status_y + 20.0 + index as f64 * 20.0,
            ),
        );
    }

    let paper_y = collection_tree::paper_list_start_y(research, HEADER_H, spacing, spacing_large);
    let sort_label = research.sort_mode.label();
    let sort_btn_w = sort_label.len() as f64 * 6.0 + 12.0;
    push_research_node(
        &mut nodes,
        &mut next_id,
        "button",
        sort_label,
        "research.paper.sort",
        Rect::new(
            regions.left.x1 - sort_btn_w - spacing,
            paper_y - 28.0,
            regions.left.x1 - spacing,
            paper_y - 14.0,
        ),
    );

    for (row, paper_idx) in research.visible_paper_indices().into_iter().enumerate() {
        let y0 = paper_y + row as f64 * 22.0;
        if y0 > regions.left.y1 {
            break;
        }
        if let Some(paper) = research.papers.get(paper_idx) {
            push_research_node(
                &mut nodes,
                &mut next_id,
                "button",
                &paper.title,
                format!("research.paper.{paper_idx}"),
                Rect::new(regions.left.x0, y0, regions.left.x1, y0 + 22.0),
            );
        }
    }

    inspector::push_inspector_nodes(
        research,
        i18n,
        regions,
        spacing,
        spacing_large,
        &mut nodes,
        &mut next_id,
    );

    pdf::push_pdf_nodes(
        research,
        regions,
        spacing,
        spacing_large,
        &mut nodes,
        &mut next_id,
    );

    // Shortcut help modal
    if research.show_shortcut_help {
        let modal_w = 360.0_f64.min(size.width - 40.0);
        let modal_h = 280.0_f64.min(size.height - 40.0);
        let cx = size.width / 2.0;
        let cy = size.height / 2.0;
        push_research_node(
            &mut nodes,
            &mut next_id,
            "dialog",
            "Keyboard Shortcuts",
            "research.modal.shortcuts",
            Rect::new(
                cx - modal_w / 2.0,
                cy - modal_h / 2.0,
                cx + modal_w / 2.0,
                cy + modal_h / 2.0,
            ),
        );
    }

    nodes
}

fn push_research_node(
    nodes: &mut Vec<UiAutomationNode>,
    next_id: &mut u64,
    role: &str,
    label: impl Into<String>,
    debug_id: impl Into<String>,
    rect: Rect,
) {
    *next_id = next_id.saturating_add(1);
    nodes.push(research_automation_node(
        *next_id,
        role,
        Some(label.into()),
        Some(debug_id.into()),
        rect,
    ));
}

fn push_research_child_node(
    parent: &mut UiAutomationNode,
    next_id: &mut u64,
    role: &str,
    label: impl Into<String>,
    debug_id: impl Into<String>,
    rect: Rect,
) {
    *next_id = next_id.saturating_add(1);
    parent.children.push(research_automation_node(
        *next_id,
        role,
        Some(label.into()),
        Some(debug_id.into()),
        rect,
    ));
}

fn research_automation_node(
    id: u64,
    role: &str,
    label: Option<String>,
    debug_id: Option<String>,
    rect: Rect,
) -> UiAutomationNode {
    UiAutomationNode {
        id,
        debug_id,
        role: role.to_string(),
        label,
        value: None,
        bounds: UiAutomationRect {
            x: rect.x0,
            y: rect.y0,
            width: rect.width(),
            height: rect.height(),
        },
        enabled: true,
        focused: false,
        hovered: false,
        children: Vec::new(),
    }
}
