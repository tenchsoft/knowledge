pub const DEFAULT_LOCALE: &str = "en-US";

pub fn study_i18n_entries(locale: &str) -> Vec<tench_app_core::I18nEntry> {
    [
        ("study.app.title", "Tench Study", "window title"),
        ("study.dashboard.title", "Dashboard", "dashboard heading"),
        ("study.curriculum.title", "Curriculum", "curriculum panel"),
        ("study.practice.title", "Practice", "practice panel"),
        ("study.review.title", "Review", "review queue panel"),
        ("study.notes.title", "Notes", "notes panel"),
        ("study.cards.title", "Cards", "cards panel"),
        (
            "study.authoring.title",
            "Authoring",
            "custom curriculum authoring",
        ),
        ("study.subject.math", "Mathematics", "mathematics subject"),
        ("study.subject.science", "Science", "science subject"),
        ("study.subject.languages", "Languages", "languages subject"),
        (
            "study.subject.programming",
            "Programming",
            "programming subject",
        ),
        ("study.stage.learn", "learn", "learn stage"),
        ("study.stage.practice", "practice", "practice stage"),
        ("study.stage.review", "review", "review stage"),
        ("study.header.streak", "streak", "streak label"),
        ("study.header.stats", "Stats", "stats button"),
        (
            "study.curriculum.search",
            "Search curriculum",
            "curriculum search prompt",
        ),
        ("study.review.queue", "Review queue", "review queue button"),
        (
            "study.learn.definition",
            "DEFINITION",
            "definition block label",
        ),
        (
            "study.learn.visuals_available",
            "interactive visuals available",
            "visual count suffix",
        ),
        ("study.learn.example", "EXAMPLE", "example block label"),
        (
            "study.learn.example_prefix",
            "builds from the",
            "example sentence middle",
        ),
        (
            "study.learn.example_suffix",
            "progression.",
            "example sentence suffix",
        ),
        (
            "study.learn.quick_check",
            "QUICK CHECK",
            "quick check label",
        ),
        (
            "study.learn.quick_question",
            "What is one core idea from this level?",
            "quick check question",
        ),
        ("study.learn.visual", "VISUAL", "visual control label"),
        (
            "study.learn.start_practice",
            "Start practice",
            "start practice button",
        ),
        (
            "study.learn.review_concept",
            "Review concept",
            "review concept button",
        ),
        ("study.practice.problem", "Problem", "problem counter label"),
        ("study.practice.answer", "Your answer", "answer input label"),
        (
            "study.practice.answer_prompt",
            "Type an answer...",
            "answer input prompt",
        ),
        ("study.practice.submit", "Submit", "submit answer button"),
        ("study.practice.correct", "Correct", "correct feedback"),
        (
            "study.practice.needs_review",
            "Needs review",
            "incorrect feedback",
        ),
        ("study.practice.retry", "Retry", "retry answer button"),
        ("study.practice.next", "Next", "next problem button"),
        ("study.review.wrong", "Wrong", "wrong answer label"),
        ("study.review.correct", "Correct", "correct answer label"),
        ("study.review.cause", "Cause", "cause label"),
        ("study.review.related", "Related", "related concept label"),
        ("study.tutor.title", "Tutor", "tutor panel title"),
        ("study.tutor.context", "Context", "tutor context heading"),
        ("study.tutor.hints", "Hints", "hints heading"),
        (
            "study.tutor.reveal_hint",
            "Reveal hint",
            "reveal hint button",
        ),
        (
            "study.tutor.weak_points",
            "Weak points",
            "weak points heading",
        ),
        (
            "study.tutor.no_weak_points",
            "No weak points yet.",
            "empty weak points state",
        ),
        ("study.tutor.glossary", "Glossary", "glossary heading"),
        ("study.stats.accuracy", "Accuracy", "accuracy metric label"),
        ("study.stats.reviews", "Reviews", "reviews metric label"),
        (
            "study.modal.session_result",
            "Session Result",
            "result modal title",
        ),
        ("study.modal.stats", "Stats Dashboard", "stats modal title"),
        ("study.modal.close", "x", "modal close control"),
        ("study.stats.solved", "Solved", "solved count label"),
        ("study.stats.wrong", "Wrong", "wrong count label"),
        (
            "study.stats.total_sessions",
            "Total sessions",
            "total sessions label",
        ),
        (
            "study.stats.pending_reviews",
            "Pending reviews",
            "pending reviews label",
        ),
        ("study.stats.streak", "Streak", "streak metric label"),
        (
            "study.stats.builtin_curricula",
            "Built-in curricula",
            "built-in curricula count label",
        ),
        (
            "study.stats.lessons_visuals",
            "Lessons / visuals",
            "lesson and visual count label",
        ),
        (
            "study.stats.glossary",
            "Glossary terms",
            "glossary term count label",
        ),
        (
            "study.hint.one",
            "Look at one row and one column.",
            "first practice hint",
        ),
        (
            "study.hint.two",
            "Multiply matching entries, then add.",
            "second practice hint",
        ),
        (
            "study.hint.three",
            "The top-left entry is 1*5 + 2*7.",
            "third practice hint",
        ),
        ("study.action.import", "Import", "import action"),
        ("study.action.export", "Export", "export action"),
        ("study.action.validate", "Validate", "validation action"),
        ("study.action.start", "Start", "start action"),
        ("study.action.pause", "Pause", "pause action"),
        ("study.action.step", "Step", "step visual action"),
        ("study.action.replay", "Replay", "replay visual action"),
        (
            "study.shortcut.start_or_submit",
            "Start practice or submit answer",
            "keyboard shortcut label",
        ),
        (
            "study.shortcut.cycle_stage",
            "Cycle study stage",
            "keyboard shortcut label",
        ),
        (
            "study.shortcut.previous_concept",
            "Previous concept",
            "keyboard shortcut label",
        ),
        (
            "study.shortcut.next_concept",
            "Next concept",
            "keyboard shortcut label",
        ),
        (
            "study.shortcut.open_stats",
            "Open stats",
            "keyboard shortcut label",
        ),
        (
            "study.shortcut.open_review",
            "Open review queue",
            "keyboard shortcut label",
        ),
        (
            "study.shortcut.close",
            "Close modal",
            "keyboard shortcut label",
        ),
        ("study.a11y.header", "Study header", "accessibility label"),
        (
            "study.a11y.curriculum",
            "Curriculum outline",
            "accessibility label",
        ),
        (
            "study.a11y.learn_surface",
            "Learning surface",
            "accessibility label",
        ),
        (
            "study.a11y.practice_surface",
            "Practice surface",
            "accessibility label",
        ),
        (
            "study.a11y.review_surface",
            "Review surface",
            "accessibility label",
        ),
        (
            "study.a11y.tutor_panel",
            "Tutor panel",
            "accessibility label",
        ),
        (
            "study.a11y.stats_modal",
            "Stats modal",
            "accessibility label",
        ),
        (
            "study.error.save_failed",
            "Save failed.",
            "save error summary",
        ),
        (
            "study.error.import_failed",
            "Import failed.",
            "import error summary",
        ),
        (
            "study.ai.approval_required",
            "AI output requires approval before saving.",
            "AI approval policy",
        ),
    ]
    .into_iter()
    .map(|(key, source, description)| localized_entry(locale, key, source, description))
    .collect()
}

pub fn study_i18n_catalog(locale: &str) -> tench_app_core::I18nCatalog {
    tench_app_core::I18nCatalog {
        product_id: "tench-study".to_string(),
        locale: locale.to_string(),
        entries: study_i18n_entries(locale),
    }
}

pub fn study_i18n_required_keys() -> Vec<String> {
    study_i18n_entries(DEFAULT_LOCALE)
        .into_iter()
        .map(|entry| entry.key)
        .collect()
}

pub fn resolve<'a>(catalog: &'a tench_app_core::I18nCatalog, key: &'a str) -> &'a str {
    catalog.resolve(key).unwrap_or(key)
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
            let catalog = study_i18n_catalog(locale);
            let required = study_i18n_required_keys();
            let report = catalog.coverage_report(&required);

            assert!(
                report.is_release_ready(),
                "{locale} should have no missing or fallback keys: {report:?}"
            );
        }
    }
}
