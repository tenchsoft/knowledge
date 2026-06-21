macro_rules! research_id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                let value = value.into();
                tench_storage_core::validate_safe_id(&value)
                    .expect("research id must be a safe storage identifier");
                Self(value)
            }

            pub fn parse(value: impl AsRef<str>) -> Result<Self, tench_storage_core::SafeIdError> {
                tench_storage_core::validate_safe_id(value.as_ref())?;
                Ok(Self(value.as_ref().to_string()))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                tench_storage_core::validate_safe_id(&value).map_err(serde::de::Error::custom)?;
                Ok(Self(value))
            }
        }
    };
}

pub(crate) use research_id_type;

mod ai;

#[cfg(test)]
use ai::phase12_ai_feature_kinds;

pub mod citation;
pub mod domain;
pub mod exchange;
pub mod indexing;
pub mod library;
pub mod manuscript;
pub mod pdf;
pub mod release;
pub mod storage;
pub mod visual;

pub use ai::*;
pub use citation::*;
pub use domain::*;
pub use exchange::*;
pub use indexing::*;
pub use library::*;
pub use manuscript::*;
pub use pdf::*;
pub use release::*;
pub use storage::*;
pub use visual::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadingStatus {
    Unread,
    Reading,
    Reviewed,
    Archived,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PaperIdentifier {
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
    pub isbn: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub year: u16,
    pub venue: String,
    pub abstract_text: String,
    pub identifiers: PaperIdentifier,
    pub collection_id: String,
    pub tags: Vec<String>,
    pub status: ReadingStatus,
    pub favorite: bool,
    pub file_name: String,
    pub file_size_mb: f32,
    pub pages: u16,
    pub added_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub paper_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub label: String,
    pub color: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub paper_id: String,
    pub title: String,
    pub content_markdown: String,
    pub tags: Vec<String>,
    pub word_count: usize,
    pub updated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisRole {
    User,
    Assistant,
    System,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnalysisMessage {
    pub id: String,
    pub role: AnalysisRole,
    pub content: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnalysisThread {
    pub id: String,
    pub paper_id: String,
    pub page: Option<u16>,
    pub selection: Option<String>,
    pub messages: Vec<AnalysisMessage>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchSnapshot {
    pub papers: Vec<Paper>,
    pub collections: Vec<Collection>,
    pub tags: Vec<Tag>,
    pub notes: Vec<Note>,
    pub analysis_threads: Vec<AnalysisThread>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PaperSearchFilter {
    pub query: Option<String>,
    pub collection_id: Option<String>,
    pub tag: Option<String>,
    pub status: Option<ReadingStatus>,
    pub favorites_only: bool,
}

pub fn example_snapshot() -> ResearchSnapshot {
    let tags = vec![
        Tag {
            id: "transformers".to_string(),
            label: "transformers".to_string(),
            color: "#346f62".to_string(),
        },
        Tag {
            id: "retrieval".to_string(),
            label: "retrieval".to_string(),
            color: "#7a5a2f".to_string(),
        },
        Tag {
            id: "evaluation".to_string(),
            label: "evaluation".to_string(),
            color: "#4b5f8a".to_string(),
        },
        Tag {
            id: "notes".to_string(),
            label: "notes".to_string(),
            color: "#8a4f62".to_string(),
        },
    ];

    let papers = vec![
        Paper {
            id: "paper-attention".to_string(),
            title: "Attention Is All You Need".to_string(),
            authors: vec![
                "Ashish Vaswani".to_string(),
                "Noam Shazeer".to_string(),
                "Niki Parmar".to_string(),
            ],
            year: 2017,
            venue: "NeurIPS".to_string(),
            abstract_text: "Introduces the Transformer architecture based entirely on attention mechanisms, removing recurrent and convolutional layers for sequence transduction.".to_string(),
            identifiers: PaperIdentifier {
                doi: None,
                arxiv_id: Some("1706.03762".to_string()),
                isbn: None,
            },
            collection_id: "core-ml".to_string(),
            tags: vec!["transformers".to_string(), "evaluation".to_string()],
            status: ReadingStatus::Reviewed,
            favorite: true,
            file_name: "attention-is-all-you-need.pdf".to_string(),
            file_size_mb: 2.4,
            pages: 15,
            added_at: "2026-04-28T08:30:00Z".to_string(),
            updated_at: "2026-04-28T09:10:00Z".to_string(),
        },
        Paper {
            id: "paper-rag".to_string(),
            title: "Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks".to_string(),
            authors: vec![
                "Patrick Lewis".to_string(),
                "Ethan Perez".to_string(),
                "Aleksandra Piktus".to_string(),
            ],
            year: 2020,
            venue: "NeurIPS".to_string(),
            abstract_text: "Combines parametric sequence-to-sequence models with non-parametric memory accessed through dense retrieval for open-domain question answering.".to_string(),
            identifiers: PaperIdentifier {
                doi: None,
                arxiv_id: Some("2005.11401".to_string()),
                isbn: None,
            },
            collection_id: "retrieval".to_string(),
            tags: vec!["retrieval".to_string(), "transformers".to_string()],
            status: ReadingStatus::Reading,
            favorite: true,
            file_name: "rag-knowledge-intensive-nlp.pdf".to_string(),
            file_size_mb: 1.8,
            pages: 16,
            added_at: "2026-04-27T19:20:00Z".to_string(),
            updated_at: "2026-04-28T07:42:00Z".to_string(),
        },
        Paper {
            id: "paper-evals".to_string(),
            title: "Holistic Evaluation of Language Models".to_string(),
            authors: vec![
                "Percy Liang".to_string(),
                "Rishi Bommasani".to_string(),
                "Tony Lee".to_string(),
            ],
            year: 2022,
            venue: "Transactions on Machine Learning Research".to_string(),
            abstract_text: "Defines a broad benchmark framework that evaluates language models across scenarios, metrics, and risk dimensions.".to_string(),
            identifiers: PaperIdentifier {
                doi: None,
                arxiv_id: Some("2211.09110".to_string()),
                isbn: None,
            },
            collection_id: "evaluation".to_string(),
            tags: vec!["evaluation".to_string()],
            status: ReadingStatus::Unread,
            favorite: false,
            file_name: "helm.pdf".to_string(),
            file_size_mb: 5.6,
            pages: 62,
            added_at: "2026-04-26T16:05:00Z".to_string(),
            updated_at: "2026-04-26T16:05:00Z".to_string(),
        },
    ];

    let collections = vec![
        Collection {
            id: "core-ml".to_string(),
            name: "Core ML Papers".to_string(),
            description:
                "Foundational papers that define model architectures and training patterns."
                    .to_string(),
            paper_count: papers
                .iter()
                .filter(|paper| paper.collection_id == "core-ml")
                .count(),
        },
        Collection {
            id: "retrieval".to_string(),
            name: "Retrieval & RAG".to_string(),
            description: "Dense retrieval, hybrid search, and grounded generation papers."
                .to_string(),
            paper_count: papers
                .iter()
                .filter(|paper| paper.collection_id == "retrieval")
                .count(),
        },
        Collection {
            id: "evaluation".to_string(),
            name: "Evaluation".to_string(),
            description: "Benchmarks, measurement, and model risk evaluation.".to_string(),
            paper_count: papers
                .iter()
                .filter(|paper| paper.collection_id == "evaluation")
                .count(),
        },
    ];

    let notes = vec![
        Note {
            id: "note-attention-review".to_string(),
            paper_id: "paper-attention".to_string(),
            title: "Transformer reading notes".to_string(),
            content_markdown: "## Key points\n\n- Self-attention replaces recurrence.\n- Positional encoding keeps order information.\n- Multi-head attention separates relation subspaces.\n".to_string(),
            tags: vec!["notes".to_string(), "transformers".to_string()],
            word_count: 18,
            updated_at: "2026-04-28T09:10:00Z".to_string(),
        },
        Note {
            id: "note-rag-open-questions".to_string(),
            paper_id: "paper-rag".to_string(),
            title: "RAG follow-up questions".to_string(),
            content_markdown: "## Questions\n\n- How should passage freshness be tracked?\n- What is the minimum useful retrieval evaluation set?\n".to_string(),
            tags: vec!["retrieval".to_string()],
            word_count: 17,
            updated_at: "2026-04-28T07:42:00Z".to_string(),
        },
    ];

    let analysis_threads = vec![AnalysisThread {
        id: "analysis-attention-page-3".to_string(),
        paper_id: "paper-attention".to_string(),
        page: Some(3),
        selection: Some("Scaled Dot-Product Attention".to_string()),
        messages: vec![
            AnalysisMessage {
                id: "analysis-attention-page-3-system".to_string(),
                role: AnalysisRole::System,
                content: "Example Engine response using paper context.".to_string(),
                created_at: "2026-04-28T09:15:00Z".to_string(),
            },
            AnalysisMessage {
                id: "analysis-attention-page-3-assistant".to_string(),
                role: AnalysisRole::Assistant,
                content: "Scaled dot-product attention computes compatibility between query and key vectors, scales by the key dimension, and uses the resulting weights to mix value vectors.".to_string(),
                created_at: "2026-04-28T09:15:02Z".to_string(),
            },
        ],
    }];

    ResearchSnapshot {
        papers,
        collections,
        tags,
        notes,
        analysis_threads,
    }
}

pub fn search_papers(papers: &[Paper], filter: &PaperSearchFilter) -> Vec<Paper> {
    let query = filter
        .query
        .as_deref()
        .map(normalize_query)
        .filter(|query| !query.is_empty());
    let collection_id = filter
        .collection_id
        .as_deref()
        .filter(|value| !value.is_empty());
    let tag = filter.tag.as_deref().filter(|value| !value.is_empty());

    papers
        .iter()
        .filter(|paper| {
            if let Some(collection_id) = collection_id {
                if paper.collection_id != collection_id {
                    return false;
                }
            }
            if let Some(tag) = tag {
                if !paper.tags.iter().any(|paper_tag| paper_tag == tag) {
                    return false;
                }
            }
            if let Some(status) = &filter.status {
                if &paper.status != status {
                    return false;
                }
            }
            if filter.favorites_only && !paper.favorite {
                return false;
            }
            if let Some(query) = &query {
                return searchable_text(paper).contains(query);
            }
            true
        })
        .cloned()
        .collect()
}

pub fn notes_for_paper(notes: &[Note], paper_id: &str) -> Vec<Note> {
    notes
        .iter()
        .filter(|note| note.paper_id == paper_id)
        .cloned()
        .collect()
}

fn searchable_text(paper: &Paper) -> String {
    normalize_query(&format!(
        "{} {} {} {} {} {:?}",
        paper.title,
        paper.authors.join(" "),
        paper.venue,
        paper.abstract_text,
        paper.tags.join(" "),
        paper.identifiers
    ))
}

fn normalize_query(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests;
