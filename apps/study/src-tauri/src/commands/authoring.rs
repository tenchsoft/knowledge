#[tauri::command]
pub fn validate_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
) -> tench_study_core::DraftValidationReport {
    draft.validate_for_distribution()
}

#[tauri::command]
pub fn export_curriculum_pack_draft_json(
    draft: tench_study_core::CurriculumPackDraft,
) -> Result<String, String> {
    tench_study_core::export_curriculum_pack_draft_json(&draft).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn import_curriculum_pack_draft_json(
    text: String,
) -> Result<tench_study_core::CurriculumPackDraft, String> {
    tench_study_core::import_curriculum_pack_draft_json(&text).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn create_custom_curriculum_pack_draft(
    request: tench_study_core::CustomCurriculumDraftRequest,
) -> tench_study_core::CurriculumPackDraft {
    tench_study_core::new_custom_curriculum_pack_draft(request)
}

#[tauri::command]
pub fn add_lesson_to_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
    input: tench_study_core::LessonDraftInput,
) -> Result<tench_study_core::CurriculumPackDraft, String> {
    tench_study_core::add_lesson_to_curriculum_pack_draft(draft, input)
}

#[tauri::command]
pub fn add_practice_item_to_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
    item: tench_study_core::PracticeItem,
) -> Result<tench_study_core::CurriculumPackDraft, String> {
    tench_study_core::add_practice_item_to_curriculum_pack_draft(draft, item)
}

#[tauri::command]
pub fn add_learning_visual_to_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
    visual: tench_study_core::LearningVisualSpec,
) -> Result<tench_study_core::CurriculumPackDraft, String> {
    tench_study_core::add_learning_visual_to_curriculum_pack_draft(draft, visual)
}

#[tauri::command]
pub fn add_assessment_to_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
    assessment: tench_study_core::AssessmentDraft,
) -> Result<tench_study_core::CurriculumPackDraft, String> {
    tench_study_core::add_assessment_to_curriculum_pack_draft(draft, assessment)
}

#[tauri::command]
pub fn add_glossary_term_to_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
    term: tench_study_core::GlossaryTerm,
) -> Result<tench_study_core::CurriculumPackDraft, String> {
    tench_study_core::add_glossary_term_to_curriculum_pack_draft(draft, term)
}

#[tauri::command]
pub fn preview_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
    locale: Option<tench_study_core::ContentLocale>,
) -> tench_study_core::CurriculumPackPreviewSnapshot {
    tench_study_core::preview_curriculum_pack_draft(&draft, locale)
}

#[tauri::command]
pub fn curriculum_pack_localization_report(
    draft: tench_study_core::CurriculumPackDraft,
) -> tench_study_core::CurriculumPackLocalizationReport {
    tench_study_core::curriculum_pack_localization_report(&draft)
}

#[tauri::command]
pub fn publish_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
    published_at: String,
) -> Result<tench_study_core::PublishedCurriculumPack, tench_study_core::DraftValidationReport> {
    tench_study_core::publish_curriculum_pack_draft(draft, published_at)
}

#[tauri::command]
pub fn save_curriculum_pack_draft(
    draft: tench_study_core::CurriculumPackDraft,
) -> Result<String, String> {
    let file_name = format!("{}.json", draft.id.as_str());
    tench_study_core::write_study_json(
        tench_study_core::StudyStorageArea::ContentPacks,
        &file_name,
        &draft,
    )
    .map(|path| path.to_string_lossy().to_string())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_curriculum_pack_draft(
    draft_id: tench_study_core::CurriculumPackDraftId,
) -> Result<Option<tench_study_core::CurriculumPackDraft>, String> {
    let file_name = format!("{}.json", draft_id.as_str());
    tench_study_core::read_study_json(tench_study_core::StudyStorageArea::ContentPacks, &file_name)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_published_curriculum_pack(
    pack: tench_study_core::PublishedCurriculumPack,
) -> Result<String, String> {
    let file_name = format!("published-{}.json", pack.manifest.id.as_str());
    tench_study_core::write_study_json(
        tench_study_core::StudyStorageArea::ContentPacks,
        &file_name,
        &pack,
    )
    .map(|path| path.to_string_lossy().to_string())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_published_curriculum_pack(
    pack_id: tench_study_core::ContentPackId,
) -> Result<Option<tench_study_core::PublishedCurriculumPack>, String> {
    let file_name = format!("published-{}.json", pack_id.as_str());
    tench_study_core::read_study_json(tench_study_core::StudyStorageArea::ContentPacks, &file_name)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn export_published_content_pack_archive_json(
    pack: tench_study_core::PublishedCurriculumPack,
    exported_at: String,
) -> Result<String, String> {
    tench_study_core::export_published_content_pack_archive_json(pack, exported_at)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn import_published_content_pack_archive_json(
    text: String,
) -> Result<tench_study_core::ContentPackArchive, String> {
    tench_study_core::import_published_content_pack_archive_json(&text)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn export_published_content_pack_archive_zip(
    pack: tench_study_core::PublishedCurriculumPack,
    exported_at: String,
) -> Result<Vec<u8>, String> {
    tench_study_core::export_published_content_pack_archive_zip(pack, exported_at)
}

#[tauri::command]
pub fn import_published_content_pack_archive_zip(
    bytes: Vec<u8>,
) -> Result<tench_study_core::ContentPackArchive, String> {
    tench_study_core::import_published_content_pack_archive_zip(&bytes)
}

#[tauri::command]
pub fn install_study_content_pack_archive(
    registry: tench_study_core::InstalledContentPackRegistry,
    archive: tench_study_core::ContentPackArchive,
    installed_at: String,
) -> Result<tench_study_core::ContentPackInstallResult, String> {
    tench_study_core::install_content_pack_archive(registry, archive, installed_at)
}

#[tauri::command]
pub fn update_study_content_pack_archive(
    registry: tench_study_core::InstalledContentPackRegistry,
    archive: tench_study_core::ContentPackArchive,
    progress: Vec<tench_study_core::LearnerProgress>,
    updated_at: String,
) -> Result<tench_study_core::ContentPackUpdateResult, String> {
    tench_study_core::update_content_pack_archive(registry, archive, &progress, updated_at)
}

#[tauri::command]
pub fn rollback_study_content_pack(
    registry: tench_study_core::InstalledContentPackRegistry,
    pack_id: tench_study_core::ContentPackId,
    target_version: String,
    progress: Vec<tench_study_core::LearnerProgress>,
    rolled_back_at: String,
) -> Result<tench_study_core::ContentPackRollbackResult, String> {
    tench_study_core::rollback_content_pack(
        registry,
        pack_id,
        target_version,
        &progress,
        rolled_back_at,
    )
}

#[tauri::command]
pub fn save_study_content_pack_registry(
    registry: tench_study_core::InstalledContentPackRegistry,
) -> Result<String, String> {
    tench_study_core::write_study_json(
        tench_study_core::StudyStorageArea::ContentPacks,
        "installed-registry.json",
        &registry,
    )
    .map(|path| path.to_string_lossy().to_string())
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn load_study_content_pack_registry(
) -> Result<tench_study_core::InstalledContentPackRegistry, String> {
    tench_study_core::read_study_json(
        tench_study_core::StudyStorageArea::ContentPacks,
        "installed-registry.json",
    )
    .map(|registry| registry.unwrap_or_default())
    .map_err(|error| error.to_string())
}
