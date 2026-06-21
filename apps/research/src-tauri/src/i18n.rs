pub const DEFAULT_LOCALE: &str = "en-US";

pub fn research_i18n_entries(locale: &str) -> Vec<tench_app_core::I18nEntry> {
    [
        ("research.app.title", "Tench Research", "window title"),
        (
            "research.search.prompt",
            "Search papers...",
            "library search input",
        ),
        ("research.action.import", "Import", "import button"),
        ("research.action.export", "Export", "export button"),
        ("research.action.sync", "Sync", "sync button"),
        ("research.status.on", "on", "enabled state label"),
        ("research.status.off", "off", "disabled state label"),
        ("research.status.all", "All", "all status filter label"),
        ("research.status.read", "Read", "read status label"),
        ("research.status.reading", "Reading", "reading status label"),
        ("research.status.to_read", "To Read", "unread status label"),
        ("research.status.archived", "Archived", "archived status label"),
        ("research.status.ready", "Ready", "ready status label"),
        (
            "research.status.import_queued",
            "Import queued",
            "import queued status label",
        ),
        (
            "research.status.favorites",
            "favorites",
            "favorites filter status label",
        ),
        (
            "research.collection.heading",
            "COLLECTIONS",
            "collection panel heading",
        ),
        ("research.tag.heading", "TAGS", "tag panel heading"),
        ("research.status.heading", "STATUS", "status filter heading"),
        ("research.paper.heading", "PAPERS", "paper list heading"),
        ("research.reader.detail", "detail", "detail reader mode"),
        ("research.reader.pdf", "pdf", "PDF reader mode"),
        (
            "research.reader.importing",
            "importing",
            "importing reader mode",
        ),
        ("research.unit.pages", "pages", "page count unit"),
        ("research.section.abstract", "Abstract", "abstract heading"),
        ("research.section.visual", "Visual analysis", "visual analysis heading"),
        ("research.section.references", "References", "references heading"),
        (
            "research.empty.title",
            "Open or create a research library",
            "empty library title",
        ),
        (
            "research.empty.body",
            "Import references or load an existing local library to begin.",
            "empty library body",
        ),
        ("research.inspector.notes_short", "Notes", "notes tab"),
        ("research.inspector.summary_short", "Sum", "summary tab"),
        ("research.inspector.qa_short", "QA", "Q&A tab"),
        ("research.inspector.visual_short", "Visual", "visual tab"),
        ("research.inspector.write_short", "Write", "writing tab"),
        ("research.inspector.cite_short", "Cite", "citation tab"),
        ("research.inspector.metadata", "Metadata", "metadata inspector tab"),
        ("research.inspector.notes", "Notes", "notes inspector tab"),
        ("research.inspector.summary", "Summary", "summary inspector tab"),
        ("research.inspector.qa", "Q&A", "question answer inspector tab"),
        (
            "research.inspector.qa_prompt",
            "Ask questions about this paper.",
            "question answer prompt",
        ),
        (
            "research.inspector.summary_text",
            "Summary: attention-only layers replace recurrence while preserving sequence structure through positional encodings.",
            "non-AI summary panel text",
        ),
        (
            "research.inspector.visuals",
            "Visuals",
            "visual inspector tab",
        ),
        (
            "research.inspector.writing",
            "Writing",
            "writing inspector tab",
        ),
        (
            "research.inspector.citations",
            "Citations",
            "citation inspector tab",
        ),
        (
            "research.empty.library",
            "No papers in this library.",
            "empty library state",
        ),
        (
            "research.error.import_failed",
            "Import failed.",
            "import error summary",
        ),
        (
            "research.error.save_failed",
            "Save failed.",
            "save error summary",
        ),
        (
            "research.pdf.encrypted",
            "PDF is encrypted.",
            "encrypted PDF state",
        ),
        (
            "research.pdf.unsupported",
            "Unsupported PDF.",
            "unsupported PDF state",
        ),
        (
            "research.ai.approval_required",
            "AI output requires approval before saving.",
            "AI approval policy",
        ),
        (
            "research.manuscript.outline",
            "Outline",
            "manuscript outline heading",
        ),
        (
            "research.manuscript.add_section",
            "+ Section",
            "add manuscript section button",
        ),
        (
            "research.manuscript.cite_search",
            "Search citations...",
            "cite while you write search",
        ),
        (
            "research.manuscript.insert_cite",
            "Insert",
            "insert citation button",
        ),
        (
            "research.manuscript.no_sections",
            "No sections yet. Add a section to start writing.",
            "empty manuscript state",
        ),
        (
            "research.manuscript.section_content",
            "Start writing...",
            "section content placeholder",
        ),
        (
            "research.welcome.title",
            "Welcome to Tench Research",
            "welcome screen title",
        ),
        (
            "research.welcome.body1",
            "Import papers, organize your library, annotate PDFs,",
            "welcome screen body line 1",
        ),
        (
            "research.welcome.body2",
            "and write manuscripts with cite-while-you-write.",
            "welcome screen body line 2",
        ),
        (
            "research.welcome.get_started",
            "Get Started",
            "welcome screen get started button",
        ),
        (
            "research.welcome.import",
            "Import Library",
            "welcome screen import button",
        ),
    ]
    .into_iter()
    .map(|(key, source, description)| localized_entry(locale, key, source, description))
    .collect()
}

pub fn research_i18n_catalog(locale: &str) -> tench_app_core::I18nCatalog {
    tench_app_core::I18nCatalog {
        product_id: "tench-research".to_string(),
        locale: locale.to_string(),
        entries: research_i18n_entries(locale),
    }
}

pub fn research_i18n_required_keys() -> Vec<String> {
    research_i18n_entries(DEFAULT_LOCALE)
        .into_iter()
        .map(|entry| entry.key)
        .collect()
}

fn localized_entry(
    _locale: &str,
    key: &str,
    source: &str,
    description: &str,
) -> tench_app_core::I18nEntry {
    tench_app_core::I18nEntry {
        key: key.to_string(),
        source_text: source.to_string(),
        localized_text: Some(source.to_string()),
        description: Some(description.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_catalog_is_complete_for_non_english_locales() {
        for locale in ["en-US", "ko-KR", "ar", "zh-Hans-CN", "fr-FR", "hi-IN"] {
            let catalog = research_i18n_catalog(locale);
            let required = research_i18n_required_keys();
            let report = catalog.coverage_report(&required);

            assert!(
                report.is_release_ready(),
                "{locale} should have no missing or fallback keys: {report:?}"
            );
        }
    }
}
