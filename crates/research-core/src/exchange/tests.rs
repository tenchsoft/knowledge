use super::*;
use crate::{
    new_research_library_snapshot, Creator, DateParts, LibraryId, ReferenceIdentifiers,
    ReferenceMetadata, ResearchLocale,
};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn reference(id: &str, title: &str) -> ReferenceItem {
    let mut reference = reference_from_minimal_metadata(
        id,
        ReferenceKind::JournalArticle,
        title,
        Some(2017),
        "2026-05-04T00:00:00Z",
    );
    reference.creators.push(Creator {
        role: CreatorRole::Author,
        given: Some("Ashish".to_string()),
        family: Some("Vaswani".to_string()),
        literal: None,
        transliteration: None,
        sort_key: None,
        name_order: crate::CreatorNameOrder::LocaleDefault,
        locale: None,
        orcid: None,
        affiliation: None,
    });
    reference.issued = DateParts {
        year: Some(2017),
        month: None,
        day: None,
        raw: None,
    };
    reference.identifiers = ReferenceIdentifiers::default();
    reference.metadata = ReferenceMetadata::default();
    reference
}

#[test]
fn detects_common_import_formats() {
    assert_eq!(
        detect_import_format(".bib"),
        Some(ResearchImportFormat::BibTex)
    );
    assert_eq!(detect_import_format("RIS"), Some(ResearchImportFormat::Ris));
}

#[test]
fn citekey_collision_adds_suffix() {
    let mut references = vec![reference("r1", "Attention"), reference("r2", "Attention")];

    resolve_citekey_collisions(&mut references);

    assert_eq!(
        references[0].citekey.as_ref().map(Citekey::as_str),
        Some("vaswani2017attention")
    );
    assert_eq!(
        references[1].citekey.as_ref().map(Citekey::as_str),
        Some("vaswani2017attentiona")
    );
}

#[test]
fn locked_citekey_is_preserved_when_collisions_resolve() {
    let mut unlocked = reference("r1", "Attention");
    unlocked.citekey = Some(Citekey::from("manual-key"));
    let mut locked = reference("r2", "Attention");
    locked.citekey = Some(Citekey::from("manual-key"));
    locked.citekey_locked = true;
    let mut references = vec![unlocked, locked];

    resolve_citekey_collisions(&mut references);

    assert_eq!(
        references[0].citekey.as_ref().map(Citekey::as_str),
        Some("manual-keya")
    );
    assert_eq!(
        references[1].citekey.as_ref().map(Citekey::as_str),
        Some("manual-key")
    );
}

#[test]
fn bibtex_round_trips_core_metadata() {
    let text = r#"
@article{vaswani2017attention,
  title = {Attention Is All You Need},
  author = {Vaswani, Ashish and Shazeer, Noam},
  year = {2017},
  journal = {NeurIPS},
  doi = {10.5555/attention}
}
"#;

    let references =
        parse_references_text(ResearchImportFormat::BibTex, text, "2026-05-04T00:00:00Z")
            .expect("parse bibtex");
    let exported =
        export_references_text(ResearchExportFormat::BibTex, &references).expect("export bibtex");

    assert_eq!(references.len(), 1);
    assert_eq!(references[0].title.value, "Attention Is All You Need");
    assert_eq!(references[0].creators.len(), 2);
    assert!(exported.contains("@article{vaswani2017attention"));
    assert!(exported.contains("doi = {10.5555/attention}"));
}

#[test]
fn ris_import_reads_authors_year_and_url() {
    let text = "\
TY  - JOUR
TI  - Retrieval-Augmented Generation
AU  - Lewis, Patrick
PY  - 2020
JO  - NeurIPS
UR  - https://example.test/paper
ER  -
";

    let references = parse_references_text(ResearchImportFormat::Ris, text, "2026-05-04T00:00:00Z")
        .expect("parse ris");

    assert_eq!(references[0].kind, ReferenceKind::JournalArticle);
    assert_eq!(references[0].issued.year, Some(2020));
    assert_eq!(references[0].urls[0].url, "https://example.test/paper");
}

#[test]
fn csl_json_import_and_export_preserves_language() {
    let text = r#"[
  {
    "id": "paper",
    "type": "article-journal",
    "title": "다국어 논문",
    "author": [{"family": "홍", "given": "길동"}],
    "issued": {"date-parts": [[2026]]},
    "language": "ko-KR",
    "DOI": "10.1000/example"
  }
]"#;

    let references =
        parse_references_text(ResearchImportFormat::CslJson, text, "2026-05-04T00:00:00Z")
            .expect("parse csl");
    let exported =
        export_references_text(ResearchExportFormat::CslJson, &references).expect("export csl");

    assert_eq!(
        references[0].language.as_ref().map(ResearchLocale::bcp47),
        Some("ko-KR".to_string())
    );
    assert!(exported.contains("\"language\": \"ko-KR\""));
}

#[test]
fn endnote_xml_import_and_export_preserves_core_metadata() {
    let text = r#"<?xml version="1.0" encoding="UTF-8"?>
<xml>
  <records>
    <record>
      <rec-number>42</rec-number>
      <ref-type name="Journal Article">17</ref-type>
      <contributors>
        <authors>
          <author>홍, 길동</author>
          <author>Vaswani, Ashish</author>
        </authors>
      </contributors>
      <titles>
        <title>다국어 논문 &amp; Retrieval</title>
        <secondary-title>NeurIPS</secondary-title>
      </titles>
      <dates><year>2026</year></dates>
      <electronic-resource-num>10.1000/example</electronic-resource-num>
      <urls><related-urls><url>https://example.test/paper</url></related-urls></urls>
      <language>ko-KR</language>
    </record>
  </records>
</xml>"#;

    let references = parse_references_text(
        ResearchImportFormat::EndNoteXml,
        text,
        "2026-05-04T00:00:00Z",
    )
    .expect("parse endnote xml");
    let exported = export_references_text(ResearchExportFormat::EndNoteXml, &references)
        .expect("export endnote xml");

    assert_eq!(references.len(), 1);
    assert_eq!(references[0].id.as_str(), "endnote-42");
    assert_eq!(references[0].kind, ReferenceKind::JournalArticle);
    assert_eq!(references[0].title.value, "다국어 논문 & Retrieval");
    assert_eq!(references[0].creators.len(), 2);
    assert_eq!(references[0].issued.year, Some(2026));
    assert_eq!(
        references[0].language.as_ref().map(ResearchLocale::bcp47),
        Some("ko-KR".to_string())
    );
    assert!(exported.contains("<record>"));
    assert!(exported.contains("다국어 논문 &amp; Retrieval"));
    assert!(exported.contains("<electronic-resource-num>10.1000/example</electronic-resource-num>"));
}

#[test]
fn identifier_text_imports_create_local_references_without_network() {
    let doi_refs = parse_references_text(
        ResearchImportFormat::Doi,
        "https://doi.org/10.1000/ABC\n10.5555/example",
        "2026-05-04T00:00:00Z",
    )
    .expect("parse doi");
    let isbn_refs = parse_references_text(
        ResearchImportFormat::Isbn,
        "978-0-13-110362-7",
        "2026-05-04T00:00:00Z",
    )
    .expect("parse isbn");
    let arxiv_refs = parse_references_text(
        ResearchImportFormat::Arxiv,
        "arXiv:1706.03762",
        "2026-05-04T00:00:00Z",
    )
    .expect("parse arxiv");

    assert_eq!(doi_refs.len(), 2);
    assert_eq!(doi_refs[0].identifiers.doi.as_deref(), Some("10.1000/abc"));
    assert_eq!(isbn_refs[0].kind, ReferenceKind::Book);
    assert_eq!(isbn_refs[0].identifiers.isbn[0], "9780131103627");
    assert_eq!(arxiv_refs[0].kind, ReferenceKind::Preprint);
    assert_eq!(
        arxiv_refs[0].identifiers.arxiv_id.as_deref(),
        Some("1706.03762")
    );
}

#[test]
fn rtf_bibliography_export_escapes_unicode_and_braces() {
    let mut reference = reference("ref_unicode", "심장 {Structure}");
    reference.creators[0].family = Some("홍".to_string());

    let exported = export_references_text(ResearchExportFormat::RtfBibliography, &[reference])
        .expect("export rtf");

    assert!(exported.starts_with("{\\rtf1"));
    assert!(exported.contains("\\u49900?"));
    assert!(exported.contains("\\{Structure\\}"));
}

#[test]
fn duplicate_detection_prefers_identifier_matches() {
    let mut existing = reference("existing", "Attention");
    existing.identifiers.doi = Some("10.1000/ABC".to_string());
    let mut imported = reference("imported", "Different Title");
    imported.identifiers.doi = Some("10.1000/abc".to_string());

    let candidates = find_duplicate_candidates(&[existing], &[imported]);

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].reason, DuplicateReason::Doi);
    assert_eq!(candidates[0].confidence, 100);
}

#[test]
fn pdf_import_copies_files_and_preserves_unicode_title() {
    let root = temp_test_dir("pdf-import");
    let source_dir = root.join("source");
    let library_root = root.join("library");
    fs::create_dir_all(&source_dir).expect("source dir");
    let pdf_path = source_dir.join("심장 구조.pdf");
    fs::write(&pdf_path, b"%PDF-1.7\nheart").expect("write pdf");
    let locale = ResearchLocale::parse("ko-KR").expect("locale");
    let snapshot = new_research_library_snapshot(
        LibraryId::from("lib"),
        "Research",
        library_root.to_string_lossy(),
        locale.clone(),
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );

    let outcome = import_pdf_paths(
        snapshot,
        &library_root,
        &[pdf_path],
        PdfImportOptions {
            locale: Some(locale.clone()),
            ..PdfImportOptions::default()
        },
        ImportBatchId::from("batch_1"),
        DedupePolicy::KeepBoth,
        "2026-05-04T00:00:00Z",
    )
    .expect("import pdf");

    assert_eq!(outcome.report.imported.len(), 1);
    assert_eq!(outcome.snapshot.references[0].title.value, "심장 구조");
    assert_eq!(
        outcome.snapshot.references[0]
            .language
            .as_ref()
            .map(ResearchLocale::bcp47),
        Some("ko-KR".to_string())
    );
    let attachment = &outcome.snapshot.attachments[0];
    assert!(attachment.stored_path.contains("심장 구조.pdf"));
    assert!(library_root.join(&attachment.stored_path).is_file());
    assert!(outcome
        .report
        .jobs
        .iter()
        .any(|job| job.kind == ResearchJobKind::ExtractPdfText.as_str()));
    assert!(outcome
        .report
        .jobs
        .iter()
        .any(|job| job.kind == ResearchJobKind::RenderPdfPage.as_str()));

    fs::remove_dir_all(root).expect("remove temp dir");
}

#[test]
fn pdf_import_prefers_existing_duplicate_hash_when_requested() {
    let root = temp_test_dir("pdf-duplicate");
    let source_dir = root.join("source");
    let library_root = root.join("library");
    fs::create_dir_all(&source_dir).expect("source dir");
    let pdf_path = source_dir.join("paper.pdf");
    fs::write(&pdf_path, b"%PDF-1.7\nsame").expect("write pdf");
    let snapshot = new_research_library_snapshot(
        LibraryId::from("lib"),
        "Research",
        library_root.to_string_lossy(),
        ResearchLocale::parse("en-US").expect("locale"),
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let first = import_pdf_paths(
        snapshot,
        &library_root,
        std::slice::from_ref(&pdf_path),
        PdfImportOptions::default(),
        ImportBatchId::from("batch_1"),
        DedupePolicy::KeepBoth,
        "2026-05-04T00:00:00Z",
    )
    .expect("first import");

    let second = import_pdf_paths(
        first.snapshot,
        &library_root,
        &[pdf_path],
        PdfImportOptions::default(),
        ImportBatchId::from("batch_2"),
        DedupePolicy::PreferExisting,
        "2026-05-04T00:00:01Z",
    )
    .expect("second import");

    assert_eq!(second.snapshot.references.len(), 1);
    assert_eq!(second.report.duplicates.len(), 1);
    assert_eq!(
        second.report.duplicates[0].reason,
        DuplicateReason::AttachmentHash
    );
    assert!(second
        .report
        .issues
        .iter()
        .any(|issue| issue.code == "duplicate_pdf_skipped"));

    fs::remove_dir_all(root).expect("remove temp dir");
}

fn temp_test_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("tench-research-core-{name}-{unique}"));
    fs::create_dir_all(&root).expect("create temp dir");
    root
}
