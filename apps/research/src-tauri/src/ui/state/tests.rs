use super::*;
use tench_research_core::{
    CreatorRole, LocalizedField, ReadingStatus, ReferenceKind, ResearchLocale, Timestamp,
};

#[test]
fn search_filters_papers_and_keeps_selection_visible() {
    let mut state = ResearchState::example();

    state.set_search_query("retrieval");

    assert_eq!(state.visible_paper_indices(), vec![1]);
    assert_eq!(
        state.selected_paper().map(|paper| paper.title.as_str()),
        Some("Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks")
    );
}

#[test]
fn favorites_filter_uses_visible_selection_navigation() {
    let mut state = ResearchState::example();
    state.toggle_favorites_only();

    assert_eq!(state.visible_paper_indices(), vec![0, 1]);
    assert!(state.move_selection(1));

    assert_eq!(state.selected_paper, 1);
}

#[test]
fn library_snapshot_v2_binds_reference_items_to_ui_rows() {
    let now = Timestamp("2026-05-04T00:00:00Z".to_string());
    let locale = ResearchLocale::parse("ko-KR").expect("locale");
    let mut snapshot = tench_research_core::new_research_library_snapshot(
        tench_research_core::LibraryId::from("library"),
        "Research Library",
        "/tmp/library",
        locale,
        now.clone(),
    );
    snapshot.tags.push(tench_research_core::ResearchTag {
        id: tench_research_core::ResearchTagId::from("tag-ml"),
        label: "machine learning".to_string(),
        color: "#346f62".to_string(),
    });
    snapshot
        .collections
        .push(tench_research_core::ResearchCollection {
            id: tench_research_core::ResearchCollectionId::from("collection-ai"),
            parent_id: None,
            name: "AI Papers".to_string(),
            description: None,
            sort_order: 0,
            rules: None,
        });
    let mut reference = tench_research_core::reference_from_minimal_metadata(
        "ref-1",
        ReferenceKind::JournalArticle,
        "다국어 검색 논문",
        Some(2026),
        now.0.clone(),
    );
    reference.creators.push(tench_research_core::Creator {
        role: CreatorRole::Author,
        given: Some("Min".to_string()),
        family: Some("Kim".to_string()),
        literal: None,
        transliteration: None,
        sort_key: None,
        name_order: tench_research_core::CreatorNameOrder::default(),
        locale: None,
        orcid: None,
        affiliation: None,
    });
    reference.abstract_text = Some(LocalizedField::plain("Unicode-safe research search."));
    reference.status = ReadingStatus::Reading;
    reference.favorite = true;
    reference
        .tags
        .push(tench_research_core::ResearchTagId::from("tag-ml"));
    reference
        .collections
        .push(tench_research_core::ResearchCollectionId::from(
            "collection-ai",
        ));
    snapshot.references.push(reference);
    snapshot.attachments.push(tench_research_core::Attachment {
        id: tench_research_core::AttachmentId::from("attachment-1"),
        reference_id: tench_research_core::ReferenceId::from("ref-1"),
        kind: tench_research_core::AttachmentKind::Pdf,
        title: "paper.pdf".to_string(),
        stored_path: "attachments/paper.pdf".to_string(),
        original_path: None,
        mime_type: "application/pdf".to_string(),
        size_bytes: 1024,
        content_hash: "hash".to_string(),
        page_count: Some(12),
        text_indexed: true,
        created_at: now.clone(),
        updated_at: now.clone(),
    });
    snapshot.notes.push(tench_research_core::ResearchNote {
        id: tench_research_core::ResearchNoteId::from("note-1"),
        reference_id: Some(tench_research_core::ReferenceId::from("ref-1")),
        annotation_id: None,
        title: "Note".to_string(),
        body_markdown: "Preserve original language metadata.".to_string(),
        tags: Vec::new(),
        backlinks: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    });

    let state = ResearchState::from_library_snapshot(snapshot);

    assert_eq!(state.papers.len(), 1);
    assert_eq!(state.papers[0].title, "다국어 검색 논문");
    assert_eq!(state.papers[0].authors, "Min Kim");
    assert_eq!(state.papers[0].status, ReadingStatus::Reading);
    assert_eq!(state.papers[0].file_name, "paper.pdf");
    assert_eq!(state.papers[0].pages, 12);
    assert_eq!(state.papers[0].tags, vec!["machine learning"]);
    assert!(state.papers[0].notes.contains("original language"));
    assert_eq!(state.collections[1].count, 1);
    assert!(state.visual_draw_plan.is_some());
}

#[test]
fn notes_and_favorite_actions_update_selected_paper() {
    let mut state = ResearchState::example();
    state.selected_paper = 2;

    assert!(state.toggle_selected_favorite());
    assert!(state.append_selected_note("Check dataset assumptions."));

    let paper = state.selected_paper().expect("selected paper");
    assert!(paper.favorite);
    assert!(paper.notes.contains("Check dataset assumptions."));
}

#[test]
fn example_state_builds_visual_summary_from_core_snapshot() {
    let state = ResearchState::example();

    assert!(state
        .visual_summary_lines
        .iter()
        .any(|line| line.contains("Reference timeline")));
    assert!(state
        .visual_summary_lines
        .iter()
        .any(|line| line.contains("influence graph")));
}

#[test]
fn example_state_builds_manuscript_summary_from_core_skeleton() {
    let state = ResearchState::example();

    assert!(state
        .manuscript_summary_lines
        .iter()
        .any(|line| line.contains("Required sections")));
    assert!(state
        .manuscript_summary_lines
        .iter()
        .any(|line| line.contains("Export formats")));
    assert!(state.writing_visual_draw_plan.is_some());
}

#[test]
fn reader_mode_toggles_between_detail_and_pdf() {
    let mut state = ResearchState::example();

    state.toggle_reader_mode();
    assert_eq!(state.reader_mode, ReaderMode::Pdf);

    state.toggle_reader_mode();
    assert_eq!(state.reader_mode, ReaderMode::Detail);
}

#[test]
fn keyboard_state_actions_cover_search_import_and_citation_modes() {
    let mut state = ResearchState::example();

    state.push_search_text("retrieval");
    assert_eq!(state.search_query, "retrieval");
    assert!(!state.visible_paper_indices().is_empty());
    state.pop_search_text();
    assert_eq!(state.search_query, "retrieva");

    state.cycle_citation_format();
    assert_eq!(state.citation_format, CitationFormat::Apa);
    state.cycle_citation_format();
    assert_eq!(state.citation_format, CitationFormat::Mla);

    state.queue_import();
    assert_eq!(state.reader_mode, ReaderMode::Importing);
    assert_eq!(state.import_status, ImportStatus::Queued);
}

#[test]
fn phase11_drag_context_palette_multiselect_and_batch_edit_state() {
    let mut state = ResearchState::example();

    assert_eq!(
        state.handle_dropped_import_paths(vec![
            "/tmp/a.pdf".to_string(),
            "".to_string(),
            "/tmp/a.pdf".to_string(),
            "/tmp/b.ris".to_string(),
        ]),
        2
    );
    assert_eq!(state.reader_mode, ReaderMode::Importing);
    assert_eq!(state.dropped_import_paths.len(), 2);
    assert!(state
        .progress_history
        .iter()
        .any(|event| event.kind == ResearchProgressKind::Import));

    state.open_context_menu(ResearchContextTarget::Paper { index: 0 });
    let menu = state.context_menu.as_ref().expect("context menu");
    assert!(menu.commands.contains(&ResearchUiCommand::MergeDuplicates));
    assert!(menu.commands.contains(&ResearchUiCommand::ExportSelected));
    state.close_context_menu();
    assert!(state.context_menu.is_none());

    state.open_command_palette();
    state.update_command_palette_query("create_backup");
    assert_eq!(
        state.command_palette.commands,
        vec![ResearchUiCommand::CreateBackup]
    );
    state.close_command_palette();
    assert!(!state.command_palette.open);

    let selected = state.select_visible_paper_range(0, 1);
    assert_eq!(selected, 2);
    assert_eq!(state.batch_set_selected_status(ReadingStatus::Reviewed), 2);
    assert_eq!(state.batch_add_selected_tag("important"), 2);
    assert!(state.selected_papers.iter().all(|index| {
        state.papers[*index].status == ReadingStatus::Reviewed
            && state.papers[*index]
                .tags
                .iter()
                .any(|tag| tag == "important")
    }));
}

#[test]
fn phase11_merge_repair_backup_progress_and_i18n_state() {
    let mut state = ResearchState::example();

    state.record_duplicate_candidates(3);
    state.select_duplicate_merge_pair("ref_a", "ref_b");
    let merge = state.duplicate_merge.as_ref().expect("merge state");
    assert_eq!(merge.candidate_count, 3);
    assert_eq!(
        merge.selected_pair,
        Some(("ref_a".to_string(), "ref_b".to_string()))
    );

    state.record_missing_attachment_scan(5, 2);
    assert_eq!(state.missing_attachment_repair.missing_count, 5);
    assert_eq!(state.missing_attachment_repair.unresolved_count, 2);
    assert!(state
        .progress_history
        .iter()
        .any(|event| event.kind == ResearchProgressKind::Repair));

    state.record_library_backup("/tmp/research-backup.zip");
    state.record_library_restore("backup restored");
    state.record_export_progress(
        "selected references",
        ResearchProgressStatus::Running,
        2,
        10,
    );
    assert_eq!(
        state.backup_restore.last_backup_path.as_deref(),
        Some("/tmp/research-backup.zip")
    );
    assert_eq!(
        state.backup_restore.last_restore_label.as_deref(),
        Some("backup restored")
    );
    assert!(state
        .progress_history
        .iter()
        .any(|event| event.kind == ResearchProgressKind::Export
            && event.status == ResearchProgressStatus::Running));

    state.record_i18n_coverage(vec!["research.export".to_string()]);
    assert_eq!(state.i18n_missing_keys, vec!["research.export"]);
}
