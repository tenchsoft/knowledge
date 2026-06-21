use super::*;

pub fn create_library_backup(
    layout: &LibraryLayout,
    snapshot: &ResearchSnapshotV2,
    id: ResearchBackupId,
    now: crate::Timestamp,
) -> Result<ResearchBackupManifest, ResearchStorageError> {
    fs::create_dir_all(&layout.backups_dir)?;
    let backup_name = format!("{}.snapshot.json", sanitize_storage_token(id.as_str()));
    let backup_path = layout.backups_dir.join(&backup_name);
    atomic_write_encrypted_json(&backup_path, snapshot)?;
    let manifest = ResearchBackupManifest {
        id,
        library_id: snapshot.library.id.clone(),
        created_at: now,
        relative_path: format!("backups/{backup_name}"),
        schema_version: snapshot.library.schema_version,
        reference_count: snapshot.references.len(),
        attachment_count: snapshot.attachments.len(),
        annotation_count: snapshot.annotations.len(),
        note_count: snapshot.notes.len(),
    };
    atomic_write_json(
        &layout.backups_dir.join(format!(
            "{}.manifest.json",
            sanitize_storage_token(manifest.id.as_str())
        )),
        &manifest,
    )?;
    Ok(manifest)
}

pub fn load_library_backup(
    layout: &LibraryLayout,
    manifest: &ResearchBackupManifest,
) -> Result<ResearchSnapshotV2, ResearchStorageError> {
    let backup_path = safe_library_relative_path(layout, &manifest.relative_path)?;
    let value = read_research_json_value(&backup_path)?;
    migrate_research_snapshot_value(value).map(|result| result.snapshot)
}

pub fn create_library_backup_archive(
    snapshot: ResearchSnapshotV2,
    id: ResearchBackupId,
    now: crate::Timestamp,
) -> ResearchLibraryBackupArchive {
    let backup_name = format!("{}.snapshot.json", sanitize_storage_token(id.as_str()));
    let manifest = ResearchBackupManifest {
        id,
        library_id: snapshot.library.id.clone(),
        created_at: now,
        relative_path: format!("backups/{backup_name}"),
        schema_version: snapshot.library.schema_version,
        reference_count: snapshot.references.len(),
        attachment_count: snapshot.attachments.len(),
        annotation_count: snapshot.annotations.len(),
        note_count: snapshot.notes.len(),
    };
    ResearchLibraryBackupArchive {
        schema_version: 1,
        manifest,
        snapshot,
    }
}

pub fn export_library_backup_archive_zip(
    archive: &ResearchLibraryBackupArchive,
) -> Result<Vec<u8>, String> {
    ensure_library_backup_archive_valid(archive)?;
    let archive_json = serde_json::to_string_pretty(archive).map_err(|error| error.to_string())?;
    let manifest_json =
        serde_json::to_string_pretty(&archive.manifest).map_err(|error| error.to_string())?;
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "tench-research-library-backup.json",
        &archive_json,
        options,
    )
    .map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "manifest.json",
        &manifest_json,
        options,
    )
    .map_err(|error| error.to_string())?;
    writer
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| error.to_string())
}

pub fn import_library_backup_archive_zip(
    bytes: &[u8],
) -> Result<ResearchLibraryBackupArchive, String> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|error| error.to_string())?;
    let mut archive_json = String::new();
    archive
        .by_name("tench-research-library-backup.json")
        .map_err(|error| format!("missing tench-research-library-backup.json: {error}"))?
        .read_to_string(&mut archive_json)
        .map_err(|error| error.to_string())?;
    let mut backup_value: Value =
        serde_json::from_str(&archive_json).map_err(|error| error.to_string())?;
    if backup_value.get("snapshot").is_some() {
        let snapshot_value = backup_value
            .get_mut("snapshot")
            .expect("snapshot was checked")
            .take();
        let migrated =
            migrate_research_snapshot_value(snapshot_value).map_err(|error| error.to_string())?;
        if migrated.migration.changed() {
            if let Some(manifest) = backup_value
                .get_mut("manifest")
                .and_then(Value::as_object_mut)
            {
                manifest.insert(
                    "schema_version".to_string(),
                    json!(migrated.snapshot.library.schema_version),
                );
                manifest.insert(
                    "reference_count".to_string(),
                    json!(migrated.snapshot.references.len()),
                );
                manifest.insert(
                    "attachment_count".to_string(),
                    json!(migrated.snapshot.attachments.len()),
                );
                manifest.insert(
                    "annotation_count".to_string(),
                    json!(migrated.snapshot.annotations.len()),
                );
                manifest.insert(
                    "note_count".to_string(),
                    json!(migrated.snapshot.notes.len()),
                );
            }
        }
        let snapshot_slot = backup_value
            .get_mut("snapshot")
            .expect("snapshot was checked");
        *snapshot_slot =
            serde_json::to_value(migrated.snapshot).map_err(|error| error.to_string())?;
    }
    let backup: ResearchLibraryBackupArchive =
        serde_json::from_value(backup_value).map_err(|error| error.to_string())?;
    ensure_library_backup_archive_valid(&backup)?;
    Ok(backup)
}

pub fn restore_library_backup_archive(
    layout: &LibraryLayout,
    archive: &ResearchLibraryBackupArchive,
) -> Result<ResearchBackupManifest, String> {
    ensure_library_backup_archive_valid(archive)?;
    save_library_snapshot(layout, &archive.snapshot).map_err(|error| error.to_string())?;
    create_library_backup(
        layout,
        &archive.snapshot,
        archive.manifest.id.clone(),
        archive.manifest.created_at.clone(),
    )
    .map_err(|error| error.to_string())
}

fn ensure_library_backup_archive_valid(
    archive: &ResearchLibraryBackupArchive,
) -> Result<(), String> {
    if archive.schema_version != 1 {
        return Err(format!(
            "unsupported research library backup archive schema {}",
            archive.schema_version
        ));
    }
    if archive.manifest.library_id != archive.snapshot.library.id {
        return Err("research backup archive library id mismatch".to_string());
    }
    if archive.manifest.schema_version != archive.snapshot.library.schema_version {
        return Err("research backup archive schema version mismatch".to_string());
    }
    if archive.manifest.reference_count != archive.snapshot.references.len()
        || archive.manifest.attachment_count != archive.snapshot.attachments.len()
        || archive.manifest.annotation_count != archive.snapshot.annotations.len()
        || archive.manifest.note_count != archive.snapshot.notes.len()
    {
        return Err("research backup archive manifest counts do not match snapshot".to_string());
    }
    Ok(())
}

fn sanitize_storage_token(value: &str) -> String {
    let token = value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '_')
        .take(80)
        .collect::<String>();
    if token.is_empty() {
        "backup".to_string()
    } else {
        token
    }
}
