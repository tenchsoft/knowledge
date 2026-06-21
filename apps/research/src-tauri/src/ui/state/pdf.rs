use super::*;

impl ResearchState {
    pub fn pdf_next_page(&mut self) {
        if self.pdf_page_count > 0 && self.pdf_current_page < self.pdf_page_count {
            self.pdf_current_page += 1;
        }
    }

    pub fn pdf_prev_page(&mut self) {
        if self.pdf_current_page > 1 {
            self.pdf_current_page -= 1;
        }
    }

    pub fn pdf_set_page(&mut self, page: u32) {
        if page >= 1 && (self.pdf_page_count == 0 || page <= self.pdf_page_count) {
            self.pdf_current_page = page;
        }
    }

    pub fn pdf_zoom_in(&mut self) {
        self.pdf_zoom = (self.pdf_zoom * 1.2).min(8.0);
    }

    pub fn pdf_zoom_out(&mut self) {
        self.pdf_zoom = (self.pdf_zoom / 1.2).max(0.1);
    }

    pub fn pdf_reset_zoom(&mut self) {
        self.pdf_zoom = 1.0;
    }

    pub fn pdf_rotate(&mut self) {
        self.pdf_rotation = (self.pdf_rotation + 90.0) % 360.0;
    }

    pub fn set_pdf_annotation_tool(&mut self, tool: PdfAnnotationTool) {
        self.pdf_annotation_tool = tool;
    }

    pub fn set_pdf_page_image_data(&mut self, data: Option<PdfPageImageData>) {
        self.pdf_page_image_data = data;
    }

    pub fn set_pdf_search_results(&mut self, results: Vec<PdfSearchResult>) {
        self.pdf_search_active_index = if results.is_empty() { None } else { Some(0) };
        self.pdf_search_results = results;
    }

    pub fn advance_pdf_search(&mut self) {
        if let Some(idx) = self.pdf_search_active_index {
            if !self.pdf_search_results.is_empty() {
                self.pdf_search_active_index = Some((idx + 1) % self.pdf_search_results.len());
            }
        }
    }

    pub fn toggle_annotation_list(&mut self) {
        self.pdf_show_annotation_list = !self.pdf_show_annotation_list;
    }

    pub fn push_pdf_search_text(&mut self, text: &str) {
        self.pdf_search_query.push_str(text);
    }

    pub fn pop_pdf_search_text(&mut self) {
        self.pdf_search_query.pop();
    }

    pub fn clear_pdf_search(&mut self) {
        self.pdf_search_query.clear();
        self.pdf_search_results.clear();
        self.pdf_search_active_index = None;
    }

    pub fn set_citation_export_format(&mut self, format: CitationExportFormat) {
        self.citation_export_format = format;
    }

    pub fn push_doi_input(&mut self, text: &str) {
        self.doi_input.push_str(text);
    }

    pub fn pop_doi_input(&mut self) {
        self.doi_input.pop();
    }

    pub fn clear_doi_input(&mut self) {
        self.doi_input.clear();
    }

    pub fn fetch_doi_metadata(&mut self) {
        let doi = self.doi_input.trim().to_string();
        if doi.is_empty() {
            return;
        }
        // TODO: connect to Tauri command for DOI/arXiv metadata fetch
        self.add_toast(format!("Fetching metadata for: {doi}"), ToastKind::Info);
    }

    pub fn import_bibtex(&mut self) {
        // TODO: connect to Tauri command for BibTeX import
        self.add_toast("BibTeX import queued", ToastKind::Info);
    }

    pub fn add_pdf_annotation(&mut self, annotation: PdfAnnotationState) {
        self.pdf_annotations.push(annotation);
    }

    pub fn remove_pdf_annotation(&mut self, id: &str) {
        self.pdf_annotations.retain(|a| a.id != id);
        if self.pdf_selected_annotation.as_deref() == Some(id) {
            self.pdf_selected_annotation = None;
        }
    }

    pub fn select_pdf_annotation(&mut self, id: Option<String>) {
        self.pdf_selected_annotation = id;
    }
}
