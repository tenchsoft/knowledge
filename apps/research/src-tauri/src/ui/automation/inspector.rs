use super::super::state::ResearchState;
use super::super::{paper_list, state, ResearchRegions, HEADER_H};
use super::push_research_node;
use tench_ui::prelude::*;

pub(super) fn push_inspector_nodes(
    research: &ResearchState,
    i18n: &tench_app_core::I18nCatalog,
    regions: ResearchRegions,
    spacing: f64,
    spacing_large: f64,
    nodes: &mut Vec<UiAutomationNode>,
    next_id: &mut u64,
) {
    let t = |key: &'static str| i18n.resolve(key).unwrap_or(key).to_string();
    if regions.right.width() >= 160.0 {
        let rx = regions.center.x1;
        let ry = HEADER_H + spacing_large;
        for (index, label_key) in [
            "research.inspector.notes_short",
            "research.inspector.summary_short",
            "research.inspector.qa_short",
            "research.inspector.visual_short",
            "research.inspector.write_short",
            "research.inspector.cite_short",
        ]
        .into_iter()
        .enumerate()
        {
            let tab_x = rx + spacing + index as f64 * (36.0 + spacing);
            push_research_node(
                nodes,
                next_id,
                "tab",
                t(label_key),
                format!("research.inspector.tab.{index}"),
                Rect::new(tab_x, ry - 10.0, tab_x + 36.0, ry + 6.0),
            );
        }

        if research.active_inspector_tab == 2 {
            let input_w = regions.right.width() - 80.0;
            let mut qa_ry = HEADER_H + spacing_large + 24.0;
            for (_role, message) in &research.analysis_messages {
                let lines = paper_list::wrap_text(message, 40);
                qa_ry += 14.0 * lines.len() as f64 + 20.0;
            }
            qa_ry += spacing + spacing;
            push_research_node(
                nodes,
                next_id,
                "text_input",
                "Ask about this paper",
                "research.qa.input",
                Rect::new(
                    rx + spacing,
                    qa_ry - 10.0,
                    rx + spacing + input_w,
                    qa_ry + 22.0,
                ),
            );
            let send_x = rx + spacing + input_w + 4.0;
            push_research_node(
                nodes,
                next_id,
                "button",
                "Send",
                "research.qa.send",
                Rect::new(send_x, qa_ry - 10.0, send_x + 36.0, qa_ry + 22.0),
            );
            qa_ry += 32.0 + spacing;
            let mut qa_x = rx + spacing;
            for (label, debug_id) in [
                ("Summarize", "research.qa.quick.summarize"),
                ("Key Points", "research.qa.quick.key_points"),
                ("Limitations", "research.qa.quick.limitations"),
            ] {
                push_research_node(
                    nodes,
                    next_id,
                    "button",
                    label,
                    debug_id,
                    Rect::new(qa_x, qa_ry - 8.0, qa_x + 72.0, qa_ry + 6.0),
                );
                qa_x += 76.0;
            }
        }

        if research.active_inspector_tab == 4 {
            let mut write_y = HEADER_H + spacing_large + 24.0;
            write_y += research.manuscript_summary_lines.len() as f64 * 16.0 + 6.0;
            for (section_idx, section) in research.manuscript_sections.iter().enumerate() {
                push_research_node(
                    nodes,
                    next_id,
                    "button",
                    &section.title,
                    format!("research.manuscript.section.{section_idx}"),
                    Rect::new(
                        rx + spacing,
                        write_y - 2.0,
                        rx + regions.right.width() - spacing,
                        write_y + 18.0,
                    ),
                );
                write_y += 20.0;
                if research.manuscript_active_section == Some(section_idx)
                    && !section.content.is_empty()
                {
                    write_y += 16.0;
                }
            }
            write_y += 4.0;
            push_research_node(
                nodes,
                next_id,
                "button",
                t("research.manuscript.add_section"),
                "research.manuscript.add_section",
                Rect::new(rx + spacing, write_y, rx + spacing + 80.0, write_y + 24.0),
            );
            write_y += 32.0;
            if research.manuscript_active_section.is_some() {
                push_research_node(
                    nodes,
                    next_id,
                    "text_input",
                    t("research.manuscript.cite_search"),
                    "research.manuscript.cite_search",
                    Rect::new(
                        rx + spacing,
                        write_y,
                        rx + regions.right.width() - spacing,
                        write_y + 24.0,
                    ),
                );
                write_y += 30.0;
                for (result_idx, (_paper_idx, paper_title)) in
                    research.filtered_cite_results().into_iter().enumerate()
                {
                    push_research_node(
                        nodes,
                        next_id,
                        "button",
                        paper_title,
                        format!("research.manuscript.cite_result.{result_idx}.insert"),
                        Rect::new(
                            rx + regions.right.width() - spacing - 48.0,
                            write_y,
                            rx + regions.right.width() - spacing - 4.0,
                            write_y + 20.0,
                        ),
                    );
                    write_y += 24.0;
                }
            }
        }

        if research.active_inspector_tab == 5 {
            let mut cite_y = HEADER_H + spacing_large + 24.0 + 20.0;
            let formats = [
                (
                    state::CitationExportFormat::BibTex,
                    "research.citation.format.bibtex",
                ),
                (
                    state::CitationExportFormat::Ris,
                    "research.citation.format.ris",
                ),
                (
                    state::CitationExportFormat::Apa,
                    "research.citation.format.apa",
                ),
                (
                    state::CitationExportFormat::Chicago,
                    "research.citation.format.chicago",
                ),
                (
                    state::CitationExportFormat::Mla,
                    "research.citation.format.mla",
                ),
            ];
            let mut fmt_x = rx + spacing;
            for (fmt, debug_id) in formats {
                let label = fmt.label();
                let btn_w = label.len() as f64 * 7.0 + 12.0;
                push_research_node(
                    nodes,
                    next_id,
                    "button",
                    label,
                    debug_id,
                    Rect::new(fmt_x, cite_y - 8.0, fmt_x + btn_w, cite_y + 6.0),
                );
                fmt_x += btn_w + 4.0;
            }

            if let Some(paper) = research.selected_paper() {
                cite_y += 24.0 + paper.references.len() as f64 * 18.0 + spacing + 16.0;
            }
            push_research_node(
                nodes,
                next_id,
                "text_input",
                "DOI / arXiv ID",
                "research.citation.doi",
                Rect::new(
                    rx + spacing,
                    cite_y - 8.0,
                    rx + regions.right.width() - spacing - 48.0,
                    cite_y + 8.0,
                ),
            );
            let fetch_x = rx + regions.right.width() - spacing - 44.0;
            push_research_node(
                nodes,
                next_id,
                "button",
                "Fetch",
                "research.citation.fetch",
                Rect::new(fetch_x, cite_y - 8.0, fetch_x + 44.0, cite_y + 8.0),
            );
            cite_y += 24.0;
            push_research_node(
                nodes,
                next_id,
                "button",
                "Import BibTeX",
                "research.citation.import_bibtex",
                Rect::new(
                    rx + spacing,
                    cite_y - 8.0,
                    rx + spacing + 100.0,
                    cite_y + 8.0,
                ),
            );
        }
    }
}
