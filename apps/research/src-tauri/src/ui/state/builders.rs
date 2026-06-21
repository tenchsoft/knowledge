use super::types::{ResearchContextTarget, ResearchStatusFilter, ResearchUiCommand};
use tench_research_core::{
    AssetRule, CreatorRole, LocalizedField, ManuscriptTarget, ReadingStatus, ReferenceItem,
    ResearchLocale, ResearchManuscript, ResearchSnapshot, ResearchSnapshotV2, SectionKind,
    SectionRule, TargetKind, Timestamp, VisualSpecId, WritingExportFormat,
};

pub(super) fn all_research_ui_commands() -> Vec<ResearchUiCommand> {
    vec![
        ResearchUiCommand::OpenSelected,
        ResearchUiCommand::ToggleFavorite,
        ResearchUiCommand::CopyCitation,
        ResearchUiCommand::ExportSelected,
        ResearchUiCommand::ImportDroppedFiles,
        ResearchUiCommand::BatchSetStatusReviewed,
        ResearchUiCommand::BatchAddTag,
        ResearchUiCommand::MergeDuplicates,
        ResearchUiCommand::RepairMissingAttachment,
        ResearchUiCommand::CreateBackup,
        ResearchUiCommand::RestoreBackup,
        ResearchUiCommand::ShowI18nCoverage,
    ]
}

pub(super) fn context_commands_for_target(
    target: &ResearchContextTarget,
) -> Vec<ResearchUiCommand> {
    match target {
        ResearchContextTarget::Paper { .. } => vec![
            ResearchUiCommand::OpenSelected,
            ResearchUiCommand::ToggleFavorite,
            ResearchUiCommand::CopyCitation,
            ResearchUiCommand::ExportSelected,
            ResearchUiCommand::BatchSetStatusReviewed,
            ResearchUiCommand::BatchAddTag,
            ResearchUiCommand::MergeDuplicates,
        ],
        ResearchContextTarget::PaperList => vec![
            ResearchUiCommand::ImportDroppedFiles,
            ResearchUiCommand::CreateBackup,
            ResearchUiCommand::RestoreBackup,
            ResearchUiCommand::ShowI18nCoverage,
        ],
        ResearchContextTarget::Collection { .. } | ResearchContextTarget::Tag { .. } => vec![
            ResearchUiCommand::BatchSetStatusReviewed,
            ResearchUiCommand::BatchAddTag,
            ResearchUiCommand::ExportSelected,
        ],
        ResearchContextTarget::Attachment { .. } => vec![
            ResearchUiCommand::RepairMissingAttachment,
            ResearchUiCommand::ExportSelected,
        ],
    }
}

pub(super) fn default_status_filters() -> Vec<ResearchStatusFilter> {
    vec![
        ResearchStatusFilter::All,
        ResearchStatusFilter::Status(ReadingStatus::Reviewed),
        ResearchStatusFilter::Status(ReadingStatus::Reading),
        ResearchStatusFilter::Status(ReadingStatus::Unread),
        ResearchStatusFilter::Status(ReadingStatus::Archived),
    ]
}

pub(super) fn reference_author_label(reference: &ReferenceItem) -> String {
    let authors = reference
        .creators
        .iter()
        .filter(|creator| creator.role == CreatorRole::Author)
        .map(|creator| creator.sort_name())
        .filter(|name| !name.trim().is_empty())
        .collect::<Vec<_>>();
    if authors.is_empty() {
        "Unknown author".to_string()
    } else {
        authors.join(", ")
    }
}

pub(super) fn file_name_from_path(path: &str) -> String {
    path.rsplit(['/', '\\'])
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(path)
        .to_string()
}

pub(super) fn visual_state_from_references(
    references: &[ReferenceItem],
    manuscript: &ResearchManuscript,
) -> (
    Vec<String>,
    Option<tench_research_core::ResearchVisualDrawPlan>,
    Vec<String>,
    Option<tench_research_core::ResearchVisualDrawPlan>,
) {
    let timeline = tench_research_core::build_reference_timeline_visual(
        VisualSpecId::from("ui-timeline"),
        "ui-library",
        references,
    );
    let influence = tench_research_core::build_reference_influence_graph_visual(
        VisualSpecId::from("ui-influence"),
        "ui-library",
        references,
    );
    let manuscript_checks = tench_research_core::run_non_ai_writing_checks(manuscript);
    let visual_draw_plan =
        tench_research_core::build_research_visual_draw_plan(&timeline, false).ok();
    let writing_visual = tench_research_core::build_manuscript_readiness_dashboard_visual(
        VisualSpecId::from("ui-manuscript-readiness"),
        manuscript,
    );
    let writing_visual_draw_plan =
        tench_research_core::build_research_visual_draw_plan(&writing_visual, false).ok();

    (
        vec![
            format!(
                "{}: {} references",
                timeline.title.value,
                timeline.data_query.reference_ids.len()
            ),
            format!(
                "Timeline range: {}-{}",
                timeline
                    .state
                    .timeline_range
                    .map(|range| range.0.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                timeline
                    .state
                    .timeline_range
                    .map(|range| range.1.to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            ),
            format!(
                "{}: {} encodings",
                influence.title.value,
                influence.encodings.len()
            ),
        ],
        visual_draw_plan,
        vec![
            format!("Draft: {}", manuscript.title.value),
            format!(
                "Required sections: {}",
                manuscript.outline.required_sections.len()
            ),
            format!("Current checks: {}", manuscript_checks.len()),
            format!("Export formats: {}", manuscript.target.export_formats.len()),
        ],
        writing_visual_draw_plan,
    )
}

pub(super) fn manuscript_from_snapshot(snapshot: &ResearchSnapshot) -> ResearchManuscript {
    let locale = ResearchLocale::parse("en-US").expect("locale");
    tench_research_core::create_manuscript_skeleton(
        tench_research_core::ManuscriptId::from("ui-manuscript"),
        tench_research_core::LibraryId::from("ui-library"),
        LocalizedField::plain("Research draft"),
        default_manuscript_target(&locale),
        locale,
        Timestamp(
            snapshot
                .papers
                .first()
                .map(|paper| paper.updated_at.clone())
                .unwrap_or_else(|| "2026-05-04T00:00:00Z".to_string()),
        ),
    )
}

pub(super) fn default_manuscript_target(locale: &ResearchLocale) -> ManuscriptTarget {
    ManuscriptTarget {
        kind: TargetKind::Journal,
        name: "Journal article".to_string(),
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
                kind: SectionKind::Introduction,
                required: true,
                word_limit: None,
            },
            SectionRule {
                kind: SectionKind::Methods,
                required: true,
                word_limit: None,
            },
            SectionRule {
                kind: SectionKind::Results,
                required: true,
                word_limit: None,
            },
            SectionRule {
                kind: SectionKind::Discussion,
                required: true,
                word_limit: None,
            },
            SectionRule {
                kind: SectionKind::References,
                required: true,
                word_limit: None,
            },
        ],
        figure_table_rules: vec![AssetRule {
            kind: tench_research_core::AssetKind::Figure,
            alt_text_required: true,
            max_size_bytes: None,
        }],
        export_formats: vec![WritingExportFormat::Docx, WritingExportFormat::Markdown],
    }
}

pub(super) fn manuscript_from_library_snapshot(
    snapshot: &ResearchSnapshotV2,
) -> ResearchManuscript {
    let locale = snapshot.library.settings.default_locale.clone();
    tench_research_core::create_manuscript_skeleton(
        tench_research_core::ManuscriptId::from("ui-manuscript"),
        snapshot.library.id.clone(),
        LocalizedField::plain("Research draft"),
        default_manuscript_target(&locale),
        locale,
        snapshot.library.updated_at.clone(),
    )
}
