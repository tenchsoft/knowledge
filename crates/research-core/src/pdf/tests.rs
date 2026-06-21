use super::*;
use unicode_segmentation::UnicodeSegmentation;

#[test]
fn annotation_markdown_preserves_page_and_text() {
    let markdown = export_annotations_markdown(&[PdfAnnotationExportItem {
        id: AnnotationId::from("a1"),
        kind: PdfAnnotationKind::Highlight,
        page: 3,
        color: ColorRgba {
            r: 255,
            g: 220,
            b: 0,
            a: 255,
        },
        selected_text: Some("important\nclaim".to_string()),
        note_markdown: Some("Check source.".to_string()),
    }]);

    assert!(markdown.contains("page 3"));
    assert!(markdown.contains("important claim"));
}

#[test]
fn cache_manifest_invalidates_attachment_entries() {
    let attachment_id = AttachmentId::from("pdf_1");
    let key = PdfCacheKey {
        attachment_id: attachment_id.clone(),
        attachment_hash: "abc123".to_string(),
        page: Some(1),
        annotation_updated_at: None,
        kind: PdfCacheKind::AnnotationOverlay,
    };
    let mut manifest = PdfCacheManifest::default();
    manifest.upsert(PdfCacheEntry {
        relative_path: key.stable_file_name(),
        key,
        bytes: 12,
        created_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
    });

    assert_eq!(manifest.invalidate_annotation_overlays(&attachment_id), 1);
    assert!(manifest.entries.is_empty());
}

#[test]
fn reader_action_searches_and_navigates_pages() {
    let attachment_id = AttachmentId::from("pdf_1");
    let document = PdfDocumentText {
        attachment_id: attachment_id.clone(),
        pages: vec![
            PdfPageText {
                page: 1,
                text: "Transformer attention".to_string(),
                locale: None,
            },
            PdfPageText {
                page: 2,
                text: "Retrieval attention evidence".to_string(),
                locale: None,
            },
        ],
    };
    let state = PdfReaderState {
        attachment_id,
        current_page: 1,
        zoom: 1.0,
        pan_x: 0.0,
        pan_y: 0.0,
        mode: PdfReaderMode::SinglePage,
        theme: PdfReaderTheme::Paper,
        search: None,
        selected_text: None,
    };

    let state = apply_pdf_reader_action(state, PdfReaderAction::NextPage, 2, None);
    let state = apply_pdf_reader_action(
        state,
        PdfReaderAction::Search {
            query: "attention".to_string(),
        },
        2,
        Some(&document),
    );

    assert_eq!(state.current_page, 2);
    assert_eq!(state.search.as_ref().unwrap().results.len(), 2);
}

#[test]
fn pdf_inspection_detects_metadata_pages_and_text() {
    let bytes = b"%PDF-1.7
1 0 obj << /Title (Heart Structure) /Author (Ada) >>
2 0 obj << /Type /Pages >>
3 0 obj << /Type /Page >>
BT (left ventricle) Tj ET
%%EOF";

    let result = inspect_pdf_document_bytes(AttachmentId::from("pdf_1"), bytes);
    let text =
        extract_pdf_literal_text(AttachmentId::from("pdf_1"), bytes, None).expect("extract text");

    assert_eq!(result.metadata.status, PdfDocumentStatus::Ready);
    assert_eq!(result.metadata.pdf_version.as_deref(), Some("1.7"));
    assert_eq!(result.metadata.page_count, Some(1));
    assert_eq!(result.metadata.title.as_deref(), Some("Heart Structure"));
    assert!(result.metadata.text_extractable);
    assert!(text.pages[0].text.contains("left ventricle"));
}

#[test]
fn pdf_inspection_reports_encrypted_and_image_only_states() {
    let encrypted = inspect_pdf_document_bytes(
        AttachmentId::from("pdf_1"),
        b"%PDF-1.7
1 0 obj << /Encrypt 2 0 R >>",
    );
    let image_only = inspect_pdf_document_bytes(
        AttachmentId::from("pdf_2"),
        b"%PDF-1.7
1 0 obj << /Type /Page >>",
    );

    assert_eq!(encrypted.metadata.status, PdfDocumentStatus::Encrypted);
    assert_eq!(image_only.metadata.status, PdfDocumentStatus::ImageOnly);
    assert!(image_only.warning.is_some());
}

#[test]
fn render_page_returns_text_preview_bitmap_and_cache_key() {
    let attachment_id = AttachmentId::from("pdf_1");
    let document = PdfDocumentText {
        attachment_id: attachment_id.clone(),
        pages: vec![PdfPageText {
            page: 2,
            text: "Rendered page preview uses extracted text.".to_string(),
            locale: None,
        }],
    };
    let page = render_pdf_page(
        PdfRenderRequest {
            attachment_id,
            attachment_hash: "hash".to_string(),
            page: 2,
            zoom: 0.5,
            max_dimension_px: 128,
            theme: Some(PdfReaderTheme::Sepia),
        },
        Some(&document),
    );

    assert_eq!(page.page, 2);
    assert_eq!(page.pixel_format, PdfPixelFormat::Rgba8);
    assert_eq!(
        page.pixels.len(),
        (page.width_px * page.height_px * 4) as usize
    );
    assert_eq!(page.cache_key.kind, PdfCacheKind::RenderedPageBitmap);
    assert_eq!(page.render_quality, PdfRenderQuality::TextPreview);
    assert!(page.accessibility_summary.contains("text preview"));
}

#[test]
fn render_page_from_bytes_uses_pdf_text_pipeline() {
    let page = render_pdf_page_from_bytes(
        PdfRenderRequest {
            attachment_id: AttachmentId::from("pdf_1"),
            attachment_hash: "hash".to_string(),
            page: 1,
            zoom: 1.0,
            max_dimension_px: 128,
            theme: Some(PdfReaderTheme::Paper),
        },
        b"%PDF-1.7
1 0 obj << /Type /Page >>
BT (left ventricle) Tj ET
%%EOF",
        None,
    )
    .expect("render bytes");

    assert_eq!(page.render_quality, PdfRenderQuality::TextPreview);
}

#[test]
fn render_page_without_text_reports_document_shell_quality() {
    let page = render_pdf_page(
        PdfRenderRequest {
            attachment_id: AttachmentId::from("pdf_shell"),
            attachment_hash: "hash".to_string(),
            page: 1,
            zoom: 0.5,
            max_dimension_px: 128,
            theme: Some(PdfReaderTheme::Sepia),
        },
        None,
    );

    assert_eq!(page.page, 1);
    assert_eq!(page.pixel_format, PdfPixelFormat::Rgba8);
    assert_eq!(
        page.pixels.len(),
        (page.width_px * page.height_px * 4) as usize
    );
    assert_eq!(page.render_quality, PdfRenderQuality::DocumentShell);
}

#[test]
fn thumbnail_strip_uses_cache_window_and_thumbnail_keys() {
    let document = PdfDocumentText {
        attachment_id: AttachmentId::from("pdf_1"),
        pages: vec![
            PdfPageText {
                page: 2,
                text: "middle page".to_string(),
                locale: None,
            },
            PdfPageText {
                page: 3,
                text: "selected page".to_string(),
                locale: None,
            },
        ],
    };

    let strip = build_pdf_thumbnail_strip(
        PdfThumbnailRequest {
            attachment_id: AttachmentId::from("pdf_1"),
            attachment_hash: "hash".to_string(),
            current_page: 3,
            page_count: 5,
            radius: 1,
            max_dimension_px: 96,
            theme: Some(PdfReaderTheme::Dark),
        },
        Some(&document),
    );

    assert_eq!(strip.pages, vec![2, 3, 4]);
    assert_eq!(strip.thumbnails.len(), 3);
    assert!(strip.thumbnails.iter().any(|thumbnail| {
        thumbnail.page == 3
            && thumbnail.selected
            && thumbnail.cache_key.kind == PdfCacheKind::Thumbnail
    }));
    assert!(strip
        .thumbnails
        .iter()
        .all(|thumbnail| !thumbnail.pixels.is_empty()));
}

#[test]
fn pdf_render_job_descriptor_uses_research_job_contract() {
    let request = PdfRenderRequest {
        attachment_id: AttachmentId::from("pdf_1"),
        attachment_hash: "hash".to_string(),
        page: 3,
        zoom: 1.25,
        max_dimension_px: 1024,
        theme: Some(PdfReaderTheme::Dark),
    };
    let job = pdf_render_job_descriptor(&request, "batch_1");

    assert_eq!(job.kind, crate::ResearchJobKind::RenderPdfPage.as_str());
    assert_eq!(job.state, JobState::Queued);
    assert_eq!(job.payload["attachment_id"], "pdf_1");
    assert_eq!(job.payload["page"], 3);
}

#[test]
fn search_handles_unicode_and_collapsed_whitespace() {
    let document = PdfDocumentText {
        attachment_id: AttachmentId::from("pdf_1"),
        pages: vec![
            PdfPageText {
                page: 1,
                text: "심장\n구조를 입체적으로 설명한다.".to_string(),
                locale: crate::ResearchLocale::parse("ko-KR"),
            },
            PdfPageText {
                page: 2,
                text: "القلب   구조 비교".to_string(),
                locale: crate::ResearchLocale::parse("ar"),
            },
        ],
    };

    let results = search_pdf_text_with_limit(&document, "심장 구조", 10);
    assert_eq!(results.results.len(), 1);
    assert!(results.results[0].snippet.contains("심장"));

    let results = search_pdf_text_with_limit(&document, "القلب 구조", 10);
    assert_eq!(results.results.len(), 1);
    assert_eq!(results.results[0].page, 2);
}

#[test]
fn pdf_search_normalizes_accents_combining_marks_and_rtl_vowels() {
    let document = PdfDocumentText {
        attachment_id: AttachmentId::from("pdf_1"),
        pages: vec![PdfPageText {
            page: 1,
            text: "Café cafe\u{301} قَلْب".to_string(),
            locale: crate::ResearchLocale::parse("fr-FR"),
        }],
    };

    let latin = search_pdf_text_with_limit(&document, "cafe", 10);
    let rtl = search_pdf_text_with_limit(&document, "قلب", 10);

    assert!(!latin.results.is_empty());
    assert_eq!(rtl.results.len(), 1);
    assert_eq!(rtl.results[0].page, 1);
}

#[test]
fn pdf_search_snippet_does_not_split_grapheme_clusters() {
    let text = format!("{}{}", "a".repeat(39), "👩‍🔬".repeat(60));
    let document = PdfDocumentText {
        attachment_id: AttachmentId::from("pdf_1"),
        pages: vec![PdfPageText {
            page: 1,
            text,
            locale: crate::ResearchLocale::parse("en-US"),
        }],
    };

    let results = search_pdf_text_with_limit(&document, "👩‍🔬", 1);
    let snippet = &results.results[0].snippet;

    assert!(!snippet.ends_with('\u{200d}'));
    assert!(snippet
        .graphemes(true)
        .all(|grapheme| grapheme == "a" || grapheme == "👩‍🔬"));
}

#[test]
fn search_respects_limit() {
    let document = PdfDocumentText {
        attachment_id: AttachmentId::from("pdf_1"),
        pages: vec![PdfPageText {
            page: 1,
            text: "model model model".to_string(),
            locale: None,
        }],
    };

    let results = search_pdf_text_with_limit(&document, "model", 2);

    assert_eq!(results.results.len(), 2);
}

#[test]
fn clear_selection_and_copy_selection_text() {
    let state = PdfReaderState {
        attachment_id: AttachmentId::from("pdf_1"),
        current_page: 1,
        zoom: 1.0,
        pan_x: 0.0,
        pan_y: 0.0,
        mode: PdfReaderMode::SinglePage,
        theme: PdfReaderTheme::Paper,
        search: None,
        selected_text: Some(PdfTextSelection {
            page: 1,
            rects: vec![PageRect {
                x: 1.0,
                y: 1.0,
                width: 4.0,
                height: 1.0,
            }],
            text: "selected claim".to_string(),
            locale: None,
        }),
    };

    assert_eq!(
        copy_pdf_selection_text(&state),
        Some("selected claim".to_string())
    );

    let state = apply_pdf_reader_action(state, PdfReaderAction::ClearSelection, 1, None);
    assert_eq!(copy_pdf_selection_text(&state), None);
}

#[test]
fn overlay_plan_builds_annotation_commands() {
    let attachment_id = AttachmentId::from("pdf_1");
    let annotations = vec![PdfAnnotation {
        id: AnnotationId::from("ann_1"),
        attachment_id: attachment_id.clone(),
        reference_id: crate::ReferenceId::from("ref_1"),
        kind: PdfAnnotationKind::Highlight,
        page: 4,
        rects: vec![PageRect {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0,
        }],
        color: ColorRgba {
            r: 255,
            g: 220,
            b: 0,
            a: 255,
        },
        selected_text: Some("claim".to_string()),
        note_markdown: None,
        created_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
        updated_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
    }];

    let plan = build_pdf_annotation_overlay_plan(
        attachment_id,
        4,
        &annotations,
        Some(&AnnotationId::from("ann_1")),
    );

    assert_eq!(plan.commands.len(), 1);
    assert!(matches!(
        &plan.commands[0],
        PdfOverlayCommand::Highlight { selected: true, .. }
    ));
}

#[test]
fn reader_overlay_includes_search_and_selection_for_current_page() {
    let attachment_id = AttachmentId::from("pdf_1");
    let state = PdfReaderState {
        attachment_id,
        current_page: 2,
        zoom: 1.0,
        pan_x: 0.0,
        pan_y: 0.0,
        mode: PdfReaderMode::SinglePage,
        theme: PdfReaderTheme::Paper,
        search: Some(PdfSearchState {
            query: "claim".to_string(),
            active_result: Some(PdfSearchResultId::from("hit_2")),
            results: vec![
                PdfSearchResult {
                    id: PdfSearchResultId::from("hit_1"),
                    page: 1,
                    rects: vec![PageRect {
                        x: 0.0,
                        y: 0.0,
                        width: 5.0,
                        height: 1.0,
                    }],
                    snippet: "page one claim".to_string(),
                },
                PdfSearchResult {
                    id: PdfSearchResultId::from("hit_2"),
                    page: 2,
                    rects: vec![PageRect {
                        x: 2.0,
                        y: 3.0,
                        width: 5.0,
                        height: 1.0,
                    }],
                    snippet: "page two claim".to_string(),
                },
            ],
        }),
        selected_text: Some(PdfTextSelection {
            page: 2,
            rects: vec![PageRect {
                x: 4.0,
                y: 5.0,
                width: 6.0,
                height: 1.0,
            }],
            text: "selected claim".to_string(),
            locale: None,
        }),
    };

    let plan = build_pdf_reader_overlay_plan(&state, &[], None);

    assert_eq!(plan.commands.len(), 2);
    assert!(plan.commands.iter().any(|command| matches!(
        command,
        PdfOverlayCommand::SearchResult { active: true, .. }
    )));
    assert!(plan
        .commands
        .iter()
        .any(|command| matches!(command, PdfOverlayCommand::TextSelection { .. })));
}
