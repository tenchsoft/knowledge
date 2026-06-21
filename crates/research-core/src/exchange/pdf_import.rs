use std::fs;
use std::path::{Path, PathBuf};

use tench_fs_core::{file_entry_from_path, scan_folder, FileEntry, FolderScanOptions};
use tench_job_core::{JobProgress, JobState};

use crate::{
    Attachment, AttachmentId, AttachmentKind, AttachmentStoragePolicy, KeyValue, LibraryLayout,
    ReferenceId, ReferenceKind, ResearchSnapshotV2, Timestamp,
};

use super::{
    generate_citekey, reference_from_minimal_metadata, research_job_descriptor, DedupePolicy,
    DuplicateCandidate, DuplicateReason, ImportBatchId, ImportIssue, ImportIssueId,
    ImportIssueSeverity, ImportReport, PdfImportOptions, PdfImportOutcome, ResearchJobKind,
};

pub fn import_pdf_paths(
    mut snapshot: ResearchSnapshotV2,
    library_root: impl AsRef<Path>,
    paths: &[PathBuf],
    options: PdfImportOptions,
    batch_id: ImportBatchId,
    dedupe_policy: DedupePolicy,
    now: impl Into<String>,
) -> Result<PdfImportOutcome, String> {
    let now = Timestamp(now.into());
    let layout = LibraryLayout::for_root(library_root.as_ref());
    let storage_policy = options
        .storage_policy
        .unwrap_or(snapshot.library.settings.attachment_policy);
    let mut report = ImportReport {
        batch_id: batch_id.clone(),
        imported: Vec::new(),
        duplicates: Vec::new(),
        issues: Vec::new(),
        jobs: vec![research_job_descriptor(
            format!("{}-import", batch_id.as_str()),
            ResearchJobKind::Import,
            JobState::Running,
            batch_id.as_str(),
        )],
    };
    let entries = collect_pdf_import_entries(paths, &options, &batch_id, &mut report.issues);

    if entries.is_empty() {
        report.issues.push(import_issue(
            &batch_id,
            report.issues.len(),
            ImportIssueSeverity::Warning,
            "no_pdf_files",
            "No PDF files were found to import.",
            None,
            false,
        ));
    }

    for entry in entries {
        let source_path = PathBuf::from(&entry.path);
        let bytes = match fs::read(&source_path) {
            Ok(bytes) => bytes,
            Err(error) => {
                report.issues.push(import_issue(
                    &batch_id,
                    report.issues.len(),
                    ImportIssueSeverity::Error,
                    "pdf_read_failed",
                    format!("Failed to read PDF: {error}"),
                    Some(entry.path.clone()),
                    true,
                ));
                continue;
            }
        };
        let content_hash = stable_bytes_id(&bytes);
        let reference_id = unique_reference_id(&snapshot, &format!("pdf-{content_hash}"));

        if let Some(existing) = snapshot
            .attachments
            .iter()
            .find(|attachment| attachment.content_hash == content_hash)
        {
            report.duplicates.push(DuplicateCandidate {
                existing_id: existing.reference_id.clone(),
                imported_id: reference_id.clone(),
                reason: DuplicateReason::AttachmentHash,
                confidence: 100,
            });
            match dedupe_policy {
                DedupePolicy::PreferExisting
                | DedupePolicy::MergeMetadata
                | DedupePolicy::AskUser => {
                    report.issues.push(import_issue(
                        &batch_id,
                        report.issues.len(),
                        ImportIssueSeverity::Warning,
                        "duplicate_pdf_skipped",
                        "A PDF with the same content hash already exists in the library.",
                        Some(entry.path.clone()),
                        false,
                    ));
                    continue;
                }
                DedupePolicy::KeepBoth | DedupePolicy::PreferImported => {}
            }
        }

        let attachment_id = unique_attachment_id(&snapshot, &format!("att-{content_hash}"));
        let stored_path =
            match store_pdf_attachment(&layout, &entry, &attachment_id, storage_policy) {
                Ok(stored_path) => stored_path,
                Err(error) => {
                    report.issues.push(import_issue(
                        &batch_id,
                        report.issues.len(),
                        ImportIssueSeverity::Error,
                        "pdf_store_failed",
                        format!("Failed to store PDF: {error}"),
                        Some(entry.path.clone()),
                        true,
                    ));
                    continue;
                }
            };
        let mut reference = reference_from_minimal_metadata(
            reference_id.as_str(),
            ReferenceKind::Unknown,
            pdf_title_from_entry(&entry),
            None,
            now.0.clone(),
        );
        reference.language = options.locale.clone();
        reference.metadata.source_format = Some("pdf".to_string());
        reference.metadata.imported_from = Some(entry.path.clone());
        if let Some(locale) = &options.locale {
            reference.metadata.locale_hints.push(locale.clone());
        }
        reference.metadata.extra.push(KeyValue {
            key: "pdf_content_hash".to_string(),
            value: content_hash.clone(),
        });
        reference.citekey = Some(generate_citekey(&reference));
        reference.validate()?;

        let attachment = Attachment {
            id: attachment_id.clone(),
            reference_id: reference.id.clone(),
            kind: AttachmentKind::Pdf,
            title: entry.name.clone(),
            stored_path,
            original_path: Some(entry.path.clone()),
            mime_type: "application/pdf".to_string(),
            size_bytes: entry.size_bytes.unwrap_or(bytes.len() as u64),
            content_hash,
            page_count: None,
            text_indexed: false,
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        report.imported.push(reference.id.clone());
        report.jobs.push(research_job_descriptor(
            format!("{}-extract-text", attachment_id.as_str()),
            ResearchJobKind::ExtractPdfText,
            JobState::Queued,
            batch_id.as_str(),
        ));
        report.jobs.push(research_job_descriptor(
            format!("{}-render-page", attachment_id.as_str()),
            ResearchJobKind::RenderPdfPage,
            JobState::Queued,
            batch_id.as_str(),
        ));
        snapshot.references.push(reference);
        snapshot.attachments.push(attachment);
    }

    let has_blockers = report.has_blockers();
    if let Some(import_job) = report.jobs.first_mut() {
        import_job.state = if has_blockers {
            JobState::Failed
        } else {
            JobState::Completed
        };
        import_job.progress = Some(JobProgress {
            current: report.imported.len() as u64,
            total: Some(report.imported.len() as u64),
            message: Some(format!("Imported {} PDF files", report.imported.len())),
        });
    }
    snapshot.library.updated_at = now;

    Ok(PdfImportOutcome { snapshot, report })
}

fn stable_bytes_id(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn collect_pdf_import_entries(
    paths: &[PathBuf],
    options: &PdfImportOptions,
    batch_id: &ImportBatchId,
    issues: &mut Vec<ImportIssue>,
) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    for path in paths {
        if !options.include_hidden && is_path_hidden(path) {
            issues.push(import_issue(
                batch_id,
                issues.len(),
                ImportIssueSeverity::Info,
                "hidden_path_skipped",
                "Hidden path skipped by import options.",
                Some(path.to_string_lossy().to_string()),
                false,
            ));
            continue;
        }
        if path.is_dir() {
            let scan_options = FolderScanOptions {
                recursive: options.recursive,
                include_hidden: options.include_hidden,
                allowed_extensions: vec!["pdf".to_string()],
                max_depth: options.max_depth,
            };
            match scan_folder(path, &scan_options) {
                Ok(mut scanned) => entries.append(&mut scanned),
                Err(error) => issues.push(import_issue(
                    batch_id,
                    issues.len(),
                    ImportIssueSeverity::Error,
                    "folder_scan_failed",
                    format!("Failed to scan folder: {error}"),
                    Some(path.to_string_lossy().to_string()),
                    true,
                )),
            }
            continue;
        }
        match file_entry_from_path(path) {
            Ok(entry)
                if entry.kind == tench_fs_core::FileEntryKind::File && is_pdf_entry(&entry) =>
            {
                entries.push(entry);
            }
            Ok(entry) => issues.push(import_issue(
                batch_id,
                issues.len(),
                ImportIssueSeverity::Warning,
                "unsupported_import_path",
                "Only PDF files can be imported by this pipeline.",
                Some(entry.path),
                false,
            )),
            Err(error) => issues.push(import_issue(
                batch_id,
                issues.len(),
                ImportIssueSeverity::Error,
                "path_read_failed",
                format!("Failed to inspect import path: {error}"),
                Some(path.to_string_lossy().to_string()),
                true,
            )),
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries
}

fn import_issue(
    batch_id: &ImportBatchId,
    index: usize,
    severity: ImportIssueSeverity,
    code: impl Into<String>,
    message: impl Into<String>,
    path: Option<String>,
    retryable: bool,
) -> ImportIssue {
    ImportIssue {
        id: ImportIssueId::from(format!("{}-issue-{index}", batch_id.as_str())),
        severity,
        code: code.into(),
        message: message.into(),
        path,
        retryable,
    }
}

fn is_pdf_entry(entry: &FileEntry) -> bool {
    entry
        .extension
        .as_deref()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pdf"))
}

fn is_path_hidden(path: &Path) -> bool {
    path.components()
        .any(|component| tench_fs_core::is_hidden_path(Path::new(component.as_os_str())))
}

fn pdf_title_from_entry(entry: &FileEntry) -> String {
    Path::new(&entry.name)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::trim)
        .filter(|stem| !stem.is_empty())
        .unwrap_or("Imported PDF")
        .to_string()
}

fn store_pdf_attachment(
    layout: &LibraryLayout,
    entry: &FileEntry,
    attachment_id: &AttachmentId,
    policy: AttachmentStoragePolicy,
) -> std::io::Result<String> {
    match policy {
        AttachmentStoragePolicy::LinkOriginal => Ok(entry.path.clone()),
        AttachmentStoragePolicy::CopyIntoLibrary => {
            fs::create_dir_all(&layout.attachments_dir)?;
            let file_name = format!(
                "{}-{}",
                attachment_id.as_str(),
                sanitize_pdf_file_name(&entry.name)
            );
            let destination = layout.attachments_dir.join(&file_name);
            fs::copy(&entry.path, &destination)?;
            Ok(format!("attachments/{file_name}"))
        }
    }
}

fn sanitize_pdf_file_name(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            ch if ch.is_control() => '-',
            ch => ch,
        })
        .collect::<String>();
    if sanitized.to_ascii_lowercase().ends_with(".pdf") && !sanitized.trim().is_empty() {
        sanitized
    } else {
        format!("{}.pdf", sanitized.trim())
    }
}

fn unique_reference_id(snapshot: &ResearchSnapshotV2, base: &str) -> ReferenceId {
    let mut candidate = ReferenceId::from(base.to_string());
    let mut suffix_index = 1u32;
    while snapshot
        .references
        .iter()
        .any(|reference| reference.id == candidate)
    {
        candidate = ReferenceId::from(format!("{base}-{suffix_index}"));
        suffix_index += 1;
    }
    candidate
}

fn unique_attachment_id(snapshot: &ResearchSnapshotV2, base: &str) -> AttachmentId {
    let mut candidate = AttachmentId::from(base.to_string());
    let mut suffix_index = 1u32;
    while snapshot
        .attachments
        .iter()
        .any(|attachment| attachment.id == candidate)
    {
        candidate = AttachmentId::from(format!("{base}-{suffix_index}"));
        suffix_index += 1;
    }
    candidate
}
