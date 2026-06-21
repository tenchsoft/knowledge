use super::*;
use crate::ResearchLocale;

pub fn manuscript_target_for_template(
    kind: ManuscriptTemplateKind,
    locale: ResearchLocale,
) -> ManuscriptTarget {
    let (target_kind, name, sections, exports) = match kind {
        ManuscriptTemplateKind::JournalArticle => (
            TargetKind::Journal,
            "Journal article",
            vec![
                (SectionKind::Abstract, Some(250)),
                (SectionKind::Introduction, None),
                (SectionKind::Methods, None),
                (SectionKind::Results, None),
                (SectionKind::Discussion, None),
                (SectionKind::References, None),
            ],
            vec![
                WritingExportFormat::Docx,
                WritingExportFormat::Pdf,
                WritingExportFormat::Markdown,
                WritingExportFormat::Html,
                WritingExportFormat::Latex,
            ],
        ),
        ManuscriptTemplateKind::ConferencePaper => (
            TargetKind::Conference,
            "Conference paper",
            vec![
                (SectionKind::Abstract, Some(200)),
                (SectionKind::Introduction, None),
                (SectionKind::RelatedWork, None),
                (SectionKind::Methods, None),
                (SectionKind::Results, None),
                (SectionKind::Conclusion, None),
                (SectionKind::References, None),
            ],
            vec![
                WritingExportFormat::Pdf,
                WritingExportFormat::Markdown,
                WritingExportFormat::Latex,
            ],
        ),
        ManuscriptTemplateKind::LiteratureReview => (
            TargetKind::Report,
            "Literature review",
            vec![
                (SectionKind::Abstract, Some(250)),
                (SectionKind::Introduction, None),
                (SectionKind::Background, None),
                (SectionKind::RelatedWork, None),
                (SectionKind::Discussion, None),
                (SectionKind::Conclusion, None),
                (SectionKind::References, None),
            ],
            vec![
                WritingExportFormat::Docx,
                WritingExportFormat::Pdf,
                WritingExportFormat::Markdown,
                WritingExportFormat::Html,
            ],
        ),
        ManuscriptTemplateKind::ThesisChapter => (
            TargetKind::Thesis,
            "Thesis chapter",
            vec![
                (SectionKind::Introduction, None),
                (SectionKind::Background, None),
                (SectionKind::Methods, None),
                (SectionKind::Results, None),
                (SectionKind::Discussion, None),
                (SectionKind::Conclusion, None),
                (SectionKind::References, None),
                (SectionKind::Appendix, None),
            ],
            vec![
                WritingExportFormat::Docx,
                WritingExportFormat::Pdf,
                WritingExportFormat::Markdown,
                WritingExportFormat::Latex,
                WritingExportFormat::Archive,
            ],
        ),
    };

    ManuscriptTarget {
        kind: target_kind,
        name: name.to_string(),
        citation_style: Some("apa".to_string()),
        bibliography_locale: locale,
        word_limit: None,
        abstract_limit: sections
            .iter()
            .find(|(section_kind, _)| *section_kind == SectionKind::Abstract)
            .and_then(|(_, limit)| *limit),
        section_rules: sections
            .into_iter()
            .map(|(section_kind, word_limit)| SectionRule {
                kind: section_kind,
                required: true,
                word_limit,
            })
            .collect(),
        figure_table_rules: vec![
            AssetRule {
                kind: AssetKind::Figure,
                alt_text_required: true,
                max_size_bytes: None,
            },
            AssetRule {
                kind: AssetKind::Table,
                alt_text_required: true,
                max_size_bytes: None,
            },
        ],
        export_formats: exports,
    }
}

pub(super) fn section_label(kind: SectionKind) -> &'static str {
    match kind {
        SectionKind::Title => "Title",
        SectionKind::Abstract => "Abstract",
        SectionKind::Introduction => "Introduction",
        SectionKind::Background => "Background",
        SectionKind::RelatedWork => "Related Work",
        SectionKind::Methods => "Methods",
        SectionKind::Results => "Results",
        SectionKind::Discussion => "Discussion",
        SectionKind::Conclusion => "Conclusion",
        SectionKind::References => "References",
        SectionKind::Appendix => "Appendix",
        SectionKind::Custom => "Custom",
    }
}

pub(super) fn section_slug(kind: SectionKind) -> &'static str {
    match kind {
        SectionKind::Title => "title",
        SectionKind::Abstract => "abstract",
        SectionKind::Introduction => "introduction",
        SectionKind::Background => "background",
        SectionKind::RelatedWork => "related-work",
        SectionKind::Methods => "methods",
        SectionKind::Results => "results",
        SectionKind::Discussion => "discussion",
        SectionKind::Conclusion => "conclusion",
        SectionKind::References => "references",
        SectionKind::Appendix => "appendix",
        SectionKind::Custom => "custom",
    }
}
