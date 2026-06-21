mod jobs;
mod pdf_import;
mod text;
mod types;

pub use jobs::{research_job_descriptor, ResearchJobKind};
pub use pdf_import::import_pdf_paths;
pub use text::{export_references_text, parse_references_text, render_plain_bibliography};
pub use types::*;

use std::collections::{HashMap, HashSet};

use crate::{
    Citekey, Creator, CreatorRole, LocalizedField, ReferenceId, ReferenceItem, ReferenceKind,
    Timestamp,
};

pub fn detect_import_format(path_or_kind: &str) -> Option<ResearchImportFormat> {
    let value = path_or_kind
        .trim()
        .trim_start_matches('.')
        .to_ascii_lowercase();
    match value.as_str() {
        "pdf" => Some(ResearchImportFormat::Pdf),
        "bib" | "bibtex" => Some(ResearchImportFormat::BibTex),
        "ris" => Some(ResearchImportFormat::Ris),
        "json" | "csl" | "csljson" => Some(ResearchImportFormat::CslJson),
        "xml" | "enw" | "endnote" => Some(ResearchImportFormat::EndNoteXml),
        "doi" => Some(ResearchImportFormat::Doi),
        "isbn" => Some(ResearchImportFormat::Isbn),
        "arxiv" => Some(ResearchImportFormat::Arxiv),
        _ => None,
    }
}

pub fn generate_citekey(reference: &ReferenceItem) -> Citekey {
    let author = reference
        .creators
        .iter()
        .find(|creator| creator.role == CreatorRole::Author)
        .and_then(|creator| {
            creator
                .family
                .as_ref()
                .or(creator.literal.as_ref())
                .or(creator.given.as_ref())
        })
        .map(|value| ascii_slug(value))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "ref".to_string());
    let year = reference
        .issued
        .year
        .map(|year| year.to_string())
        .unwrap_or_else(|| "nd".to_string());
    let title = reference
        .title
        .value
        .split_whitespace()
        .find(|word| word.chars().any(|ch| ch.is_alphanumeric()))
        .map(ascii_slug)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "item".to_string());

    Citekey::new(format!("{author}{year}{title}"))
}

pub fn resolve_citekey_collisions(references: &mut [ReferenceItem]) {
    let mut used_keys = HashSet::<String>::new();
    let mut suffix_indexes = HashMap::<String, u32>::new();

    for reference in references.iter() {
        if reference.citekey_locked {
            if let Some(citekey) = &reference.citekey {
                used_keys.insert(citekey.as_str().to_string());
            }
        }
    }

    for reference in references {
        let base = reference
            .citekey
            .as_ref()
            .map(|citekey| citekey.as_str().to_string())
            .unwrap_or_else(|| generate_citekey(reference).0);

        if reference.citekey_locked && reference.citekey.is_some() {
            continue;
        }

        let mut candidate = base.clone();
        let suffix_index = suffix_indexes.entry(base.clone()).or_insert(0);
        while used_keys.contains(&candidate) {
            *suffix_index += 1;
            candidate = format!("{base}{}", suffix(*suffix_index));
        }
        reference.citekey = Some(Citekey::new(candidate.clone()));
        used_keys.insert(candidate);
    }
}

pub fn find_duplicate_candidates(
    existing: &[ReferenceItem],
    imported: &[ReferenceItem],
) -> Vec<DuplicateCandidate> {
    let mut candidates = Vec::new();
    for imported_reference in imported {
        for existing_reference in existing {
            if let Some((reason, confidence)) =
                duplicate_reason(existing_reference, imported_reference)
            {
                candidates.push(DuplicateCandidate {
                    existing_id: existing_reference.id.clone(),
                    imported_id: imported_reference.id.clone(),
                    reason,
                    confidence,
                });
            }
        }
    }
    candidates.sort_by(|a, b| {
        b.confidence
            .cmp(&a.confidence)
            .then_with(|| a.imported_id.cmp(&b.imported_id))
            .then_with(|| a.existing_id.cmp(&b.existing_id))
    });
    candidates
}

pub fn reference_from_minimal_metadata(
    id: impl Into<String>,
    kind: ReferenceKind,
    title: impl Into<String>,
    year: Option<u16>,
    now: impl Into<String>,
) -> ReferenceItem {
    let now = Timestamp(now.into());
    ReferenceItem {
        id: ReferenceId::new(id),
        kind,
        title: LocalizedField::plain(title),
        subtitle: None,
        creators: Vec::new(),
        issued: crate::DateParts {
            year,
            month: None,
            day: None,
            raw: None,
        },
        abstract_text: None,
        language: None,
        venue: None,
        identifiers: crate::ReferenceIdentifiers::default(),
        urls: Vec::new(),
        collections: Vec::new(),
        tags: Vec::new(),
        status: crate::ReadingStatus::Unread,
        favorite: false,
        rating: None,
        citekey: None,
        citekey_locked: false,
        metadata: crate::ReferenceMetadata::default(),
        created_at: now.clone(),
        updated_at: now,
    }
}

fn duplicate_reason(
    existing: &ReferenceItem,
    imported: &ReferenceItem,
) -> Option<(DuplicateReason, u8)> {
    if same_optional_str(&existing.identifiers.doi, &imported.identifiers.doi) {
        return Some((DuplicateReason::Doi, 100));
    }
    if !existing.identifiers.isbn.is_empty()
        && existing
            .identifiers
            .isbn
            .iter()
            .any(|isbn| imported.identifiers.isbn.contains(isbn))
    {
        return Some((DuplicateReason::Isbn, 98));
    }
    if same_optional_str(
        &existing.identifiers.arxiv_id,
        &imported.identifiers.arxiv_id,
    ) {
        return Some((DuplicateReason::ArxivId, 96));
    }
    if same_title_year_creator(existing, imported) {
        return Some((DuplicateReason::TitleYearCreator, 85));
    }
    None
}

fn same_optional_str(left: &Option<String>, right: &Option<String>) -> bool {
    left.as_deref()
        .zip(right.as_deref())
        .is_some_and(|(left, right)| {
            !left.trim().is_empty() && normalize_token(left) == normalize_token(right)
        })
}

fn same_title_year_creator(existing: &ReferenceItem, imported: &ReferenceItem) -> bool {
    if existing.issued.year != imported.issued.year {
        return false;
    }
    if normalize_token(&existing.title.value) != normalize_token(&imported.title.value) {
        return false;
    }
    let existing_creator = existing.creators.first().map(Creator::sort_name);
    let imported_creator = imported.creators.first().map(Creator::sort_name);
    existing_creator
        .zip(imported_creator)
        .is_some_and(|(left, right)| normalize_token(&left) == normalize_token(&right))
}

fn normalize_token(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn ascii_slug(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn suffix(index: u32) -> char {
    char::from_u32('a' as u32 + index - 1).unwrap_or('z')
}

#[cfg(test)]
mod tests;
