//! Tench Study native UI.
//!
//! Mirrors the React `StudyExperience`: header with breadcrumb/session
//! progress, curriculum outline, learn/practice/review surfaces, tutor panel,
//! session result modal, and stats modal.

mod automation;
pub mod curriculum;
pub mod learn;
pub mod practice;
pub mod state;
pub mod tutor;
mod widget;

use tench_ui::prelude::*;

use state::StudyState;

pub struct StudyApp {
    state: StudyState,
    size: Size,
    i18n: tench_app_core::I18nCatalog,
}

impl Default for StudyApp {
    fn default() -> Self {
        Self::new()
    }
}

impl StudyApp {
    pub fn new() -> Self {
        Self {
            state: StudyState::default(),
            size: Size::ZERO,
            i18n: crate::i18n::study_i18n_catalog(crate::i18n::DEFAULT_LOCALE),
        }
    }
}
