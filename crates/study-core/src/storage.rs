use std::sync::atomic::{AtomicU64, Ordering};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::{fs, io};
use tench_storage_core::{
    app_data_dir, decrypt_data, encrypt_data, is_encrypted_payload, DataBoundary, EncryptionKey,
    StorageClass, StorageNamespace, StoragePolicy,
};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

static ATOMIC_WRITE_COUNTER: AtomicU64 = AtomicU64::new(0);
fn counter_next() -> u64 {
    ATOMIC_WRITE_COUNTER.fetch_add(1, Ordering::Relaxed)
}

use crate::{
    InstalledContentPackRegistry, LearnerProfile, LearnerProgress, ReviewQueueItem, StudyCard,
    StudyDeck, StudyNote,
};

crate::study_id_type!(StudyProgressBackupId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyStorageArea {
    Config,
    Curriculum,
    ContentPacks,
    LearnerProfiles,
    Progress,
    Reviews,
    VisualAssets,
    Cache,
    Backups,
}

impl StudyStorageArea {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Config => "config",
            Self::Curriculum => "curriculum",
            Self::ContentPacks => "content_packs",
            Self::LearnerProfiles => "learner_profiles",
            Self::Progress => "progress",
            Self::Reviews => "reviews",
            Self::VisualAssets => "visual_assets",
            Self::Cache => "cache",
            Self::Backups => "backups",
        }
    }
}

pub fn study_storage_policy(area: StudyStorageArea) -> StoragePolicy {
    let class = study_storage_class(area);

    StoragePolicy {
        namespace: StorageNamespace {
            product_id: "study".to_string(),
            class: class.clone(),
            name: area.as_str().to_string(),
        },
        boundary: DataBoundary::LocalOnly,
        encrypted_at_rest: matches!(
            area,
            StudyStorageArea::LearnerProfiles | StudyStorageArea::Progress
        ),
        user_exportable: matches!(
            area,
            StudyStorageArea::Config
                | StudyStorageArea::Curriculum
                | StudyStorageArea::ContentPacks
                | StudyStorageArea::LearnerProfiles
                | StudyStorageArea::Progress
                | StudyStorageArea::Reviews
                | StudyStorageArea::VisualAssets
                | StudyStorageArea::Backups
        ),
        retention_days: match area {
            StudyStorageArea::Cache => Some(30),
            _ => None,
        },
    }
}

pub fn study_app_data_dir() -> PathBuf {
    app_data_dir("Tench", "Study")
}

pub fn study_storage_dir(area: StudyStorageArea) -> PathBuf {
    study_app_data_dir().join(area.as_str())
}

pub fn write_study_json<T: Serialize>(
    area: StudyStorageArea,
    file_name: impl AsRef<str>,
    value: &T,
) -> Result<PathBuf, StudyStorageError> {
    tench_storage_core::validate_safe_file_name(file_name.as_ref())
        .map_err(|error| StudyStorageError::InvalidPath(error.to_string()))?;
    let path = study_storage_dir(area).join(file_name.as_ref());
    let policy = study_storage_policy(area);
    if policy.encrypted_at_rest {
        atomic_write_encrypted_json(&path, value)?;
    } else {
        atomic_write_json(&path, value)?;
    }
    Ok(path)
}

pub fn read_study_json<T: DeserializeOwned>(
    area: StudyStorageArea,
    file_name: impl AsRef<str>,
) -> Result<Option<T>, StudyStorageError> {
    tench_storage_core::validate_safe_file_name(file_name.as_ref())
        .map_err(|error| StudyStorageError::InvalidPath(error.to_string()))?;
    let path = study_storage_dir(area).join(file_name.as_ref());
    if !path.exists() {
        return Ok(None);
    }
    let policy = study_storage_policy(area);
    if policy.encrypted_at_rest {
        read_encrypted_json(&path)
    } else {
        let data = fs::read_to_string(&path)?;
        let value = serde_json::from_str(&data)?;
        Ok(Some(value))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyProgressBackup {
    pub id: StudyProgressBackupId,
    pub schema_version: u32,
    pub exported_at: String,
    #[serde(default)]
    pub profiles: Vec<LearnerProfile>,
    #[serde(default)]
    pub progress: Vec<LearnerProgress>,
    #[serde(default)]
    pub review_items: Vec<ReviewQueueItem>,
    #[serde(default)]
    pub notes: Vec<StudyNote>,
    #[serde(default)]
    pub decks: Vec<StudyDeck>,
    #[serde(default)]
    pub cards: Vec<StudyCard>,
    #[serde(default)]
    pub installed_packs: InstalledContentPackRegistry,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyProgressBackupManifest {
    pub id: StudyProgressBackupId,
    pub schema_version: u32,
    pub exported_at: String,
    pub profile_count: usize,
    pub progress_count: usize,
    pub review_item_count: usize,
    pub note_count: usize,
    pub deck_count: usize,
    pub card_count: usize,
    pub installed_pack_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyProgressRestoreReport {
    pub manifest: StudyProgressBackupManifest,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StudyProgressBackupPayload {
    #[serde(default)]
    pub profiles: Vec<LearnerProfile>,
    #[serde(default)]
    pub progress: Vec<LearnerProgress>,
    #[serde(default)]
    pub review_items: Vec<ReviewQueueItem>,
    #[serde(default)]
    pub notes: Vec<StudyNote>,
    #[serde(default)]
    pub decks: Vec<StudyDeck>,
    #[serde(default)]
    pub cards: Vec<StudyCard>,
    #[serde(default)]
    pub installed_packs: InstalledContentPackRegistry,
}

impl StudyProgressBackup {
    pub fn manifest(&self) -> StudyProgressBackupManifest {
        StudyProgressBackupManifest {
            id: self.id.clone(),
            schema_version: self.schema_version,
            exported_at: self.exported_at.clone(),
            profile_count: self.profiles.len(),
            progress_count: self.progress.len(),
            review_item_count: self.review_items.len(),
            note_count: self.notes.len(),
            deck_count: self.decks.len(),
            card_count: self.cards.len(),
            installed_pack_count: self.installed_packs.entries.len(),
        }
    }
}

pub fn create_study_progress_backup(
    id: StudyProgressBackupId,
    exported_at: impl Into<String>,
    payload: StudyProgressBackupPayload,
) -> StudyProgressBackup {
    StudyProgressBackup {
        id,
        schema_version: 1,
        exported_at: exported_at.into(),
        profiles: payload.profiles,
        progress: payload.progress,
        review_items: payload.review_items,
        notes: payload.notes,
        decks: payload.decks,
        cards: payload.cards,
        installed_packs: payload.installed_packs,
    }
}

pub fn export_study_progress_backup_json(backup: &StudyProgressBackup) -> Result<String, String> {
    ensure_progress_backup_valid(backup)?;
    serde_json::to_string_pretty(backup).map_err(|error| error.to_string())
}

pub fn import_study_progress_backup_json(text: &str) -> Result<StudyProgressBackup, String> {
    let backup: StudyProgressBackup =
        serde_json::from_str(text).map_err(|error| error.to_string())?;
    ensure_progress_backup_valid(&backup)?;
    Ok(backup)
}

pub fn export_study_progress_backup_zip(backup: &StudyProgressBackup) -> Result<Vec<u8>, String> {
    let backup_json = export_study_progress_backup_json(backup)?;
    let manifest_json =
        serde_json::to_string_pretty(&backup.manifest()).map_err(|error| error.to_string())?;
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    writer
        .start_file("tench-study-progress-backup.json", options)
        .map_err(|error| error.to_string())?;
    writer
        .write_all(backup_json.as_bytes())
        .map_err(|error| error.to_string())?;
    writer
        .start_file("manifest.json", options)
        .map_err(|error| error.to_string())?;
    writer
        .write_all(manifest_json.as_bytes())
        .map_err(|error| error.to_string())?;

    writer
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| error.to_string())
}

pub fn import_study_progress_backup_zip(bytes: &[u8]) -> Result<StudyProgressBackup, String> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|error| error.to_string())?;
    check_study_archive_limits(&mut archive)?;
    let mut backup_json = String::new();
    archive
        .by_name("tench-study-progress-backup.json")
        .map_err(|error| format!("missing tench-study-progress-backup.json: {error}"))?
        .read_to_string(&mut backup_json)
        .map_err(|error| error.to_string())?;
    import_study_progress_backup_json(&backup_json)
}

pub fn preview_study_progress_restore(
    backup: &StudyProgressBackup,
) -> Result<StudyProgressRestoreReport, String> {
    ensure_progress_backup_valid(backup)?;
    let manifest = backup.manifest();
    let mut warnings = Vec::new();
    if manifest.profile_count == 0 {
        warnings.push("backup_contains_no_profiles".to_string());
    }
    if manifest.progress_count == 0
        && manifest.review_item_count == 0
        && manifest.note_count == 0
        && manifest.deck_count == 0
        && manifest.card_count == 0
    {
        warnings.push("backup_contains_no_learner_data".to_string());
    }
    Ok(StudyProgressRestoreReport { manifest, warnings })
}

pub fn restore_study_progress_backup_payload(
    backup: StudyProgressBackup,
) -> Result<StudyProgressBackupPayload, String> {
    ensure_progress_backup_valid(&backup)?;
    Ok(StudyProgressBackupPayload {
        profiles: backup.profiles,
        progress: backup.progress,
        review_items: backup.review_items,
        notes: backup.notes,
        decks: backup.decks,
        cards: backup.cards,
        installed_packs: backup.installed_packs,
    })
}

pub fn save_study_progress_backup(
    backup: &StudyProgressBackup,
) -> Result<PathBuf, StudyStorageError> {
    let file_name = format!(
        "{}.backup.json",
        sanitize_storage_token(backup.id.as_str(), "study-progress-backup")
    );
    write_study_json(StudyStorageArea::Backups, file_name, backup)
}

pub fn load_study_progress_backup(
    id: &StudyProgressBackupId,
) -> Result<Option<StudyProgressBackup>, StudyStorageError> {
    let file_name = format!(
        "{}.backup.json",
        sanitize_storage_token(id.as_str(), "study-progress-backup")
    );
    read_study_json(StudyStorageArea::Backups, file_name)
}

pub fn atomic_write_json<T: Serialize>(
    path: &std::path::Path,
    value: &T,
) -> Result<(), StudyStorageError> {
    let parent = path.parent().ok_or_else(|| {
        StudyStorageError::InvalidPath("storage path requires a parent directory".to_string())
    })?;
    fs::create_dir_all(parent)?;
    let suffix = format!(
        "{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
        counter_next(),
    );
    let tmp = path.with_extension(format!("{}.atomictmp", suffix));
    let data = serde_json::to_vec_pretty(value)?;
    {
        let mut file = std::fs::File::create(&tmp)?;
        std::io::Write::write_all(&mut file, &data)?;
        file.sync_all()?;
    }
    match fs::rename(&tmp, path) {
        Ok(()) => {
            sync_parent_dir(parent)?;
            Ok(())
        }
        Err(rename_err) => {
            let _ = fs::remove_file(&tmp);
            Err(rename_err.into())
        }
    }
}

/// Writes a serializable value to disk as encrypted binary data using an
/// atomic write (write-to-tmp, then rename).
fn atomic_write_encrypted_json<T: Serialize>(
    path: &std::path::Path,
    value: &T,
) -> Result<(), StudyStorageError> {
    let parent = path.parent().ok_or_else(|| {
        StudyStorageError::InvalidPath("storage path requires a parent directory".to_string())
    })?;
    fs::create_dir_all(parent)?;

    let json_data = serde_json::to_vec_pretty(value)?;
    let key = EncryptionKey::from_machine();
    let encrypted = encrypt_data(&json_data, &key.derived_key())
        .map_err(|e| StudyStorageError::Encryption(e.to_string()))?;

    let suffix = format!(
        "{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
        counter_next(),
    );
    let tmp = path.with_extension(format!("{}.atomictmp", suffix));
    {
        let mut file = std::fs::File::create(&tmp)?;
        std::io::Write::write_all(&mut file, &encrypted)?;
        file.sync_all()?;
    }
    match fs::rename(&tmp, path) {
        Ok(()) => {
            sync_parent_dir(parent)?;
            Ok(())
        }
        Err(rename_err) => {
            let _ = fs::remove_file(&tmp);
            Err(rename_err.into())
        }
    }
}

/// Reads a file that may be encrypted or plaintext JSON and deserializes it.
///
/// If the file starts with the encrypted magic prefix, it is decrypted using
/// the machine-derived key.  Otherwise it is read as plaintext JSON for
/// backwards compatibility with files written before encryption was enabled.
fn read_encrypted_json<T: DeserializeOwned>(
    path: &std::path::Path,
) -> Result<Option<T>, StudyStorageError> {
    let raw = fs::read(path)?;
    if !is_encrypted_payload(&raw) {
        return Err(StudyStorageError::Encryption(
            "encrypted storage area contains plaintext data".to_string(),
        ));
    }
    let key = EncryptionKey::from_machine();
    let json_data = decrypt_data(&raw, &key.derived_key())
        .map_err(|e| StudyStorageError::Encryption(e.to_string()))?;
    let json_str =
        String::from_utf8(json_data).map_err(|e| StudyStorageError::InvalidData(e.to_string()))?;
    let value = serde_json::from_str(&json_str)?;
    Ok(Some(value))
}

fn sync_parent_dir(parent: &std::path::Path) -> Result<(), StudyStorageError> {
    #[cfg(unix)]
    {
        let dir = std::fs::File::open(parent)?;
        dir.sync_all()?;
    }
    #[cfg(not(unix))]
    {
        let _ = parent;
    }
    Ok(())
}

fn ensure_progress_backup_valid(backup: &StudyProgressBackup) -> Result<(), String> {
    if backup.schema_version != 1 {
        return Err(format!(
            "unsupported study progress backup schema {}",
            backup.schema_version
        ));
    }
    if backup.id.as_str().trim().is_empty() {
        return Err("study progress backup id is required".to_string());
    }
    if backup.exported_at.trim().is_empty() {
        return Err("study progress backup exported_at is required".to_string());
    }
    Ok(())
}

fn sanitize_storage_token(value: &str, fallback: &str) -> String {
    let token = value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
        .take(80)
        .collect::<String>();
    if token.is_empty() {
        fallback.to_string()
    } else {
        token
    }
}

/// Validate a ZIP archive against safe limits for study-core imports.
///
/// Checks entry count, individual entry size, total uncompressed size, and
/// rejects entries with path-traversal components.
pub fn check_study_archive_limits<R: std::io::Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
) -> Result<(), String> {
    let max_entries = 10_000usize;
    let max_entry_bytes: u64 = 100 * 1024 * 1024; // 100 MB
    let max_total_bytes: u64 = 2 * 1024 * 1024 * 1024; // 2 GB

    if archive.len() > max_entries {
        return Err(format!(
            "archive has {} entries, maximum is {}",
            archive.len(),
            max_entries
        ));
    }

    let mut total: u64 = 0;
    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = entry.name().to_string();
        if name.starts_with('/') || name.contains("..") || name.contains('\\') {
            return Err(format!("archive entry has invalid path: {name}"));
        }
        let size = entry.size();
        if size > max_entry_bytes {
            return Err(format!(
                "archive entry {name} is {size} bytes, maximum is {max_entry_bytes}"
            ));
        }
        total = total.saturating_add(size);
        if total > max_total_bytes {
            return Err(format!(
                "archive total uncompressed size exceeds {max_total_bytes} bytes"
            ));
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum StudyStorageError {
    Io(io::Error),
    Serde(serde_json::Error),
    InvalidPath(String),
    Encryption(String),
    InvalidData(String),
}

impl std::fmt::Display for StudyStorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "study storage I/O error: {error}"),
            Self::Serde(error) => {
                write!(formatter, "study storage serialization error: {error}")
            }
            Self::InvalidPath(message) => write!(formatter, "{message}"),
            Self::Encryption(message) => {
                write!(formatter, "study storage encryption error: {message}")
            }
            Self::InvalidData(message) => {
                write!(formatter, "study storage invalid data: {message}")
            }
        }
    }
}

impl std::error::Error for StudyStorageError {}

impl From<io::Error> for StudyStorageError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for StudyStorageError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}

fn study_storage_class(area: StudyStorageArea) -> StorageClass {
    match area {
        StudyStorageArea::Config => StorageClass::Config,
        StudyStorageArea::Cache => StorageClass::Cache,
        StudyStorageArea::Curriculum
        | StudyStorageArea::ContentPacks
        | StudyStorageArea::LearnerProfiles
        | StudyStorageArea::Progress
        | StudyStorageArea::Reviews
        | StudyStorageArea::VisualAssets
        | StudyStorageArea::Backups => StorageClass::UserContent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_is_local_and_encrypted() {
        let policy = study_storage_policy(StudyStorageArea::Progress);

        assert_eq!(policy.boundary, DataBoundary::LocalOnly);
        assert!(policy.encrypted_at_rest);
        assert!(policy.user_exportable);
    }

    #[test]
    fn atomic_json_helpers_round_trip_value() {
        let root = std::env::temp_dir()
            .join("tench-study-core-tests")
            .join(format!("state-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let path = root.join("state.json");

        atomic_write_json(&path, &vec!["math", "science"]).expect("write json");
        let data = fs::read_to_string(&path).expect("read json");
        let decoded: Vec<String> = serde_json::from_str(&data).expect("decode json");

        assert_eq!(decoded, vec!["math", "science"]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn storage_rejects_id_path_escape_security_regression() {
        let result =
            read_study_json::<serde_json::Value>(StudyStorageArea::Progress, "../learner.json");

        assert!(matches!(result, Err(StudyStorageError::InvalidPath(_))));
    }

    #[test]
    fn encrypted_storage_rejects_plaintext_security_regression() {
        let root = std::env::temp_dir()
            .join("tench-study-core-tests")
            .join(format!("plaintext-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("root");
        let path = root.join("progress.json");
        fs::write(&path, br#"{"learner_id":"learner"}"#).expect("write plaintext");

        let result = read_encrypted_json::<serde_json::Value>(&path);

        assert!(matches!(result, Err(StudyStorageError::Encryption(_))));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn progress_backup_zip_preserves_learner_data() {
        let locale = crate::ContentLocale::parse("ko-KR").expect("locale");
        let learner_id = crate::LearnerId::from("learner-1");
        let node_id = crate::CurriculumNodeId::from("math-k-counting");
        let profile = crate::LearnerProfile {
            id: learner_id.clone(),
            display_name: "Yoon".to_string(),
            primary_locale: locale.clone(),
            target_locales: vec![locale.clone()],
            accommodations: vec![crate::LearningAccommodation::ReducedMotion],
        };
        let progress = crate::LearnerProgress {
            learner_id: learner_id.clone(),
            node_id: node_id.clone(),
            mastery: crate::MasteryState {
                score: 1.0,
                attempts: 1,
                correct: 1,
            },
            attempts: vec![crate::AttemptRecord {
                id: crate::AttemptId::from("attempt-1"),
                item_id: crate::PracticeItemId::from("practice-1"),
                correct: true,
                score: 1.0,
                response: "42".to_string(),
                created_at: "2026-05-04T00:00:00Z".to_string(),
            }],
            review_state: crate::SpacedRepetitionState::default(),
        };
        let review_item = crate::ReviewQueueItem {
            id: crate::ReviewQueueItemId::from("review-1"),
            node_id: node_id.clone(),
            practice_item_id: crate::PracticeItemId::from("practice-1"),
            wrong_answer: "41".to_string(),
            correct_answer: "42".to_string(),
            cause_tag: "off_by_one".to_string(),
            explanation: crate::LocalizedText::localized("정답은 42입니다.", locale.clone()),
            rating: crate::ReviewRating::Good,
            reviewed: false,
        };
        let note = crate::create_study_note(
            crate::StudyNoteId::from("note-1"),
            learner_id.clone(),
            node_id.clone(),
            crate::LocalizedText::localized("노트", locale.clone()),
            "중요 개념",
            "2026-05-04T00:00:00Z",
        )
        .expect("note");
        let card = crate::create_card_from_note(
            crate::StudyCardId::from("card-1"),
            crate::StudyDeckId::from("deck-1"),
            &note,
            crate::StudyCardKind::Basic,
            crate::LocalizedText::localized("질문", locale.clone()),
            crate::LocalizedText::localized("답", locale),
            "2026-05-04T00:00:00Z",
        )
        .expect("card");
        let deck = crate::StudyDeck {
            id: crate::StudyDeckId::from("deck-1"),
            learner_id,
            title: crate::LocalizedText::plain("Math"),
            cards: vec![card.clone()],
            created_at: "2026-05-04T00:00:00Z".to_string(),
            updated_at: "2026-05-04T00:00:00Z".to_string(),
        };

        let backup = create_study_progress_backup(
            StudyProgressBackupId::from("backup-1"),
            "2026-05-04T00:00:00Z",
            StudyProgressBackupPayload {
                profiles: vec![profile],
                progress: vec![progress],
                review_items: vec![review_item],
                notes: vec![note],
                decks: vec![deck],
                cards: vec![card],
                installed_packs: crate::InstalledContentPackRegistry::default(),
            },
        );

        let bytes = export_study_progress_backup_zip(&backup).expect("export backup zip");
        let restored = import_study_progress_backup_zip(&bytes).expect("import backup zip");
        let preview = preview_study_progress_restore(&restored).expect("restore preview");
        let payload =
            restore_study_progress_backup_payload(restored.clone()).expect("restore payload");

        assert_eq!(restored.manifest().profile_count, 1);
        assert_eq!(restored.manifest().progress_count, 1);
        assert_eq!(restored.manifest().review_item_count, 1);
        assert_eq!(restored.manifest().note_count, 1);
        assert_eq!(restored.manifest().deck_count, 1);
        assert_eq!(restored.manifest().card_count, 1);
        assert_eq!(restored.progress[0].node_id.as_str(), "math-k-counting");
        assert_eq!(restored.cards[0].front.value, "질문");
        assert!(preview.warnings.is_empty());
        assert_eq!(payload.profiles.len(), 1);
        assert_eq!(payload.installed_packs.entries.len(), 0);
    }

    // -----------------------------------------------------------------------
    // check_study_archive_limits tests
    // -----------------------------------------------------------------------

    /// Helper: build a ZIP archive in memory from a list of `(name, size)` pairs.
    /// Each entry is filled with `0xAB` bytes.
    fn build_zip_archive(entries: Vec<(&str, usize)>) -> Vec<u8> {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        for (name, size) in &entries {
            writer.start_file(*name, options).expect("start_file");
            writer.write_all(&vec![0xAB_u8; *size]).expect("write_all");
        }
        writer.finish().expect("finish").into_inner()
    }

    #[test]
    fn archive_limits_rejects_too_many_entries() {
        // 10_001 entries exceeds the limit of 10_000.
        let entries: Vec<(&str, usize)> = (0..=10_000)
            .map(|i| {
                let name = format!("file_{i}.bin");
                // Leak the string to get a 'static str for the test helper.
                Box::leak(name.into_boxed_str()) as &str
            })
            .map(|name| (name, 1))
            .collect();
        let bytes = build_zip_archive(entries);
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("open archive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_err(), "expected error for too many entries");
        let err = result.unwrap_err();
        assert!(
            err.contains("entries"),
            "error should mention entries: {err}"
        );
    }

    #[test]
    fn archive_limits_rejects_oversized_entry() {
        // 101 MB exceeds the 100 MB per-entry limit.
        let entries = vec![("big.bin", 101 * 1024 * 1024)];
        let bytes = build_zip_archive(entries);
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("open archive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_err(), "expected error for oversized entry");
        let err = result.unwrap_err();
        assert!(
            err.contains("big.bin"),
            "error should mention entry name: {err}"
        );
    }

    #[test]
    fn archive_limits_rejects_path_traversal_dot_dot() {
        let entries = vec![("../etc/passwd", 10)];
        let bytes = build_zip_archive(entries);
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("open archive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_err(), "expected error for path traversal");
        let err = result.unwrap_err();
        assert!(
            err.contains("invalid path"),
            "error should mention invalid path: {err}"
        );
    }

    #[test]
    fn archive_limits_rejects_absolute_path() {
        let entries = vec![("/etc/passwd", 10)];
        let bytes = build_zip_archive(entries);
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("open archive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_err(), "expected error for absolute path");
        let err = result.unwrap_err();
        assert!(
            err.contains("invalid path"),
            "error should mention invalid path: {err}"
        );
    }

    #[test]
    fn archive_limits_rejects_backslash_in_entry_name() {
        let entries = vec![("foo\\bar", 10)];
        let bytes = build_zip_archive(entries);
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("open archive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_err(), "expected error for backslash in name");
        let err = result.unwrap_err();
        assert!(
            err.contains("invalid path"),
            "error should mention invalid path: {err}"
        );
    }

    #[test]
    fn archive_limits_accepts_valid_archive() {
        let entries = vec![
            ("manifest.json", 100),
            ("tench-study-progress-backup.json", 200),
        ];
        let bytes = build_zip_archive(entries);
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).expect("open archive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_ok(), "valid archive should pass: {:?}", result);
    }

    // -----------------------------------------------------------------------
    // Security regression tests
    // -----------------------------------------------------------------------

    #[test]
    fn archive_limits_reject_zip_bomb_security() {
        use std::io::{Cursor, Write};

        // Create a ZIP with more than the allowed 10 000 entries.
        let mut buf = Cursor::new(Vec::new());
        {
            let mut writer = ZipWriter::new(&mut buf);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
            for i in 0..10_001 {
                writer
                    .start_file(format!("file_{i}"), options)
                    .expect("start_file");
                writer.write_all(b"x").expect("write");
            }
            writer.finish().expect("finish");
        }
        let bytes = buf.into_inner();
        let mut archive = ZipArchive::new(Cursor::new(bytes.as_slice())).expect("ZipArchive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_err(), "expected too-many-entries rejection");
        assert!(
            result.unwrap_err().contains("entries"),
            "error should mention entries"
        );
    }

    #[test]
    fn archive_rejects_path_traversal_security() {
        use std::io::{Cursor, Write};

        let mut buf = Cursor::new(Vec::new());
        {
            let mut writer = ZipWriter::new(&mut buf);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
            writer
                .start_file("../../../etc/passwd", options)
                .expect("start_file");
            writer.write_all(b"root:x:0:0").expect("write");
            writer.finish().expect("finish");
        }
        let bytes = buf.into_inner();
        let mut archive = ZipArchive::new(Cursor::new(bytes.as_slice())).expect("ZipArchive");
        let result = check_study_archive_limits(&mut archive);
        assert!(result.is_err(), "expected path-traversal rejection");
        assert!(
            result.unwrap_err().contains("invalid path"),
            "error should mention invalid path"
        );
    }

    // -----------------------------------------------------------------------
    // Release validation tests
    // -----------------------------------------------------------------------

    #[test]
    fn backup_integrity_release() {
        let backup = create_study_progress_backup(
            StudyProgressBackupId::from("release-backup-1"),
            "2026-05-05T00:00:00Z",
            StudyProgressBackupPayload::default(),
        );
        assert_eq!(backup.schema_version, 1);
        assert_eq!(backup.id.as_str(), "release-backup-1");

        let json = export_study_progress_backup_json(&backup).expect("export json");
        let restored = import_study_progress_backup_json(&json).expect("import json");
        assert_eq!(restored.id, backup.id);
        assert_eq!(restored.schema_version, backup.schema_version);

        let zip_bytes = export_study_progress_backup_zip(&backup).expect("export zip");
        let restored_zip = import_study_progress_backup_zip(&zip_bytes).expect("import zip");
        assert_eq!(restored_zip.id, backup.id);
    }
}
