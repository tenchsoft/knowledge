use super::checks::run_non_ai_writing_checks;
use super::citations::refresh_manuscript_bibliography;
use super::export::{
    manuscript_export_file_name, render_manuscript_archive_bytes,
    render_manuscript_archive_manifest, render_manuscript_docx_bytes, render_manuscript_html,
    render_manuscript_latex, render_manuscript_markdown, render_manuscript_pdf_bytes,
};
use super::*;
use crate::{ReferenceItem, Timestamp};
use std::collections::BTreeSet;

pub fn export_manuscript(
    manuscript: ResearchManuscript,
    references: &[ReferenceItem],
    format: WritingExportFormat,
    now: Timestamp,
) -> Result<ManuscriptExport, String> {
    let manuscript = refresh_manuscript_bibliography(manuscript, references, now);
    let diagnostics = run_non_ai_writing_checks(&manuscript);
    if let Some(blocker) = diagnostics.iter().find(|result| result.export_blocker) {
        return Err(blocker.message.clone());
    }
    if !manuscript.target.export_formats.contains(&format) {
        return Err(format!(
            "{} target does not allow {:?} export",
            manuscript.target.name, format
        ));
    }

    let (body, body_bytes, media_type) = match format {
        WritingExportFormat::Markdown => {
            let body = render_manuscript_markdown(&manuscript);
            (
                body.clone(),
                body.into_bytes(),
                "text/markdown; charset=utf-8".to_string(),
            )
        }
        WritingExportFormat::Docx => {
            let body = render_manuscript_markdown(&manuscript);
            let body_bytes = render_manuscript_docx_bytes(&manuscript)?;
            (
                body,
                body_bytes,
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    .to_string(),
            )
        }
        WritingExportFormat::Pdf => {
            let body = render_manuscript_markdown(&manuscript);
            let body_bytes = render_manuscript_pdf_bytes(&manuscript);
            (body, body_bytes, "application/pdf".to_string())
        }
        WritingExportFormat::Html => {
            let body = render_manuscript_html(&manuscript);
            (
                body.clone(),
                body.into_bytes(),
                "text/html; charset=utf-8".to_string(),
            )
        }
        WritingExportFormat::Latex => {
            let body = render_manuscript_latex(&manuscript);
            (
                body.clone(),
                body.into_bytes(),
                "application/x-tex; charset=utf-8".to_string(),
            )
        }
        WritingExportFormat::Archive => {
            let body = render_manuscript_archive_manifest(&manuscript);
            let body_bytes = render_manuscript_archive_bytes(&manuscript, references)?;
            (body.clone(), body_bytes, "application/zip".to_string())
        }
    };

    Ok(ManuscriptExport {
        format,
        file_name: manuscript_export_file_name(&manuscript, format),
        body,
        body_bytes,
        media_type,
        bibliography: manuscript.citation_state.bibliography,
        diagnostics,
    })
}

pub fn create_manuscript_snapshot(
    manuscript: &ResearchManuscript,
    id: SnapshotId,
    now: Timestamp,
) -> ManuscriptSnapshot {
    ManuscriptSnapshot {
        id,
        manuscript_id: manuscript.id.clone(),
        title: manuscript.title.clone(),
        body_plain_text: manuscript.document.to_plain_text(),
        bibliography: manuscript.citation_state.bibliography.clone(),
        created_at: now,
    }
}

pub fn compare_manuscript_snapshots(
    before: &ManuscriptSnapshot,
    after: &ManuscriptSnapshot,
) -> ManuscriptDiff {
    let before_words = before.body_plain_text.split_whitespace().count() as i64;
    let after_words = after.body_plain_text.split_whitespace().count() as i64;
    let before_lines = before
        .body_plain_text
        .lines()
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    let after_lines = after
        .body_plain_text
        .lines()
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();

    ManuscriptDiff {
        before_id: before.id.clone(),
        after_id: after.id.clone(),
        word_delta: after_words - before_words,
        added_lines: after_lines.difference(&before_lines).cloned().collect(),
        removed_lines: before_lines.difference(&after_lines).cloned().collect(),
        bibliography_changed: before.bibliography != after.bibliography,
    }
}
