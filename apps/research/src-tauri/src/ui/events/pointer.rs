use super::super::{
    collection_tree, inspector, paper_list, research_regions, state, ResearchApp, HEADER_H,
};
use tench_ui::core::events::PointerButton;
use tench_ui::prelude::*;

impl ResearchApp {
    pub(crate) fn on_pointer_event_impl(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        let PointerEvent::Down(e) = event else {
            return;
        };

        let size = ctx.state.size;
        let regions = research_regions(size);
        let header_h = HEADER_H;
        let left_w = regions.left.width();
        let spacing = 8.0;
        let spacing_large = 16.0;

        // ── Welcome screen click handling ────────────────────────────
        if self.state.show_welcome {
            let cx = size.width / 2.0;
            let cy = size.height / 2.0;
            let btn_w = 160.0;
            // Get Started button
            let btn_rect = Rect::new(cx - btn_w / 2.0, cy + 20.0, cx + btn_w / 2.0, cy + 52.0);
            if btn_rect.contains(e.pos) {
                self.state.show_welcome = false;
                ctx.request_paint();
                return;
            }
            // Import Library button
            let import_btn_rect =
                Rect::new(cx - btn_w / 2.0, cy + 64.0, cx + btn_w / 2.0, cy + 96.0);
            if import_btn_rect.contains(e.pos) {
                self.state.show_welcome = false;
                self.state.queue_import();
                ctx.request_paint();
                return;
            }
            // Click outside card dismisses
            let card_w = 420.0_f64.min(size.width - 40.0);
            let card_rect = Rect::new(cx - card_w / 2.0, cy - 120.0, cx + card_w / 2.0, cy + 120.0);
            if !card_rect.contains(e.pos) {
                self.state.show_welcome = false;
                ctx.request_paint();
                return;
            }
            return;
        }

        // ── Toast dismiss on click ───────────────────────────────────
        if !self.state.toasts.is_empty() {
            let mut toast_y = size.height - 60.0;
            for toast in self.state.toasts.iter().rev().take(5) {
                let msg_w = toast.message.len() as f64 * 7.0 + 24.0;
                let toast_x = (size.width - msg_w) / 2.0;
                let toast_rect = Rect::new(toast_x, toast_y, toast_x + msg_w, toast_y + 28.0);
                if toast_rect.contains(e.pos) {
                    let id = toast.id.clone();
                    self.state.dismiss_toast(&id);
                    ctx.request_paint();
                    return;
                }
                toast_y -= 36.0;
            }
        }

        if e.pos.x < left_w && e.pos.y > header_h {
            // Collection row click
            if let Some(col_idx) = collection_tree::hit_test_collection_row(
                &self.state,
                e.pos.y,
                header_h,
                spacing_large,
            ) {
                let col = &self.state.collections[col_idx];
                let col_id = col.id.clone();
                let expand_x = spacing + 10.0;
                if e.pos.x < expand_x {
                    self.state.toggle_collection_expanded(&col_id);
                } else {
                    self.state.select_collection(Some(col_id));
                }
                ctx.request_paint();
                return;
            }

            // Smart collection row click
            if let Some(sc_idx) = collection_tree::hit_test_smart_collection_row(
                &self.state,
                e.pos.y,
                header_h,
                spacing_large,
            ) {
                let sc = &self.state.smart_collections[sc_idx];
                let query = match &sc.rule {
                    state::SmartCollectionRule::RecentlyAdded { days } => {
                        format!("added:{}d", days)
                    }
                    state::SmartCollectionRule::Tagged { tag } => tag.clone(),
                    state::SmartCollectionRule::Unread => "unread".to_string(),
                    state::SmartCollectionRule::Favorites => "favorites".to_string(),
                    state::SmartCollectionRule::ByAuthor { author } => author.clone(),
                    state::SmartCollectionRule::YearRange { from, to } => {
                        format!("year:{}-{}", from, to)
                    }
                };
                self.state.set_search_query(query);
                ctx.request_paint();
                return;
            }

            // Tag row click
            if let Some(tag_idx) =
                collection_tree::hit_test_tag_row(&self.state, e.pos.y, header_h, spacing_large)
            {
                let tag = &self.state.tags[tag_idx];
                self.state.set_search_query(tag.clone());
                ctx.request_paint();
                return;
            }

            // Saved search row click
            if let Some(ss_idx) = collection_tree::hit_test_saved_search_row(
                &self.state,
                e.pos.y,
                header_h,
                spacing_large,
            ) {
                let id = self.state.saved_searches[ss_idx].id.clone();
                self.state.load_saved_search(&id);
                ctx.request_paint();
                return;
            }

            // Status filter row click
            if let Some(status_idx) =
                collection_tree::hit_test_status_row(&self.state, e.pos.y, header_h, spacing_large)
            {
                let filter = &self.state.statuses[status_idx];
                let label = filter.label();
                self.state.set_search_query(label);
                ctx.request_paint();
                return;
            }

            // Paper list click
            let paper_y =
                collection_tree::paper_list_start_y(&self.state, header_h, spacing, spacing_large);

            // Sort button click
            let sort_label = self.state.sort_mode.label();
            let sort_btn_w = sort_label.len() as f64 * 6.0 + 12.0;
            let sort_btn_rect = Rect::new(
                left_w - sort_btn_w - spacing,
                paper_y - 24.0 - 4.0,
                left_w - spacing,
                paper_y - 24.0 + 10.0,
            );
            if sort_btn_rect.contains(e.pos) {
                self.state.cycle_sort_mode();
                ctx.request_paint();
                return;
            }

            if e.pos.y >= paper_y {
                let idx = ((e.pos.y - paper_y) / 22.0) as usize;
                let visible = self.state.visible_paper_indices();
                if let Some(&paper_idx) = visible.get(idx) {
                    // Right-click toggles multi-select; left-click selects normally
                    if e.button == PointerButton::Secondary {
                        self.state.toggle_multi_select(paper_idx);
                    } else {
                        self.state.selected_papers.clear();
                        let _ = self.state.select_visible_paper(idx);
                    }
                    ctx.request_paint();
                }
            }
            return;
        }

        // Header button clicks
        if e.pos.y >= 8.0 && e.pos.y <= 8.0 + 32.0 {
            let search_x = 180.0;
            let search_w = (size.width - 540.0).clamp(180.0, 280.0);
            let btn_x = search_x + search_w + spacing;

            // Advanced search toggle click
            let adv_x = search_x + search_w - 24.0;
            let adv_btn = Rect::new(adv_x, 10.0, adv_x + 20.0, 10.0 + 20.0);
            if adv_btn.contains(e.pos) {
                self.state.toggle_advanced_search();
                ctx.request_paint();
                return;
            }

            // Search box click
            let search_rect = Rect::new(search_x, 8.0, search_x + search_w, 8.0 + 32.0);
            if search_rect.contains(e.pos) {
                self.state.set_focus(state::FocusTarget::SearchBox);
                ctx.request_paint();
                return;
            }

            for (i, _label) in ["import", "export", "sync"].iter().enumerate() {
                let bx = btn_x + (i as f64) * (80.0 + spacing);
                let btn_rect = Rect::new(bx, 8.0, bx + 80.0, 8.0 + 32.0);
                if btn_rect.contains(e.pos) {
                    match i {
                        0 => {
                            self.state.queue_import();
                            self.state
                                .add_toast("Import started", state::ToastKind::Info);
                        }
                        1 => {
                            self.state.export_action();
                        }
                        2 => {
                            self.state.sync_action();
                        }
                        _ => {}
                    }
                    ctx.request_paint();
                    return;
                }
            }
        }

        // Click outside search box loses focus
        self.state.set_focus(state::FocusTarget::None);

        // Center panel: PDF surface annotation click
        if self.state.reader_mode == state::ReaderMode::Pdf
            && self.state.pdf_annotation_tool != state::PdfAnnotationTool::None
        {
            let Some(paper) = self.state.selected_paper() else {
                let center_right = regions.center.x1;
                if e.pos.x >= center_right && e.pos.y >= header_h + spacing_large - 10.0 {
                    if let Some(tab) =
                        inspector::hit_test_tab(e.pos.x, e.pos.y, center_right, header_h, spacing)
                    {
                        self.state.active_inspector_tab = tab;
                        ctx.request_paint();
                    }
                }
                return;
            };
            let center_x = regions.center.x0;
            let center_right = regions.center.x1;
            // PDF surface rect (matches paint calculation)
            let abstract_lines = paper_list::wrap_text(&paper.abstract_text, 80);
            let search_y = header_h
                + spacing_large
                + 28.0
                + 24.0
                + 24.0
                + 24.0
                + 20.0
                + 18.0 * abstract_lines.len() as f64
                + spacing_large;
            let fig_y = search_y + 28.0;
            let fig_rect = Rect::new(
                center_x + spacing_large,
                fig_y,
                center_right - spacing_large,
                fig_y + 180.0,
            );
            if fig_rect.contains(e.pos) && e.pos.x >= center_x {
                let tool = self.state.pdf_annotation_tool;
                let annotation_id = format!("ann-{}", self.state.pdf_annotations.len() + 1);
                let page = self.state.pdf_current_page;
                // Convert click position to page-relative rect
                let rel_x = (e.pos.x - fig_rect.x0) / fig_rect.width();
                let rel_y = (e.pos.y - fig_rect.y0) / fig_rect.height();
                let ann_rect = Rect::new(
                    rel_x * 612.0 - 40.0,
                    rel_y * 792.0 - 8.0,
                    rel_x * 612.0 + 40.0,
                    rel_y * 792.0 + 8.0,
                );
                let color = match tool {
                    state::PdfAnnotationTool::Highlight => Color::rgba8(255, 235, 59, 90),
                    state::PdfAnnotationTool::Underline => Color::rgba8(33, 150, 243, 200),
                    state::PdfAnnotationTool::Strikeout => Color::rgba8(244, 67, 54, 200),
                    state::PdfAnnotationTool::StickyNote => Color::rgba8(255, 193, 7, 220),
                    _ => Color::BLACK,
                };
                self.state.add_pdf_annotation(state::PdfAnnotationState {
                    id: annotation_id,
                    page,
                    rect: ann_rect,
                    kind: tool,
                    color,
                    text: if tool == state::PdfAnnotationTool::StickyNote {
                        Some("New note".to_string())
                    } else {
                        None
                    },
                });
                self.state
                    .add_toast("Annotation added", state::ToastKind::Success);
                ctx.request_paint();
                return;
            }
        }

        // Center panel: PDF navigation buttons
        if self.state.reader_mode == state::ReaderMode::Pdf {
            let Some(paper) = self.state.selected_paper() else {
                // fall through
                let center_right = regions.center.x1;
                if e.pos.x >= center_right && e.pos.y >= header_h + spacing_large - 10.0 {
                    if let Some(tab) =
                        inspector::hit_test_tab(e.pos.x, e.pos.y, center_right, header_h, spacing)
                    {
                        self.state.active_inspector_tab = tab;
                        ctx.request_paint();
                    }
                }
                return;
            };
            let center_x = regions.center.x0;
            let center_right = regions.center.x1;
            let abstract_lines = paper_list::wrap_text(&paper.abstract_text, 80);
            let search_y = header_h
                + spacing_large
                + 28.0
                + 24.0
                + 24.0
                + 24.0
                + 20.0
                + 18.0 * abstract_lines.len() as f64
                + spacing_large;
            let search_rect = Rect::new(
                center_x + spacing_large,
                search_y,
                center_right - spacing_large,
                search_y + 24.0,
            );
            if search_rect.contains(e.pos) {
                self.state.set_focus(state::FocusTarget::PdfSearch);
                ctx.request_paint();
                return;
            }
            let nav_y = search_y + 28.0 + 196.0;
            let nav_x = center_x + spacing_large;
            let btn_w = 28.0;
            let btn_h = 20.0;

            // Check if click is in the PDF nav bar area
            if e.pos.y >= nav_y - 14.0 && e.pos.y <= nav_y + 10.0 && e.pos.x >= nav_x {
                let prev_x = nav_x + 100.0;
                let prev_rect =
                    Rect::new(prev_x, nav_y - 12.0, prev_x + btn_w, nav_y - 12.0 + btn_h);
                if prev_rect.contains(e.pos) {
                    self.state.pdf_prev_page();
                    ctx.request_paint();
                    return;
                }

                let next_x = prev_x + btn_w + spacing;
                let next_rect =
                    Rect::new(next_x, nav_y - 12.0, next_x + btn_w, nav_y - 12.0 + btn_h);
                if next_rect.contains(e.pos) {
                    self.state.pdf_next_page();
                    ctx.request_paint();
                    return;
                }

                let zoom_x = next_x + btn_w + spacing_large;
                let zoom_out_x = zoom_x + 50.0;
                let zoom_out_rect = Rect::new(
                    zoom_out_x,
                    nav_y - 12.0,
                    zoom_out_x + btn_w,
                    nav_y - 12.0 + btn_h,
                );
                if zoom_out_rect.contains(e.pos) {
                    self.state.pdf_zoom_out();
                    ctx.request_paint();
                    return;
                }

                let zoom_in_x = zoom_out_x + btn_w + 4.0;
                let zoom_in_rect = Rect::new(
                    zoom_in_x,
                    nav_y - 12.0,
                    zoom_in_x + btn_w,
                    nav_y - 12.0 + btn_h,
                );
                if zoom_in_rect.contains(e.pos) {
                    self.state.pdf_zoom_in();
                    ctx.request_paint();
                    return;
                }

                let rot_x = zoom_in_x + btn_w + spacing;
                let rot_rect = Rect::new(rot_x, nav_y - 12.0, rot_x + 36.0, nav_y - 12.0 + btn_h);
                if rot_rect.contains(e.pos) {
                    self.state.pdf_rotate();
                    ctx.request_paint();
                    return;
                }

                let tool_x = rot_x + 44.0 + spacing;
                let tools = [
                    state::PdfAnnotationTool::Highlight,
                    state::PdfAnnotationTool::Underline,
                    state::PdfAnnotationTool::Strikeout,
                    state::PdfAnnotationTool::StickyNote,
                ];
                for (i, tool) in tools.iter().enumerate() {
                    let tx = tool_x + (i as f64) * (24.0 + 4.0);
                    let tool_rect = Rect::new(tx, nav_y - 12.0, tx + 24.0, nav_y - 12.0 + btn_h);
                    if tool_rect.contains(e.pos) {
                        if self.state.pdf_annotation_tool == *tool {
                            self.state
                                .set_pdf_annotation_tool(state::PdfAnnotationTool::None);
                        } else {
                            self.state.set_pdf_annotation_tool(*tool);
                        }
                        ctx.request_paint();
                        return;
                    }
                }

                // Annotation list toggle
                let ann_list_x = tool_x + tools.len() as f64 * (24.0 + 4.0) + spacing;
                let ann_list_rect = Rect::new(
                    ann_list_x,
                    nav_y - 12.0,
                    ann_list_x + 32.0,
                    nav_y - 12.0 + btn_h,
                );
                if ann_list_rect.contains(e.pos) {
                    self.state.toggle_annotation_list();
                    ctx.request_paint();
                    return;
                }
            }

            // Annotation list row click (when visible in PDF mode)
            if self.state.pdf_show_annotation_list
                && !self.state.pdf_annotations.is_empty()
                && e.pos.x >= center_x
                && e.pos.x <= center_right
            {
                let abstract_lines = paper_list::wrap_text(&paper.abstract_text, 80);
                let search_y = header_h
                    + spacing_large
                    + 28.0
                    + 24.0
                    + 24.0
                    + 24.0
                    + 20.0
                    + 18.0 * abstract_lines.len() as f64
                    + spacing_large;
                let fig_y = search_y + 28.0;
                let nav_y = fig_y + 196.0;
                let mut ann_y = nav_y + 14.0 + spacing_large + 20.0;
                for ann in &self.state.pdf_annotations {
                    let row_rect = Rect::new(
                        center_x + spacing_large,
                        ann_y - 10.0,
                        center_right - spacing_large,
                        ann_y + 8.0,
                    );
                    if row_rect.contains(e.pos) {
                        self.state.select_pdf_annotation(Some(ann.id.clone()));
                        ctx.request_paint();
                        return;
                    }
                    ann_y += 22.0;
                }
            }
        }

        let center_right = regions.center.x1;
        if e.pos.x >= center_right && e.pos.y >= header_h + spacing_large - 10.0 {
            if let Some(tab) =
                inspector::hit_test_tab(e.pos.x, e.pos.y, center_right, header_h, spacing)
            {
                self.state.active_inspector_tab = tab;
                ctx.request_paint();
                return;
            }

            // Citation tab interactive elements (tab 5)
            if self.state.active_inspector_tab == 5 {
                let rx = center_right;
                let mut fmt_y = header_h + spacing_large + 24.0 + 20.0;

                // Format buttons
                let formats = [
                    state::CitationExportFormat::BibTex,
                    state::CitationExportFormat::Ris,
                    state::CitationExportFormat::Apa,
                    state::CitationExportFormat::Chicago,
                    state::CitationExportFormat::Mla,
                ];
                let mut fmt_x = rx + spacing;
                for fmt in &formats {
                    let label = fmt.label();
                    let btn_w = label.len() as f64 * 7.0 + 12.0;
                    let btn_rect = Rect::new(fmt_x, fmt_y - 8.0, fmt_x + btn_w, fmt_y + 6.0);
                    if btn_rect.contains(e.pos) {
                        self.state.set_citation_export_format(*fmt);
                        ctx.request_paint();
                        return;
                    }
                    fmt_x += btn_w + 4.0;
                }

                // DOI input field click
                if let Some(paper) = self.state.selected_paper() {
                    let ref_count = paper.references.len();
                    fmt_y += 24.0 + ref_count as f64 * 18.0 + spacing + 16.0;
                }
                let fetch_x = rx + regions.right.width() - spacing - 44.0;
                let doi_rect = Rect::new(rx + spacing, fmt_y - 8.0, fetch_x - 4.0, fmt_y + 8.0);
                if doi_rect.contains(e.pos) {
                    self.state.set_focus(state::FocusTarget::DoiInput);
                    ctx.request_paint();
                    return;
                }

                // Fetch button
                let fetch_rect = Rect::new(fetch_x, fmt_y - 8.0, fetch_x + 44.0, fmt_y + 8.0);
                if fetch_rect.contains(e.pos) {
                    self.state.fetch_doi_metadata();
                    ctx.request_paint();
                    return;
                }

                // Import BibTeX button
                fmt_y += 24.0;
                let import_rect =
                    Rect::new(rx + spacing, fmt_y - 8.0, rx + spacing + 100.0, fmt_y + 8.0);
                if import_rect.contains(e.pos) {
                    self.state.import_bibtex();
                    ctx.request_paint();
                    return;
                }
            }

            // Write tab interactive elements (tab 4)
            if self.state.active_inspector_tab == 4 {
                let rx = center_right;
                let right_w = regions.right.width();
                let mut write_ry = header_h + spacing_large + 24.0;

                // Skip summary lines
                write_ry += self.state.manuscript_summary_lines.len() as f64 * 16.0 + 6.0;

                // Section list clicks
                for (si, _section) in self.state.manuscript_sections.iter().enumerate() {
                    let section_rect = Rect::new(
                        rx + spacing,
                        write_ry - 2.0,
                        rx + spacing + right_w - spacing * 2.0,
                        write_ry + 18.0,
                    );
                    if section_rect.contains(e.pos) {
                        self.state.manuscript_active_section = Some(si);
                        ctx.request_paint();
                        return;
                    }
                    write_ry += 20.0;
                    if self.state.manuscript_active_section == Some(si)
                        && !_section.content.is_empty()
                    {
                        write_ry += 16.0;
                    }
                }

                // Add section button
                write_ry += 4.0;
                let add_btn_rect =
                    Rect::new(rx + spacing, write_ry, rx + spacing + 80.0, write_ry + 24.0);
                if add_btn_rect.contains(e.pos) {
                    let title = format!("Section {}", self.state.manuscript_sections.len() + 1);
                    self.state.add_manuscript_section(title);
                    ctx.request_paint();
                    return;
                }
                write_ry += 32.0;

                // Cite search input click
                if self.state.manuscript_active_section.is_some() {
                    let cite_search_rect = Rect::new(
                        rx + spacing,
                        write_ry,
                        rx + spacing + right_w - spacing * 2.0,
                        write_ry + 24.0,
                    );
                    if cite_search_rect.contains(e.pos) {
                        self.state
                            .set_focus(state::FocusTarget::ManuscriptCiteSearch);
                        ctx.request_paint();
                        return;
                    }
                    write_ry += 30.0;

                    // Cite result insert buttons
                    let cite_results = self.state.filtered_cite_results();
                    for (paper_idx, _paper_title) in &cite_results {
                        let insert_x = rx + spacing + right_w - spacing - 48.0;
                        let insert_rect =
                            Rect::new(insert_x, write_ry, insert_x + 44.0, write_ry + 20.0);
                        if insert_rect.contains(e.pos) {
                            if let Some(section_idx) = self.state.manuscript_active_section {
                                let cite_key = format!("paper-{}", paper_idx);
                                self.state
                                    .insert_citation_into_section(section_idx, &cite_key);
                            }
                            ctx.request_paint();
                            return;
                        }
                        write_ry += 24.0;
                    }
                }
            }

            // Q&A tab interactive elements
            if self.state.active_inspector_tab == 2 {
                let rx = center_right;
                let input_w = regions.right.width() - 80.0;
                // Calculate ry for Q&A input area (approximate)
                let mut qa_ry = header_h + spacing_large + 24.0;
                for (_role, message) in &self.state.analysis_messages {
                    let lines = paper_list::wrap_text(message, 40);
                    qa_ry += 14.0 * lines.len() as f64 + 20.0;
                }
                qa_ry += spacing + spacing;

                // Q&A input field click
                let input_rect = Rect::new(
                    rx + spacing,
                    qa_ry - 10.0,
                    rx + spacing + input_w,
                    qa_ry - 10.0 + 32.0,
                );
                if input_rect.contains(e.pos) {
                    self.state.set_focus(state::FocusTarget::QaInput);
                    ctx.request_paint();
                    return;
                }

                // Send button click
                let send_x = rx + spacing + input_w + 4.0;
                let send_rect = Rect::new(send_x, qa_ry - 10.0, send_x + 36.0, qa_ry - 10.0 + 32.0);
                if send_rect.contains(e.pos) {
                    self.state.send_qa_message();
                    ctx.request_paint();
                    return;
                }

                // Quick action buttons
                qa_ry += 32.0 + spacing;
                let quick_actions = ["Summarize", "Key Points", "Limitations"];
                let mut qa_x = rx + spacing;
                for action in &quick_actions {
                    let action_w = 72.0;
                    let action_rect = Rect::new(qa_x, qa_ry - 8.0, qa_x + action_w, qa_ry + 6.0);
                    if action_rect.contains(e.pos) {
                        self.state.qa_input = action.to_string();
                        self.state.send_qa_message();
                        ctx.request_paint();
                        return;
                    }
                    qa_x += action_w + 4.0;
                }
            }
        }
    }
}
