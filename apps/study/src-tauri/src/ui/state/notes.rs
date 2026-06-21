use super::*;

impl StudyState {
    pub fn toggle_unit_expand(&mut self, unit_idx: usize) {
        if unit_idx < self.expanded_units.len() {
            self.expanded_units[unit_idx] = !self.expanded_units[unit_idx];
        } else {
            self.expanded_units.resize(unit_idx + 1, true);
            self.expanded_units[unit_idx] = false;
        }
    }

    pub fn toggle_bookmark(&mut self) {
        let concept_id = self.active_concept().id.clone();
        if !self.bookmarked_concept_ids.remove(&concept_id) {
            self.bookmarked_concept_ids.insert(concept_id);
        }
    }

    pub fn toggle_notes_panel(&mut self) {
        self.show_notes_panel = !self.show_notes_panel;
    }

    pub fn save_note(&mut self) {
        if self.notes_input.trim().is_empty() {
            return;
        }
        let note = StudyNote {
            id: format!("note-{}", self.notes.len() + 1),
            concept_id: self.active_concept().id.clone(),
            text: self.notes_input.clone(),
            created_at: format!("{}s", self.elapsed_seconds),
        };
        self.notes.push(note);
        self.notes_input.clear();
    }

    pub fn search_matches(&self, query: &str) -> Vec<(usize, usize)> {
        if query.is_empty() {
            return Vec::new();
        }
        let lower = query.to_lowercase();
        let mut matches = Vec::new();
        for (unit_idx, unit) in self.units.iter().enumerate() {
            if unit.label.to_lowercase().contains(&lower) {
                for (concept_idx, _) in unit.concepts.iter().enumerate() {
                    matches.push((unit_idx, concept_idx));
                }
                continue;
            }
            for (concept_idx, concept) in unit.concepts.iter().enumerate() {
                if concept.label.to_lowercase().contains(&lower) {
                    matches.push((unit_idx, concept_idx));
                }
            }
        }
        matches
    }
}
