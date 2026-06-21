use super::*;
use crate::{AttachmentId, PdfDocumentText, ReferenceId, ResearchSnapshotV2};
use unicode_segmentation::UnicodeSegmentation;

#[test]
fn shared_query_includes_tags_and_limit() {
    let request = ResearchSearchRequest {
        query: "  transformer   attention ".to_string(),
        locale: None,
        filters: vec![SearchFilter {
            field: SearchField::Tag,
            value: "nlp".to_string(),
        }],
        sort: SearchSort::default(),
        limit: 0,
    };

    let query = to_shared_search_query(&request);

    assert_eq!(query.text, "transformer attention");
    assert_eq!(query.tags, vec!["nlp"]);
    assert_eq!(query.limit, 1);
}

#[test]
fn parser_extracts_field_filters() {
    let request = parse_research_search_request(
        "tag:nlp year:2020 sort:title order:asc \"dense retrieval\"",
        10,
    );

    assert_eq!(request.query, "dense retrieval");
    assert_eq!(request.filters.len(), 2);
    assert_eq!(request.filters[0].field, SearchField::Tag);
    assert_eq!(
        request.sort,
        SearchSort {
            field: SearchSortField::Title,
            direction: SortDirection::Asc
        }
    );
}

#[test]
fn snapshot_search_finds_reference_notes_and_annotations() {
    let now = crate::Timestamp("2026-05-04T00:00:00Z".to_string());
    let locale = crate::ResearchLocale::parse("en-US").unwrap();
    let mut snapshot = crate::new_research_library_snapshot(
        crate::LibraryId::from("lib"),
        "Library",
        "/tmp/lib",
        locale,
        now.clone(),
    );
    let mut reference = crate::reference_from_minimal_metadata(
        "ref_1",
        crate::ReferenceKind::JournalArticle,
        "Retrieval Paper",
        Some(2020),
        now.0.clone(),
    );
    reference.tags.push(crate::ResearchTagId::from("nlp"));
    snapshot.references.push(reference);
    snapshot.notes.push(crate::ResearchNote {
        id: crate::ResearchNoteId::from("note_1"),
        reference_id: Some(ReferenceId::from("ref_1")),
        annotation_id: None,
        title: "Reading note".to_string(),
        body_markdown: "dense passage retrieval benchmark".to_string(),
        tags: vec![crate::ResearchTagId::from("nlp")],
        backlinks: Vec::new(),
        created_at: now.clone(),
        updated_at: now.clone(),
    });
    snapshot.annotations.push(crate::PdfAnnotation {
        id: crate::AnnotationId::from("ann_1"),
        attachment_id: crate::AttachmentId::from("att_1"),
        reference_id: ReferenceId::from("ref_1"),
        kind: crate::PdfAnnotationKind::Highlight,
        page: 3,
        rects: Vec::new(),
        color: crate::ColorRgba {
            r: 255,
            g: 220,
            b: 0,
            a: 255,
        },
        selected_text: Some("important retrieval evidence".to_string()),
        note_markdown: None,
        created_at: now.clone(),
        updated_at: now,
    });

    let request = parse_research_search_request("tag:nlp retrieval", 10);
    let results = search_research_snapshot(&snapshot, &request);

    assert!(results.iter().any(|result| result.id == "reference:ref_1"));
    assert!(results.iter().any(|result| result.id == "note:note_1"));
    assert!(results.iter().any(|result| result.id == "annotation:ann_1"));
    assert!(results.iter().all(|result| result
        .location
        .as_deref()
        .unwrap_or("")
        .starts_with("research://")));
}

#[test]
fn snapshot_search_matches_accent_folded_metadata() {
    let now = crate::Timestamp("2026-05-04T00:00:00Z".to_string());
    let locale = crate::ResearchLocale::parse("fr-FR").unwrap();
    let mut snapshot = crate::new_research_library_snapshot(
        crate::LibraryId::from("lib"),
        "Library",
        "/tmp/lib",
        locale,
        now.clone(),
    );
    snapshot
        .references
        .push(crate::reference_from_minimal_metadata(
            "ref_1",
            crate::ReferenceKind::JournalArticle,
            "Café et cognition",
            Some(2026),
            now.0,
        ));

    let request = parse_research_search_request("cafe", 10);
    let results = search_research_snapshot(&snapshot, &request);

    assert_eq!(results[0].id, "reference:ref_1");
}

#[test]
fn snapshot_search_snippet_uses_normalized_grapheme_safe_offsets() {
    let now = crate::Timestamp("2026-05-04T00:00:00Z".to_string());
    let locale = crate::ResearchLocale::parse("fr-FR").unwrap();
    let mut snapshot = crate::new_research_library_snapshot(
        crate::LibraryId::from("lib"),
        "Library",
        "/tmp/lib",
        locale,
        now.clone(),
    );
    snapshot.notes.push(crate::ResearchNote {
        id: crate::ResearchNoteId::from("note_1"),
        reference_id: None,
        annotation_id: None,
        title: "Reading note".to_string(),
        body_markdown: format!(
            "{} Café\u{301} retrieval evidence stays near the hit.",
            "background ".repeat(220)
        ),
        tags: Vec::new(),
        backlinks: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    });

    let request = parse_research_search_request("cafe retrieval", 10);
    let results = search_research_snapshot(&snapshot, &request);

    assert_eq!(results[0].id, "note:note_1");
    assert!(results[0].snippet.contains("Café\u{301} retrieval"));
    assert!(!results[0].snippet.starts_with("background background"));
    assert!(results[0]
        .snippet
        .graphemes(true)
        .all(|grapheme| { !grapheme.ends_with('\u{301}') || grapheme.chars().count() > 1 }));
}

#[test]
fn pdf_full_text_index_provides_hit_locations() {
    let (snapshot, pdf_texts) = sample_pdf_snapshot();
    let request = parse_research_search_request("tag:#nlp transformer", 10);

    let results = search_research_snapshot_with_hits(&snapshot, &pdf_texts, &request);

    let pdf_result = results
        .iter()
        .find(|result| result.result.id == "pdf:att_1:page:2")
        .expect("pdf page result");
    let pdf_hit = pdf_result
        .hits
        .iter()
        .find(|hit| hit.field == ResearchSearchHitField::PdfPage)
        .expect("pdf page hit");
    assert_eq!(pdf_hit.attachment_id, Some(AttachmentId::from("att_1")));
    assert_eq!(pdf_hit.page, Some(2));
    assert!(pdf_result
        .result
        .location
        .as_deref()
        .unwrap_or("")
        .ends_with("/page/2"));
}

#[test]
fn incremental_reindex_plans_changed_retryable_and_removed_documents() {
    let (snapshot, pdf_texts) = sample_pdf_snapshot();
    let mut existing = build_research_index_manifest(
        &build_research_index_documents_with_pdf_text(&snapshot, &pdf_texts),
    );
    existing
        .iter_mut()
        .find(|entry| entry.document_id == "reference:ref_1")
        .expect("reference entry")
        .content_hash = "old".to_string();
    existing.push(ResearchIndexManifestEntry {
        document_id: "note:removed".to_string(),
        content_hash: "stale".to_string(),
    });

    let plan = plan_incremental_research_reindex(
        &snapshot,
        &pdf_texts,
        &existing,
        vec![ResearchIndexFailure {
            reference_id: None,
            attachment_id: Some(AttachmentId::from("att_1")),
            code: "extract_failed".to_string(),
            message: "retry".to_string(),
            retryable: true,
        }],
        Some("2026-05-04T00:00:00Z".to_string()),
    );

    assert!(plan
        .upsert_documents
        .iter()
        .any(|document| document.id == "reference:ref_1"));
    assert!(plan
        .upsert_documents
        .iter()
        .any(|document| document.id == "pdf:att_1:page:2"));
    assert_eq!(plan.remove_document_ids, vec!["note:removed"]);
    assert_eq!(
        plan.state.pending_references,
        vec![ReferenceId::from("ref_1")]
    );
    assert_eq!(plan.state.stats.failed_items, 1);
}

#[test]
fn index_repair_rebuilds_current_documents_and_reports_stale_ids() {
    let (snapshot, pdf_texts) = sample_pdf_snapshot();

    let report = repair_research_index(
        &snapshot,
        &pdf_texts,
        &[
            "reference:ref_1".to_string(),
            "pdf:missing:page:1".to_string(),
        ],
        Vec::new(),
        None,
    );

    assert!(report
        .documents
        .iter()
        .any(|document| document.id == "pdf:att_1:page:2"));
    assert_eq!(report.removed_document_ids, vec!["pdf:missing:page:1"]);
    assert_eq!(
        report.state.pending_references,
        vec![ReferenceId::from("ref_1")]
    );
    assert_eq!(report.manifest.len(), report.documents.len());
}

#[test]
fn benchmark_report_measures_index_build_and_search() {
    let report = benchmark_research_indexing(ResearchIndexBenchmarkInput {
        reference_count: 12,
        pdf_attachment_count: 3,
        pages_per_pdf: 2,
        chars_per_pdf_page: 128,
        query: "benchmark retrieval".to_string(),
        search_limit: 5,
    });

    assert_eq!(report.reference_count, 12);
    assert_eq!(report.pdf_attachment_count, 3);
    assert!(report.document_count >= 18);
    assert!(report.indexed_bytes > 0);
    assert!(report.query_result_count > 0);
    assert_eq!(
        report.ten_k_reference_target_micros,
        RESEARCH_10K_REFERENCE_INDEX_TARGET_MICROS
    );
}

fn sample_pdf_snapshot() -> (ResearchSnapshotV2, Vec<PdfDocumentText>) {
    let now = crate::Timestamp("2026-05-04T00:00:00Z".to_string());
    let locale = crate::ResearchLocale::parse("en-US").unwrap();
    let mut snapshot = crate::new_research_library_snapshot(
        crate::LibraryId::from("lib"),
        "Library",
        "/tmp/lib",
        locale,
        now.clone(),
    );
    let mut reference = crate::reference_from_minimal_metadata(
        "ref_1",
        crate::ReferenceKind::JournalArticle,
        "Transformer Search",
        Some(2024),
        now.0.clone(),
    );
    reference.tags.push(crate::ResearchTagId::from("nlp"));
    snapshot.references.push(reference);
    snapshot.attachments.push(crate::Attachment {
        id: AttachmentId::from("att_1"),
        reference_id: ReferenceId::from("ref_1"),
        kind: crate::AttachmentKind::Pdf,
        title: "Transformer PDF".to_string(),
        stored_path: "attachments/att_1.pdf".to_string(),
        original_path: None,
        mime_type: "application/pdf".to_string(),
        size_bytes: 2048,
        content_hash: "hash".to_string(),
        page_count: Some(2),
        text_indexed: true,
        created_at: now.clone(),
        updated_at: now,
    });
    let pdf_texts = vec![PdfDocumentText {
        attachment_id: AttachmentId::from("att_1"),
        pages: vec![
            crate::PdfPageText {
                page: 1,
                text: "background".to_string(),
                locale: None,
            },
            crate::PdfPageText {
                page: 2,
                text: "The transformer attention result appears here.".to_string(),
                locale: None,
            },
        ],
    }];
    (snapshot, pdf_texts)
}
