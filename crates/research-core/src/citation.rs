use serde::{Deserialize, Serialize};

use crate::exchange::generate_citekey;
use crate::{Citekey, Creator, CreatorRole, ReferenceId, ReferenceItem, ResearchLocale};

crate::research_id_type!(CitationStyleId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationRenderMode {
    InText,
    Footnote,
    Endnote,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationOutputFormat {
    PlainText,
    Markdown,
    Html,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CslStyleSummary {
    pub id: CitationStyleId,
    pub title: String,
    pub has_style_root: bool,
    pub has_citation: bool,
    pub has_bibliography: bool,
    pub valid: bool,
    pub default_mode: CitationRenderMode,
    pub issues: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CslLocaleSummary {
    pub locale: ResearchLocale,
    pub declares_locale: bool,
    pub term_count: usize,
    pub valid: bool,
    pub issues: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CitationRenderRequest {
    pub style_id: CitationStyleId,
    pub locale: ResearchLocale,
    #[serde(default)]
    pub fallback_locale: Option<ResearchLocale>,
    pub mode: CitationRenderMode,
    pub output_format: CitationOutputFormat,
    pub include_bibliography: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CitationPreview {
    pub reference_id: ReferenceId,
    pub citekey: Citekey,
    pub rendered: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CitationRenderOutput {
    pub inline_citations: Vec<CitationPreview>,
    #[serde(default)]
    pub bibliography: Option<String>,
    pub warnings: Vec<String>,
    pub style_id_used: CitationStyleId,
    pub locale_used: ResearchLocale,
    pub output_format: CitationOutputFormat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CitationStyleFamily {
    Apa,
    Mla,
    Chicago,
    Numeric,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct LocaleTerms {
    no_date: &'static str,
    and: &'static str,
    et_al: &'static str,
    unknown_author: &'static str,
}

pub fn validate_csl_style_text(id: CitationStyleId, text: &str) -> CslStyleSummary {
    let has_style_root = contains_xml_element(text, "style");
    let has_citation = contains_xml_element(text, "citation");
    let has_bibliography = contains_xml_element(text, "bibliography");
    let mut issues = Vec::new();

    if !has_style_root {
        issues.push("CSL style root element is missing.".to_string());
    }
    if !has_citation {
        issues.push("CSL citation section is missing.".to_string());
    }
    if !has_bibliography {
        issues.push("CSL bibliography section is missing.".to_string());
    }

    let title = extract_xml_text(text, "title")
        .filter(|title| !title.trim().is_empty())
        .unwrap_or_else(|| id.as_str().to_string());
    let default_mode = match citation_style_family(&id) {
        CitationStyleFamily::Chicago => CitationRenderMode::Footnote,
        _ => CitationRenderMode::InText,
    };

    CslStyleSummary {
        id,
        title,
        has_style_root,
        has_citation,
        has_bibliography,
        valid: issues.is_empty(),
        default_mode,
        issues,
    }
}

pub fn validate_csl_locale_text(locale: ResearchLocale, text: &str) -> CslLocaleSummary {
    let lower = text.to_ascii_lowercase();
    let locale_tag = locale.bcp47().to_ascii_lowercase();
    let declares_locale = contains_xml_element(text, "locale")
        && (lower.contains(&format!("xml:lang=\"{locale_tag}\""))
            || lower.contains(&format!("xml:lang='{locale_tag}'"))
            || lower.contains(&format!("lang=\"{locale_tag}\""))
            || lower.contains(&format!("lang='{locale_tag}'")));
    let term_count = lower.matches("<term ").count() + lower.matches("<term>").count();
    let mut issues = Vec::new();

    if !contains_xml_element(text, "locale") {
        issues.push("CSL locale root element is missing.".to_string());
    }
    if !declares_locale {
        issues.push(format!("CSL locale does not declare {}.", locale.bcp47()));
    }
    if term_count == 0 {
        issues.push("CSL locale does not define any terms.".to_string());
    }

    CslLocaleSummary {
        locale,
        declares_locale,
        term_count,
        valid: issues.is_empty(),
        issues,
    }
}

pub fn render_citation_preview(
    request: &CitationRenderRequest,
    references: &[ReferenceItem],
) -> CitationRenderOutput {
    let mut warnings = Vec::new();
    let style_family = citation_style_family(&request.style_id);
    let effective_family = if style_family == CitationStyleFamily::Unsupported {
        warnings.push(format!(
            "Unsupported citation style '{}'; using APA-compatible fallback rendering.",
            request.style_id.as_str()
        ));
        CitationStyleFamily::Apa
    } else {
        style_family
    };
    let locale_used = choose_locale(
        &request.locale,
        request.fallback_locale.as_ref(),
        &mut warnings,
    );
    let terms = locale_terms(&locale_used).unwrap_or_else(default_locale_terms);

    let inline_citations = references
        .iter()
        .enumerate()
        .map(|(index, reference)| {
            append_reference_warnings(reference, &mut warnings);
            let citekey = reference
                .citekey
                .clone()
                .unwrap_or_else(|| generate_citekey(reference));
            CitationPreview {
                reference_id: reference.id.clone(),
                citekey,
                rendered: format_citation_output(
                    &render_inline_citation(
                        reference,
                        index,
                        effective_family,
                        request.mode,
                        terms,
                    ),
                    request.output_format,
                ),
            }
        })
        .collect::<Vec<_>>();
    let bibliography = request
        .include_bibliography
        .then(|| render_bibliography_with_style(request, references));

    CitationRenderOutput {
        inline_citations,
        bibliography,
        warnings,
        style_id_used: request.style_id.clone(),
        locale_used,
        output_format: request.output_format,
    }
}

pub fn render_bibliography_with_style(
    request: &CitationRenderRequest,
    references: &[ReferenceItem],
) -> String {
    let style_family = match citation_style_family(&request.style_id) {
        CitationStyleFamily::Unsupported => CitationStyleFamily::Apa,
        family => family,
    };
    let locale = locale_terms(&request.locale)
        .or_else(|| request.fallback_locale.as_ref().and_then(locale_terms))
        .unwrap_or_else(default_locale_terms);
    let lines = references
        .iter()
        .enumerate()
        .map(|(index, reference)| format_bibliography_entry(reference, index, style_family, locale))
        .collect::<Vec<_>>();

    match request.output_format {
        CitationOutputFormat::PlainText => lines.join("\n"),
        CitationOutputFormat::Markdown => lines
            .iter()
            .enumerate()
            .map(|(index, line)| format!("{}. {line}", index + 1))
            .collect::<Vec<_>>()
            .join("\n"),
        CitationOutputFormat::Html => format!(
            "<ol>{}</ol>",
            lines
                .iter()
                .map(|line| format!("<li>{}</li>", html_escape(line)))
                .collect::<Vec<_>>()
                .join("")
        ),
    }
}

pub fn citation_clipboard_payload(
    output: &CitationRenderOutput,
    include_bibliography: bool,
) -> String {
    let mut parts = output
        .inline_citations
        .iter()
        .map(|citation| citation.rendered.clone())
        .collect::<Vec<_>>();
    if include_bibliography {
        if let Some(bibliography) = &output.bibliography {
            if !bibliography.trim().is_empty() {
                parts.push(bibliography.clone());
            }
        }
    }
    parts.join("\n")
}

fn render_inline_citation(
    reference: &ReferenceItem,
    index: usize,
    family: CitationStyleFamily,
    mode: CitationRenderMode,
    terms: LocaleTerms,
) -> String {
    match mode {
        CitationRenderMode::Footnote => {
            format!(
                "[^{}: {}]",
                index + 1,
                footnote_reference(reference, family, terms)
            )
        }
        CitationRenderMode::Endnote => {
            format!(
                "[endnote {}: {}]",
                index + 1,
                footnote_reference(reference, family, terms)
            )
        }
        CitationRenderMode::InText => match family {
            CitationStyleFamily::Apa | CitationStyleFamily::Unsupported => {
                format!(
                    "({author}, {year})",
                    author = inline_author(reference, terms),
                    year = reference_year(reference, terms)
                )
            }
            CitationStyleFamily::Mla => format!("({})", inline_author(reference, terms)),
            CitationStyleFamily::Chicago => format!(
                "({author} {year})",
                author = inline_author(reference, terms),
                year = reference_year(reference, terms)
            ),
            CitationStyleFamily::Numeric => format!("[{}]", index + 1),
        },
    }
}

fn footnote_reference(
    reference: &ReferenceItem,
    family: CitationStyleFamily,
    terms: LocaleTerms,
) -> String {
    let author = bibliography_authors(reference, terms);
    let title = reference.title.value.trim();
    let title = if title.is_empty() { "Untitled" } else { title };
    let venue = reference
        .venue
        .as_ref()
        .map(|venue| venue.name.value.as_str())
        .filter(|venue| !venue.trim().is_empty());
    let year = reference_year(reference, terms);

    match family {
        CitationStyleFamily::Mla => match venue {
            Some(venue) => format!("{author}, \"{title},\" {venue}"),
            None => format!("{author}, \"{title}\""),
        },
        CitationStyleFamily::Numeric => match venue {
            Some(venue) => format!("{author}, {title}, {venue}, {year}"),
            None => format!("{author}, {title}, {year}"),
        },
        CitationStyleFamily::Apa
        | CitationStyleFamily::Chicago
        | CitationStyleFamily::Unsupported => match venue {
            Some(venue) => format!("{author}, \"{title},\" {venue} ({year})"),
            None => format!("{author}, \"{title}\" ({year})"),
        },
    }
}

fn format_bibliography_entry(
    reference: &ReferenceItem,
    index: usize,
    family: CitationStyleFamily,
    terms: LocaleTerms,
) -> String {
    let author = bibliography_authors(reference, terms);
    let year = reference_year(reference, terms);
    let title = reference.title.value.trim();
    let title = if title.is_empty() { "Untitled" } else { title };
    let venue = reference
        .venue
        .as_ref()
        .map(|venue| venue.name.value.as_str())
        .filter(|venue| !venue.trim().is_empty());
    let doi = reference
        .identifiers
        .doi
        .as_deref()
        .filter(|doi| !doi.trim().is_empty());

    match family {
        CitationStyleFamily::Apa | CitationStyleFamily::Unsupported => {
            let mut entry = format!("{author}. ({year}). {title}.");
            if let Some(venue) = venue {
                entry.push(' ');
                entry.push_str(venue);
                entry.push('.');
            }
            if let Some(doi) = doi {
                entry.push(' ');
                entry.push_str("https://doi.org/");
                entry.push_str(doi);
            }
            entry
        }
        CitationStyleFamily::Mla => match venue {
            Some(venue) => format!("{author}. \"{title}.\" {venue}, {year}."),
            None => format!("{author}. \"{title}.\" {year}."),
        },
        CitationStyleFamily::Chicago => match venue {
            Some(venue) => format!("{author}. \"{title}.\" {venue} ({year})."),
            None => format!("{author}. \"{title}.\" {year}."),
        },
        CitationStyleFamily::Numeric => match venue {
            Some(venue) => format!("[{}] {author}, \"{title},\" {venue}, {year}.", index + 1),
            None => format!("[{}] {author}, \"{title},\" {year}.", index + 1),
        },
    }
}

fn citation_style_family(style_id: &CitationStyleId) -> CitationStyleFamily {
    let id = style_id.as_str().to_ascii_lowercase();
    if id.contains("apa") {
        CitationStyleFamily::Apa
    } else if id.contains("mla") || id.contains("modern-language-association") {
        CitationStyleFamily::Mla
    } else if id.contains("chicago") || id.contains("turabian") {
        CitationStyleFamily::Chicago
    } else if id.contains("ieee") || id.contains("vancouver") || id.contains("numeric") {
        CitationStyleFamily::Numeric
    } else {
        CitationStyleFamily::Unsupported
    }
}

fn choose_locale(
    requested: &ResearchLocale,
    fallback: Option<&ResearchLocale>,
    warnings: &mut Vec<String>,
) -> ResearchLocale {
    if locale_terms(requested).is_some() {
        return requested.clone();
    }
    if let Some(fallback) = fallback {
        if locale_terms(fallback).is_some() {
            warnings.push(format!(
                "Citation locale '{}' is not available in the fallback renderer; using '{}'.",
                requested.bcp47(),
                fallback.bcp47()
            ));
            return fallback.clone();
        }
    }
    warnings.push(format!(
        "Citation locale '{}' is not available in the fallback renderer; using 'en-US'.",
        requested.bcp47()
    ));
    ResearchLocale::parse("en-US").expect("built-in locale")
}

fn locale_terms(locale: &ResearchLocale) -> Option<LocaleTerms> {
    match locale.language.as_str() {
        "ar" => Some(LocaleTerms {
            no_date: "بدون تاريخ",
            and: "و",
            et_al: "وآخرون",
            unknown_author: "مؤلف غير معروف",
        }),
        "de" => Some(LocaleTerms {
            no_date: "o. J.",
            and: "und",
            et_al: "et al.",
            unknown_author: "Unbekannter Autor",
        }),
        "en" => Some(default_locale_terms()),
        "es" => Some(LocaleTerms {
            no_date: "s. f.",
            and: "y",
            et_al: "et al.",
            unknown_author: "Autor desconocido",
        }),
        "fr" => Some(LocaleTerms {
            no_date: "s. d.",
            and: "et",
            et_al: "et al.",
            unknown_author: "Auteur inconnu",
        }),
        "ja" => Some(LocaleTerms {
            no_date: "日付なし",
            and: "、",
            et_al: "ほか",
            unknown_author: "著者不明",
        }),
        "ko" => Some(LocaleTerms {
            no_date: "날짜 없음",
            and: "및",
            et_al: "외",
            unknown_author: "저자 미상",
        }),
        "pt" => Some(LocaleTerms {
            no_date: "s. d.",
            and: "e",
            et_al: "et al.",
            unknown_author: "Autor desconhecido",
        }),
        "zh" => Some(LocaleTerms {
            no_date: "无日期",
            and: "和",
            et_al: "等",
            unknown_author: "未知作者",
        }),
        _ => None,
    }
}

fn default_locale_terms() -> LocaleTerms {
    LocaleTerms {
        no_date: "n.d.",
        and: "&",
        et_al: "et al.",
        unknown_author: "Unknown author",
    }
}

fn inline_author(reference: &ReferenceItem, terms: LocaleTerms) -> String {
    let authors = author_display_names(reference)
        .into_iter()
        .map(|creator| short_author_name(&creator))
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    match authors.as_slice() {
        [] => fallback_title_or_author(reference, terms),
        [author] => author.clone(),
        [first, second] => format!("{first} {} {second}", terms.and),
        [first, ..] => format!("{first} {}", terms.et_al),
    }
}

fn bibliography_authors(reference: &ReferenceItem, terms: LocaleTerms) -> String {
    let authors = author_display_names(reference)
        .into_iter()
        .map(|creator| full_author_name(&creator))
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    if authors.is_empty() {
        return terms.unknown_author.to_string();
    }
    authors.join("; ")
}

fn author_display_names(reference: &ReferenceItem) -> Vec<Creator> {
    reference
        .creators
        .iter()
        .filter(|creator| creator.role == CreatorRole::Author)
        .cloned()
        .collect()
}

fn short_author_name(creator: &Creator) -> String {
    creator
        .literal
        .clone()
        .or_else(|| creator.family.clone())
        .or_else(|| creator.given.clone())
        .unwrap_or_default()
}

fn full_author_name(creator: &Creator) -> String {
    if let Some(literal) = &creator.literal {
        return literal.clone();
    }
    match (&creator.family, &creator.given) {
        (Some(family), Some(given)) => format!("{family}, {given}"),
        (Some(family), None) => family.clone(),
        (None, Some(given)) => given.clone(),
        _ => String::new(),
    }
}

fn fallback_title_or_author(reference: &ReferenceItem, terms: LocaleTerms) -> String {
    reference
        .title
        .value
        .split_whitespace()
        .take(4)
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
        .if_empty(|| terms.unknown_author.to_string())
}

fn reference_year(reference: &ReferenceItem, terms: LocaleTerms) -> String {
    reference
        .issued
        .year
        .map(|year| year.to_string())
        .unwrap_or_else(|| terms.no_date.to_string())
}

fn append_reference_warnings(reference: &ReferenceItem, warnings: &mut Vec<String>) {
    if reference.title.value.trim().is_empty() {
        warnings.push(format!(
            "Reference '{}' has no title.",
            reference.id.as_str()
        ));
    }
    if reference
        .creators
        .iter()
        .all(|creator| creator.role != CreatorRole::Author || !creator.has_display_name())
    {
        warnings.push(format!(
            "Reference '{}' has no author metadata.",
            reference.id.as_str()
        ));
    }
    if reference.issued.year.is_none() {
        warnings.push(format!(
            "Reference '{}' has no issued year.",
            reference.id.as_str()
        ));
    }
}

fn format_citation_output(value: &str, output_format: CitationOutputFormat) -> String {
    match output_format {
        CitationOutputFormat::PlainText | CitationOutputFormat::Markdown => value.to_string(),
        CitationOutputFormat::Html => html_escape(value),
    }
}

fn contains_xml_element(text: &str, element_name: &str) -> bool {
    text.to_ascii_lowercase()
        .contains(&format!("<{}", element_name.to_ascii_lowercase()))
}

fn extract_xml_text(text: &str, element_name: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let open_start = lower.find(&format!("<{}", element_name.to_ascii_lowercase()))?;
    let open_end = lower[open_start..].find('>')? + open_start;
    let close_start = lower[open_end + 1..].find(&format!("</{element_name}>"))? + open_end + 1;
    Some(
        strip_xml_tags(&text[open_end + 1..close_start])
            .trim()
            .to_string(),
    )
}

fn strip_xml_tags(value: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in value.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

trait StringEmptyExt {
    fn if_empty(self, fallback: impl FnOnce() -> String) -> String;
}

impl StringEmptyExt for String {
    fn if_empty(self, fallback: impl FnOnce() -> String) -> String {
        if self.is_empty() {
            fallback()
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        reference_from_minimal_metadata, CreatorNameOrder, LocalizedField, PublicationVenue,
        ReferenceIdentifiers, ReferenceKind,
    };

    fn locale(tag: &str) -> ResearchLocale {
        ResearchLocale::parse(tag).expect("locale")
    }

    fn reference(id: &str, title: &str, year: Option<u16>) -> ReferenceItem {
        let mut reference = reference_from_minimal_metadata(
            id,
            ReferenceKind::JournalArticle,
            title,
            year,
            "2026-05-04T00:00:00Z",
        );
        reference.creators.push(Creator {
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
        });
        reference.venue = Some(PublicationVenue {
            name: LocalizedField::plain("NeurIPS"),
            volume: None,
            issue: None,
            pages: None,
            publisher: None,
        });
        reference.identifiers = ReferenceIdentifiers {
            doi: Some("10.5555/attention".to_string()),
            ..ReferenceIdentifiers::default()
        };
        reference
    }

    #[test]
    fn csl_style_validation_reports_missing_sections() {
        let summary = validate_csl_style_text(
            CitationStyleId::from("apa"),
            r#"<style><info><title>APA</title></info><citation /></style>"#,
        );

        assert!(!summary.valid);
        assert_eq!(summary.title, "APA");
        assert!(summary.has_citation);
        assert!(!summary.has_bibliography);
    }

    #[test]
    fn csl_locale_validation_counts_terms() {
        let summary = validate_csl_locale_text(
            locale("ko-KR"),
            r#"<locale xml:lang="ko-KR"><terms><term name="no date">날짜 없음</term></terms></locale>"#,
        );

        assert!(summary.valid);
        assert_eq!(summary.term_count, 1);
    }

    #[test]
    fn apa_preview_renders_inline_and_bibliography() {
        let request = CitationRenderRequest {
            style_id: CitationStyleId::from("apa"),
            locale: locale("en-US"),
            fallback_locale: None,
            mode: CitationRenderMode::InText,
            output_format: CitationOutputFormat::PlainText,
            include_bibliography: true,
        };
        let output = render_citation_preview(
            &request,
            &[reference(
                "ref-attention",
                "Attention Is All You Need",
                Some(2017),
            )],
        );

        assert_eq!(output.inline_citations[0].rendered, "(Vaswani, 2017)");
        assert!(output
            .bibliography
            .as_deref()
            .is_some_and(|value| value.contains("Attention Is All You Need")));
    }

    #[test]
    fn unsupported_style_uses_locale_fallback_and_preserves_title() {
        let mut reference = reference("ref-ko", "다국어 논문", None);
        reference.creators.clear();
        let request = CitationRenderRequest {
            style_id: CitationStyleId::from("experimental-style"),
            locale: locale("tlh"),
            fallback_locale: Some(locale("ko-KR")),
            mode: CitationRenderMode::InText,
            output_format: CitationOutputFormat::PlainText,
            include_bibliography: true,
        };
        let output = render_citation_preview(&request, &[reference]);

        assert_eq!(output.locale_used.bcp47(), "ko-KR");
        assert!(output
            .warnings
            .iter()
            .any(|warning| warning.contains("Unsupported")));
        assert!(output
            .bibliography
            .as_deref()
            .is_some_and(|value| value.contains("다국어 논문") && value.contains("날짜 없음")));
    }

    #[test]
    fn clipboard_payload_can_include_bibliography() {
        let output = CitationRenderOutput {
            inline_citations: vec![CitationPreview {
                reference_id: ReferenceId::from("ref"),
                citekey: Citekey::from("vaswani2017attention"),
                rendered: "(Vaswani, 2017)".to_string(),
            }],
            bibliography: Some("Vaswani. (2017). Attention.".to_string()),
            warnings: Vec::new(),
            style_id_used: CitationStyleId::from("apa"),
            locale_used: locale("en-US"),
            output_format: CitationOutputFormat::PlainText,
        };

        let payload = citation_clipboard_payload(&output, true);

        assert!(payload.contains("(Vaswani, 2017)"));
        assert!(payload.contains("Attention"));
    }
}
