use serde::{Deserialize, Serialize};
use tench_document_core::TenchDocument;

use crate::{
    AnswerSubmission, CurriculumNodeId, GradingResult, LearnerId, LocalizedText, PracticeItemId,
};

crate::study_id_type!(StudyNoteId);
crate::study_id_type!(StudyCardId);
crate::study_id_type!(StudyDeckId);
crate::study_id_type!(ExamSessionId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyNote {
    pub id: StudyNoteId,
    pub learner_id: LearnerId,
    pub node_id: CurriculumNodeId,
    pub title: LocalizedText,
    pub document: TenchDocument,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyCard {
    pub id: StudyCardId,
    pub deck_id: StudyDeckId,
    pub node_id: CurriculumNodeId,
    #[serde(default)]
    pub source_note_id: Option<StudyNoteId>,
    pub kind: StudyCardKind,
    pub front: LocalizedText,
    pub back: LocalizedText,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub media: StudyCardMedia,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StudyCardMedia {
    #[serde(default)]
    pub image_ref: Option<String>,
    #[serde(default)]
    pub audio_ref: Option<String>,
    #[serde(default)]
    pub code: Option<StudyCardCodePayload>,
    #[serde(default)]
    pub occlusions: Vec<ImageOcclusionMask>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageOcclusionMask {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyCardCodePayload {
    pub language: String,
    pub code: String,
    #[serde(default)]
    pub expected_output: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyCardKind {
    Basic,
    Cloze,
    ImageOcclusion,
    Audio,
    Code,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyDeck {
    pub id: StudyDeckId,
    pub learner_id: LearnerId,
    pub title: LocalizedText,
    #[serde(default)]
    pub cards: Vec<StudyCard>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyCardExchangeFormat {
    AnkiTsv,
    Csv,
    Tsv,
    Markdown,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyNoteExchangeFormat {
    Markdown,
    PlainText,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyProgressExportFormat {
    Markdown,
    Csv,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyExamReportExportFormat {
    Markdown,
    Csv,
    Json,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyProgressReport {
    pub generated_at: String,
    pub progress_count: u32,
    pub average_mastery: f32,
    pub mastered_nodes: u32,
    pub needs_practice_nodes: u32,
    pub suspended_reviews: u32,
    pub due_reviews: u32,
    pub total_attempts: u32,
    pub total_correct: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamSession {
    pub id: ExamSessionId,
    pub learner_id: LearnerId,
    pub title: LocalizedText,
    #[serde(default)]
    pub item_ids: Vec<PracticeItemId>,
    #[serde(default)]
    pub submissions: Vec<AnswerSubmission>,
    #[serde(default)]
    pub results: Vec<ExamQuestionResult>,
    #[serde(default)]
    pub time_limit_seconds: Option<u32>,
    #[serde(default)]
    pub started_at: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamQuestionResult {
    pub item_id: PracticeItemId,
    pub result: GradingResult,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamReport {
    pub session_id: ExamSessionId,
    pub score: f32,
    pub correct: u32,
    pub total: u32,
    #[serde(default)]
    pub weak_item_ids: Vec<PracticeItemId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamBlueprint {
    pub id: ExamSessionId,
    pub learner_id: LearnerId,
    pub title: LocalizedText,
    pub item_count: u32,
    #[serde(default)]
    pub coverage_constraints: Vec<ExamCoverageConstraint>,
    #[serde(default)]
    pub time_limit_seconds: Option<u32>,
    #[serde(default)]
    pub started_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamCoverageConstraint {
    pub node_id: CurriculumNodeId,
    pub min_items: u32,
    #[serde(default)]
    pub max_items: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamBuildReport {
    pub session: ExamSession,
    #[serde(default)]
    pub selected_item_ids: Vec<PracticeItemId>,
    #[serde(default)]
    pub coverage_issues: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamTimingStatus {
    pub elapsed_seconds: u32,
    pub remaining_seconds: Option<u32>,
    pub expired: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RubricScore {
    pub item_id: PracticeItemId,
    pub score: f32,
    #[serde(default)]
    pub feedback_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamResultReview {
    pub session_id: ExamSessionId,
    #[serde(default)]
    pub questions: Vec<ExamQuestionReview>,
    #[serde(default)]
    pub weak_node_ids: Vec<CurriculumNodeId>,
    #[serde(default)]
    pub mastered_node_ids: Vec<CurriculumNodeId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExamQuestionReview {
    pub item_id: PracticeItemId,
    pub node_id: CurriculumNodeId,
    pub prompt: LocalizedText,
    pub response: String,
    pub expected: String,
    pub explanation: LocalizedText,
    pub result: GradingResult,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyProgressReportView {
    pub report: StudyProgressReport,
    #[serde(default)]
    pub rows: Vec<StudyProgressReportRow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyProgressReportRow {
    pub node_id: CurriculumNodeId,
    pub mastery_percent: f32,
    pub attempts: u32,
    pub correct: u32,
    pub review_due: Option<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyDuplicateCandidate {
    pub existing_id: String,
    pub duplicate_id: String,
    pub reason: String,
    pub confidence: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyCardImportCleanupReport {
    pub cards: Vec<StudyCard>,
    #[serde(default)]
    pub duplicates: Vec<StudyDuplicateCandidate>,
    pub removed_count: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyNoteImportCleanupReport {
    pub notes: Vec<StudyNote>,
    #[serde(default)]
    pub duplicates: Vec<StudyDuplicateCandidate>,
    pub removed_count: u32,
}
