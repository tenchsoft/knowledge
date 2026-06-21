use serde::{Deserialize, Serialize};

use crate::{
    CurriculumNodeId, GlossaryTermId, LearningVisualId, LocalizedStringSet, LocalizedText,
};

use super::{AssessmentId, LessonId};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackOwner {
    pub name: String,
    #[serde(default)]
    pub organization: Option<String>,
    #[serde(default)]
    pub contact: Option<String>,
    pub responsibility_statement: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    pub permits_redistribution: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PackUpdatePolicy {
    pub channel: PackUpdateChannel,
    #[serde(default)]
    pub cadence: Option<String>,
    #[serde(default)]
    pub deprecation_policy: Option<String>,
    pub preserve_progress_on_stable_node_ids: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackUpdateChannel {
    Stable,
    Periodic,
    Extension,
    UserManaged,
    Unspecified,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LessonDraft {
    pub id: LessonId,
    pub node_id: CurriculumNodeId,
    pub title: LocalizedStringSet,
    #[serde(default)]
    pub blocks: Vec<LessonBlock>,
    #[serde(default)]
    pub visual_ids: Vec<LearningVisualId>,
    #[serde(default)]
    pub glossary_terms: Vec<GlossaryTermId>,
    pub accessibility_summary: LocalizedText,
    #[serde(default)]
    pub reading_level: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum LessonBlock {
    Paragraph {
        text: LocalizedText,
    },
    WorkedExample {
        prompt: LocalizedText,
        solution: LocalizedText,
    },
    Callout {
        label: LocalizedText,
        body: LocalizedText,
    },
    Visual {
        visual_id: LearningVisualId,
    },
    PracticeHook {
        item_ids: Vec<crate::PracticeItemId>,
    },
    GlossaryLink {
        term_id: GlossaryTermId,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentDraft {
    pub id: AssessmentId,
    pub title: LocalizedStringSet,
    pub kind: AssessmentKind,
    pub node_ids: Vec<CurriculumNodeId>,
    pub item_ids: Vec<crate::PracticeItemId>,
    #[serde(default)]
    pub time_limit_seconds: Option<u32>,
    #[serde(default)]
    pub coverage_constraints: Vec<AssessmentCoverageConstraint>,
    pub report_template: LocalizedText,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentKind {
    Quiz,
    Exam,
    PlacementTest,
    MasteryCheckpoint,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentCoverageConstraint {
    pub label: String,
    pub minimum_score: f32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DraftValidationReport {
    pub issues: Vec<DraftValidationIssue>,
}

impl DraftValidationReport {
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity == DraftValidationSeverity::Error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DraftValidationIssue {
    pub severity: DraftValidationSeverity,
    pub code: String,
    pub message: String,
}

impl DraftValidationIssue {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DraftValidationSeverity::Error,
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: DraftValidationSeverity::Warning,
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftValidationSeverity {
    Info,
    Warning,
    Error,
}
