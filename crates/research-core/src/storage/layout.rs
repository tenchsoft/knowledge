use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LibraryLayout {
    pub root: PathBuf,
    pub library_json: PathBuf,
    pub items_jsonl: PathBuf,
    pub attachments_jsonl: PathBuf,
    pub annotations_jsonl: PathBuf,
    pub notes_jsonl: PathBuf,
    pub collections_jsonl: PathBuf,
    pub tags_jsonl: PathBuf,
    pub citekeys_json: PathBuf,
    pub attachments_dir: PathBuf,
    pub index_dir: PathBuf,
    pub thumbnails_dir: PathBuf,
    pub recovery_dir: PathBuf,
    pub backups_dir: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchBackupManifest {
    pub id: ResearchBackupId,
    pub library_id: crate::LibraryId,
    pub created_at: crate::Timestamp,
    pub relative_path: String,
    pub schema_version: u32,
    pub reference_count: usize,
    pub attachment_count: usize,
    pub annotation_count: usize,
    pub note_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchLibraryBackupArchive {
    pub schema_version: u32,
    pub manifest: ResearchBackupManifest,
    pub snapshot: ResearchSnapshotV2,
}
