use super::super::{state, ResearchApp};
use tench_ui::core::events::{LogicalKey, NamedKey};
use tench_ui::prelude::*;

impl ResearchApp {
    pub(crate) fn on_text_event_impl(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        if let TextEvent::Keyboard(kb) = event {
            if kb.is_pressed {
                let search_focused = self.state.focus == state::FocusTarget::SearchBox;
                let qa_focused = self.state.focus == state::FocusTarget::QaInput;
                let pdf_search_focused = self.state.focus == state::FocusTarget::PdfSearch;
                let doi_focused = self.state.focus == state::FocusTarget::DoiInput;
                let cite_search_focused =
                    self.state.focus == state::FocusTarget::ManuscriptCiteSearch;

                match &kb.logical_key {
                    // Ctrl+F: focus search (or PDF search in PDF mode)
                    LogicalKey::Character(c) if c == "f" && kb.modifiers.control => {
                        if self.state.reader_mode == state::ReaderMode::Pdf {
                            self.state.set_focus(state::FocusTarget::PdfSearch);
                        } else {
                            self.state.set_focus(state::FocusTarget::SearchBox);
                        }
                        ctx.request_paint();
                    }
                    // Alt+F: toggle favorites
                    LogicalKey::Character(c) if c == "f" && kb.modifiers.alt => {
                        self.state.toggle_favorites_only();
                        ctx.request_paint();
                    }
                    // Ctrl+Shift+C: cycle citation format
                    LogicalKey::Character(c)
                        if c == "c" && kb.modifiers.control && kb.modifiers.shift =>
                    {
                        self.state.cycle_citation_format();
                        ctx.request_paint();
                    }
                    // Ctrl+C: clipboard copy (placeholder)
                    LogicalKey::Character(c) if c == "c" && kb.modifiers.control => {
                        if let Some(paper) = self.state.selected_paper() {
                            self.state.add_toast(
                                format!("Copied: {}", paper.title),
                                state::ToastKind::Info,
                            );
                        }
                        ctx.request_paint();
                    }
                    // Ctrl+I: import
                    LogicalKey::Character(c) if c == "i" && kb.modifiers.control => {
                        self.state.queue_import();
                        self.state
                            .add_toast("Import started", state::ToastKind::Info);
                        ctx.request_paint();
                    }
                    // Ctrl+A: select all visible papers
                    LogicalKey::Character(c)
                        if c == "a"
                            && kb.modifiers.control
                            && !search_focused
                            && !qa_focused
                            && !pdf_search_focused
                            && !doi_focused
                            && !cite_search_focused =>
                    {
                        self.state.select_all_visible();
                        self.state
                            .add_toast("All papers selected", state::ToastKind::Info);
                        ctx.request_paint();
                    }
                    // Alt+P: toggle reader mode
                    LogicalKey::Character(c) if c == "p" && kb.modifiers.alt => {
                        self.state.toggle_reader_mode();
                        ctx.request_paint();
                    }
                    // ?: shortcut help (when not focused)
                    LogicalKey::Character(c)
                        if c == "?"
                            && !search_focused
                            && !qa_focused
                            && !pdf_search_focused
                            && !doi_focused
                            && !cite_search_focused =>
                    {
                        self.state.toggle_shortcut_help();
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Escape) => {
                        if self.state.show_shortcut_help {
                            self.state.show_shortcut_help = false;
                        } else if pdf_search_focused {
                            self.state.clear_pdf_search();
                            self.state.set_focus(state::FocusTarget::None);
                        } else if cite_search_focused {
                            self.state.manuscript_cite_search.clear();
                            self.state.set_focus(state::FocusTarget::None);
                        } else if qa_focused || doi_focused {
                            self.state.set_focus(state::FocusTarget::None);
                        } else {
                            self.state.set_focus(state::FocusTarget::None);
                            self.state.set_search_query("");
                            self.state.reader_mode = state::ReaderMode::Detail;
                        }
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::ArrowUp)
                        if !search_focused
                            && !qa_focused
                            && !doi_focused
                            && !cite_search_focused =>
                    {
                        self.state.move_selection(-1);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::ArrowDown)
                        if !search_focused
                            && !qa_focused
                            && !doi_focused
                            && !cite_search_focused =>
                    {
                        self.state.move_selection(1);
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Enter) if qa_focused => {
                        self.state.send_qa_message();
                        ctx.request_paint();
                    }
                    // Enter in DOI input: fetch metadata
                    LogicalKey::Named(NamedKey::Enter) if doi_focused => {
                        self.state.fetch_doi_metadata();
                        ctx.request_paint();
                    }
                    // Enter in PDF search: advance to next result
                    LogicalKey::Named(NamedKey::Enter) if pdf_search_focused => {
                        self.state.advance_pdf_search();
                        ctx.request_paint();
                    }
                    // PDF page navigation
                    LogicalKey::Named(NamedKey::PageUp) => {
                        self.state.pdf_prev_page();
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::PageDown) => {
                        self.state.pdf_next_page();
                        ctx.request_paint();
                    }
                    // +/- for zoom in PDF mode
                    LogicalKey::Character(c)
                        if c == "+" && self.state.reader_mode == state::ReaderMode::Pdf =>
                    {
                        self.state.pdf_zoom_in();
                        ctx.request_paint();
                    }
                    LogicalKey::Character(c)
                        if c == "-" && self.state.reader_mode == state::ReaderMode::Pdf =>
                    {
                        self.state.pdf_zoom_out();
                        ctx.request_paint();
                    }
                    LogicalKey::Named(NamedKey::Backspace) => {
                        if search_focused {
                            self.state.pop_search_text();
                        } else if qa_focused {
                            self.state.pop_qa_input();
                        } else if pdf_search_focused {
                            self.state.pop_pdf_search_text();
                        } else if doi_focused {
                            self.state.pop_doi_input();
                        } else if cite_search_focused {
                            self.state.manuscript_cite_search.pop();
                        }
                        ctx.request_paint();
                    }
                    // Letter keys: route to focused input or ignore
                    LogicalKey::Character(c) if !kb.modifiers.control && !kb.modifiers.alt => {
                        if search_focused {
                            self.state.push_search_text(c);
                        } else if qa_focused {
                            self.state.push_qa_input(c);
                        } else if pdf_search_focused {
                            self.state.push_pdf_search_text(c);
                        } else if doi_focused {
                            self.state.push_doi_input(c);
                        } else if cite_search_focused {
                            self.state
                                .manuscript_cite_search
                                .push(c.chars().next().unwrap_or(' '));
                        }
                        ctx.request_paint();
                    }
                    _ => {}
                }
            }
        }
    }
}
