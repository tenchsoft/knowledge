use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchStorageArea {
    Libraries,
    Attachments,
    Thumbnails,
    Indexes,
    Config,
    Cache,
    Recovery,
    Backups,
}

impl ResearchStorageArea {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Libraries => "libraries",
            Self::Attachments => "attachments",
            Self::Thumbnails => "thumbnails",
            Self::Indexes => "indexes",
            Self::Config => "config",
            Self::Cache => "cache",
            Self::Recovery => "recovery",
            Self::Backups => "backups",
        }
    }

    pub fn storage_class(self) -> StorageClass {
        match self {
            Self::Config => StorageClass::Config,
            Self::Cache | Self::Thumbnails | Self::Indexes => StorageClass::Cache,
            Self::Libraries | Self::Attachments | Self::Recovery | Self::Backups => {
                StorageClass::UserContent
            }
        }
    }
}

pub fn research_storage_policy(
    product_id: impl Into<String>,
    area: ResearchStorageArea,
) -> StoragePolicy {
    let class = area.storage_class();
    StoragePolicy {
        namespace: StorageNamespace {
            product_id: product_id.into(),
            class: class.clone(),
            name: area.as_str().to_string(),
        },
        boundary: DataBoundary::LocalOnly,
        encrypted_at_rest: matches!(
            area,
            ResearchStorageArea::Libraries
                | ResearchStorageArea::Attachments
                | ResearchStorageArea::Recovery
                | ResearchStorageArea::Backups
        ),
        user_exportable: matches!(class, StorageClass::UserContent | StorageClass::Config),
        retention_days: match area {
            ResearchStorageArea::Cache | ResearchStorageArea::Thumbnails => Some(30),
            ResearchStorageArea::Recovery => Some(7),
            _ => None,
        },
    }
}

pub fn research_app_data_dir(product_id: impl AsRef<str>) -> PathBuf {
    app_data_dir("Tench", product_id.as_ref())
}

pub fn research_storage_dir(product_id: impl AsRef<str>, area: ResearchStorageArea) -> PathBuf {
    research_app_data_dir(product_id).join(area.as_str())
}
