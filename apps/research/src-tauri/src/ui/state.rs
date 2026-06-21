mod builders;
mod init;
mod library;
mod manuscript;
mod pdf;
#[cfg(test)]
mod tests;
mod types;

pub use types::*;

use tench_research_core::ReadingStatus;

pub fn status_label(status: &ReadingStatus) -> &'static str {
    match status {
        ReadingStatus::Unread => "To Read",
        ReadingStatus::Reading => "Reading",
        ReadingStatus::Reviewed => "Read",
        ReadingStatus::Archived => "Archived",
    }
}
