use super::*;

impl StudyState {
    pub fn toggle_shortcut_help(&mut self) {
        self.show_shortcut_help = !self.show_shortcut_help;
    }

    pub fn toggle_high_contrast(&mut self) {
        self.high_contrast_mode = !self.high_contrast_mode;
    }

    pub fn toggle_hamburger_menu(&mut self) {
        self.show_hamburger_menu = !self.show_hamburger_menu;
    }

    /// Update the focus indicator rectangle based on the current focus target.
    /// Called from the widget paint cycle.
    pub fn update_focus_indicator(&mut self) {
        // Focus indicator is set during paint when focus_target is active
        self.focus_indicator = if self.focus_target != StudyFocusTarget::None {
            // A non-None focus target means we should show the indicator.
            // The exact rect is computed in paint() based on regions.
            None
        } else {
            None
        };
    }

    /// Set the focus target and immediately mark that a focus indicator
    /// should be shown. The actual rect is computed during paint.
    pub fn set_focus(&mut self, target: StudyFocusTarget) {
        self.focus_target = target;
    }
}
