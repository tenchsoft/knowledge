mod accessibility;
mod authoring;
mod builders;
mod dashboard;
mod defaults;
mod learning;
mod notes;
mod profile;
mod progress;
mod session;
#[cfg(test)]
mod tests;
mod tutor;
mod types;
mod visual;

pub use types::*;

use tench_ui::prelude::Color;

pub const NEUTRAL_900: Color = Color::rgb8(0x0F, 0x0F, 0x0F);
pub const NEUTRAL_800: Color = Color::rgb8(0x1A, 0x1A, 0x1A);
pub const NEUTRAL_700: Color = Color::rgb8(0x2A, 0x2A, 0x2A);
pub const NEUTRAL_600: Color = Color::rgb8(0x3A, 0x3A, 0x3A);
pub const NEUTRAL_500: Color = Color::rgb8(0x4A, 0x4A, 0x4A);
pub const NEUTRAL_400: Color = Color::rgb8(0x6A, 0x6A, 0x6A);
pub const NEUTRAL_300: Color = Color::rgb8(0x8A, 0x8A, 0x8A);
pub const NEUTRAL_100: Color = Color::rgb8(0xD4, 0xD4, 0xD4);
pub const ACCENT_STUDY: Color = Color::rgb8(0x34, 0xD3, 0x99);
pub const ACCENT_EDITOR: Color = Color::rgb8(0xA7, 0x8B, 0xFA);
pub const STATUS_READY: Color = Color::rgb8(0x22, 0xC5, 0x5E);
pub const STATUS_RUNNING: Color = Color::rgb8(0x3B, 0x82, 0xF6);
pub const STATUS_WARNING: Color = Color::rgb8(0xF5, 0x9E, 0x0B);
pub const STATUS_ERROR: Color = Color::rgb8(0xEF, 0x44, 0x44);

/// Returns a high-contrast foreground color when high_contrast_mode is enabled.
pub fn fg_color(high_contrast: bool) -> Color {
    if high_contrast {
        Color::WHITE
    } else {
        NEUTRAL_100
    }
}

/// Returns a high-contrast background color when high_contrast_mode is enabled.
pub fn bg_color(high_contrast: bool) -> Color {
    if high_contrast {
        NEUTRAL_900
    } else {
        NEUTRAL_800
    }
}

/// Returns a high-contrast accent color.
pub fn accent_color(high_contrast: bool) -> Color {
    if high_contrast {
        Color::rgb8(0x00, 0xFF, 0xAA)
    } else {
        ACCENT_STUDY
    }
}
