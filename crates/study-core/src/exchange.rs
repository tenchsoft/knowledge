use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tench_fs_core::{
    file_entry_from_path, scan_folder, FileAccessGrant, FileEntry, FileEntryKind,
    FilePermissionScope, FolderScanOptions,
};

use crate::{StudyCardExchangeFormat, StudyNoteExchangeFormat};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyImportFormat {
    AnkiPackage,
    AnkiTsv,
    Csv,
    Tsv,
    Markdown,
    Json,
    ContentPackArchive,
    ProgressBackup,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyImportTarget {
    Cards,
    Notes,
    ContentPack,
    ProgressBackup,
    Auto,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyFileImportOptions {
    pub recursive: bool,
    pub include_hidden: bool,
    #[serde(default)]
    pub max_depth: Option<u8>,
}

impl Default for StudyFileImportOptions {
    fn default() -> Self {
        Self {
            recursive: true,
            include_hidden: false,
            max_depth: Some(12),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyFileImportPlan {
    pub candidates: Vec<StudyFileImportCandidate>,
    pub skipped: Vec<StudyFileImportSkip>,
    pub grants: Vec<FileAccessGrant>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyFileImportCandidate {
    pub entry: FileEntry,
    pub format: StudyImportFormat,
    pub target: StudyImportTarget,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyFileImportSkip {
    pub path: String,
    pub reason: String,
}

pub fn detect_study_import_format(path_or_kind: &str) -> Option<StudyImportFormat> {
    let value = path_or_kind
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase();
    if value.ends_with(".tench-study-backup")
        || value.ends_with(".tench-study-backup.zip")
        || value.ends_with(".study-backup")
        || value.ends_with(".study-backup.zip")
        || value.ends_with(".backup.json")
    {
        return Some(StudyImportFormat::ProgressBackup);
    }
    if value.ends_with(".tench-study-pack")
        || value.ends_with(".tench-study-pack.zip")
        || value.ends_with(".study-pack")
        || value.ends_with(".study-pack.zip")
    {
        return Some(StudyImportFormat::ContentPackArchive);
    }
    match value.as_str() {
        "apkg" => Some(StudyImportFormat::AnkiPackage),
        "txt" | "anki" | "anki-tsv" => Some(StudyImportFormat::AnkiTsv),
        "csv" => Some(StudyImportFormat::Csv),
        "tsv" => Some(StudyImportFormat::Tsv),
        "md" | "markdown" => Some(StudyImportFormat::Markdown),
        "json" => Some(StudyImportFormat::Json),
        "tench-study-backup" | "study-backup" | "backup" | "progress-backup" => {
            Some(StudyImportFormat::ProgressBackup)
        }
        "tench-study-pack" | "study-pack" | "pack" => Some(StudyImportFormat::ContentPackArchive),
        "zip" => Some(StudyImportFormat::ContentPackArchive),
        _ => Path::new(path_or_kind)
            .extension()
            .and_then(|extension| extension.to_str())
            .and_then(detect_study_import_format),
    }
}

pub fn study_card_exchange_format(format: StudyImportFormat) -> Option<StudyCardExchangeFormat> {
    match format {
        StudyImportFormat::AnkiTsv => Some(StudyCardExchangeFormat::AnkiTsv),
        StudyImportFormat::Csv => Some(StudyCardExchangeFormat::Csv),
        StudyImportFormat::Tsv => Some(StudyCardExchangeFormat::Tsv),
        StudyImportFormat::Markdown => Some(StudyCardExchangeFormat::Markdown),
        StudyImportFormat::Json => Some(StudyCardExchangeFormat::Json),
        StudyImportFormat::AnkiPackage
        | StudyImportFormat::ContentPackArchive
        | StudyImportFormat::ProgressBackup => None,
    }
}

pub fn study_note_exchange_format(format: StudyImportFormat) -> Option<StudyNoteExchangeFormat> {
    match format {
        StudyImportFormat::Markdown => Some(StudyNoteExchangeFormat::Markdown),
        StudyImportFormat::Json => Some(StudyNoteExchangeFormat::Json),
        StudyImportFormat::AnkiTsv
        | StudyImportFormat::Csv
        | StudyImportFormat::Tsv
        | StudyImportFormat::AnkiPackage
        | StudyImportFormat::ContentPackArchive
        | StudyImportFormat::ProgressBackup => None,
    }
}

pub fn plan_study_file_import_paths(
    paths: &[PathBuf],
    options: &StudyFileImportOptions,
) -> Result<StudyFileImportPlan, String> {
    let mut candidates = Vec::new();
    let mut skipped = Vec::new();
    let mut grants = Vec::new();

    for path in paths {
        if path.is_dir() {
            grants.push(FileAccessGrant {
                scope: if options.recursive {
                    FilePermissionScope::RecursiveDirectory
                } else {
                    FilePermissionScope::Directory
                },
                path: path.to_string_lossy().to_string(),
                reason: "Import Study cards, notes, and content packs from a local folder."
                    .to_string(),
            });
            let scan_options = FolderScanOptions {
                recursive: options.recursive,
                include_hidden: options.include_hidden,
                allowed_extensions: supported_study_import_extensions(),
                max_depth: options.max_depth,
            };
            let entries = scan_folder(path, &scan_options).map_err(|error| error.to_string())?;
            for entry in entries {
                push_import_candidate(entry, &mut candidates, &mut skipped);
            }
        } else {
            grants.push(FileAccessGrant {
                scope: FilePermissionScope::SingleFile,
                path: path.to_string_lossy().to_string(),
                reason: "Import a local Study file.".to_string(),
            });
            let entry = file_entry_from_path(path).map_err(|error| error.to_string())?;
            push_import_candidate(entry, &mut candidates, &mut skipped);
        }
    }

    candidates.sort_by(|left, right| left.entry.path.cmp(&right.entry.path));
    skipped.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(StudyFileImportPlan {
        candidates,
        skipped,
        grants,
    })
}

fn push_import_candidate(
    entry: FileEntry,
    candidates: &mut Vec<StudyFileImportCandidate>,
    skipped: &mut Vec<StudyFileImportSkip>,
) {
    if entry.kind != FileEntryKind::File {
        skipped.push(StudyFileImportSkip {
            path: entry.path,
            reason: "not_a_file".to_string(),
        });
        return;
    }

    let format = detect_study_import_format(&entry.path).or_else(|| {
        entry
            .extension
            .as_deref()
            .and_then(detect_study_import_format)
    });
    let Some(format) = format else {
        skipped.push(StudyFileImportSkip {
            path: entry.path,
            reason: "unsupported_extension".to_string(),
        });
        return;
    };

    candidates.push(StudyFileImportCandidate {
        target: import_target_for_format(format),
        entry,
        format,
    });
}

fn import_target_for_format(format: StudyImportFormat) -> StudyImportTarget {
    match format {
        StudyImportFormat::AnkiPackage
        | StudyImportFormat::AnkiTsv
        | StudyImportFormat::Csv
        | StudyImportFormat::Tsv => StudyImportTarget::Cards,
        StudyImportFormat::Markdown | StudyImportFormat::Json => StudyImportTarget::Auto,
        StudyImportFormat::ContentPackArchive => StudyImportTarget::ContentPack,
        StudyImportFormat::ProgressBackup => StudyImportTarget::ProgressBackup,
    }
}

fn supported_study_import_extensions() -> Vec<String> {
    [
        "apkg",
        "txt",
        "csv",
        "tsv",
        "md",
        "markdown",
        "json",
        "zip",
        "pack",
        "study-pack",
        "tench-study-pack",
        "backup",
        "study-backup",
        "tench-study-backup",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn detects_study_import_formats_from_paths_and_extensions() {
        assert_eq!(
            detect_study_import_format("deck.apkg"),
            Some(StudyImportFormat::AnkiPackage)
        );
        assert_eq!(
            detect_study_import_format(".csv"),
            Some(StudyImportFormat::Csv)
        );
        assert_eq!(
            detect_study_import_format("curriculum.tench-study-pack.zip"),
            Some(StudyImportFormat::ContentPackArchive)
        );
        assert_eq!(
            detect_study_import_format("learner.backup.json"),
            Some(StudyImportFormat::ProgressBackup)
        );
        assert_eq!(
            study_card_exchange_format(StudyImportFormat::Tsv),
            Some(StudyCardExchangeFormat::Tsv)
        );
        assert_eq!(
            study_note_exchange_format(StudyImportFormat::Markdown),
            Some(StudyNoteExchangeFormat::Markdown)
        );
    }

    #[test]
    fn file_import_plan_uses_fs_core_entries_and_grants() {
        let root = temp_test_dir("study-file-import");
        let nested = root.join("nested");
        fs::create_dir_all(&nested).expect("nested dir");
        fs::write(root.join("cards.tsv"), "front\tback").expect("cards");
        fs::write(nested.join("notes.md"), "# Note\nBody").expect("notes");
        fs::write(root.join("curriculum.tench-study-pack"), "{}").expect("content pack");
        fs::write(root.join("learner.backup.json"), "{}").expect("backup");
        fs::write(root.join("ignore.bin"), [1, 2, 3]).expect("ignored");

        let plan = plan_study_file_import_paths(
            std::slice::from_ref(&root),
            &StudyFileImportOptions {
                recursive: true,
                include_hidden: false,
                max_depth: Some(4),
            },
        )
        .expect("plan import");

        assert_eq!(plan.candidates.len(), 4);
        assert!(plan
            .candidates
            .iter()
            .any(|candidate| candidate.format == StudyImportFormat::Tsv));
        assert!(plan
            .candidates
            .iter()
            .any(|candidate| candidate.target == StudyImportTarget::Auto));
        assert!(plan
            .candidates
            .iter()
            .any(|candidate| candidate.target == StudyImportTarget::ContentPack));
        assert!(plan
            .candidates
            .iter()
            .any(|candidate| candidate.target == StudyImportTarget::ProgressBackup));
        assert_eq!(
            plan.grants[0].scope,
            FilePermissionScope::RecursiveDirectory
        );
    }

    fn temp_test_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("tench-study-core-{name}-{nonce}"));
        fs::create_dir_all(&path).expect("temp dir");
        path
    }
}
