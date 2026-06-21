use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchMissingAttachment {
    pub attachment_id: crate::AttachmentId,
    pub reference_id: crate::ReferenceId,
    pub stored_path: String,
    pub expected_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchAttachmentPathRepair {
    pub attachment_id: crate::AttachmentId,
    pub new_stored_path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchAttachmentRepairReport {
    pub snapshot: ResearchSnapshotV2,
    #[serde(default)]
    pub repaired: Vec<crate::AttachmentId>,
    #[serde(default)]
    pub unresolved: Vec<ResearchMissingAttachment>,
    #[serde(default)]
    pub issues: Vec<String>,
}

impl LibraryLayout {
    pub fn for_root(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Self {
            library_json: root.join("library.json"),
            items_jsonl: root.join("items.jsonl"),
            attachments_jsonl: root.join("attachments.jsonl"),
            annotations_jsonl: root.join("annotations.jsonl"),
            notes_jsonl: root.join("notes.jsonl"),
            collections_jsonl: root.join("collections.jsonl"),
            tags_jsonl: root.join("tags.jsonl"),
            citekeys_json: root.join("citekeys.json"),
            attachments_dir: root.join("attachments"),
            index_dir: root.join("index"),
            thumbnails_dir: root.join("thumbnails"),
            recovery_dir: root.join("recovery"),
            backups_dir: root.join("backups"),
            root,
        }
    }

    pub fn user_content_dirs(&self) -> [&PathBuf; 4] {
        [
            &self.attachments_dir,
            &self.recovery_dir,
            &self.backups_dir,
            &self.root,
        ]
    }
}

pub fn find_missing_research_attachments(
    layout: &LibraryLayout,
    snapshot: &ResearchSnapshotV2,
) -> Vec<ResearchMissingAttachment> {
    snapshot
        .attachments
        .iter()
        .filter_map(|attachment| {
            let path = match attachment_storage_path(layout, &attachment.stored_path) {
                Ok(path) => path,
                Err(_) => {
                    return Some(ResearchMissingAttachment {
                        attachment_id: attachment.id.clone(),
                        reference_id: attachment.reference_id.clone(),
                        stored_path: attachment.stored_path.clone(),
                        expected_path: "invalid attachment path".to_string(),
                    });
                }
            };
            (!path.exists()).then(|| ResearchMissingAttachment {
                attachment_id: attachment.id.clone(),
                reference_id: attachment.reference_id.clone(),
                stored_path: attachment.stored_path.clone(),
                expected_path: path.to_string_lossy().to_string(),
            })
        })
        .collect()
}

pub fn repair_research_attachment_paths(
    mut snapshot: ResearchSnapshotV2,
    layout: &LibraryLayout,
    repairs: Vec<ResearchAttachmentPathRepair>,
    now: crate::Timestamp,
) -> ResearchAttachmentRepairReport {
    let mut repaired = Vec::new();
    let mut issues = Vec::new();

    for repair in repairs {
        let path = match attachment_storage_path(layout, &repair.new_stored_path) {
            Ok(path) => path,
            Err(error) => {
                issues.push(error.to_string());
                continue;
            }
        };
        if !path.exists() {
            issues.push(format!(
                "replacement path for attachment {} does not exist: {}",
                repair.attachment_id.as_str(),
                path.to_string_lossy()
            ));
            continue;
        }

        let Some(attachment) = snapshot
            .attachments
            .iter_mut()
            .find(|attachment| attachment.id == repair.attachment_id)
        else {
            issues.push(format!(
                "attachment {} does not exist",
                repair.attachment_id.as_str()
            ));
            continue;
        };

        attachment.stored_path = repair.new_stored_path;
        attachment.updated_at = now.clone();
        repaired.push(attachment.id.clone());
    }

    if !repaired.is_empty() {
        snapshot.library.updated_at = now;
    }
    let unresolved = find_missing_research_attachments(layout, &snapshot);

    ResearchAttachmentRepairReport {
        snapshot,
        repaired,
        unresolved,
        issues,
    }
}

pub(super) fn attachment_storage_path(
    layout: &LibraryLayout,
    stored_path: &str,
) -> Result<PathBuf, ResearchStorageError> {
    let path = PathBuf::from(stored_path);
    if path.is_absolute()
        || stored_path.contains('\\')
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(ResearchStorageError::InvalidPath(format!(
            "attachment path escapes the library root: {stored_path}"
        )));
    }
    Ok(layout.root.join(path))
}
