use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchRecentLibrary {
    pub library_id: crate::LibraryId,
    pub name: String,
    pub root_dir: String,
    pub last_opened_at: crate::Timestamp,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchRecentLibraryRegistry {
    pub schema_version: u32,
    #[serde(default)]
    pub entries: Vec<ResearchRecentLibrary>,
}

impl Default for ResearchRecentLibraryRegistry {
    fn default() -> Self {
        Self {
            schema_version: 1,
            entries: Vec::new(),
        }
    }
}

pub fn update_recent_research_library(
    mut registry: ResearchRecentLibraryRegistry,
    snapshot: &ResearchSnapshotV2,
    opened_at: crate::Timestamp,
    max_entries: usize,
) -> ResearchRecentLibraryRegistry {
    registry.schema_version = 1;
    registry.entries.retain(|entry| {
        entry.library_id != snapshot.library.id || entry.root_dir != snapshot.library.root_dir
    });
    registry.entries.insert(
        0,
        ResearchRecentLibrary {
            library_id: snapshot.library.id.clone(),
            name: snapshot.library.name.clone(),
            root_dir: snapshot.library.root_dir.clone(),
            last_opened_at: opened_at,
            schema_version: snapshot.library.schema_version,
        },
    );
    registry.entries.truncate(max_entries.max(1));
    registry
}

pub fn save_recent_research_libraries(
    product_id: impl AsRef<str>,
    registry: &ResearchRecentLibraryRegistry,
) -> Result<PathBuf, ResearchStorageError> {
    let path = recent_research_libraries_path(product_id);
    atomic_write_json(&path, registry)?;
    Ok(path)
}

pub fn load_recent_research_libraries(
    product_id: impl AsRef<str>,
) -> Result<ResearchRecentLibraryRegistry, ResearchStorageError> {
    let path = recent_research_libraries_path(product_id);
    if !path.exists() {
        return Ok(ResearchRecentLibraryRegistry::default());
    }
    let data = fs::read_to_string(path)?;
    let registry = serde_json::from_str(&data)?;
    Ok(registry)
}

pub fn recent_research_libraries_path(product_id: impl AsRef<str>) -> PathBuf {
    research_storage_dir(product_id, ResearchStorageArea::Config).join("recent-libraries.json")
}
