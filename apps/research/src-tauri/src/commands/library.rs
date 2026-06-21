use tench_research_core::*;

#[tauri::command]
pub fn research_library_layout(library_id: String, root_dir: String) -> LibraryLayout {
    LibraryLayout::for_root(std::path::PathBuf::from(root_dir).join(library_id))
}

#[tauri::command]
pub fn create_research_library_snapshot(
    id: tench_research_core::LibraryId,
    name: String,
    root_dir: String,
    locale: tench_research_core::ResearchLocale,
    now: Timestamp,
) -> ResearchSnapshotV2 {
    tench_research_core::new_research_library_snapshot(id, name, root_dir, locale, now)
}

#[tauri::command]
pub fn create_research_library(
    id: tench_research_core::LibraryId,
    name: String,
    root_dir: String,
    locale: tench_research_core::ResearchLocale,
    now: Timestamp,
) -> Result<ResearchSnapshotV2, String> {
    let snapshot =
        tench_research_core::new_research_library_snapshot(id, name, root_dir.clone(), locale, now);
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::save_library_snapshot(&layout, &snapshot)
        .map_err(|error| error.to_string())?;
    Ok(snapshot)
}

#[tauri::command]
pub fn open_research_library(root_dir: String) -> Result<ResearchSnapshotV2, String> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::load_library_snapshot(&layout).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_research_library_snapshot(root_dir: String) -> Result<ResearchSnapshotV2, String> {
    open_research_library(root_dir)
}

#[tauri::command]
pub fn save_research_library_snapshot(
    root_dir: String,
    snapshot: ResearchSnapshotV2,
) -> Result<(), String> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::save_library_snapshot(&layout, &snapshot)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_research_library_snapshot(root_dir: String) -> Result<ResearchSnapshotV2, String> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::load_library_snapshot(&layout).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_research_library_snapshot_with_report(
    root_dir: String,
) -> Result<tench_research_core::ResearchSnapshotLoadResult, String> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::load_library_snapshot_with_report(&layout)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_research_recent_libraries(
    product_id: String,
) -> Result<tench_research_core::ResearchRecentLibraryRegistry, String> {
    tench_research_core::load_recent_research_libraries(product_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_research_recent_libraries(
    product_id: String,
    registry: tench_research_core::ResearchRecentLibraryRegistry,
) -> Result<String, String> {
    tench_research_core::save_recent_research_libraries(product_id, &registry)
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_research_recent_library(
    registry: tench_research_core::ResearchRecentLibraryRegistry,
    snapshot: ResearchSnapshotV2,
    opened_at: Timestamp,
    max_entries: usize,
) -> tench_research_core::ResearchRecentLibraryRegistry {
    tench_research_core::update_recent_research_library(registry, &snapshot, opened_at, max_entries)
}

#[tauri::command]
pub fn find_missing_research_attachments(
    root_dir: String,
    snapshot: ResearchSnapshotV2,
) -> Vec<tench_research_core::ResearchMissingAttachment> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::find_missing_research_attachments(&layout, &snapshot)
}

#[tauri::command]
pub fn repair_research_attachment_paths(
    root_dir: String,
    snapshot: ResearchSnapshotV2,
    repairs: Vec<tench_research_core::ResearchAttachmentPathRepair>,
    now: Timestamp,
) -> tench_research_core::ResearchAttachmentRepairReport {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::repair_research_attachment_paths(snapshot, &layout, repairs, now)
}

#[tauri::command]
pub fn upsert_research_reference(
    snapshot: ResearchSnapshotV2,
    reference: ReferenceItem,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::upsert_research_reference(snapshot, reference, now)
}

#[tauri::command]
pub fn remove_research_reference(
    snapshot: ResearchSnapshotV2,
    reference_id: tench_research_core::ReferenceId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::remove_research_reference(snapshot, &reference_id, now)
}

#[tauri::command]
pub fn upsert_research_collection(
    snapshot: ResearchSnapshotV2,
    collection: tench_research_core::ResearchCollection,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::upsert_research_collection(snapshot, collection, now)
}

#[tauri::command]
pub fn remove_research_collection(
    snapshot: ResearchSnapshotV2,
    collection_id: tench_research_core::ResearchCollectionId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::remove_research_collection(snapshot, &collection_id, now)
}

#[tauri::command]
pub fn upsert_research_tag(
    snapshot: ResearchSnapshotV2,
    tag: tench_research_core::ResearchTag,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::upsert_research_tag(snapshot, tag, now)
}

#[tauri::command]
pub fn remove_research_tag(
    snapshot: ResearchSnapshotV2,
    tag_id: tench_research_core::ResearchTagId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::remove_research_tag(snapshot, &tag_id, now)
}

#[tauri::command]
pub fn upsert_research_attachment(
    snapshot: ResearchSnapshotV2,
    attachment: tench_research_core::Attachment,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::upsert_research_attachment(snapshot, attachment, now)
}

#[tauri::command]
pub fn upsert_research_annotation(
    snapshot: ResearchSnapshotV2,
    annotation: tench_research_core::PdfAnnotation,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::upsert_research_annotation(snapshot, annotation, now)
}

#[tauri::command]
pub fn upsert_research_note(
    snapshot: ResearchSnapshotV2,
    note: tench_research_core::ResearchNote,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::upsert_research_note(snapshot, note, now)
}

#[tauri::command]
pub fn convert_research_annotation_to_note(
    snapshot: ResearchSnapshotV2,
    note_id: tench_research_core::ResearchNoteId,
    annotation_id: tench_research_core::AnnotationId,
    title: String,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::convert_annotation_to_note(snapshot, note_id, &annotation_id, title, now)
}

#[tauri::command]
pub fn merge_research_references(
    snapshot: ResearchSnapshotV2,
    target_id: tench_research_core::ReferenceId,
    source_id: tench_research_core::ReferenceId,
    now: Timestamp,
) -> Result<(ResearchSnapshotV2, ResearchMutationReport), String> {
    tench_research_core::merge_research_references(snapshot, &target_id, &source_id, now)
}

#[tauri::command]
pub fn create_research_library_backup(
    root_dir: String,
    snapshot: ResearchSnapshotV2,
    id: ResearchBackupId,
    now: Timestamp,
) -> Result<ResearchBackupManifest, String> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::create_library_backup(&layout, &snapshot, id, now)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_research_library_backup(
    root_dir: String,
    manifest: ResearchBackupManifest,
) -> Result<ResearchSnapshotV2, String> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::load_library_backup(&layout, &manifest).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_research_library_backup_archive(
    snapshot: ResearchSnapshotV2,
    id: ResearchBackupId,
    now: Timestamp,
) -> ResearchLibraryBackupArchive {
    tench_research_core::create_library_backup_archive(snapshot, id, now)
}

#[tauri::command]
pub fn export_research_library_backup_archive_zip(
    archive: ResearchLibraryBackupArchive,
) -> Result<Vec<u8>, String> {
    tench_research_core::export_library_backup_archive_zip(&archive)
}

#[tauri::command]
pub fn import_research_library_backup_archive_zip(
    bytes: Vec<u8>,
) -> Result<ResearchLibraryBackupArchive, String> {
    tench_research_core::import_library_backup_archive_zip(&bytes)
}

#[tauri::command]
pub fn restore_research_library_backup_archive(
    root_dir: String,
    archive: ResearchLibraryBackupArchive,
) -> Result<ResearchBackupManifest, String> {
    let layout = LibraryLayout::for_root(root_dir);
    tench_research_core::restore_library_backup_archive(&layout, &archive)
}
