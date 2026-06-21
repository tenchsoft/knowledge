use super::search::first_graphemes;
use super::*;
use std::time::Instant;

use crate::{AttachmentId, PdfDocumentText, ResearchLocale, ResearchSnapshotV2};

pub fn benchmark_research_indexing(
    input: ResearchIndexBenchmarkInput,
) -> ResearchIndexBenchmarkReport {
    let input = normalize_benchmark_input(input);
    let (snapshot, pdf_texts) = synthetic_benchmark_snapshot(&input);
    let build_started = Instant::now();
    let documents = build_research_index_documents_with_pdf_text(&snapshot, &pdf_texts);
    let build_micros = build_started.elapsed().as_micros();
    let indexed_bytes = documents
        .iter()
        .map(|document| {
            document.id.len()
                + document.title.len()
                + document.body.len()
                + document.tags.iter().map(String::len).sum::<usize>()
                + document
                    .location
                    .as_ref()
                    .map(String::len)
                    .unwrap_or_default()
        })
        .sum::<usize>() as u64;

    let request = parse_research_search_request(&input.query, input.search_limit);
    let search_started = Instant::now();
    let results = search_research_snapshot_with_pdf_text(&snapshot, &pdf_texts, &request);
    let search_micros = search_started.elapsed().as_micros();

    ResearchIndexBenchmarkReport {
        reference_count: input.reference_count,
        pdf_attachment_count: input.pdf_attachment_count,
        document_count: documents.len(),
        indexed_bytes,
        build_micros,
        search_micros,
        query_result_count: results.len(),
        ten_k_reference_target_micros: RESEARCH_10K_REFERENCE_INDEX_TARGET_MICROS,
        thousand_pdf_target_micros: RESEARCH_1K_PDF_INDEX_TARGET_MICROS,
        meets_10k_reference_target: input.reference_count >= 10_000
            && build_micros <= RESEARCH_10K_REFERENCE_INDEX_TARGET_MICROS,
        meets_1k_pdf_target: input.pdf_attachment_count >= 1_000
            && build_micros <= RESEARCH_1K_PDF_INDEX_TARGET_MICROS,
    }
}

fn normalize_benchmark_input(input: ResearchIndexBenchmarkInput) -> ResearchIndexBenchmarkInput {
    ResearchIndexBenchmarkInput {
        reference_count: input.reference_count.max(1),
        pdf_attachment_count: input.pdf_attachment_count,
        pages_per_pdf: input.pages_per_pdf.max(1),
        chars_per_pdf_page: input.chars_per_pdf_page.max(64),
        query: if input.query.trim().is_empty() {
            "benchmark retrieval".to_string()
        } else {
            input.query
        },
        search_limit: input.search_limit.max(1),
    }
}

fn synthetic_benchmark_snapshot(
    input: &ResearchIndexBenchmarkInput,
) -> (ResearchSnapshotV2, Vec<PdfDocumentText>) {
    let now = crate::Timestamp("2026-05-04T00:00:00Z".to_string());
    let locale = ResearchLocale::parse("en-US").expect("benchmark locale");
    let mut snapshot = crate::new_research_library_snapshot(
        crate::LibraryId::from("benchmark"),
        "Benchmark",
        "/tmp/tench-research-benchmark",
        locale,
        now.clone(),
    );
    for index in 0..input.reference_count {
        let mut reference = crate::reference_from_minimal_metadata(
            format!("ref_{index}"),
            crate::ReferenceKind::JournalArticle,
            format!("Benchmark Retrieval Paper {index}"),
            Some(1990 + (index % 37) as u16),
            now.0.clone(),
        );
        reference
            .tags
            .push(crate::ResearchTagId::from(format!("topic_{}", index % 25)));
        snapshot.references.push(reference);
    }

    let mut pdf_texts = Vec::new();
    for index in 0..input.pdf_attachment_count {
        let reference_id =
            crate::ReferenceId::from(format!("ref_{}", index % input.reference_count));
        let attachment_id = AttachmentId::from(format!("att_{index}"));
        snapshot.attachments.push(crate::Attachment {
            id: attachment_id.clone(),
            reference_id,
            kind: crate::AttachmentKind::Pdf,
            title: format!("Benchmark PDF {index}"),
            stored_path: format!("attachments/att_{index}.pdf"),
            original_path: None,
            mime_type: "application/pdf".to_string(),
            size_bytes: input.chars_per_pdf_page as u64 * input.pages_per_pdf as u64,
            content_hash: format!("hash-{index}"),
            page_count: Some(input.pages_per_pdf as u32),
            text_indexed: true,
            created_at: now.clone(),
            updated_at: now.clone(),
        });
        let repeated = "benchmark retrieval evidence ".repeat(
            input
                .chars_per_pdf_page
                .saturating_div("benchmark retrieval evidence ".len())
                .max(1),
        );
        let pages = (1..=input.pages_per_pdf)
            .map(|page| crate::PdfPageText {
                page: page as u32,
                text: first_graphemes(&repeated, input.chars_per_pdf_page),
                locale: None,
            })
            .collect::<Vec<_>>();
        pdf_texts.push(PdfDocumentText {
            attachment_id,
            pages,
        });
    }
    (snapshot, pdf_texts)
}
