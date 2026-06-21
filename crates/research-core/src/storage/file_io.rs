use super::*;

pub fn save_library_snapshot(
    layout: &LibraryLayout,
    snapshot: &ResearchSnapshotV2,
) -> Result<(), ResearchStorageError> {
    fs::create_dir_all(&layout.root)?;
    atomic_write_encrypted_json(&layout.library_json, snapshot)
}

pub fn load_library_snapshot(
    layout: &LibraryLayout,
) -> Result<ResearchSnapshotV2, ResearchStorageError> {
    load_library_snapshot_with_report(layout).map(|result| result.snapshot)
}

pub fn load_library_snapshot_with_report(
    layout: &LibraryLayout,
) -> Result<ResearchSnapshotLoadResult, ResearchStorageError> {
    let value = read_research_json_value(&layout.library_json)?;
    migrate_research_snapshot_value(value)
}

static ATOMIC_WRITE_COUNTER: AtomicU64 = AtomicU64::new(0);
fn counter_next() -> u64 {
    ATOMIC_WRITE_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn atomic_write_json<T: Serialize>(
    path: &std::path::Path,
    value: &T,
) -> Result<(), ResearchStorageError> {
    let data = serde_json::to_vec_pretty(value)?;
    atomic_write_bytes(path, &data)
}

pub(super) fn atomic_write_encrypted_json<T: Serialize>(
    path: &std::path::Path,
    value: &T,
) -> Result<(), ResearchStorageError> {
    let data = serde_json::to_vec_pretty(value)?;
    let key = EncryptionKey::from_machine();
    let encrypted = encrypt_data(&data, &key.derived_key())
        .map_err(|error| ResearchStorageError::Encryption(error.to_string()))?;
    atomic_write_bytes(path, &encrypted)
}

fn atomic_write_bytes(path: &std::path::Path, data: &[u8]) -> Result<(), ResearchStorageError> {
    let parent = path.parent().ok_or_else(|| {
        ResearchStorageError::InvalidPath("storage path requires a parent directory".to_string())
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
    {
        let mut file = std::fs::File::create(&tmp)?;
        file.write_all(data)?;
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

pub(super) fn read_research_json_value(
    path: &std::path::Path,
) -> Result<Value, ResearchStorageError> {
    let raw = fs::read(path)?;
    let data = if is_encrypted_payload(&raw) {
        let key = EncryptionKey::from_machine();
        decrypt_data(&raw, &key.derived_key())
            .map_err(|error| ResearchStorageError::Encryption(error.to_string()))?
    } else {
        raw
    };
    serde_json::from_slice(&data).map_err(ResearchStorageError::Serde)
}

pub(super) fn safe_library_relative_path(
    layout: &LibraryLayout,
    relative_path: &str,
) -> Result<PathBuf, ResearchStorageError> {
    let path = PathBuf::from(relative_path);
    if path.is_absolute()
        || relative_path.contains('\\')
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(ResearchStorageError::InvalidPath(format!(
            "research storage path escapes the library root: {relative_path}"
        )));
    }
    Ok(layout.root.join(path))
}

fn sync_parent_dir(parent: &std::path::Path) -> Result<(), ResearchStorageError> {
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
