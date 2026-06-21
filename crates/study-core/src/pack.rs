use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
use std::io::{Cursor, Read, Write};
use tench_job_core::{JobDescriptor, JobProgress, JobState};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    compute_locale_coverage, AssessmentId, ContentLocale, CurriculumId, CurriculumIssueSeverity,
    CurriculumNodeId, GlossaryTermId, LearnerProgress, LearningVisualId, LocaleCoverage,
    PracticeItemId, PublishedCurriculumPack,
};

crate::study_id_type!(ContentPackId);
crate::study_id_type!(AssetId);
crate::study_id_type!(ContentPackArchiveId);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContentPackManifest {
    pub id: ContentPackId,
    pub title: String,
    pub curriculum_id: CurriculumId,
    pub version: String,
    pub default_locale: ContentLocale,
    #[serde(default)]
    pub required_locales: Vec<ContentLocale>,
    #[serde(default)]
    pub provided_locales: Vec<ContentLocale>,
    #[serde(default)]
    pub lessons: Vec<LessonManifest>,
    #[serde(default)]
    pub assets: Vec<AssetManifest>,
    #[serde(default)]
    pub visuals: Vec<LearningVisualId>,
    #[serde(default)]
    pub practice_items: Vec<PracticeItemId>,
    #[serde(default)]
    pub assessments: Vec<AssessmentId>,
    #[serde(default)]
    pub glossary_terms: Vec<GlossaryTermId>,
    pub integrity: PackIntegrity,
}

impl ContentPackManifest {
    pub fn locale_coverage(&self) -> LocaleCoverage {
        compute_locale_coverage(&self.required_locales, &self.provided_locales)
    }

    pub fn validate_for_release(&self) -> PackValidationReport {
        let mut issues = Vec::new();
        if self.lessons.is_empty() {
            issues.push(PackValidationIssue::error(
                "lessons_required",
                "content pack requires at least one lesson",
            ));
        }
        if self.glossary_terms.is_empty() {
            issues.push(PackValidationIssue::warning(
                "glossary_empty",
                "content pack has no glossary terms",
            ));
        }
        let coverage = self.locale_coverage();
        if !coverage.missing.is_empty() {
            issues.push(PackValidationIssue::error(
                "locale_coverage_incomplete",
                "content pack is missing required locales",
            ));
        }
        for asset in &self.assets {
            if asset.alt_text_required && asset.alt_text.trim().is_empty() {
                issues.push(PackValidationIssue::error(
                    "asset_alt_text_required",
                    format!("asset {} requires alt text", asset.id.as_str()),
                ));
            }
        }
        if self.integrity.content_hash.trim().is_empty() {
            issues.push(PackValidationIssue::error(
                "content_hash_required",
                "content pack requires an integrity hash",
            ));
        }
        PackValidationReport { issues, coverage }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LessonManifest {
    pub node_id: CurriculumNodeId,
    pub locale: ContentLocale,
    pub path: String,
    #[serde(default)]
    pub estimated_minutes: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetManifest {
    pub id: AssetId,
    pub path: String,
    pub media_type: String,
    pub bytes: u64,
    pub content_hash: String,
    pub alt_text_required: bool,
    #[serde(default)]
    pub alt_text: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackIntegrity {
    pub algorithm: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackValidationReport {
    pub issues: Vec<PackValidationIssue>,
    pub coverage: LocaleCoverage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentPackArchive {
    pub id: ContentPackArchiveId,
    pub schema_version: u32,
    pub pack: PublishedCurriculumPack,
    pub exported_at: String,
    pub validation: PackValidationReport,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstalledContentPackRegistry {
    #[serde(default)]
    pub entries: Vec<InstalledContentPackEntry>,
}

impl InstalledContentPackRegistry {
    pub fn active_pack(&self, pack_id: &ContentPackId) -> Option<&ContentPackArchive> {
        self.entries
            .iter()
            .find(|entry| &entry.pack_id == pack_id)
            .and_then(InstalledContentPackEntry::active_archive)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstalledContentPackEntry {
    pub pack_id: ContentPackId,
    pub active_version: String,
    pub installed_at: String,
    pub updated_at: String,
    pub owner: String,
    pub license_name: String,
    pub update_policy: crate::PackUpdatePolicy,
    #[serde(default)]
    pub versions: Vec<ContentPackArchive>,
}

impl InstalledContentPackEntry {
    pub fn active_archive(&self) -> Option<&ContentPackArchive> {
        self.versions
            .iter()
            .find(|archive| archive.pack.manifest.version == self.active_version)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentPackInstallResult {
    pub registry: InstalledContentPackRegistry,
    pub job: JobDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentPackUpdateResult {
    pub registry: InstalledContentPackRegistry,
    pub migration: ProgressMigrationReport,
    pub job: JobDescriptor,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentPackRollbackResult {
    pub registry: InstalledContentPackRegistry,
    pub migration: ProgressMigrationReport,
    pub job: JobDescriptor,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgressMigrationReport {
    pub pack_id: ContentPackId,
    pub from_version: Option<String>,
    pub to_version: String,
    #[serde(default)]
    pub preserved_progress_nodes: Vec<CurriculumNodeId>,
    #[serde(default)]
    pub orphaned_progress_nodes: Vec<CurriculumNodeId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyJobKind {
    PackImport,
    PackExport,
    PackValidate,
    PackUpdate,
    PackRollback,
    CodeGrade,
}

impl StudyJobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PackImport => "study.pack_import",
            Self::PackExport => "study.pack_export",
            Self::PackValidate => "study.pack_validate",
            Self::PackUpdate => "study.pack_update",
            Self::PackRollback => "study.pack_rollback",
            Self::CodeGrade => "study.code_grade",
        }
    }
}

pub fn study_job_descriptor(
    id: impl Into<String>,
    kind: StudyJobKind,
    state: JobState,
    pack_id: impl Into<String>,
) -> JobDescriptor {
    JobDescriptor {
        id: id.into(),
        product_id: "tench-study".to_string(),
        kind: kind.as_str().to_string(),
        state,
        progress: Some(JobProgress {
            current: 0,
            total: None,
            message: None,
        }),
        payload: json!({ "pack_id": pack_id.into() }),
    }
}

pub fn validate_published_content_pack(pack: &PublishedCurriculumPack) -> PackValidationReport {
    let mut report = pack.manifest.validate_for_release();
    for issue in pack.curriculum.validate().issues {
        if issue.severity == CurriculumIssueSeverity::Error {
            report
                .issues
                .push(PackValidationIssue::error(issue.code, issue.message));
        }
    }
    report
}

pub fn export_published_content_pack_archive_json(
    pack: PublishedCurriculumPack,
    exported_at: String,
) -> Result<String, serde_json::Error> {
    let archive = ContentPackArchive {
        id: ContentPackArchiveId::from(format!(
            "{}-{}",
            pack.manifest.id.as_str(),
            safe_archive_version_component(&pack.manifest.version)
        )),
        schema_version: 1,
        validation: validate_published_content_pack(&pack),
        pack,
        exported_at,
    };
    serde_json::to_string_pretty(&archive)
}

pub fn import_published_content_pack_archive_json(
    text: &str,
) -> Result<ContentPackArchive, String> {
    let archive: ContentPackArchive =
        serde_json::from_str(text).map_err(|error| error.to_string())?;
    ensure_archive_valid(&archive)?;
    Ok(archive)
}

pub fn export_published_content_pack_archive_zip(
    pack: PublishedCurriculumPack,
    exported_at: String,
) -> Result<Vec<u8>, String> {
    let archive_json = export_published_content_pack_archive_json(pack, exported_at)
        .map_err(|error| error.to_string())?;
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    writer
        .start_file("tench-study-pack.json", options)
        .map_err(|error| error.to_string())?;
    writer
        .write_all(archive_json.as_bytes())
        .map_err(|error| error.to_string())?;
    writer
        .start_file("manifest.json", options)
        .map_err(|error| error.to_string())?;
    let archive: ContentPackArchive =
        serde_json::from_str(&archive_json).map_err(|error| error.to_string())?;
    let manifest =
        serde_json::to_string_pretty(&archive.pack.manifest).map_err(|error| error.to_string())?;
    writer
        .write_all(manifest.as_bytes())
        .map_err(|error| error.to_string())?;
    writer
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| error.to_string())
}

pub fn import_published_content_pack_archive_zip(
    bytes: &[u8],
) -> Result<ContentPackArchive, String> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|error| error.to_string())?;
    crate::storage::check_study_archive_limits(&mut archive)?;
    let mut json = String::new();
    archive
        .by_name("tench-study-pack.json")
        .map_err(|error| format!("missing tench-study-pack.json: {error}"))?
        .read_to_string(&mut json)
        .map_err(|error| error.to_string())?;
    let pack_archive = import_published_content_pack_archive_json(&json)?;
    ensure_archive_valid(&pack_archive)?;
    Ok(pack_archive)
}

pub fn install_content_pack_archive(
    mut registry: InstalledContentPackRegistry,
    archive: ContentPackArchive,
    installed_at: String,
) -> Result<ContentPackInstallResult, String> {
    ensure_archive_valid(&archive)?;
    let pack_id = archive.pack.manifest.id.clone();
    let version = archive.pack.manifest.version.clone();
    let mut job = study_job_descriptor(
        format!("install-{}-{version}", pack_id.as_str()),
        StudyJobKind::PackImport,
        JobState::Running,
        pack_id.as_str(),
    );

    if let Some(entry) = registry
        .entries
        .iter_mut()
        .find(|entry| entry.pack_id == pack_id)
    {
        upsert_archive_version(entry, archive);
        entry.active_version = version;
        entry.updated_at = installed_at;
    } else {
        registry.entries.push(InstalledContentPackEntry {
            pack_id: pack_id.clone(),
            active_version: version,
            installed_at: installed_at.clone(),
            updated_at: installed_at,
            owner: archive.pack.owner.name.clone(),
            license_name: archive.pack.license.name.clone(),
            update_policy: archive.pack.update_policy.clone(),
            versions: vec![archive],
        });
    }

    finish_pack_job(&mut job, 1, "Content pack installed");
    Ok(ContentPackInstallResult { registry, job })
}

pub fn update_content_pack_archive(
    mut registry: InstalledContentPackRegistry,
    archive: ContentPackArchive,
    progress: &[LearnerProgress],
    updated_at: String,
) -> Result<ContentPackUpdateResult, String> {
    ensure_archive_valid(&archive)?;
    let pack_id = archive.pack.manifest.id.clone();
    let target_version = archive.pack.manifest.version.clone();
    let entry = registry
        .entries
        .iter_mut()
        .find(|entry| entry.pack_id == pack_id)
        .ok_or_else(|| format!("pack {} is not installed", pack_id.as_str()))?;
    let from_version = Some(entry.active_version.clone());
    let migration = build_progress_migration_report(
        pack_id.clone(),
        from_version,
        target_version.clone(),
        &archive,
        progress,
        archive
            .pack
            .update_policy
            .preserve_progress_on_stable_node_ids,
    );
    upsert_archive_version(entry, archive);
    entry.active_version = target_version;
    entry.updated_at = updated_at;
    entry.update_policy = entry
        .active_archive()
        .map(|archive| archive.pack.update_policy.clone())
        .unwrap_or_else(|| entry.update_policy.clone());
    let mut job = study_job_descriptor(
        format!("update-{}-{}", pack_id.as_str(), entry.active_version),
        StudyJobKind::PackUpdate,
        JobState::Running,
        pack_id.as_str(),
    );
    finish_pack_job(&mut job, 1, "Content pack updated");

    Ok(ContentPackUpdateResult {
        registry,
        migration,
        job,
    })
}

pub fn rollback_content_pack(
    mut registry: InstalledContentPackRegistry,
    pack_id: ContentPackId,
    target_version: String,
    progress: &[LearnerProgress],
    rolled_back_at: String,
) -> Result<ContentPackRollbackResult, String> {
    let entry = registry
        .entries
        .iter_mut()
        .find(|entry| entry.pack_id == pack_id)
        .ok_or_else(|| format!("pack {} is not installed", pack_id.as_str()))?;
    let archive = entry
        .versions
        .iter()
        .find(|archive| archive.pack.manifest.version == target_version)
        .cloned()
        .ok_or_else(|| {
            format!(
                "version {} is not available for pack {}",
                target_version,
                pack_id.as_str()
            )
        })?;
    let from_version = Some(entry.active_version.clone());
    let migration = build_progress_migration_report(
        pack_id.clone(),
        from_version,
        target_version.clone(),
        &archive,
        progress,
        archive
            .pack
            .update_policy
            .preserve_progress_on_stable_node_ids,
    );
    entry.active_version = target_version.clone();
    entry.updated_at = rolled_back_at;
    entry.update_policy = archive.pack.update_policy.clone();
    let mut job = study_job_descriptor(
        format!("rollback-{}-{target_version}", pack_id.as_str()),
        StudyJobKind::PackRollback,
        JobState::Running,
        pack_id.as_str(),
    );
    finish_pack_job(&mut job, 1, "Content pack rolled back");

    Ok(ContentPackRollbackResult {
        registry,
        migration,
        job,
    })
}

impl PackValidationReport {
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity == PackValidationSeverity::Error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackValidationIssue {
    pub severity: PackValidationSeverity,
    pub code: String,
    pub message: String,
}

impl PackValidationIssue {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: PackValidationSeverity::Error,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: PackValidationSeverity::Warning,
            code: code.into(),
            message: message.into(),
        }
    }
}

fn ensure_archive_valid(archive: &ContentPackArchive) -> Result<(), String> {
    if archive.schema_version != 1 {
        return Err(format!(
            "unsupported content pack archive schema {}",
            archive.schema_version
        ));
    }
    let report = validate_published_content_pack(&archive.pack);
    if !report.is_valid() {
        return Err(format!(
            "content pack {} failed validation: {}",
            archive.pack.manifest.id.as_str(),
            report
                .issues
                .iter()
                .map(|issue| issue.code.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    Ok(())
}

fn upsert_archive_version(entry: &mut InstalledContentPackEntry, archive: ContentPackArchive) {
    if let Some(existing) = entry
        .versions
        .iter_mut()
        .find(|existing| existing.pack.manifest.version == archive.pack.manifest.version)
    {
        *existing = archive;
    } else {
        entry.versions.push(archive);
    }
    entry
        .versions
        .sort_by(|left, right| left.pack.manifest.version.cmp(&right.pack.manifest.version));
}

fn build_progress_migration_report(
    pack_id: ContentPackId,
    from_version: Option<String>,
    to_version: String,
    target_archive: &ContentPackArchive,
    progress: &[LearnerProgress],
    preserve_stable_node_ids: bool,
) -> ProgressMigrationReport {
    let node_ids = target_archive
        .pack
        .curriculum
        .graph
        .nodes
        .iter()
        .map(|node| node.id.clone())
        .collect::<HashSet<_>>();
    let mut preserved_progress_nodes = Vec::new();
    let mut orphaned_progress_nodes = Vec::new();

    for record in progress {
        if preserve_stable_node_ids && node_ids.contains(&record.node_id) {
            if !preserved_progress_nodes.contains(&record.node_id) {
                preserved_progress_nodes.push(record.node_id.clone());
            }
        } else if !orphaned_progress_nodes.contains(&record.node_id) {
            orphaned_progress_nodes.push(record.node_id.clone());
        }
    }

    ProgressMigrationReport {
        pack_id,
        from_version,
        to_version,
        preserved_progress_nodes,
        orphaned_progress_nodes,
    }
}

fn finish_pack_job(job: &mut JobDescriptor, current: u64, message: impl Into<String>) {
    job.state = JobState::Completed;
    job.progress = Some(JobProgress {
        current,
        total: Some(current),
        message: Some(message.into()),
    });
}

fn safe_archive_version_component(version: &str) -> String {
    let mut safe = String::new();
    let mut previous_dash = false;

    for ch in version.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            safe.push(ch);
            previous_dash = false;
        } else if !previous_dash {
            safe.push('-');
            previous_dash = true;
        }
    }

    let safe = safe.trim_matches('-');
    if safe.is_empty() {
        "version".to_string()
    } else {
        safe.to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackValidationSeverity {
    Info,
    Warning,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Curriculum, CurriculumAuthority, CurriculumEdge, CurriculumEdgeKind, CurriculumGraph,
        CurriculumMetadata, CurriculumNode, CurriculumNodeKind, EducationLevel, LearnerId,
        LevelRange, LicenseInfo, LocalizedStringSet, MasteryState, PackOwner, PackUpdateChannel,
        PackUpdatePolicy, SpacedRepetitionState, SubjectDomain,
    };

    #[test]
    fn release_validation_requires_locale_coverage() {
        let en = ContentLocale::parse("en-US").unwrap();
        let ko = ContentLocale::parse("ko-KR").unwrap();
        let manifest = ContentPackManifest {
            id: ContentPackId::from("pack"),
            title: "Pack".to_string(),
            curriculum_id: CurriculumId::from("curriculum"),
            version: "1".to_string(),
            default_locale: en.clone(),
            required_locales: vec![en.clone(), ko],
            provided_locales: vec![en.clone()],
            lessons: vec![LessonManifest {
                node_id: CurriculumNodeId::from("lesson"),
                locale: en,
                path: "lesson.json".to_string(),
                estimated_minutes: Some(10),
            }],
            assets: Vec::new(),
            visuals: Vec::new(),
            practice_items: Vec::new(),
            assessments: Vec::new(),
            glossary_terms: vec![GlossaryTermId::from("term")],
            integrity: PackIntegrity {
                algorithm: "sha256".to_string(),
                content_hash: "hash".to_string(),
            },
        };

        let report = manifest.validate_for_release();

        assert!(!report.is_valid());
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "locale_coverage_incomplete"));
    }

    #[test]
    fn content_pack_archive_exports_imports_and_installs_offline() {
        let pack = published_pack("1.0.0", &["lesson-a"]);

        let archive_json =
            export_published_content_pack_archive_json(pack, "2026-05-04T00:00:00Z".to_string())
                .expect("export archive");
        let archive =
            import_published_content_pack_archive_json(&archive_json).expect("import archive");
        let result = install_content_pack_archive(
            InstalledContentPackRegistry::default(),
            archive,
            "2026-05-04T00:00:01Z".to_string(),
        )
        .expect("install pack");

        assert_eq!(result.registry.entries.len(), 1);
        assert_eq!(
            result.registry.entries[0].versions[0].id.as_str(),
            "pack-1-0-0"
        );
        assert_eq!(result.registry.entries[0].active_version, "1.0.0");
        assert_eq!(result.job.kind, StudyJobKind::PackImport.as_str());
        assert_eq!(result.job.state, JobState::Completed);
    }

    #[test]
    fn content_pack_zip_archive_round_trips_offline() {
        let pack = published_pack("1.0.0", &["lesson-a"]);
        let bytes =
            export_published_content_pack_archive_zip(pack, "2026-05-04T00:00:00Z".to_string())
                .expect("export zip archive");
        let archive =
            import_published_content_pack_archive_zip(&bytes).expect("import zip archive");

        assert_eq!(archive.schema_version, 1);
        assert_eq!(archive.pack.manifest.version, "1.0.0");
        assert!(archive.validation.is_valid());
    }

    #[test]
    fn content_pack_update_and_rollback_preserve_stable_progress_nodes() {
        let initial_archive =
            archive_for_pack(published_pack("1.0.0", &["lesson-a", "lesson-old"]));
        let registry = install_content_pack_archive(
            InstalledContentPackRegistry::default(),
            initial_archive,
            "2026-05-04T00:00:00Z".to_string(),
        )
        .expect("install")
        .registry;
        let progress = vec![
            learner_progress("lesson-a"),
            learner_progress("lesson-old"),
            learner_progress("external-node"),
        ];
        let updated_archive =
            archive_for_pack(published_pack("1.1.0", &["lesson-a", "lesson-new"]));

        let updated = update_content_pack_archive(
            registry,
            updated_archive,
            &progress,
            "2026-05-04T00:01:00Z".to_string(),
        )
        .expect("update");

        assert_eq!(updated.registry.entries[0].active_version, "1.1.0");
        assert_eq!(
            updated.migration.preserved_progress_nodes,
            vec![CurriculumNodeId::from("lesson-a")]
        );
        assert!(updated
            .migration
            .orphaned_progress_nodes
            .contains(&CurriculumNodeId::from("lesson-old")));
        assert!(updated
            .migration
            .orphaned_progress_nodes
            .contains(&CurriculumNodeId::from("external-node")));

        let rolled_back = rollback_content_pack(
            updated.registry,
            ContentPackId::from("pack"),
            "1.0.0".to_string(),
            &progress,
            "2026-05-04T00:02:00Z".to_string(),
        )
        .expect("rollback");

        assert_eq!(rolled_back.registry.entries[0].active_version, "1.0.0");
        assert!(rolled_back
            .migration
            .preserved_progress_nodes
            .contains(&CurriculumNodeId::from("lesson-old")));
        assert_eq!(rolled_back.job.kind, StudyJobKind::PackRollback.as_str());
    }

    fn archive_for_pack(pack: PublishedCurriculumPack) -> ContentPackArchive {
        let json =
            export_published_content_pack_archive_json(pack, "2026-05-04T00:00:00Z".to_string())
                .expect("export archive");
        import_published_content_pack_archive_json(&json).expect("archive")
    }

    fn learner_progress(node_id: &str) -> LearnerProgress {
        LearnerProgress {
            learner_id: LearnerId::from("learner"),
            node_id: CurriculumNodeId::from(node_id),
            mastery: MasteryState::default(),
            attempts: Vec::new(),
            review_state: SpacedRepetitionState::default(),
        }
    }

    fn published_pack(version: &str, lesson_ids: &[&str]) -> PublishedCurriculumPack {
        let locale = ContentLocale::parse("en-US").expect("locale");
        let course_id = CurriculumNodeId::from("course");
        let mut nodes = vec![CurriculumNode {
            id: course_id.clone(),
            kind: CurriculumNodeKind::Course,
            title: LocalizedStringSet::plain("Custom Course"),
            summary: LocalizedStringSet::plain("Custom course summary"),
            level: EducationLevel::HighSchool,
            strand: None,
            objectives: Vec::new(),
            skills: Vec::new(),
            standards: Vec::new(),
            visuals: Vec::new(),
            estimated_minutes: None,
        }];
        let mut edges = Vec::new();
        let mut lessons = Vec::new();
        for lesson_id in lesson_ids {
            let node_id = CurriculumNodeId::from(*lesson_id);
            nodes.push(CurriculumNode {
                id: node_id.clone(),
                kind: CurriculumNodeKind::Lesson,
                title: LocalizedStringSet::plain(format!("Lesson {lesson_id}")),
                summary: LocalizedStringSet::plain("Lesson summary"),
                level: EducationLevel::HighSchool,
                strand: None,
                objectives: Vec::new(),
                skills: Vec::new(),
                standards: Vec::new(),
                visuals: Vec::new(),
                estimated_minutes: Some(10),
            });
            edges.push(CurriculumEdge {
                from: course_id.clone(),
                to: node_id.clone(),
                relation: CurriculumEdgeKind::Contains,
            });
            lessons.push(LessonManifest {
                node_id,
                locale: locale.clone(),
                path: format!("lessons/{lesson_id}.json"),
                estimated_minutes: Some(10),
            });
        }

        PublishedCurriculumPack {
            owner: PackOwner {
                name: "School".to_string(),
                organization: Some("School".to_string()),
                contact: None,
                responsibility_statement: "school-owned custom curriculum".to_string(),
            },
            license: LicenseInfo {
                name: "CC-BY".to_string(),
                url: None,
                permits_redistribution: true,
            },
            update_policy: PackUpdatePolicy {
                channel: PackUpdateChannel::Stable,
                cadence: Some("term".to_string()),
                deprecation_policy: Some("keep previous stable version".to_string()),
                preserve_progress_on_stable_node_ids: true,
            },
            curriculum: Curriculum {
                id: CurriculumId::from("curriculum"),
                domain: SubjectDomain::Custom {
                    id: crate::CustomDomainId::from("social"),
                    label: "Social Studies".to_string(),
                    owner: "School".to_string(),
                },
                title: LocalizedStringSet::plain("Custom Social Studies"),
                description: LocalizedStringSet::plain("Custom curriculum"),
                locale: locale.clone(),
                supported_locales: vec![locale.clone()],
                authority: CurriculumAuthority {
                    owner: "School".to_string(),
                    source_url: None,
                    version: Some(version.to_string()),
                    custom: true,
                },
                level_range: LevelRange {
                    start: EducationLevel::HighSchool,
                    end: EducationLevel::HighSchool,
                },
                graph: CurriculumGraph { nodes, edges },
                standards: Vec::new(),
                metadata: CurriculumMetadata::default(),
            },
            manifest: ContentPackManifest {
                id: ContentPackId::from("pack"),
                title: "Custom Social Studies".to_string(),
                curriculum_id: CurriculumId::from("curriculum"),
                version: version.to_string(),
                default_locale: locale.clone(),
                required_locales: vec![locale.clone()],
                provided_locales: vec![locale],
                lessons,
                assets: Vec::new(),
                visuals: Vec::new(),
                practice_items: Vec::new(),
                assessments: Vec::new(),
                glossary_terms: lesson_ids
                    .iter()
                    .map(|lesson_id| GlossaryTermId::from(format!("term-{lesson_id}")))
                    .collect(),
                integrity: PackIntegrity {
                    algorithm: "sha256".to_string(),
                    content_hash: format!("hash-{version}"),
                },
            },
            lessons: Vec::new(),
            problems: Vec::new(),
            visuals: Vec::new(),
            assessments: Vec::new(),
            glossary: Vec::new(),
            published_at: "2026-05-04T00:00:00Z".to_string(),
        }
    }
}
