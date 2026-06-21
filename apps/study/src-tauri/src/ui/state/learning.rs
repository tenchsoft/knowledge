use super::*;

impl StudyState {
    pub fn active_concept(&self) -> &Concept {
        &self.units[self.active_unit_idx].concepts[self.active_concept_idx]
    }

    pub fn active_unit(&self) -> &Unit {
        &self.units[self.active_unit_idx]
    }

    pub fn active_glossary_terms(&self) -> &[GlossaryPreview] {
        &self.active_concept().glossary_terms
    }

    pub fn concept_problems(&self) -> Vec<&Problem> {
        self.problems
            .iter()
            .filter(|problem| problem.concept_id == self.active_concept().id)
            .collect()
    }

    pub fn current_problem(&self) -> Option<&Problem> {
        self.concept_problems()
            .get(self.problem_index.saturating_sub(1))
            .copied()
    }

    pub fn current_review(&self) -> Option<&ReviewItem> {
        self.review_queue.get(self.review_index.saturating_sub(1))
    }

    pub fn total_problems(&self) -> usize {
        self.concept_problems().len()
    }

    pub fn select_concept(&mut self, unit: usize, concept: usize) {
        if unit >= self.units.len() || concept >= self.units[unit].concepts.len() {
            return;
        }
        self.active_unit_idx = unit;
        self.active_concept_idx = concept;
        self.active_subject = self.units[unit].label.clone();
        self.selection.domain = self.units[unit].domain.clone();
        self.selection.level = self.units[unit].concepts[concept].level;
        self.stage = Stage::Learn;
        self.problem_index = 1;
        self.feedback = None;
        self.session_results.clear();
        self.input_text.clear();
        self.input_cursor_pos = 0;
        self.hint_level = 0;
        self.pending_rating = None;
        self.refresh_daily_dashboard();
    }

    pub fn move_concept(&mut self, delta: isize) {
        let positions = self
            .units
            .iter()
            .enumerate()
            .flat_map(|(unit_idx, unit)| {
                (0..unit.concepts.len()).map(move |concept_idx| (unit_idx, concept_idx))
            })
            .collect::<Vec<_>>();
        if positions.is_empty() {
            return;
        }
        let current = positions
            .iter()
            .position(|(unit, concept)| {
                *unit == self.active_unit_idx && *concept == self.active_concept_idx
            })
            .unwrap_or(0);
        let next = (current as isize + delta).clamp(0, positions.len() as isize - 1) as usize;
        let (unit, concept) = positions[next];
        self.select_concept(unit, concept);
    }

    pub fn cycle_stage(&mut self, reverse: bool) {
        self.stage = match (self.stage, reverse) {
            (Stage::Learn, false) => {
                if self.total_problems() > 0 {
                    Stage::Practice
                } else if self.review_queue.is_empty() {
                    Stage::Learn
                } else {
                    Stage::Review
                }
            }
            (Stage::Practice, false) => {
                if self.review_queue.is_empty() {
                    Stage::Learn
                } else {
                    Stage::Review
                }
            }
            (Stage::Review, false) => Stage::Learn,
            (Stage::Learn, true) => {
                if self.review_queue.is_empty() {
                    Stage::Practice
                } else {
                    Stage::Review
                }
            }
            (Stage::Practice, true) => Stage::Learn,
            (Stage::Review, true) => Stage::Practice,
        };
        if self.stage == Stage::Practice && self.total_problems() == 0 {
            self.stage = Stage::Learn;
        }
        if self.stage == Stage::Review && self.review_queue.is_empty() {
            self.stage = Stage::Learn;
        }
        self.feedback = None;
        self.input_text.clear();
        self.input_cursor_pos = 0;
    }

    pub fn activate_primary_keyboard_action(&mut self) {
        match self.stage {
            Stage::Learn => self.start_practice(),
            Stage::Practice => {
                if self.feedback.is_some() {
                    self.next_problem();
                } else {
                    self.submit_answer();
                }
            }
            Stage::Review => {
                self.stage = Stage::Learn;
            }
        }
    }

    pub fn start_practice(&mut self) {
        if self.total_problems() == 0 {
            return;
        }
        self.stage = Stage::Practice;
        self.problem_index = 1;
        self.feedback = None;
        self.session_results.clear();
        self.input_text.clear();
        self.input_cursor_pos = 0;
        self.session_paused = false;
        self.pending_rating = None;
    }

    pub fn submit_answer(&mut self) {
        if self.stage != Stage::Practice
            || self.input_text.trim().is_empty()
            || self.feedback.is_some()
        {
            return;
        }
        let Some(problem) = self.current_problem() else {
            return;
        };
        let grading = tench_study_core::grade_answer(&problem.answer_key, &self.input_text);
        let correct = grading.correct;
        let review_item = (!correct).then(|| ReviewItem {
            problem_text: problem.text.clone(),
            wrong_answer: self.input_text.trim().into(),
            correct_answer: problem.answer.clone(),
            cause_tag: problem.cause_tag.clone(),
            related_concept: problem.related_concept.clone(),
            solution: problem.solution.clone(),
            spaced_repetition: None,
        });
        self.feedback = Some(correct);
        self.session_results.push(correct);
        if let Some(review_item) = review_item {
            self.review_queue.push(review_item);
        }
        self.refresh_daily_dashboard();
    }

    pub fn next_problem(&mut self) {
        if self.problem_index < self.total_problems() {
            self.problem_index += 1;
            self.feedback = None;
            self.input_text.clear();
            self.input_cursor_pos = 0;
            self.hint_level = 0;
            self.pending_rating = None;
        } else {
            if !self.session_results.is_empty()
                && self.session_results.iter().all(|correct| *correct)
            {
                self.streak += 1;
            }
            self.show_result_modal = true;
        }
        self.refresh_daily_dashboard();
    }

    pub fn retry_answer(&mut self) {
        self.feedback = None;
        self.input_text.clear();
        self.input_cursor_pos = 0;
        self.pending_rating = None;
    }

    pub fn skip_problem(&mut self) {
        if self.stage != Stage::Practice {
            return;
        }
        // Add to review queue as a wrong answer
        if let Some(problem) = self.current_problem() {
            self.review_queue.push(ReviewItem {
                problem_text: problem.text.clone(),
                wrong_answer: "(skipped)".to_string(),
                correct_answer: problem.answer.clone(),
                cause_tag: problem.cause_tag.clone(),
                related_concept: problem.related_concept.clone(),
                solution: problem.solution.clone(),
                spaced_repetition: None,
            });
        }
        self.next_problem();
    }

    pub fn toggle_pause(&mut self) {
        self.session_paused = !self.session_paused;
    }

    pub fn toggle_math_palette(&mut self) {
        self.show_math_palette = !self.show_math_palette;
    }

    pub fn insert_math_symbol(&mut self, symbol: &str) {
        if self.stage != Stage::Practice {
            return;
        }
        self.input_text.insert_str(self.input_cursor_pos, symbol);
        self.input_cursor_pos += symbol.len();
    }

    pub fn move_cursor(&mut self, delta: isize) {
        if delta < 0 {
            self.input_cursor_pos = self.input_cursor_pos.saturating_sub((-delta) as usize);
        } else {
            self.input_cursor_pos =
                (self.input_cursor_pos + delta as usize).min(self.input_text.len());
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor_pos = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor_pos = self.input_text.len();
    }

    pub fn backspace_at_cursor(&mut self) {
        if self.input_cursor_pos > 0 {
            self.input_cursor_pos -= 1;
            self.input_text.remove(self.input_cursor_pos);
        }
    }

    pub fn delete_at_cursor(&mut self) {
        if self.input_cursor_pos < self.input_text.len() {
            self.input_text.remove(self.input_cursor_pos);
        }
    }

    pub fn insert_char_at_cursor(&mut self, ch: &str) {
        self.input_text.insert_str(self.input_cursor_pos, ch);
        self.input_cursor_pos += ch.len();
    }

    pub fn open_review_queue(&mut self) {
        self.stage = Stage::Review;
        self.review_index = 1;
        self.feedback = None;
    }

    pub fn open_stats(&mut self) {
        self.show_stats_modal = true;
    }

    pub fn reveal_hint(&mut self, level: u8) {
        self.hint_level = self.hint_level.max(level.min(3));
    }

    pub fn close_modals(&mut self) {
        self.show_result_modal = false;
        self.show_stats_modal = false;
        self.show_shortcut_help = false;
        self.show_goal_modal = false;
        self.show_hamburger_menu = false;
        if self.stage == Stage::Practice && self.problem_index >= self.total_problems().max(1) {
            self.stage = Stage::Learn;
        }
    }
}
