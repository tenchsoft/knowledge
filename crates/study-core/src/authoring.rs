use serde::{Deserialize, Serialize};

mod draft;
mod preview;
#[cfg(test)]
mod tests;
mod types;
mod validation;

pub use draft::*;
pub use preview::*;
pub use types::*;

use crate::{
    ContentLocale, ContentPackId, ContentPackManifest, Curriculum, CurriculumId, CurriculumNodeId,
    CurriculumNodeKind, CustomDomainId, EducationLevel, GlossaryTerm, GlossaryTermId,
    LearningVisualId, LearningVisualSpec, LevelRange, LocalizedStringSet, LocalizedText,
    ObjectiveTaxonomy, PracticeItem,
};

crate::study_id_type!(CurriculumPackDraftId);
crate::study_id_type!(LessonId);
crate::study_id_type!(AssessmentId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurriculumPackDraft {
    pub id: CurriculumPackDraftId,
    pub owner: PackOwner,
    pub license: LicenseInfo,
    pub update_policy: PackUpdatePolicy,
    pub curriculum: Curriculum,
    pub manifest: ContentPackManifest,
    #[serde(default)]
    pub lessons: Vec<LessonDraft>,
    #[serde(default)]
    pub problems: Vec<PracticeItem>,
    #[serde(default)]
    pub visuals: Vec<LearningVisualSpec>,
    #[serde(default)]
    pub assessments: Vec<AssessmentDraft>,
    #[serde(default)]
    pub glossary: Vec<GlossaryTerm>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomCurriculumDraftRequest {
    pub draft_id: CurriculumPackDraftId,
    pub pack_id: ContentPackId,
    pub curriculum_id: CurriculumId,
    pub domain_id: CustomDomainId,
    pub domain_label: String,
    pub title: LocalizedStringSet,
    pub description: LocalizedStringSet,
    pub owner: PackOwner,
    pub license: LicenseInfo,
    pub update_policy: PackUpdatePolicy,
    pub default_locale: ContentLocale,
    #[serde(default)]
    pub required_locales: Vec<ContentLocale>,
    pub level_range: LevelRange,
    pub version: String,
    pub integrity_hash: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LessonDraftInput {
    pub lesson_id: LessonId,
    pub node_id: CurriculumNodeId,
    pub title: LocalizedStringSet,
    pub summary: LocalizedStringSet,
    pub level: EducationLevel,
    #[serde(default)]
    pub strand: Option<String>,
    pub objective: LocalizedText,
    pub taxonomy: ObjectiveTaxonomy,
    #[serde(default)]
    pub blocks: Vec<LessonBlock>,
    #[serde(default)]
    pub visual_ids: Vec<LearningVisualId>,
    #[serde(default)]
    pub glossary_terms: Vec<GlossaryTermId>,
    pub accessibility_summary: LocalizedText,
    #[serde(default)]
    pub reading_level: Option<String>,
    #[serde(default)]
    pub estimated_minutes: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedCurriculumPack {
    pub owner: PackOwner,
    pub license: LicenseInfo,
    pub update_policy: PackUpdatePolicy,
    pub curriculum: Curriculum,
    pub manifest: ContentPackManifest,
    #[serde(default)]
    pub lessons: Vec<LessonDraft>,
    #[serde(default)]
    pub problems: Vec<PracticeItem>,
    #[serde(default)]
    pub visuals: Vec<LearningVisualSpec>,
    #[serde(default)]
    pub assessments: Vec<AssessmentDraft>,
    #[serde(default)]
    pub glossary: Vec<GlossaryTerm>,
    pub published_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurriculumPackPreviewSnapshot {
    pub draft_id: CurriculumPackDraftId,
    pub title: String,
    pub domain_label: String,
    pub locale: ContentLocale,
    pub node_count: usize,
    pub edge_count: usize,
    pub lesson_count: usize,
    pub practice_count: usize,
    pub visual_count: usize,
    pub assessment_count: usize,
    pub glossary_count: usize,
    pub orphan_node_count: usize,
    pub validation: DraftValidationReport,
    pub localization: CurriculumPackLocalizationReport,
    #[serde(default)]
    pub outline: Vec<CurriculumPackPreviewNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurriculumPackPreviewNode {
    pub node_id: CurriculumNodeId,
    pub title: String,
    pub kind: CurriculumNodeKind,
    pub level: EducationLevel,
    pub child_count: usize,
    pub practice_count: usize,
    pub visual_count: usize,
    pub assessment_count: usize,
    pub glossary_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurriculumPackLocalizationReport {
    pub default_locale: ContentLocale,
    #[serde(default)]
    pub required_locales: Vec<ContentLocale>,
    pub checked_field_count: u32,
    #[serde(default)]
    pub missing: Vec<CurriculumPackLocalizationGap>,
    pub release_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurriculumPackLocalizationGap {
    pub item_kind: String,
    pub item_id: String,
    pub field: String,
    pub locale: ContentLocale,
}
