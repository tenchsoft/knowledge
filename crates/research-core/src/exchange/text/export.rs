use super::*;

pub fn render_plain_bibliography(references: &[ReferenceItem]) -> String {
    references
        .iter()
        .map(|reference| {
            let creators = reference
                .creators
                .iter()
                .filter(|creator| creator.role == CreatorRole::Author)
                .filter_map(|creator| {
                    creator
                        .literal
                        .clone()
                        .or_else(|| match (&creator.family, &creator.given) {
                            (Some(family), Some(given)) => Some(format!("{family}, {given}")),
                            (Some(family), None) => Some(family.clone()),
                            (None, Some(given)) => Some(given.clone()),
                            _ => None,
                        })
                })
                .collect::<Vec<_>>()
                .join("; ");
            let year = reference
                .issued
                .year
                .map(|year| year.to_string())
                .unwrap_or_else(|| "n.d.".to_string());
            let venue = reference
                .venue
                .as_ref()
                .map(|venue| venue.name.value.as_str())
                .unwrap_or("");
            format!("{creators} ({year}). {}. {venue}", reference.title.value)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn export_references_text(
    format: ResearchExportFormat,
    references: &[ReferenceItem],
) -> Result<String, String> {
    match format {
        ResearchExportFormat::BibTex => Ok(export_bibtex(references)),
        ResearchExportFormat::Ris => Ok(export_ris(references)),
        ResearchExportFormat::CslJson => export_csl_json(references),
        ResearchExportFormat::EndNoteXml => Ok(export_endnote_xml(references)),
        ResearchExportFormat::PlainTextBibliography => Ok(render_plain_bibliography(references)),
        ResearchExportFormat::MarkdownBibliography => Ok(render_plain_bibliography(references)
            .lines()
            .map(|line| format!("- {line}"))
            .collect::<Vec<_>>()
            .join("\n")),
        ResearchExportFormat::HtmlBibliography => Ok(format!(
            "<ol>{}</ol>",
            render_plain_bibliography(references)
                .lines()
                .map(|line| format!("<li>{}</li>", html_escape(line)))
                .collect::<Vec<_>>()
                .join("")
        )),
        ResearchExportFormat::RtfBibliography => Ok(render_rtf_bibliography(references)),
        _ => Err(format!("text exporter is not available for {:?}", format)),
    }
}

fn export_bibtex(references: &[ReferenceItem]) -> String {
    references
        .iter()
        .map(|reference| {
            let citekey = reference
                .citekey
                .clone()
                .unwrap_or_else(|| generate_citekey(reference));
            let mut fields = vec![
                ("title", reference.title.value.clone()),
                (
                    "author",
                    reference
                        .creators
                        .iter()
                        .filter(|creator| creator.role == CreatorRole::Author)
                        .map(format_bibtex_creator)
                        .collect::<Vec<_>>()
                        .join(" and "),
                ),
            ];
            if let Some(year) = reference.issued.year {
                fields.push(("year", year.to_string()));
            }
            if let Some(venue) = &reference.venue {
                fields.push(("journal", venue.name.value.clone()));
            }
            if let Some(doi) = &reference.identifiers.doi {
                fields.push(("doi", doi.clone()));
            }
            if let Some(url) = reference.urls.first() {
                fields.push(("url", url.url.clone()));
            }
            let body = fields
                .into_iter()
                .filter(|(_, value)| !value.trim().is_empty())
                .map(|(key, value)| format!("  {key} = {{{}}}", value.replace('}', "\\}")))
                .collect::<Vec<_>>()
                .join(",\n");
            format!(
                "@{}{{{},\n{}\n}}",
                bibtex_kind(reference.kind),
                citekey.as_str(),
                body
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn export_ris(references: &[ReferenceItem]) -> String {
    references
        .iter()
        .map(|reference| {
            let mut lines = vec![format!("TY  - {}", ris_kind(reference.kind))];
            lines.push(format!("TI  - {}", reference.title.value));
            for creator in reference
                .creators
                .iter()
                .filter(|creator| creator.role == CreatorRole::Author)
            {
                lines.push(format!("AU  - {}", format_ris_creator(creator)));
            }
            if let Some(year) = reference.issued.year {
                lines.push(format!("PY  - {year}"));
            }
            if let Some(venue) = &reference.venue {
                lines.push(format!("JO  - {}", venue.name.value));
            }
            if let Some(doi) = &reference.identifiers.doi {
                lines.push(format!("DO  - {doi}"));
            }
            if let Some(url) = reference.urls.first() {
                lines.push(format!("UR  - {}", url.url));
            }
            lines.push("ER  -".to_string());
            lines.join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn export_csl_json(references: &[ReferenceItem]) -> Result<String, String> {
    let values = references
        .iter()
        .map(|reference| {
            let authors = reference
                .creators
                .iter()
                .filter(|creator| creator.role == CreatorRole::Author)
                .map(|creator| {
                    json!({
                        "given": creator.given,
                        "family": creator.family,
                        "literal": creator.literal,
                    })
                })
                .collect::<Vec<_>>();
            json!({
                "id": reference.citekey.as_ref().map(Citekey::as_str).unwrap_or(reference.id.as_str()),
                "type": csl_kind(reference.kind),
                "title": reference.title.value,
                "author": authors,
                "issued": reference.issued.year.map(|year| json!({"date-parts": [[year]]})),
                "container-title": reference.venue.as_ref().map(|venue| venue.name.value.clone()),
                "DOI": reference.identifiers.doi,
                "URL": reference.urls.first().map(|url| url.url.clone()),
                "language": reference.language.as_ref().map(ResearchLocale::bcp47),
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&values).map_err(|error| error.to_string())
}

fn export_endnote_xml(references: &[ReferenceItem]) -> String {
    let records = references
        .iter()
        .enumerate()
        .map(|(index, reference)| {
            let rec_number = index + 1;
            let (ref_type_name, ref_type_code) = endnote_kind(reference.kind);
            let authors = reference
                .creators
                .iter()
                .filter(|creator| creator.role == CreatorRole::Author)
                .map(|creator| {
                    format!(
                        "<author>{}</author>",
                        xml_escape_text(&format_endnote_creator(creator))
                    )
                })
                .collect::<Vec<_>>()
                .join("");
            let contributors = if authors.is_empty() {
                String::new()
            } else {
                format!("<contributors><authors>{authors}</authors></contributors>")
            };
            let secondary_title = reference
                .venue
                .as_ref()
                .map(|venue| {
                    format!(
                        "<secondary-title>{}</secondary-title>",
                        xml_escape_text(&venue.name.value)
                    )
                })
                .unwrap_or_default();
            let year = reference
                .issued
                .year
                .map(|year| format!("<dates><year>{year}</year></dates>"))
                .unwrap_or_default();
            let doi = reference
                .identifiers
                .doi
                .as_ref()
                .map(|doi| {
                    format!(
                        "<electronic-resource-num>{}</electronic-resource-num>",
                        xml_escape_text(doi)
                    )
                })
                .unwrap_or_default();
            let url = reference
                .urls
                .first()
                .map(|url| {
                    format!(
                        "<urls><related-urls><url>{}</url></related-urls></urls>",
                        xml_escape_text(&url.url)
                    )
                })
                .unwrap_or_default();
            let language = reference
                .language
                .as_ref()
                .map(|locale| format!("<language>{}</language>", xml_escape_text(&locale.bcp47())))
                .unwrap_or_default();
            let abstract_text = reference
                .abstract_text
                .as_ref()
                .map(|abstract_text| {
                    format!(
                        "<abstract>{}</abstract>",
                        xml_escape_text(&abstract_text.value)
                    )
                })
                .unwrap_or_default();

            format!(
                concat!(
                    "<record>",
                    "<rec-number>{rec_number}</rec-number>",
                    "<ref-type name=\"{ref_type_name}\">{ref_type_code}</ref-type>",
                    "{contributors}",
                    "<titles><title>{title}</title>{secondary_title}</titles>",
                    "{year}",
                    "{doi}",
                    "{url}",
                    "{language}",
                    "{abstract_text}",
                    "</record>"
                ),
                rec_number = rec_number,
                ref_type_name = ref_type_name,
                ref_type_code = ref_type_code,
                contributors = contributors,
                title = xml_escape_text(&reference.title.value),
                secondary_title = secondary_title,
                year = year,
                doi = doi,
                url = url,
                language = language,
                abstract_text = abstract_text,
            )
        })
        .collect::<Vec<_>>()
        .join("");
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><xml><records>{records}</records></xml>")
}

fn render_rtf_bibliography(references: &[ReferenceItem]) -> String {
    let body = render_plain_bibliography(references)
        .lines()
        .map(|line| format!("\\pard {}\\par", rtf_escape(line)))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{{\\rtf1\\ansi\\deff0\n{body}\n}}")
}

fn bibtex_kind(kind: ReferenceKind) -> &'static str {
    match kind {
        ReferenceKind::JournalArticle => "article",
        ReferenceKind::ConferencePaper => "inproceedings",
        ReferenceKind::Book => "book",
        ReferenceKind::BookSection => "incollection",
        ReferenceKind::Thesis => "phdthesis",
        ReferenceKind::Report => "techreport",
        ReferenceKind::WebPage => "online",
        _ => "misc",
    }
}

fn ris_kind(kind: ReferenceKind) -> &'static str {
    match kind {
        ReferenceKind::JournalArticle => "JOUR",
        ReferenceKind::ConferencePaper => "CONF",
        ReferenceKind::Book => "BOOK",
        ReferenceKind::BookSection => "CHAP",
        ReferenceKind::Thesis => "THES",
        ReferenceKind::Report => "RPRT",
        ReferenceKind::Dataset => "DATA",
        ReferenceKind::WebPage => "WEB",
        _ => "GEN",
    }
}

fn csl_kind(kind: ReferenceKind) -> &'static str {
    match kind {
        ReferenceKind::JournalArticle => "article-journal",
        ReferenceKind::ConferencePaper => "paper-conference",
        ReferenceKind::Book => "book",
        ReferenceKind::BookSection => "chapter",
        ReferenceKind::Thesis => "thesis",
        ReferenceKind::Report => "report",
        ReferenceKind::Dataset => "dataset",
        ReferenceKind::WebPage => "webpage",
        _ => "article",
    }
}

fn endnote_kind(kind: ReferenceKind) -> (&'static str, &'static str) {
    match kind {
        ReferenceKind::JournalArticle => ("Journal Article", "17"),
        ReferenceKind::ConferencePaper => ("Conference Paper", "47"),
        ReferenceKind::Book => ("Book", "10"),
        ReferenceKind::BookSection => ("Book Section", "12"),
        ReferenceKind::Thesis => ("Thesis", "32"),
        ReferenceKind::Report => ("Report", "27"),
        ReferenceKind::Dataset => ("Dataset", "59"),
        ReferenceKind::WebPage => ("Web Page", "12"),
        _ => ("Generic", "0"),
    }
}

fn format_bibtex_creator(creator: &Creator) -> String {
    creator
        .literal
        .clone()
        .unwrap_or_else(|| match (&creator.family, &creator.given) {
            (Some(family), Some(given)) => format!("{family}, {given}"),
            (Some(family), None) => family.clone(),
            (None, Some(given)) => given.clone(),
            _ => String::new(),
        })
}

fn format_ris_creator(creator: &Creator) -> String {
    format_bibtex_creator(creator)
}

fn format_endnote_creator(creator: &Creator) -> String {
    format_bibtex_creator(creator)
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn rtf_escape(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '\n' => out.push_str("\\line "),
            ch if ch.is_ascii() => out.push(ch),
            ch => out.push_str(&format!("\\u{}?", ch as i32)),
        }
    }
    out
}
