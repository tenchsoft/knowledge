use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchMigrationReport {
    pub from_schema_version: u32,
    pub to_schema_version: u32,
    #[serde(default)]
    pub steps: Vec<String>,
}

impl ResearchMigrationReport {
    pub fn changed(&self) -> bool {
        self.from_schema_version != self.to_schema_version || !self.steps.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchSnapshotLoadResult {
    pub snapshot: ResearchSnapshotV2,
    pub migration: ResearchMigrationReport,
}

pub fn migrate_research_snapshot_value(
    mut value: Value,
) -> Result<ResearchSnapshotLoadResult, ResearchStorageError> {
    let from_schema_version = snapshot_schema_version(&value)?;
    if from_schema_version > crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION {
        return Err(ResearchStorageError::UnsupportedSchema(from_schema_version));
    }

    let mut steps = Vec::new();
    if from_schema_version == 0 {
        migrate_research_snapshot_v0_to_v1(&mut value, &mut steps)?;
    }

    let mut snapshot: ResearchSnapshotV2 = serde_json::from_value(value)?;
    if snapshot.library.schema_version == 0 {
        snapshot.library.schema_version = crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION;
        steps.push("set missing library schema version".to_string());
    }
    if snapshot.library.schema_version > crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION {
        return Err(ResearchStorageError::UnsupportedSchema(
            snapshot.library.schema_version,
        ));
    }

    Ok(ResearchSnapshotLoadResult {
        snapshot,
        migration: ResearchMigrationReport {
            from_schema_version,
            to_schema_version: crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION,
            steps,
        },
    })
}

fn snapshot_schema_version(value: &Value) -> Result<u32, ResearchStorageError> {
    let Some(library) = value.get("library") else {
        return Err(ResearchStorageError::InvalidSnapshot(
            "research snapshot requires library metadata".to_string(),
        ));
    };
    Ok(library
        .get("schema_version")
        .and_then(Value::as_u64)
        .map(|version| version as u32)
        .unwrap_or(0))
}

fn migrate_research_snapshot_v0_to_v1(
    value: &mut Value,
    steps: &mut Vec<String>,
) -> Result<(), ResearchStorageError> {
    let object = value.as_object_mut().ok_or_else(|| {
        ResearchStorageError::InvalidSnapshot("research snapshot must be a JSON object".to_string())
    })?;
    let library = object
        .get_mut("library")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            ResearchStorageError::InvalidSnapshot(
                "research snapshot requires library metadata".to_string(),
            )
        })?;

    if !library.contains_key("schema_version") {
        library.insert(
            "schema_version".to_string(),
            json!(crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION),
        );
        steps.push("set library schema version to 1".to_string());
    }
    if !library.contains_key("settings") {
        library.insert(
            "settings".to_string(),
            serde_json::to_value(crate::LibrarySettings::default())?,
        );
        steps.push("added default library settings".to_string());
    }

    for field in [
        "references",
        "attachments",
        "annotations",
        "notes",
        "collections",
        "tags",
    ] {
        if !object.contains_key(field) {
            object.insert(field.to_string(), json!([]));
            steps.push(format!("added empty {field} collection"));
        }
    }

    Ok(())
}
