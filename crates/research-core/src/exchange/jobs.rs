use serde::{Deserialize, Serialize};
use serde_json::json;
use tench_job_core::{JobDescriptor, JobProgress, JobState};

pub fn research_job_descriptor(
    id: impl Into<String>,
    kind: ResearchJobKind,
    state: JobState,
    batch_id: impl Into<String>,
) -> JobDescriptor {
    JobDescriptor {
        id: id.into(),
        product_id: "tench-research".to_string(),
        kind: kind.as_str().to_string(),
        state,
        progress: Some(JobProgress {
            current: 0,
            total: None,
            message: None,
        }),
        payload: json!({ "batch_id": batch_id.into() }),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchJobKind {
    Import,
    Export,
    IndexMetadata,
    ExtractPdfText,
    RenderPdfPage,
    Backup,
}

impl ResearchJobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Import => "research.import",
            Self::Export => "research.export",
            Self::IndexMetadata => "research.index_metadata",
            Self::ExtractPdfText => "research.extract_pdf_text",
            Self::RenderPdfPage => "research.render_pdf_page",
            Self::Backup => "research.backup",
        }
    }
}
