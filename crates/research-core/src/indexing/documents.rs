use super::*;
use std::collections::{HashMap, HashSet};
use tench_search_core::{IndexStats, SearchDomain};

use crate::{PdfDocumentText, ReferenceId, ResearchSnapshotV2};

pub fn build_research_index_documents(snapshot: &ResearchSnapshotV2) -> Vec<ResearchIndexDocument> {
    let mut documents = Vec::new();
    for reference in &snapshot.references {
        let creators = reference
            .creators
            .iter()
            .map(|creator| creator.sort_name())
            .collect::<Vec<_>>()
            .join(" ");
        let body = format!(
            "{} {} {} {} {}",
            creators,
            reference
                .venue
                .as_ref()
                .map(|venue| venue.name.value.as_str())
                .unwrap_or_default(),
            reference
                .issued
                .year
                .map(|year| year.to_string())
                .unwrap_or_default(),
            reference
                .abstract_text
                .as_ref()
                .map(|abstract_text| abstract_text.value.as_str())
                .unwrap_or_default(),
            reference
                .citekey
                .as_ref()
                .map(|citekey| citekey.as_str())
                .unwrap_or_default()
        );
        documents.push(ResearchIndexDocument {
            id: format!("reference:{}", reference.id.as_str()),
            reference_id: Some(reference.id.clone()),
            domain: SearchDomain::Documents,
            title: reference.title.value.clone(),
            body,
            tags: reference
                .tags
                .iter()
                .map(|tag| tag.as_str().to_string())
                .collect(),
            location: Some(format!("research://reference/{}", reference.id.as_str())),
        });
    }
    for note in &snapshot.notes {
        documents.push(ResearchIndexDocument {
            id: format!("note:{}", note.id.as_str()),
            reference_id: note.reference_id.clone(),
            domain: SearchDomain::Notes,
            title: note.title.clone(),
            body: note.body_markdown.clone(),
            tags: note
                .tags
                .iter()
                .map(|tag| tag.as_str().to_string())
                .collect(),
            location: Some(format!("research://note/{}", note.id.as_str())),
        });
    }
    for annotation in &snapshot.annotations {
        let body = [
            annotation.selected_text.as_deref().unwrap_or_default(),
            annotation.note_markdown.as_deref().unwrap_or_default(),
        ]
        .join(" ");
        documents.push(ResearchIndexDocument {
            id: format!("annotation:{}", annotation.id.as_str()),
            reference_id: Some(annotation.reference_id.clone()),
            domain: SearchDomain::Notes,
            title: format!("Annotation page {}", annotation.page),
            body,
            tags: Vec::new(),
            location: Some(format!(
                "research://reference/{}/attachment/{}/page/{}",
                annotation.reference_id.as_str(),
                annotation.attachment_id.as_str(),
                annotation.page
            )),
        });
    }
    documents
}

pub fn build_research_index_documents_with_pdf_text(
    snapshot: &ResearchSnapshotV2,
    pdf_texts: &[PdfDocumentText],
) -> Vec<ResearchIndexDocument> {
    let mut documents = build_research_index_documents(snapshot);
    let attachments = snapshot
        .attachments
        .iter()
        .map(|attachment| (attachment.id.as_str(), attachment))
        .collect::<HashMap<_, _>>();
    let references = snapshot
        .references
        .iter()
        .map(|reference| (reference.id.as_str(), reference))
        .collect::<HashMap<_, _>>();

    for pdf_text in pdf_texts {
        let Some(attachment) = attachments.get(pdf_text.attachment_id.as_str()) else {
            continue;
        };
        let Some(reference) = references.get(attachment.reference_id.as_str()) else {
            continue;
        };
        let tags = reference
            .tags
            .iter()
            .map(|tag| tag.as_str().to_string())
            .collect::<Vec<_>>();
        for page in &pdf_text.pages {
            let page_text = page.text.trim();
            if page_text.is_empty() {
                continue;
            }
            documents.push(ResearchIndexDocument {
                id: format!("pdf:{}:page:{}", pdf_text.attachment_id.as_str(), page.page),
                reference_id: Some(attachment.reference_id.clone()),
                domain: SearchDomain::Documents,
                title: format!("{} page {}", attachment.title, page.page),
                body: page_text.to_string(),
                tags: tags.clone(),
                location: Some(format!(
                    "research://reference/{}/attachment/{}/page/{}",
                    attachment.reference_id.as_str(),
                    attachment.id.as_str(),
                    page.page
                )),
            });
        }
    }

    documents
}

pub fn build_research_index_manifest(
    documents: &[ResearchIndexDocument],
) -> Vec<ResearchIndexManifestEntry> {
    let mut manifest = documents
        .iter()
        .map(|document| ResearchIndexManifestEntry {
            document_id: document.id.clone(),
            content_hash: research_index_document_hash(document),
        })
        .collect::<Vec<_>>();
    manifest.sort_by(|left, right| left.document_id.cmp(&right.document_id));
    manifest
}

pub fn plan_incremental_research_reindex(
    snapshot: &ResearchSnapshotV2,
    pdf_texts: &[PdfDocumentText],
    existing_manifest: &[ResearchIndexManifestEntry],
    failed_jobs: Vec<ResearchIndexFailure>,
    updated_at: Option<String>,
) -> ResearchIncrementalIndexPlan {
    let documents = build_research_index_documents_with_pdf_text(snapshot, pdf_texts);
    let manifest = build_research_index_manifest(&documents);
    let existing = existing_manifest
        .iter()
        .map(|entry| (entry.document_id.as_str(), entry.content_hash.as_str()))
        .collect::<HashMap<_, _>>();
    let current_ids = manifest
        .iter()
        .map(|entry| entry.document_id.as_str())
        .collect::<HashSet<_>>();
    let stale_ids = existing_manifest
        .iter()
        .filter(|entry| !current_ids.contains(entry.document_id.as_str()))
        .map(|entry| entry.document_id.clone())
        .collect::<Vec<_>>();
    let changed_ids = manifest
        .iter()
        .filter(|entry| {
            existing
                .get(entry.document_id.as_str())
                .map(|hash| *hash != entry.content_hash.as_str())
                .unwrap_or(true)
        })
        .map(|entry| entry.document_id.as_str())
        .collect::<HashSet<_>>();
    let retryable_failed_references = failed_jobs
        .iter()
        .filter(|failure| failure.retryable)
        .filter_map(|failure| failure.reference_id.as_ref())
        .collect::<HashSet<_>>();
    let retryable_failed_attachments = failed_jobs
        .iter()
        .filter(|failure| failure.retryable)
        .filter_map(|failure| failure.attachment_id.as_ref())
        .collect::<HashSet<_>>();

    let mut upsert_seen = HashSet::new();
    let mut upsert_documents = Vec::new();
    for document in documents {
        let retry_reference = document
            .reference_id
            .as_ref()
            .map(|reference_id| retryable_failed_references.contains(reference_id))
            .unwrap_or(false);
        let retry_attachment = document
            .attachment_id()
            .as_ref()
            .map(|attachment_id| retryable_failed_attachments.contains(attachment_id))
            .unwrap_or(false);
        if (changed_ids.contains(document.id.as_str()) || retry_reference || retry_attachment)
            && upsert_seen.insert(document.id.clone())
        {
            upsert_documents.push(document);
        }
    }

    let pending_references = pending_references_for_documents(&upsert_documents, &failed_jobs);
    let state = ResearchIndexState {
        stats: IndexStats {
            indexed_items: manifest
                .len()
                .saturating_sub(upsert_documents.len())
                .try_into()
                .unwrap_or(u64::MAX),
            pending_items: upsert_documents
                .len()
                .saturating_add(stale_ids.len())
                .try_into()
                .unwrap_or(u64::MAX),
            failed_items: failed_jobs.len().try_into().unwrap_or(u64::MAX),
            updated_at,
        },
        pending_references,
        failed_jobs,
    };

    ResearchIncrementalIndexPlan {
        state,
        upsert_documents,
        remove_document_ids: stale_ids,
        manifest,
    }
}

pub fn repair_research_index(
    snapshot: &ResearchSnapshotV2,
    pdf_texts: &[PdfDocumentText],
    existing_document_ids: &[String],
    failed_jobs: Vec<ResearchIndexFailure>,
    updated_at: Option<String>,
) -> ResearchIndexRepairReport {
    let documents = build_research_index_documents_with_pdf_text(snapshot, pdf_texts);
    let manifest = build_research_index_manifest(&documents);
    let current_ids = manifest
        .iter()
        .map(|entry| entry.document_id.as_str())
        .collect::<HashSet<_>>();
    let mut removed_document_ids = existing_document_ids
        .iter()
        .filter(|document_id| !current_ids.contains(document_id.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    removed_document_ids.sort();
    removed_document_ids.dedup();

    let pending_references = pending_references_for_documents(&documents, &failed_jobs);
    let state = ResearchIndexState {
        stats: IndexStats {
            indexed_items: 0,
            pending_items: documents
                .len()
                .saturating_add(removed_document_ids.len())
                .try_into()
                .unwrap_or(u64::MAX),
            failed_items: failed_jobs.len().try_into().unwrap_or(u64::MAX),
            updated_at,
        },
        pending_references,
        failed_jobs,
    };

    ResearchIndexRepairReport {
        state,
        documents,
        removed_document_ids,
        manifest,
    }
}

fn pending_references_for_documents(
    documents: &[ResearchIndexDocument],
    failed_jobs: &[ResearchIndexFailure],
) -> Vec<ReferenceId> {
    let mut references = documents
        .iter()
        .filter_map(|document| document.reference_id.clone())
        .collect::<Vec<_>>();
    references.extend(
        failed_jobs
            .iter()
            .filter(|failure| failure.retryable)
            .filter_map(|failure| failure.reference_id.clone()),
    );
    references.sort();
    references.dedup();
    references
}

fn research_index_document_hash(document: &ResearchIndexDocument) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for part in [
        document.id.as_str(),
        document
            .reference_id
            .as_ref()
            .map(ReferenceId::as_str)
            .unwrap_or(""),
        match document.domain {
            SearchDomain::Files => "files",
            SearchDomain::Documents => "documents",
            SearchDomain::Code => "code",
            SearchDomain::Media => "media",
            SearchDomain::Notes => "notes",
            SearchDomain::EngineModels => "engine_models",
        },
        document.title.as_str(),
        document.body.as_str(),
        document.location.as_deref().unwrap_or(""),
    ] {
        stable_hash_part(&mut hash, part);
    }
    for tag in &document.tags {
        stable_hash_part(&mut hash, tag);
    }
    format!("{hash:016x}")
}

fn stable_hash_part(hash: &mut u64, value: &str) {
    for byte in value.as_bytes() {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
    *hash ^= 0xff;
    *hash = hash.wrapping_mul(0x100000001b3);
}
