use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{fs, io};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tench_storage_core::{
    app_data_dir, decrypt_data, encrypt_data, is_encrypted_payload, DataBoundary, EncryptionKey,
    StorageClass, StorageNamespace, StoragePolicy,
};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::ResearchSnapshotV2;

crate::research_id_type!(ResearchBackupId);

mod attachments;
mod backup;
mod error;
mod file_io;
mod layout;
mod migration;
mod policy;
mod recent;

#[cfg(test)]
use attachments::attachment_storage_path;
pub use attachments::{
    find_missing_research_attachments, repair_research_attachment_paths,
    ResearchAttachmentPathRepair, ResearchAttachmentRepairReport, ResearchMissingAttachment,
};
pub use backup::{
    create_library_backup, create_library_backup_archive, export_library_backup_archive_zip,
    import_library_backup_archive_zip, load_library_backup, restore_library_backup_archive,
};
pub use error::ResearchStorageError;
use file_io::{atomic_write_encrypted_json, read_research_json_value, safe_library_relative_path};
pub use file_io::{
    atomic_write_json, load_library_snapshot, load_library_snapshot_with_report,
    save_library_snapshot,
};
pub use layout::{LibraryLayout, ResearchBackupManifest, ResearchLibraryBackupArchive};
pub use migration::{
    migrate_research_snapshot_value, ResearchMigrationReport, ResearchSnapshotLoadResult,
};
pub use policy::{
    research_app_data_dir, research_storage_dir, research_storage_policy, ResearchStorageArea,
};
pub use recent::{
    load_recent_research_libraries, recent_research_libraries_path, save_recent_research_libraries,
    update_recent_research_library, ResearchRecentLibrary, ResearchRecentLibraryRegistry,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn research_storage_policy_is_local_only() {
        let policy = research_storage_policy("tench-research", ResearchStorageArea::Libraries);

        assert_eq!(policy.namespace.product_id, "tench-research");
        assert_eq!(policy.namespace.class, StorageClass::UserContent);
        assert_eq!(policy.boundary, DataBoundary::LocalOnly);
        assert!(policy.encrypted_at_rest);
        assert!(policy.user_exportable);
    }

    #[test]
    fn library_layout_uses_stable_file_names() {
        let layout = LibraryLayout::for_root("/tmp/research/lib_1");

        assert!(layout.items_jsonl.ends_with("items.jsonl"));
        assert!(layout.attachments_dir.ends_with("attachments"));
        assert!(layout.index_dir.ends_with("index"));
    }

    #[test]
    fn saves_and_loads_library_snapshot_atomically() {
        let root = std::env::temp_dir()
            .join("tench-research-core-tests")
            .join(format!("lib-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let layout = LibraryLayout::for_root(&root);
        let locale = crate::ResearchLocale::parse("en-US").unwrap();
        let snapshot = ResearchSnapshotV2 {
            library: crate::ResearchLibrary {
                id: crate::LibraryId::from("lib"),
                name: "Library".to_string(),
                root_dir: root.to_string_lossy().to_string(),
                created_at: crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
                updated_at: crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
                schema_version: 1,
                settings: crate::LibrarySettings {
                    default_locale: locale.clone(),
                    citation_locale: locale,
                    default_citation_style: "apa".to_string(),
                    attachment_policy: crate::AttachmentStoragePolicy::CopyIntoLibrary,
                },
            },
            references: Vec::new(),
            attachments: Vec::new(),
            annotations: Vec::new(),
            notes: Vec::new(),
            collections: Vec::new(),
            tags: Vec::new(),
        };

        save_library_snapshot(&layout, &snapshot).expect("save snapshot");
        let loaded = load_library_snapshot(&layout).expect("load snapshot");

        assert_eq!(loaded.library.id, snapshot.library.id);
        assert!(layout.library_json.exists());
        let raw = fs::read(&layout.library_json).expect("read encrypted snapshot");
        assert!(is_encrypted_payload(&raw));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn loads_legacy_snapshot_with_migration_report() {
        let root = std::env::temp_dir()
            .join("tench-research-core-tests")
            .join(format!("legacy-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let layout = LibraryLayout::for_root(&root);
        fs::create_dir_all(&root).expect("root");
        let legacy = json!({
            "library": {
                "id": "legacy-lib",
                "name": "Legacy Library",
                "root_dir": root.to_string_lossy(),
                "created_at": "2026-05-04T00:00:00Z",
                "updated_at": "2026-05-04T00:00:00Z"
            },
            "references": [],
            "attachments": []
        });
        fs::write(
            &layout.library_json,
            serde_json::to_vec_pretty(&legacy).expect("legacy json"),
        )
        .expect("write legacy");

        let result = load_library_snapshot_with_report(&layout).expect("load legacy");
        let loaded = load_library_snapshot(&layout).expect("load migrated");

        assert_eq!(result.migration.from_schema_version, 0);
        assert_eq!(
            result.migration.to_schema_version,
            crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION
        );
        assert!(result.migration.changed());
        assert!(result
            .migration
            .steps
            .contains(&"added default library settings".to_string()));
        assert_eq!(
            result.snapshot.library.schema_version,
            crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION
        );
        assert_eq!(
            result.snapshot.library.settings.default_citation_style,
            "apa"
        );
        assert_eq!(loaded.library.id, crate::LibraryId::from("legacy-lib"));
        assert!(loaded.notes.is_empty());
        assert!(loaded.collections.is_empty());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn recent_libraries_registry_deduplicates_and_persists_locally() {
        let product_id = format!("tench-research-core-recent-{}", std::process::id());
        let config_dir = research_storage_dir(&product_id, ResearchStorageArea::Config);
        let _ = fs::remove_dir_all(config_dir.parent().unwrap_or(&config_dir));
        let locale = crate::ResearchLocale::parse("en-US").unwrap();
        let first = crate::new_research_library_snapshot(
            crate::LibraryId::from("lib-1"),
            "First Library",
            "/tmp/lib-1",
            locale.clone(),
            crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
        );
        let second = crate::new_research_library_snapshot(
            crate::LibraryId::from("lib-2"),
            "Second Library",
            "/tmp/lib-2",
            locale,
            crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
        );

        let registry = update_recent_research_library(
            ResearchRecentLibraryRegistry::default(),
            &first,
            crate::Timestamp("2026-05-04T00:01:00Z".to_string()),
            10,
        );
        let registry = update_recent_research_library(
            registry,
            &second,
            crate::Timestamp("2026-05-04T00:02:00Z".to_string()),
            10,
        );
        let registry = update_recent_research_library(
            registry,
            &first,
            crate::Timestamp("2026-05-04T00:03:00Z".to_string()),
            10,
        );

        let path = save_recent_research_libraries(&product_id, &registry).expect("save recent");
        let loaded = load_recent_research_libraries(&product_id).expect("load recent");

        assert!(path.ends_with("recent-libraries.json"));
        assert_eq!(loaded.entries.len(), 2);
        assert_eq!(
            loaded.entries[0].library_id,
            crate::LibraryId::from("lib-1")
        );
        assert_eq!(loaded.entries[0].last_opened_at.0, "2026-05-04T00:03:00Z");
        assert_eq!(
            loaded.entries[1].library_id,
            crate::LibraryId::from("lib-2")
        );
        assert_eq!(
            loaded.entries[0].schema_version,
            crate::CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION
        );
        let _ = fs::remove_dir_all(config_dir.parent().unwrap_or(&config_dir));
    }

    #[test]
    fn missing_attachment_repair_updates_paths_and_reports_unresolved() {
        let root = std::env::temp_dir()
            .join("tench-research-core-tests")
            .join(format!("repair-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let layout = LibraryLayout::for_root(&root);
        fs::create_dir_all(root.join("attachments")).expect("attachments dir");
        fs::write(root.join("attachments/repaired.pdf"), b"%PDF").expect("replacement");
        let locale = crate::ResearchLocale::parse("en-US").unwrap();
        let mut snapshot = crate::new_research_library_snapshot(
            crate::LibraryId::from("lib"),
            "Library",
            root.to_string_lossy().to_string(),
            locale,
            crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
        );
        snapshot
            .references
            .push(crate::reference_from_minimal_metadata(
                "ref-1",
                crate::ReferenceKind::JournalArticle,
                "Paper",
                Some(2026),
                "2026-05-04T00:00:00Z",
            ));
        snapshot.attachments.push(crate::Attachment {
            id: crate::AttachmentId::from("att-1"),
            reference_id: crate::ReferenceId::from("ref-1"),
            kind: crate::AttachmentKind::Pdf,
            title: "Missing PDF".to_string(),
            stored_path: "attachments/missing.pdf".to_string(),
            original_path: None,
            mime_type: "application/pdf".to_string(),
            size_bytes: 4,
            content_hash: "hash".to_string(),
            page_count: None,
            text_indexed: false,
            created_at: crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
            updated_at: crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
        });
        snapshot.attachments.push(crate::Attachment {
            id: crate::AttachmentId::from("att-2"),
            reference_id: crate::ReferenceId::from("ref-1"),
            kind: crate::AttachmentKind::Pdf,
            title: "Still missing".to_string(),
            stored_path: "attachments/still-missing.pdf".to_string(),
            original_path: None,
            mime_type: "application/pdf".to_string(),
            size_bytes: 4,
            content_hash: "hash-2".to_string(),
            page_count: None,
            text_indexed: false,
            created_at: crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
            updated_at: crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
        });

        let missing = find_missing_research_attachments(&layout, &snapshot);
        let report = repair_research_attachment_paths(
            snapshot,
            &layout,
            vec![ResearchAttachmentPathRepair {
                attachment_id: crate::AttachmentId::from("att-1"),
                new_stored_path: "attachments/repaired.pdf".to_string(),
            }],
            crate::Timestamp("2026-05-04T00:05:00Z".to_string()),
        );

        assert_eq!(missing.len(), 2);
        assert_eq!(report.repaired, vec![crate::AttachmentId::from("att-1")]);
        assert_eq!(report.unresolved.len(), 1);
        assert_eq!(
            report.snapshot.attachments[0].stored_path,
            "attachments/repaired.pdf"
        );
        assert_eq!(report.snapshot.library.updated_at.0, "2026-05-04T00:05:00Z");
        assert!(report.issues.is_empty());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn research_attachment_paths_cannot_escape_library_security_regression() {
        let layout = LibraryLayout::for_root("/tmp/research/lib");

        assert!(attachment_storage_path(&layout, "../outside.pdf").is_err());
        assert!(attachment_storage_path(&layout, "/etc/passwd").is_err());
        assert!(attachment_storage_path(&layout, "attachments\\outside.pdf").is_err());
        assert!(attachment_storage_path(&layout, "attachments/paper.pdf").is_ok());
    }

    #[test]
    fn research_backup_manifest_path_cannot_escape_library_security_regression() {
        let layout = LibraryLayout::for_root("/tmp/research/lib");
        let manifest = ResearchBackupManifest {
            id: ResearchBackupId::from("backup-escape"),
            library_id: crate::LibraryId::from("lib"),
            created_at: crate::Timestamp("2026-05-05T00:00:00Z".to_string()),
            relative_path: "../outside.json".to_string(),
            schema_version: 1,
            reference_count: 0,
            attachment_count: 0,
            annotation_count: 0,
            note_count: 0,
        };

        let result = load_library_backup(&layout, &manifest);

        assert!(matches!(result, Err(ResearchStorageError::InvalidPath(_))));
    }

    #[test]
    fn creates_and_loads_library_backup_manifest() {
        let root = std::env::temp_dir()
            .join("tench-research-core-tests")
            .join(format!("backup-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let layout = LibraryLayout::for_root(&root);
        let locale = crate::ResearchLocale::parse("en-US").unwrap();
        let snapshot = crate::new_research_library_snapshot(
            crate::LibraryId::from("lib"),
            "Library",
            root.to_string_lossy().to_string(),
            locale,
            crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
        );

        let manifest = create_library_backup(
            &layout,
            &snapshot,
            ResearchBackupId::from("backup_1"),
            crate::Timestamp("2026-05-04T00:01:00Z".to_string()),
        )
        .expect("backup");
        let restored = load_library_backup(&layout, &manifest).expect("restore backup");

        assert_eq!(manifest.reference_count, 0);
        assert_eq!(restored.library.id, snapshot.library.id);
        assert!(layout.backups_dir.join("backup_1.manifest.json").exists());
        let backup_raw = fs::read(layout.backups_dir.join("backup_1.snapshot.json"))
            .expect("read encrypted backup");
        assert!(is_encrypted_payload(&backup_raw));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn library_backup_archive_zip_round_trips_and_restores_snapshot() {
        let root = std::env::temp_dir()
            .join("tench-research-core-tests")
            .join(format!("backup-archive-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let layout = LibraryLayout::for_root(&root);
        let locale = crate::ResearchLocale::parse("ko-KR").unwrap();
        let snapshot = crate::new_research_library_snapshot(
            crate::LibraryId::from("lib-archive"),
            "Archive Library",
            root.to_string_lossy().to_string(),
            locale,
            crate::Timestamp("2026-05-04T00:00:00Z".to_string()),
        );
        let archive = create_library_backup_archive(
            snapshot.clone(),
            ResearchBackupId::from("backup_archive_1"),
            crate::Timestamp("2026-05-04T00:02:00Z".to_string()),
        );

        let bytes = export_library_backup_archive_zip(&archive).expect("export archive");
        let imported = import_library_backup_archive_zip(&bytes).expect("import archive");
        let manifest =
            restore_library_backup_archive(&layout, &imported).expect("restore archive snapshot");
        let loaded = load_library_snapshot(&layout).expect("load restored snapshot");

        assert_eq!(imported.snapshot.library.id, snapshot.library.id);
        assert_eq!(manifest.reference_count, 0);
        assert_eq!(loaded.library.name, "Archive Library");
        assert!(layout
            .backups_dir
            .join("backup_archive_1.manifest.json")
            .exists());
        let _ = fs::remove_dir_all(&root);
    }
}
