mod authoring;
mod goals;
mod layout;
mod panel;
mod profile;
mod session;
mod shortcuts;

pub use authoring::{hit_test_authoring, paint_authoring_panel};
pub use goals::paint_goal_modal;
pub use layout::modal_close_rect;
pub(crate) use layout::modal_rect;
pub use panel::{hint_rect, paint_tutor_panel};
pub use profile::{hit_test_profile_setup, paint_profile_setup_wizard};
pub use session::paint_modals;
pub use shortcuts::paint_shortcut_help_modal;
