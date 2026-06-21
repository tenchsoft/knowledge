use super::*;

impl StudyState {
    pub fn toggle_authoring_panel(&mut self) {
        self.show_authoring_panel = !self.show_authoring_panel;
    }

    pub fn create_new_curriculum(&mut self) {
        self.authoring_title.clear();
        self.authoring_body.clear();
        self.show_authoring_panel = true;
    }

    pub fn create_new_unit(&mut self) {
        self.authoring_title.clear();
        self.authoring_body.clear();
    }

    pub fn create_new_concept(&mut self) {
        self.authoring_title.clear();
        self.authoring_body.clear();
    }

    pub fn save_authoring_draft(&mut self) {
        // In production this would call commands/authoring.rs
        self.show_authoring_panel = false;
    }

    pub fn save_authoring_problem(&mut self) {
        // Phase 9: Save problem from authoring fields
        if self.authoring_problem_text.is_empty() {
            return;
        }
        let concept_id = self.active_concept().id.clone();
        self.problems.push(Problem {
            concept_id,
            text: self.authoring_problem_text.clone(),
            matrices: String::new(),
            answer_key: tench_study_core::AnswerKey::Exact {
                value: self.authoring_problem_answer.clone(),
                case_sensitive: false,
            },
            answer: self.authoring_problem_answer.clone(),
            solution: String::new(),
            cause_tag: "authored".to_string(),
            related_concept: self.active_concept().label.clone(),
        });
        self.authoring_problem_text.clear();
        self.authoring_problem_answer.clear();
    }
}
