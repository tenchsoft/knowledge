use tench_research_core::ReadingStatus;
use tench_ui::prelude::Color;

pub fn status_color(status: &ReadingStatus) -> Color {
    match status {
        ReadingStatus::Reviewed => Color::rgb8(0x22, 0xC5, 0x5E),
        ReadingStatus::Reading => Color::rgb8(0xF5, 0x9E, 0x0B),
        _ => Color::rgb8(0x60, 0xA5, 0xFA),
    }
}

pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > max_chars && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
