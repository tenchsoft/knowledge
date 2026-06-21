use super::*;

impl StudyState {
    pub fn send_tutor_chat(&mut self) {
        if self.tutor_chat_input.trim().is_empty() {
            return;
        }
        let user_msg = TutorChatMessage {
            role: TutorChatRole::User,
            text: self.tutor_chat_input.clone(),
        };
        self.tutor_chat_messages.push(user_msg);
        // Placeholder response - in production this connects to Engine
        let response = TutorChatMessage {
            role: TutorChatRole::Assistant,
            text: format!(
                "Let me think about '{}' in the context of {}...",
                self.tutor_chat_input,
                self.active_concept().label
            ),
        };
        self.tutor_chat_messages.push(response);
        self.tutor_chat_input.clear();
    }

    pub fn toggle_glossary_expand(&mut self, idx: usize) {
        self.expanded_glossary_idx = if self.expanded_glossary_idx == Some(idx) {
            None
        } else {
            Some(idx)
        };
    }
}
