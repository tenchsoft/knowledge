use super::super::state::ResearchState;
use super::super::{paper_list, state, ResearchRegions, HEADER_H};
use super::push_research_node;
use tench_ui::prelude::*;

pub(super) fn push_pdf_nodes(
    research: &ResearchState,
    regions: ResearchRegions,
    spacing: f64,
    spacing_large: f64,
    nodes: &mut Vec<UiAutomationNode>,
    next_id: &mut u64,
) {
    if let (state::ReaderMode::Pdf, Some(paper)) = (research.reader_mode, research.selected_paper())
    {
        let center_x = regions.center.x0;
        let center_right = regions.center.x1;
        let abstract_lines = paper_list::wrap_text(&paper.abstract_text, 80);
        let search_y = HEADER_H
            + spacing_large
            + 28.0
            + 24.0
            + 24.0
            + 24.0
            + 20.0
            + 18.0 * abstract_lines.len() as f64
            + spacing_large;
        push_research_node(
            nodes,
            next_id,
            "text_input",
            "Search in PDF",
            "research.pdf.search",
            Rect::new(
                center_x + spacing_large,
                search_y,
                center_right - spacing_large,
                search_y + 24.0,
            ),
        );
        let fig_y = search_y + 28.0;
        push_research_node(
            nodes,
            next_id,
            "canvas",
            "PDF surface",
            "research.pdf.surface",
            Rect::new(
                center_x + spacing_large,
                fig_y,
                center_right - spacing_large,
                fig_y + 180.0,
            ),
        );
        let nav_y = fig_y + 196.0;
        let nav_x = center_x + spacing_large;
        let prev_x = nav_x + 100.0;
        push_research_node(
            nodes,
            next_id,
            "button",
            "previous page",
            "research.pdf.prev",
            Rect::new(prev_x, nav_y - 12.0, prev_x + 28.0, nav_y + 8.0),
        );
        let next_x = prev_x + 36.0;
        push_research_node(
            nodes,
            next_id,
            "button",
            "next page",
            "research.pdf.next",
            Rect::new(next_x, nav_y - 12.0, next_x + 28.0, nav_y + 8.0),
        );
        let zoom_x = next_x + 28.0 + spacing_large;
        let zoom_out_x = zoom_x + 50.0;
        push_research_node(
            nodes,
            next_id,
            "button",
            "zoom out",
            "research.pdf.zoom_out",
            Rect::new(zoom_out_x, nav_y - 12.0, zoom_out_x + 28.0, nav_y + 8.0),
        );
        let zoom_in_x = zoom_out_x + 32.0;
        push_research_node(
            nodes,
            next_id,
            "button",
            "zoom in",
            "research.pdf.zoom_in",
            Rect::new(zoom_in_x, nav_y - 12.0, zoom_in_x + 28.0, nav_y + 8.0),
        );
        let rotate_x = zoom_in_x + 28.0 + spacing;
        push_research_node(
            nodes,
            next_id,
            "button",
            "rotate",
            "research.pdf.rotate",
            Rect::new(rotate_x, nav_y - 12.0, rotate_x + 36.0, nav_y + 8.0),
        );
        let mut tool_x = rotate_x + 44.0 + spacing;
        for (label, debug_id) in [
            ("highlight", "research.pdf.tool.highlight"),
            ("underline", "research.pdf.tool.underline"),
            ("strikeout", "research.pdf.tool.strikeout"),
            ("sticky note", "research.pdf.tool.sticky_note"),
        ] {
            push_research_node(
                nodes,
                next_id,
                "button",
                label,
                debug_id,
                Rect::new(tool_x, nav_y - 12.0, tool_x + 24.0, nav_y + 8.0),
            );
            tool_x += 28.0;
        }
        let ann_list_x = tool_x + spacing;
        push_research_node(
            nodes,
            next_id,
            "button",
            "annotations",
            "research.pdf.annotation_list_toggle",
            Rect::new(ann_list_x, nav_y - 12.0, ann_list_x + 32.0, nav_y + 8.0),
        );

        // Annotation list rows (when visible)
        if research.pdf_show_annotation_list {
            let mut ann_y = nav_y + 14.0 + spacing_large + 20.0;
            for (index, ann) in research.pdf_annotations.iter().enumerate() {
                let kind_label = match ann.kind {
                    state::PdfAnnotationTool::Highlight => "HL",
                    state::PdfAnnotationTool::Underline => "UL",
                    state::PdfAnnotationTool::Strikeout => "ST",
                    state::PdfAnnotationTool::StickyNote => "Note",
                    _ => "?",
                };
                let label = ann.text.as_deref().unwrap_or("—");
                push_research_node(
                    nodes,
                    next_id,
                    "button",
                    format!("p{}: {} {}", ann.page, kind_label, label),
                    format!("research.pdf.annotation.{index}"),
                    Rect::new(
                        center_x + spacing_large,
                        ann_y - 10.0,
                        center_right - spacing_large,
                        ann_y + 8.0,
                    ),
                );
                ann_y += 22.0;
            }
        }
    }
}
