use std::io::Cursor;

use super::*;
use crate::{
    AttachmentId, Creator, CreatorNameOrder, CreatorRole, DateParts, LibraryId, LocalizedField,
    ReadingStatus, ReferenceId, ReferenceIdentifiers, ReferenceItem, ReferenceKind,
    ReferenceMetadata, ResearchLocale, ResearchNoteId, Timestamp,
};
use tench_document_core::TenchDocument;

#[test]
fn writing_checks_find_missing_required_section() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = ResearchManuscript {
        id: ManuscriptId::from("ms_1"),
        library_id: LibraryId::from("lib_1"),
        title: LocalizedField::plain("Draft"),
        subtitle: None,
        authors: Vec::new(),
        target: ManuscriptTarget {
            kind: TargetKind::Journal,
            name: "Journal".to_string(),
            citation_style: Some("apa".to_string()),
            bibliography_locale: locale.clone(),
            word_limit: None,
            abstract_limit: None,
            section_rules: Vec::new(),
            figure_table_rules: Vec::new(),
            export_formats: vec![WritingExportFormat::Docx],
        },
        locale: locale.clone(),
        template_id: None,
        outline: ManuscriptOutline {
            sections: Vec::new(),
            required_sections: vec![RequiredSection {
                kind: SectionKind::Abstract,
                label: "Abstract".to_string(),
            }],
        },
        document: TenchDocument::new("Draft"),
        citation_state: ManuscriptCitationState {
            style_id: "apa".to_string(),
            locale,
            citations: Vec::new(),
            bibliography: BibliographySnapshot::default(),
            unresolved_citations: Vec::new(),
            citekey_map: Vec::new(),
        },
        assets: Vec::new(),
        cross_references: Vec::new(),
        checks: Vec::new(),
        created_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
        updated_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
    };

    let checks = run_non_ai_writing_checks(&manuscript);

    assert_eq!(checks.len(), 1);
    assert!(checks[0].export_blocker);
}

#[test]
fn manuscript_skeleton_creates_required_outline_sections() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let target = ManuscriptTarget {
        kind: TargetKind::Journal,
        name: "Journal".to_string(),
        citation_style: Some("apa".to_string()),
        bibliography_locale: locale.clone(),
        word_limit: None,
        abstract_limit: Some(250),
        section_rules: vec![
            SectionRule {
                kind: SectionKind::Abstract,
                required: true,
                word_limit: Some(250),
            },
            SectionRule {
                kind: SectionKind::Methods,
                required: true,
                word_limit: None,
            },
        ],
        figure_table_rules: Vec::new(),
        export_formats: vec![WritingExportFormat::Docx],
    };

    let manuscript = create_manuscript_skeleton(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        target,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );

    assert_eq!(manuscript.outline.sections.len(), 2);
    assert_eq!(manuscript.document.metadata.title, "Draft");
    assert!(run_non_ai_writing_checks(&manuscript).is_empty());
}

#[test]
fn template_creates_release_export_targets() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );

    assert!(manuscript
        .outline
        .sections
        .iter()
        .any(|section| section.kind == SectionKind::Methods));
    assert!(manuscript
        .target
        .export_formats
        .contains(&WritingExportFormat::Markdown));
}

#[test]
fn text_insertion_updates_tdm_and_section_references() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let section_id = manuscript.outline.sections[1].id.clone();

    let manuscript = insert_manuscript_text(
        manuscript,
        ManuscriptTextInsertion {
            section_id: section_id.clone(),
            text: "This introduction motivates the claim.".to_string(),
            cited_references: vec![ReferenceId::from("ref_attention")],
            now: Timestamp("2026-05-04T01:00:00Z".to_string()),
        },
    )
    .expect("insert text");

    assert!(manuscript
        .document
        .to_plain_text()
        .contains("This introduction motivates"));
    let section = manuscript
        .outline
        .sections
        .iter()
        .find(|section| section.id == section_id)
        .expect("section");
    assert!(section
        .cited_references
        .contains(&ReferenceId::from("ref_attention")));
}

#[test]
fn notes_quotes_tables_equations_and_asset_placements_update_tdm() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let section_id = SectionId::from("section-results");
    let manuscript = link_note_to_manuscript_section(
        manuscript,
        ManuscriptSectionNoteLink {
            section_id: section_id.clone(),
            note_id: ResearchNoteId::from("note_1"),
            now: Timestamp("2026-05-04T00:01:00Z".to_string()),
        },
    )
    .expect("link note");
    let manuscript = convert_annotation_to_manuscript_quote(
        manuscript,
        ManuscriptAnnotationQuoteInsertion {
            section_id: section_id.clone(),
            note_id: Some(ResearchNoteId::from("note_1")),
            annotation: crate::PdfAnnotation {
                id: crate::AnnotationId::from("ann_1"),
                attachment_id: AttachmentId::from("att_1"),
                reference_id: ReferenceId::from("ref_attention"),
                kind: crate::PdfAnnotationKind::Highlight,
                page: 4,
                rects: Vec::new(),
                color: crate::ColorRgba {
                    r: 255,
                    g: 220,
                    b: 0,
                    a: 255,
                },
                selected_text: Some("quoted evidence".to_string()),
                note_markdown: None,
                created_at: Timestamp("2026-05-04T00:01:00Z".to_string()),
                updated_at: Timestamp("2026-05-04T00:01:00Z".to_string()),
            },
            now: Timestamp("2026-05-04T00:02:00Z".to_string()),
        },
    )
    .expect("quote annotation");
    let manuscript = insert_manuscript_table(
        manuscript,
        ManuscriptTableInsertion {
            section_id: section_id.clone(),
            caption: Some(LocalizedField::plain("Result table")),
            rows: vec![
                vec!["Metric".to_string(), "Value".to_string()],
                vec!["Accuracy".to_string(), "0.91".to_string()],
            ],
            now: Timestamp("2026-05-04T00:03:00Z".to_string()),
        },
    )
    .expect("insert table");
    let manuscript = insert_manuscript_equation(
        manuscript,
        ManuscriptEquationInsertion {
            section_id: section_id.clone(),
            label: Some("loss".to_string()),
            latex: "L = -\\sum y \\log p".to_string(),
            now: Timestamp("2026-05-04T00:04:00Z".to_string()),
        },
    )
    .expect("insert equation");
    let manuscript = create_manuscript_asset(
        manuscript,
        ManuscriptAsset {
            id: ManuscriptAssetId::from("fig-results"),
            kind: AssetKind::Figure,
            label: String::new(),
            caption: LocalizedField::plain("Results overview"),
            source: AssetSource {
                kind: AssetSourceKind::Generated,
                attachment_id: None,
                path: None,
                note_id: None,
            },
            permissions: None,
            linked_references: vec![ReferenceId::from("ref_attention")],
            linked_notes: vec![ResearchNoteId::from("note_1")],
            alt_text: Some("Results overview chart".to_string()),
            order: 1,
        },
        Timestamp("2026-05-04T00:05:00Z".to_string()),
    )
    .expect("create asset");
    let manuscript = insert_manuscript_asset_placement(
        manuscript,
        ManuscriptAssetPlacement {
            section_id: section_id.clone(),
            asset_id: ManuscriptAssetId::from("fig-results"),
            now: Timestamp("2026-05-04T00:06:00Z".to_string()),
        },
    )
    .expect("place asset");

    let section = manuscript
        .outline
        .sections
        .iter()
        .find(|section| section.id == section_id)
        .expect("section");
    assert!(section
        .source_notes
        .contains(&ResearchNoteId::from("note_1")));
    assert!(section
        .cited_references
        .contains(&ReferenceId::from("ref_attention")));
    let plain_text = manuscript.document.to_plain_text();
    assert!(plain_text.contains("quoted evidence"));
    assert!(plain_text.contains("Result table"));
    assert!(plain_text.contains("L = -\\sum y \\log p"));
    assert!(plain_text.contains("Figure 1. Results overview"));
}

#[test]
fn writing_checks_cover_limits_todos_assets_duplicates_and_uncited_refs() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let mut manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    manuscript.target.word_limit = Some(8);
    manuscript = insert_manuscript_text(
        manuscript,
        ManuscriptTextInsertion {
            section_id: SectionId::from("section-abstract"),
            text: "word ".repeat(260),
            cited_references: Vec::new(),
            now: Timestamp("2026-05-04T00:01:00Z".to_string()),
        },
    )
    .expect("insert abstract");
    manuscript = insert_manuscript_text(
        manuscript,
        ManuscriptTextInsertion {
            section_id: SectionId::from("section-introduction"),
            text: "TODO cite this claim later".to_string(),
            cited_references: Vec::new(),
            now: Timestamp("2026-05-04T00:02:00Z".to_string()),
        },
    )
    .expect("insert todo");
    manuscript.assets.push(ManuscriptAsset {
        id: ManuscriptAssetId::from("fig-missing-alt"),
        kind: AssetKind::Figure,
        label: "Figure draft".to_string(),
        caption: LocalizedField::plain(""),
        source: AssetSource {
            kind: AssetSourceKind::Generated,
            attachment_id: None,
            path: None,
            note_id: None,
        },
        permissions: None,
        linked_references: Vec::new(),
        linked_notes: Vec::new(),
        alt_text: None,
        order: 1,
    });
    manuscript.citation_state.citations.push(InlineCitation {
        id: CitationId::from("cite_dup"),
        reference_ids: vec![ReferenceId::from("ref_a"), ReferenceId::from("ref_a")],
        section_id: Some(SectionId::from("section-introduction")),
        mode: CitationMode::InText,
    });
    manuscript.citation_state.bibliography.reference_ids =
        vec![ReferenceId::from("ref_a"), ReferenceId::from("ref_uncited")];

    let checks = run_non_ai_writing_checks(&manuscript);
    for code in [
        "word_limit_exceeded",
        "abstract_limit_exceeded",
        "missing_section_citation",
        "unresolved_todo",
        "missing_asset_caption",
        "missing_asset_alt_text",
        "duplicate_citation_reference",
        "uncited_bibliography_reference",
    ] {
        assert!(checks.iter().any(|check| check.code == code), "{code}");
    }
}

#[test]
fn citation_refresh_and_markdown_export_use_references() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let section_id = manuscript.outline.sections[1].id.clone();
    let reference = reference("ref_attention", "Attention Is All You Need");

    let manuscript = insert_manuscript_citation(
        manuscript,
        ManuscriptCitationInsertion {
            id: CitationId::from("cite_1"),
            reference_ids: vec![reference.id.clone()],
            section_id: Some(section_id),
            mode: CitationMode::InText,
            now: Timestamp("2026-05-04T01:00:00Z".to_string()),
        },
        std::slice::from_ref(&reference),
    )
    .expect("insert citation");
    assert!(manuscript
        .document
        .to_plain_text()
        .contains("(Vaswani, 2017)"));
    let exported = export_manuscript(
        manuscript,
        &[reference],
        WritingExportFormat::Markdown,
        Timestamp("2026-05-04T02:00:00Z".to_string()),
    )
    .expect("export manuscript");

    assert!(exported.body.contains("## References"));
    assert_eq!(exported.media_type, "text/markdown; charset=utf-8");
    assert!(exported.body_bytes.starts_with(b"# Draft"));
    assert!(exported
        .bibliography
        .rendered
        .contains("Vaswani, Ashish. (2017). Attention Is All You Need."));
}

#[test]
fn docx_and_pdf_exports_return_real_container_bytes() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let reference = reference("ref_attention", "Attention Is All You Need");

    let docx = export_manuscript(
        manuscript.clone(),
        std::slice::from_ref(&reference),
        WritingExportFormat::Docx,
        Timestamp("2026-05-04T02:00:00Z".to_string()),
    )
    .expect("docx export");
    let pdf = export_manuscript(
        manuscript,
        &[reference],
        WritingExportFormat::Pdf,
        Timestamp("2026-05-04T02:00:00Z".to_string()),
    )
    .expect("pdf export");

    assert_eq!(docx.file_name, "draft.docx");
    assert_eq!(
        docx.media_type,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    );
    assert!(docx.body_bytes.starts_with(b"PK"));
    assert_eq!(pdf.file_name, "draft.pdf");
    assert_eq!(pdf.media_type, "application/pdf");
    assert!(pdf.body_bytes.starts_with(b"%PDF-1.4"));
}

#[test]
fn archive_export_contains_manuscript_references_and_assets() {
    use std::io::Read;

    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Thesis Chapter"),
        ManuscriptTemplateKind::ThesisChapter,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let manuscript = create_manuscript_asset(
        manuscript,
        ManuscriptAsset {
            id: ManuscriptAssetId::from("fig-a"),
            kind: AssetKind::Figure,
            label: String::new(),
            caption: LocalizedField::plain("Chapter figure"),
            source: AssetSource {
                kind: AssetSourceKind::Generated,
                attachment_id: None,
                path: None,
                note_id: None,
            },
            permissions: None,
            linked_references: Vec::new(),
            linked_notes: Vec::new(),
            alt_text: Some("Chapter figure".to_string()),
            order: 1,
        },
        Timestamp("2026-05-04T00:01:00Z".to_string()),
    )
    .expect("create asset");
    let reference = reference("ref_archive", "Archived Reference");

    let exported = export_manuscript(
        manuscript,
        std::slice::from_ref(&reference),
        WritingExportFormat::Archive,
        Timestamp("2026-05-04T00:02:00Z".to_string()),
    )
    .expect("archive export");

    assert_eq!(exported.file_name, "thesis-chapter.zip");
    assert_eq!(exported.media_type, "application/zip");
    assert!(exported.body_bytes.starts_with(b"PK"));
    let mut archive = zip::ZipArchive::new(Cursor::new(exported.body_bytes)).expect("archive zip");
    let mut manifest = String::new();
    archive
        .by_name("manifest.txt")
        .expect("manifest")
        .read_to_string(&mut manifest)
        .expect("read manifest");
    assert!(manifest.contains("assets: 1"));
    assert!(archive.by_name("manuscript.md").is_ok());
    assert!(archive.by_name("references.json").is_ok());
    assert!(archive.by_name("assets.json").is_ok());
}

#[test]
fn manuscript_assets_are_numbered_and_cross_referenced() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Paper"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let manuscript = create_manuscript_asset(
        manuscript,
        ManuscriptAsset {
            id: ManuscriptAssetId::from("fig-a"),
            kind: AssetKind::Figure,
            label: String::new(),
            caption: LocalizedField::plain("First result chart"),
            source: AssetSource {
                kind: AssetSourceKind::Generated,
                attachment_id: None,
                path: None,
                note_id: None,
            },
            permissions: None,
            linked_references: Vec::new(),
            linked_notes: Vec::new(),
            alt_text: Some("First result chart".to_string()),
            order: 1,
        },
        Timestamp("2026-05-04T00:01:00Z".to_string()),
    )
    .expect("create first figure");
    let manuscript = create_manuscript_asset(
        manuscript,
        ManuscriptAsset {
            id: ManuscriptAssetId::from("fig-b"),
            kind: AssetKind::Figure,
            label: String::new(),
            caption: LocalizedField::plain("Second result chart"),
            source: AssetSource {
                kind: AssetSourceKind::Generated,
                attachment_id: None,
                path: None,
                note_id: None,
            },
            permissions: None,
            linked_references: Vec::new(),
            linked_notes: Vec::new(),
            alt_text: Some("Second result chart".to_string()),
            order: 2,
        },
        Timestamp("2026-05-04T00:02:00Z".to_string()),
    )
    .expect("create second figure");

    let numbering = build_manuscript_asset_numbering(&manuscript);
    assert_eq!(numbering[0].label, "Figure 1");
    assert_eq!(numbering[1].label, "Figure 2");

    let manuscript = insert_manuscript_cross_reference(
        manuscript,
        ManuscriptCrossReferenceInsertion {
            id: ManuscriptCrossReferenceId::from("xref-1"),
            section_id: SectionId::from("section-introduction"),
            target: ManuscriptCrossReferenceTarget::Asset {
                asset_id: ManuscriptAssetId::from("fig-b"),
            },
            now: Timestamp("2026-05-04T00:03:00Z".to_string()),
        },
    )
    .expect("insert cross-reference");

    assert!(manuscript
        .document
        .to_plain_text()
        .contains("See Figure 2."));
    assert!(run_non_ai_writing_checks(&manuscript)
        .iter()
        .all(|check| check.code != "broken_cross_reference_asset"));

    let mut broken = manuscript;
    broken.assets.clear();
    let checks = run_non_ai_writing_checks(&broken);
    assert!(checks
        .iter()
        .any(|check| check.code == "broken_cross_reference_asset"));
}

#[test]
fn citation_refresh_reports_missing_metadata_and_blocks_missing_references() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let reference = {
        let mut reference = reference("ref_attention", "Attention Is All You Need");
        reference.creators.clear();
        reference.issued.year = None;
        reference
    };
    let manuscript = insert_manuscript_citation(
        manuscript,
        ManuscriptCitationInsertion {
            id: CitationId::from("cite_1"),
            reference_ids: vec![reference.id.clone(), ReferenceId::from("missing_ref")],
            section_id: None,
            mode: CitationMode::InText,
            now: Timestamp("2026-05-04T01:00:00Z".to_string()),
        },
        std::slice::from_ref(&reference),
    )
    .expect("insert citation");

    assert!(manuscript
        .citation_state
        .unresolved_citations
        .iter()
        .any(|issue| issue.code == "missing_reference"));
    assert!(manuscript
        .checks
        .iter()
        .any(|check| check.code == "missing_reference" && check.export_blocker));
    assert!(manuscript
        .checks
        .iter()
        .any(|check| check.code == "citation_metadata_warning"));
}

#[test]
fn snapshot_diff_reports_added_words_and_lines() {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    let manuscript = create_manuscript_from_template(
        ManuscriptId::from("ms"),
        LibraryId::from("lib"),
        LocalizedField::plain("Draft"),
        ManuscriptTemplateKind::JournalArticle,
        locale,
        Timestamp("2026-05-04T00:00:00Z".to_string()),
    );
    let before = create_manuscript_snapshot(
        &manuscript,
        SnapshotId::from("before"),
        Timestamp("2026-05-04T00:01:00Z".to_string()),
    );
    let manuscript = insert_manuscript_text(
        manuscript,
        ManuscriptTextInsertion {
            section_id: SectionId::from("section-introduction"),
            text: "A new line appears.".to_string(),
            cited_references: Vec::new(),
            now: Timestamp("2026-05-04T00:02:00Z".to_string()),
        },
    )
    .expect("insert text");
    let after = create_manuscript_snapshot(
        &manuscript,
        SnapshotId::from("after"),
        Timestamp("2026-05-04T00:03:00Z".to_string()),
    );

    let diff = compare_manuscript_snapshots(&before, &after);

    assert!(diff.word_delta > 0);
    assert!(diff
        .added_lines
        .iter()
        .any(|line| line.contains("A new line appears")));
}

fn reference(id: &str, title: &str) -> ReferenceItem {
    ReferenceItem {
        id: ReferenceId::from(id),
        kind: ReferenceKind::JournalArticle,
        title: LocalizedField::plain(title),
        subtitle: None,
        creators: vec![Creator {
            role: CreatorRole::Author,
            given: Some("Ashish".to_string()),
            family: Some("Vaswani".to_string()),
            literal: None,
            transliteration: None,
            sort_key: None,
            name_order: CreatorNameOrder::GivenFirst,
            locale: None,
            orcid: None,
            affiliation: None,
        }],
        issued: DateParts {
            year: Some(2017),
            month: None,
            day: None,
            raw: None,
        },
        abstract_text: None,
        language: None,
        venue: None,
        identifiers: ReferenceIdentifiers::default(),
        urls: Vec::new(),
        collections: Vec::new(),
        tags: Vec::new(),
        status: ReadingStatus::Reviewed,
        favorite: false,
        rating: None,
        citekey: None,
        citekey_locked: false,
        metadata: ReferenceMetadata::default(),
        created_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
        updated_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
    }
}
