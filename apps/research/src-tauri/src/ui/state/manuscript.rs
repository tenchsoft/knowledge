use super::*;

// ── Manuscript helpers ─────────────────────────────────────────────

impl ResearchState {
    pub fn add_manuscript_section(&mut self, title: String) {
        let id = format!("section-{}", self.manuscript_sections.len() + 1);
        self.manuscript_sections.push(ManuscriptSection {
            id,
            title,
            content: String::new(),
            citations: Vec::new(),
        });
        self.manuscript_active_section = Some(self.manuscript_sections.len() - 1);
    }

    pub fn remove_manuscript_section(&mut self, index: usize) {
        if index < self.manuscript_sections.len() {
            self.manuscript_sections.remove(index);
            self.manuscript_active_section = if self.manuscript_sections.is_empty() {
                None
            } else {
                Some(self.manuscript_active_section.map_or(0, |cur| {
                    if cur >= self.manuscript_sections.len() {
                        self.manuscript_sections.len() - 1
                    } else {
                        cur
                    }
                }))
            };
        }
    }

    pub fn set_manuscript_section_content(&mut self, index: usize, content: String) {
        if let Some(section) = self.manuscript_sections.get_mut(index) {
            section.content = content;
        }
    }

    pub fn insert_citation_into_section(&mut self, section_index: usize, citation_key: &str) {
        if let Some(section) = self.manuscript_sections.get_mut(section_index) {
            if !section.citations.contains(&citation_key.to_string()) {
                section.citations.push(citation_key.to_string());
            }
            // Insert citation marker at end of content
            if !section.content.is_empty() {
                section.content.push(' ');
            }
            section.content.push_str(&format!("[{}]", citation_key));
        }
    }

    pub fn filtered_cite_results(&self) -> Vec<(usize, String)> {
        if self.manuscript_cite_search.is_empty() {
            return Vec::new();
        }
        let query = self.manuscript_cite_search.to_lowercase();
        self.papers
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                p.title.to_lowercase().contains(&query)
                    || p.authors.to_lowercase().contains(&query)
                    || p.year.to_string().contains(&query)
            })
            .map(|(i, p)| (i, p.title.clone()))
            .take(10)
            .collect()
    }
}
