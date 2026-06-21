use tench_research_core::{
    load_library_snapshot, new_research_library_snapshot, save_library_snapshot, Creator,
    CreatorNameOrder, CreatorRole, DateParts, LibraryId, LibraryLayout, LocalizedField,
    PublicationVenue, ReadingStatus, ReferenceId, ReferenceIdentifiers, ReferenceItem,
    ReferenceKind, ReferenceMetadata, ResearchLocale, Timestamp,
};

fn make_layout(dir: &std::path::Path) -> LibraryLayout {
    LibraryLayout {
        root: dir.to_path_buf(),
        library_json: dir.join("library.json"),
        items_jsonl: dir.join("items.jsonl"),
        attachments_jsonl: dir.join("attachments.jsonl"),
        annotations_jsonl: dir.join("annotations.jsonl"),
        notes_jsonl: dir.join("notes.jsonl"),
        collections_jsonl: dir.join("collections.jsonl"),
        tags_jsonl: dir.join("tags.jsonl"),
        citekeys_json: dir.join("citekeys.json"),
        attachments_dir: dir.join("attachments"),
        index_dir: dir.join("index"),
        thumbnails_dir: dir.join("thumbnails"),
        recovery_dir: dir.join("recovery"),
        backups_dir: dir.join("backups"),
    }
}

fn timestamp() -> Timestamp {
    Timestamp("2026-01-01T00:00:00Z".into())
}

fn make_reference(id: &str, title: &str, favorite: bool, kind: ReferenceKind) -> ReferenceItem {
    ReferenceItem {
        id: ReferenceId::new(id),
        kind,
        title: LocalizedField::plain(title),
        subtitle: None,
        creators: vec![Creator {
            role: CreatorRole::Author,
            given: Some("Test".into()),
            family: Some("Author".into()),
            literal: None,
            transliteration: None,
            sort_key: None,
            name_order: CreatorNameOrder::LocaleDefault,
            locale: None,
            orcid: None,
            affiliation: None,
        }],
        issued: DateParts {
            year: Some(2024),
            month: None,
            day: None,
            raw: None,
        },
        abstract_text: None,
        language: None,
        venue: Some(PublicationVenue {
            name: LocalizedField::plain("Journal"),
            volume: None,
            issue: None,
            pages: None,
            publisher: None,
        }),
        identifiers: ReferenceIdentifiers::default(),
        urls: vec![],
        collections: vec![],
        tags: vec![],
        status: ReadingStatus::Unread,
        favorite,
        rating: None,
        citekey: None,
        citekey_locked: false,
        metadata: ReferenceMetadata::default(),
        created_at: timestamp(),
        updated_at: timestamp(),
    }
}

#[test]
fn format_roundtrip_preserves_reference_metadata() {
    let dir = std::env::temp_dir().join(format!("research-roundtrip-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("dir");
    let layout = make_layout(&dir);

    let mut snapshot = new_research_library_snapshot(
        LibraryId::new("test-lib"),
        "Test Library",
        dir.to_string_lossy(),
        ResearchLocale::parse("en-US").expect("locale"),
        timestamp(),
    );

    snapshot.references.push(make_reference(
        "ref-1",
        "Attention Is All You Need",
        true,
        ReferenceKind::JournalArticle,
    ));

    save_library_snapshot(&layout, &snapshot).expect("save");
    let loaded = load_library_snapshot(&layout).expect("load");

    assert_eq!(loaded.references.len(), 1);
    let ref_loaded = &loaded.references[0];
    assert_eq!(ref_loaded.title.value, "Attention Is All You Need");
    assert!(ref_loaded.favorite);
    assert_eq!(ref_loaded.kind, ReferenceKind::JournalArticle);
    assert_eq!(ref_loaded.creators.len(), 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn format_roundtrip_preserves_bibtex_import() {
    let dir = std::env::temp_dir().join(format!("research-bibtex-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("dir");
    let layout = make_layout(&dir);

    let mut snapshot = new_research_library_snapshot(
        LibraryId::new("bibtex-test"),
        "BibTeX Test Library",
        dir.to_string_lossy(),
        ResearchLocale::parse("en-US").expect("locale"),
        timestamp(),
    );

    snapshot.references.push(make_reference(
        "bibtex-1",
        "BERT: Pre-training of Deep Bidirectional Transformers",
        false,
        ReferenceKind::ConferencePaper,
    ));

    save_library_snapshot(&layout, &snapshot).expect("save");
    let loaded = load_library_snapshot(&layout).expect("load");

    assert_eq!(loaded.references.len(), 1);
    assert_eq!(
        loaded.references[0].title.value,
        "BERT: Pre-training of Deep Bidirectional Transformers"
    );
    assert_eq!(loaded.references[0].kind, ReferenceKind::ConferencePaper);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn format_roundtrip_handles_corrupted_file() {
    let dir = std::env::temp_dir().join(format!("research-corrupt-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("dir");
    let layout = make_layout(&dir);

    std::fs::write(&layout.library_json, b"not valid json{{{").expect("write corrupted");

    let result = load_library_snapshot(&layout);
    assert!(
        result.is_err(),
        "loading corrupted file should return an error"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn format_roundtrip_large_library() {
    let dir = std::env::temp_dir().join(format!("research-large-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("dir");
    let layout = make_layout(&dir);

    let mut snapshot = new_research_library_snapshot(
        LibraryId::new("large-test"),
        "Large Library",
        dir.to_string_lossy(),
        ResearchLocale::parse("en-US").expect("locale"),
        timestamp(),
    );

    for i in 0..150 {
        snapshot.references.push(make_reference(
            &format!("ref-{i}"),
            &format!("Paper Title {i}"),
            i % 5 == 0,
            ReferenceKind::JournalArticle,
        ));
    }

    save_library_snapshot(&layout, &snapshot).expect("save large library");
    let loaded = load_library_snapshot(&layout).expect("load large library");

    assert_eq!(loaded.references.len(), 150);
    assert_eq!(loaded.references[0].title.value, "Paper Title 0");
    assert_eq!(loaded.references[149].title.value, "Paper Title 149");
    assert!(loaded.references[0].favorite);
    assert!(!loaded.references[1].favorite);

    let _ = std::fs::remove_dir_all(&dir);
}
