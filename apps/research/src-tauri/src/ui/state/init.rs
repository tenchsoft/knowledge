use std::collections::{BTreeSet, HashMap};

use super::builders::{
    default_status_filters, file_name_from_path, manuscript_from_library_snapshot,
    manuscript_from_snapshot, reference_author_label, visual_state_from_references,
};
use super::*;
use tench_research_core::{ReferenceKind, ResearchSnapshot, ResearchSnapshotV2};

impl ResearchState {
    pub fn empty() -> Self {
        Self::from_snapshot(ResearchSnapshot {
            papers: Vec::new(),
            collections: Vec::new(),
            tags: Vec::new(),
            notes: Vec::new(),
            analysis_threads: Vec::new(),
        })
    }

    pub fn example() -> Self {
        Self::from_snapshot(tench_research_core::example_snapshot())
    }

    pub fn from_snapshot(snapshot: ResearchSnapshot) -> Self {
        let manuscript = manuscript_from_snapshot(&snapshot);
        let notes = snapshot.notes;
        let core_papers = snapshot.papers;
        let visual_references = core_papers
            .iter()
            .map(|paper| {
                tench_research_core::reference_from_minimal_metadata(
                    paper.id.clone(),
                    ReferenceKind::JournalArticle,
                    paper.title.clone(),
                    Some(paper.year),
                    paper.updated_at.clone(),
                )
            })
            .collect::<Vec<_>>();
        let (
            visual_summary_lines,
            visual_draw_plan,
            manuscript_summary_lines,
            writing_visual_draw_plan,
        ) = visual_state_from_references(&visual_references, &manuscript);
        let papers = core_papers
            .into_iter()
            .map(|paper| {
                let notes_for_paper = notes
                    .iter()
                    .filter(|note| note.paper_id == paper.id)
                    .map(|note| note.content_markdown.as_str())
                    .collect::<Vec<_>>()
                    .join("\n\n");
                Paper {
                    title: paper.title,
                    authors: paper.authors.join(", "),
                    venue: paper.venue,
                    file_name: paper.file_name,
                    pages: u32::from(paper.pages),
                    abstract_text: paper.abstract_text,
                    year: u32::from(paper.year),
                    tags: paper.tags,
                    status: paper.status,
                    favorite: paper.favorite,
                    notes: notes_for_paper,
                    references: Vec::new(),
                    collection_ids: Vec::new(),
                }
            })
            .collect::<Vec<_>>();
        let mut collections = vec![Collection {
            id: "all-papers".into(),
            name: "All Papers".into(),
            count: papers.len(),
            expanded: true,
            parent_id: None,
        }];
        collections.extend(
            snapshot
                .collections
                .into_iter()
                .enumerate()
                .map(|(i, collection)| Collection {
                    id: format!("collection-{i}"),
                    name: collection.name,
                    count: collection.paper_count,
                    expanded: true,
                    parent_id: None,
                }),
        );
        let tags = snapshot
            .tags
            .into_iter()
            .map(|tag| tag.label)
            .collect::<Vec<_>>();

        Self {
            papers,
            collections,
            tags,
            statuses: default_status_filters(),
            selected_paper: 0,
            search_query: String::new(),
            active_inspector_tab: 0,
            reader_mode: ReaderMode::Detail,
            citation_format: CitationFormat::BibTex,
            favorites_only: false,
            import_status: ImportStatus::Ready,
            analysis_messages: vec![
                (
                    "system".into(),
                    "Paper context loaded; prompts stay local.".into(),
                ),
                (
                    "assistant".into(),
                    "The selected paper focuses on attention-only sequence modeling.".into(),
                ),
            ],
            visual_summary_lines,
            visual_draw_plan,
            manuscript_summary_lines,
            writing_visual_draw_plan,
            selected_papers: BTreeSet::new(),
            context_menu: None,
            command_palette: ResearchCommandPaletteState::default(),
            dropped_import_paths: Vec::new(),
            progress_history: Vec::new(),
            duplicate_merge: None,
            missing_attachment_repair: ResearchAttachmentRepairUi::default(),
            backup_restore: ResearchBackupRestoreUi::default(),
            i18n_missing_keys: Vec::new(),
            focus: FocusTarget::None,
            selected_collection: None,
            sort_mode: SortMode::TitleAsc,
            qa_input: String::new(),
            advanced_search: None,
            show_advanced_search: false,
            saved_searches: Vec::new(),
            toasts: Vec::new(),
            show_shortcut_help: false,
            pdf_current_page: 1,
            pdf_page_count: 0,
            pdf_zoom: 1.0,
            pdf_rotation: 0.0,
            pdf_continuous_scroll: false,
            pdf_search_query: String::new(),
            pdf_search_results: Vec::new(),
            pdf_search_active_index: None,
            pdf_page_image_data: None,
            pdf_annotations: Vec::new(),
            pdf_selected_annotation: None,
            pdf_annotation_tool: PdfAnnotationTool::None,
            pdf_show_annotation_list: false,
            doi_input: String::new(),
            citation_export_format: CitationExportFormat::BibTex,
            manuscript_sections: Vec::new(),
            manuscript_cite_search: String::new(),
            manuscript_active_section: None,
            manuscript_show_cite_search: false,
            smart_collections: Vec::new(),
            show_welcome: false,
        }
    }

    pub fn from_library_snapshot(snapshot: ResearchSnapshotV2) -> Self {
        let manuscript = manuscript_from_library_snapshot(&snapshot);
        let (
            visual_summary_lines,
            visual_draw_plan,
            manuscript_summary_lines,
            writing_visual_draw_plan,
        ) = visual_state_from_references(&snapshot.references, &manuscript);
        let tag_labels = snapshot
            .tags
            .iter()
            .map(|tag| (tag.id.as_str().to_string(), tag.label.clone()))
            .collect::<HashMap<_, _>>();
        let papers = snapshot
            .references
            .iter()
            .map(|reference| {
                let attachment = snapshot
                    .attachments
                    .iter()
                    .find(|attachment| attachment.reference_id == reference.id);
                let notes = snapshot
                    .notes
                    .iter()
                    .filter(|note| note.reference_id.as_ref() == Some(&reference.id))
                    .map(|note| note.body_markdown.as_str())
                    .collect::<Vec<_>>()
                    .join("\n\n");
                Paper {
                    title: reference.title.value.clone(),
                    authors: reference_author_label(reference),
                    venue: reference
                        .venue
                        .as_ref()
                        .map(|venue| venue.name.value.clone())
                        .unwrap_or_default(),
                    file_name: attachment
                        .map(|attachment| file_name_from_path(&attachment.stored_path))
                        .unwrap_or_default(),
                    pages: attachment
                        .and_then(|attachment| attachment.page_count)
                        .unwrap_or_default(),
                    abstract_text: reference
                        .abstract_text
                        .as_ref()
                        .map(|field| field.value.clone())
                        .unwrap_or_default(),
                    year: reference.issued.year.map(u32::from).unwrap_or_default(),
                    tags: reference
                        .tags
                        .iter()
                        .map(|tag| {
                            tag_labels
                                .get(tag.as_str())
                                .cloned()
                                .unwrap_or_else(|| tag.as_str().to_string())
                        })
                        .collect(),
                    status: reference.status.clone(),
                    favorite: reference.favorite,
                    notes,
                    references: reference.urls.iter().map(|url| url.url.clone()).collect(),
                    collection_ids: reference
                        .collections
                        .iter()
                        .map(|id| id.as_str().to_string())
                        .collect(),
                }
            })
            .collect::<Vec<_>>();
        let mut collections = vec![Collection {
            id: "all-papers".into(),
            name: "All Papers".into(),
            count: papers.len(),
            expanded: true,
            parent_id: None,
        }];
        collections.extend(snapshot.collections.iter().map(|collection| {
            let count = snapshot
                .references
                .iter()
                .filter(|reference| reference.collections.contains(&collection.id))
                .count();
            Collection {
                id: collection.id.as_str().to_string(),
                name: collection.name.clone(),
                count,
                expanded: true,
                parent_id: collection
                    .parent_id
                    .as_ref()
                    .map(|id| id.as_str().to_string()),
            }
        }));
        let tags = snapshot
            .tags
            .into_iter()
            .map(|tag| tag.label)
            .collect::<Vec<_>>();

        Self {
            papers,
            collections,
            tags,
            statuses: default_status_filters(),
            selected_paper: 0,
            search_query: String::new(),
            active_inspector_tab: 0,
            reader_mode: ReaderMode::Detail,
            citation_format: CitationFormat::BibTex,
            favorites_only: false,
            import_status: ImportStatus::Ready,
            analysis_messages: Vec::new(),
            visual_summary_lines,
            visual_draw_plan,
            manuscript_summary_lines,
            writing_visual_draw_plan,
            selected_papers: BTreeSet::new(),
            context_menu: None,
            command_palette: ResearchCommandPaletteState::default(),
            dropped_import_paths: Vec::new(),
            progress_history: Vec::new(),
            duplicate_merge: None,
            missing_attachment_repair: ResearchAttachmentRepairUi::default(),
            backup_restore: ResearchBackupRestoreUi::default(),
            i18n_missing_keys: Vec::new(),
            focus: FocusTarget::None,
            selected_collection: None,
            sort_mode: SortMode::TitleAsc,
            qa_input: String::new(),
            advanced_search: None,
            show_advanced_search: false,
            saved_searches: Vec::new(),
            toasts: Vec::new(),
            show_shortcut_help: false,
            pdf_current_page: 1,
            pdf_page_count: 0,
            pdf_zoom: 1.0,
            pdf_rotation: 0.0,
            pdf_continuous_scroll: false,
            pdf_search_query: String::new(),
            pdf_search_results: Vec::new(),
            pdf_search_active_index: None,
            pdf_page_image_data: None,
            pdf_annotations: Vec::new(),
            pdf_selected_annotation: None,
            pdf_annotation_tool: PdfAnnotationTool::None,
            pdf_show_annotation_list: false,
            doi_input: String::new(),
            citation_export_format: CitationExportFormat::BibTex,
            manuscript_sections: Vec::new(),
            manuscript_cite_search: String::new(),
            manuscript_active_section: None,
            manuscript_show_cite_search: false,
            smart_collections: Vec::new(),
            show_welcome: false,
        }
    }
}
