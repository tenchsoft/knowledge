use super::*;

impl StudyState {
    pub fn auto_save_session(&self) -> SessionSnapshot {
        SessionSnapshot {
            stage: self.stage,
            active_unit_idx: self.active_unit_idx,
            active_concept_idx: self.active_concept_idx,
            problem_index: self.problem_index,
            input_text: self.input_text.clone(),
            input_cursor_pos: self.input_cursor_pos,
            streak: self.streak,
            elapsed_seconds: self.elapsed_seconds,
            session_results: self.session_results.clone(),
        }
    }

    pub fn restore_session(&mut self, snapshot: SessionSnapshot) {
        self.stage = snapshot.stage;
        self.active_unit_idx = snapshot.active_unit_idx;
        self.active_concept_idx = snapshot.active_concept_idx;
        self.problem_index = snapshot.problem_index;
        self.input_text = snapshot.input_text;
        self.input_cursor_pos = snapshot.input_cursor_pos;
        self.streak = snapshot.streak;
        self.elapsed_seconds = snapshot.elapsed_seconds;
        self.session_results = snapshot.session_results;
        self.feedback = None;
    }
}
