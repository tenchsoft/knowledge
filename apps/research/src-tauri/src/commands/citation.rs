use tench_research_core::*;

#[tauri::command]
pub fn detect_research_import_format(value: String) -> Option<ResearchImportFormat> {
    tench_research_core::detect_import_format(&value)
}

#[tauri::command]
pub fn generate_research_citekey(reference: ReferenceItem) -> Citekey {
    tench_research_core::generate_citekey(&reference)
}

#[tauri::command]
pub fn render_research_plain_bibliography(references: Vec<ReferenceItem>) -> String {
    tench_research_core::render_plain_bibliography(&references)
}

#[tauri::command]
pub fn validate_research_csl_style_text(
    id: tench_research_core::CitationStyleId,
    text: String,
) -> tench_research_core::CslStyleSummary {
    tench_research_core::validate_csl_style_text(id, &text)
}

#[tauri::command]
pub fn validate_research_csl_locale_text(
    locale: tench_research_core::ResearchLocale,
    text: String,
) -> tench_research_core::CslLocaleSummary {
    tench_research_core::validate_csl_locale_text(locale, &text)
}

#[tauri::command]
pub fn render_research_citation_preview(
    request: tench_research_core::CitationRenderRequest,
    references: Vec<ReferenceItem>,
) -> tench_research_core::CitationRenderOutput {
    tench_research_core::render_citation_preview(&request, &references)
}

#[tauri::command]
pub fn render_research_bibliography_with_style(
    request: tench_research_core::CitationRenderRequest,
    references: Vec<ReferenceItem>,
) -> String {
    tench_research_core::render_bibliography_with_style(&request, &references)
}

#[tauri::command]
pub fn research_citation_clipboard_payload(
    output: tench_research_core::CitationRenderOutput,
    include_bibliography: bool,
) -> String {
    tench_research_core::citation_clipboard_payload(&output, include_bibliography)
}

#[tauri::command]
pub fn parse_research_references_text(
    format: ResearchImportFormat,
    text: String,
    now: String,
) -> Result<Vec<ReferenceItem>, String> {
    tench_research_core::parse_references_text(format, &text, now)
}

#[tauri::command]
pub fn export_research_references_text(
    format: ResearchExportFormat,
    references: Vec<ReferenceItem>,
) -> Result<String, String> {
    tench_research_core::export_references_text(format, &references)
}
