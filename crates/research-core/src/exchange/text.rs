use std::collections::HashMap;

use serde_json::{json, Value};

use crate::{
    Citekey, Creator, CreatorNameOrder, CreatorRole, KeyValue, LocalizedField, PublicationVenue,
    ReferenceItem, ReferenceKind, ReferenceUrl, ResearchLocale, Timestamp,
};

use super::{
    generate_citekey, reference_from_minimal_metadata, resolve_citekey_collisions,
    ResearchExportFormat, ResearchImportFormat,
};

mod export;

pub use export::{export_references_text, render_plain_bibliography};

pub fn parse_references_text(
    format: ResearchImportFormat,
    text: &str,
    now: impl Into<String>,
) -> Result<Vec<ReferenceItem>, String> {
    let now = now.into();
    let mut references = match format {
        ResearchImportFormat::BibTex => parse_bibtex(text, &now),
        ResearchImportFormat::Ris => parse_ris(text, &now),
        ResearchImportFormat::CslJson => parse_csl_json(text, &now)?,
        ResearchImportFormat::EndNoteXml => parse_endnote_xml(text, &now),
        ResearchImportFormat::Doi | ResearchImportFormat::Isbn | ResearchImportFormat::Arxiv => {
            parse_identifier_text(format, text, &now)
        }
        _ => return Err(format!("text parser is not available for {:?}", format)),
    };
    resolve_citekey_collisions(&mut references);
    Ok(references)
}

fn parse_bibtex(text: &str, now: &str) -> Vec<ReferenceItem> {
    let mut references = Vec::new();
    let mut cursor = 0;
    let bytes = text.as_bytes();
    while let Some(relative_at) = text[cursor..].find('@') {
        let at = cursor + relative_at;
        let Some(open_relative) = text[at..].find(['{', '(']) else {
            break;
        };
        let open = at + open_relative;
        let close_char = if bytes[open] == b'{' { '}' } else { ')' };
        let kind_text = text[at + 1..open].trim();
        let Some(close) = find_matching_delimiter(text, open, close_char) else {
            break;
        };
        let body = &text[open + 1..close];
        if let Some((key, fields)) = parse_bibtex_body(body) {
            let mut reference = reference_from_fields(
                format!("bibtex-{key}"),
                map_bibtex_kind(kind_text),
                &fields,
                now,
            );
            reference.citekey = Some(Citekey::new(key));
            reference.metadata.source_format = Some("bibtex".to_string());
            references.push(reference);
        }
        cursor = close + 1;
    }
    references
}

fn parse_bibtex_body(body: &str) -> Option<(String, HashMap<String, String>)> {
    let parts = split_top_level(body, ',');
    let key = parts.first()?.trim().to_string();
    if key.is_empty() {
        return None;
    }
    let fields = parts
        .into_iter()
        .skip(1)
        .filter_map(|part| {
            let (name, value) = part.split_once('=')?;
            Some((
                name.trim().to_ascii_lowercase(),
                clean_bib_value(value.trim()),
            ))
        })
        .collect::<HashMap<_, _>>();
    Some((key, fields))
}

fn parse_ris(text: &str, now: &str) -> Vec<ReferenceItem> {
    let mut references = Vec::new();
    let mut record = Vec::<(String, String)>::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let tag = line.get(0..2).unwrap_or("").trim().to_ascii_uppercase();
        let value = line
            .split_once("  -")
            .map(|(_, value)| value.trim())
            .unwrap_or_else(|| line.get(6..).unwrap_or("").trim())
            .to_string();
        if tag == "ER" {
            if !record.is_empty() {
                references.push(reference_from_ris_record(&record, now));
                record.clear();
            }
        } else if !tag.is_empty() {
            record.push((tag, value));
        }
    }
    if !record.is_empty() {
        references.push(reference_from_ris_record(&record, now));
    }
    references
}

fn parse_csl_json(text: &str, now: &str) -> Result<Vec<ReferenceItem>, String> {
    let value: Value = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let items = match value {
        Value::Array(items) => items,
        Value::Object(_) => vec![value],
        _ => return Err("CSL JSON must be an object or array".to_string()),
    };
    Ok(items
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| reference_from_csl_value(index, item, now))
        .collect())
}

fn parse_endnote_xml(text: &str, now: &str) -> Vec<ReferenceItem> {
    xml_blocks(text, "record")
        .into_iter()
        .enumerate()
        .filter_map(|(index, record)| reference_from_endnote_record(index, &record, now))
        .collect()
}

fn parse_identifier_text(
    format: ResearchImportFormat,
    text: &str,
    now: &str,
) -> Vec<ReferenceItem> {
    text.lines()
        .flat_map(|line| line.split(','))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .enumerate()
        .map(|(index, value)| reference_from_identifier(format, value, index, now))
        .collect()
}

fn reference_from_fields(
    id: String,
    kind: ReferenceKind,
    fields: &HashMap<String, String>,
    now: &str,
) -> ReferenceItem {
    let title = fields
        .get("title")
        .cloned()
        .unwrap_or_else(|| "Untitled".to_string());
    let mut reference = reference_from_minimal_metadata(
        id,
        kind,
        title,
        fields
            .get("year")
            .or_else(|| fields.get("date"))
            .and_then(|value| parse_year(value)),
        now,
    );
    reference.creators = fields
        .get("author")
        .or_else(|| fields.get("editor"))
        .map(|authors| parse_creators(authors))
        .unwrap_or_default();
    reference.venue = fields
        .get("journal")
        .or_else(|| fields.get("journaltitle"))
        .or_else(|| fields.get("booktitle"))
        .or_else(|| fields.get("publisher"))
        .map(|venue| PublicationVenue {
            name: LocalizedField::plain(venue.clone()),
            volume: fields.get("volume").cloned(),
            issue: fields
                .get("number")
                .or_else(|| fields.get("issue"))
                .cloned(),
            pages: fields.get("pages").cloned(),
            publisher: fields.get("publisher").cloned().map(LocalizedField::plain),
        });
    reference.identifiers.doi = fields.get("doi").cloned();
    if let Some(isbn) = fields.get("isbn") {
        reference.identifiers.isbn.push(isbn.clone());
    }
    if let Some(arxiv) = fields.get("eprint").or_else(|| fields.get("arxiv")) {
        reference.identifiers.arxiv_id = Some(arxiv.clone());
    }
    if let Some(url) = fields.get("url") {
        reference.urls.push(ReferenceUrl {
            url: url.clone(),
            label: None,
            accessed_at: fields.get("urldate").cloned().map(Timestamp),
        });
    }
    if let Some(abstract_text) = fields.get("abstract") {
        reference.abstract_text = Some(LocalizedField::plain(abstract_text.clone()));
    }
    if let Some(language) = fields
        .get("language")
        .and_then(|value| ResearchLocale::parse(value))
    {
        reference.language = Some(language);
    }
    reference
}

fn reference_from_ris_record(record: &[(String, String)], now: &str) -> ReferenceItem {
    let values = |tag: &str| {
        record
            .iter()
            .filter_map(move |(candidate, value)| (candidate == tag).then_some(value.clone()))
            .collect::<Vec<_>>()
    };
    let first = |tags: &[&str]| {
        tags.iter()
            .find_map(|tag| record.iter().find(|(candidate, _)| candidate == tag))
            .map(|(_, value)| value.clone())
    };
    let kind = first(&["TY"])
        .as_deref()
        .map(map_ris_kind)
        .unwrap_or(ReferenceKind::Unknown);
    let mut fields = HashMap::new();
    if let Some(title) = first(&["TI", "T1"]) {
        fields.insert("title".to_string(), title);
    }
    if let Some(year) = first(&["PY", "Y1", "DA"]) {
        fields.insert("year".to_string(), year);
    }
    if let Some(doi) = first(&["DO"]) {
        fields.insert("doi".to_string(), doi);
    }
    if let Some(url) = first(&["UR"]) {
        fields.insert("url".to_string(), url);
    }
    if let Some(venue) = first(&["JO", "JF", "T2", "BT", "PB"]) {
        fields.insert("journal".to_string(), venue);
    }
    let mut reference = reference_from_fields(
        format!("ris-{}", stable_text_id(&format!("{:?}", record))),
        kind,
        &fields,
        now,
    );
    reference.creators = values("AU")
        .into_iter()
        .chain(values("A1"))
        .flat_map(|value| parse_creators(&value))
        .collect();
    reference.metadata.source_format = Some("ris".to_string());
    reference
}

fn reference_from_csl_value(index: usize, item: Value, now: &str) -> Option<ReferenceItem> {
    let object = item.as_object()?;
    let id = object
        .get("id")
        .and_then(Value::as_str)
        .map(|value| format!("csl-{value}"))
        .unwrap_or_else(|| format!("csl-{index}"));
    let title = object
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or("Untitled");
    let mut reference = reference_from_minimal_metadata(
        id,
        object
            .get("type")
            .and_then(Value::as_str)
            .map(map_csl_kind)
            .unwrap_or(ReferenceKind::Unknown),
        title,
        object.get("issued").and_then(extract_csl_year),
        now,
    );
    reference.creators = object
        .get("author")
        .and_then(Value::as_array)
        .map(|authors| {
            authors
                .iter()
                .filter_map(creator_from_csl_value)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    reference.identifiers.doi = object
        .get("DOI")
        .or_else(|| object.get("doi"))
        .and_then(Value::as_str)
        .map(str::to_string);
    if let Some(url) = object
        .get("URL")
        .or_else(|| object.get("url"))
        .and_then(Value::as_str)
    {
        reference.urls.push(ReferenceUrl {
            url: url.to_string(),
            label: None,
            accessed_at: None,
        });
    }
    reference.venue = object
        .get("container-title")
        .or_else(|| object.get("publisher"))
        .and_then(Value::as_str)
        .map(|venue| PublicationVenue {
            name: LocalizedField::plain(venue),
            volume: object
                .get("volume")
                .and_then(Value::as_str)
                .map(str::to_string),
            issue: object
                .get("issue")
                .and_then(Value::as_str)
                .map(str::to_string),
            pages: object
                .get("page")
                .and_then(Value::as_str)
                .map(str::to_string),
            publisher: object
                .get("publisher")
                .and_then(Value::as_str)
                .map(LocalizedField::plain),
        });
    reference.language = object
        .get("language")
        .and_then(Value::as_str)
        .and_then(ResearchLocale::parse);
    reference.metadata.source_format = Some("csl_json".to_string());
    Some(reference)
}

fn reference_from_endnote_record(index: usize, record: &str, now: &str) -> Option<ReferenceItem> {
    let title = xml_text_first(record, &["title", "short-title"]).unwrap_or_else(|| {
        xml_text_first(record, &["rec-number"])
            .map(|number| format!("Untitled EndNote record {number}"))
            .unwrap_or_else(|| "Untitled".to_string())
    });
    let kind = xml_text_first(record, &["ref-type"])
        .as_deref()
        .map(map_endnote_kind)
        .unwrap_or(ReferenceKind::Unknown);
    let id = xml_text_first(record, &["rec-number"])
        .map(|value| format!("endnote-{value}"))
        .unwrap_or_else(|| format!("endnote-{}", stable_text_id(record)));
    let mut fields = HashMap::new();
    fields.insert("title".to_string(), title);
    if let Some(year) = xml_text_first(record, &["year", "date"]) {
        fields.insert("year".to_string(), year);
    }
    if let Some(venue) = xml_text_first(record, &["secondary-title", "publisher"]) {
        fields.insert("journal".to_string(), venue);
    }
    if let Some(doi) = xml_text_first(record, &["electronic-resource-num", "doi"]) {
        fields.insert("doi".to_string(), doi);
    }
    if let Some(url) = xml_text_first(record, &["url"]) {
        fields.insert("url".to_string(), url);
    }
    if let Some(isbn) = xml_text_first(record, &["isbn"]) {
        fields.insert("isbn".to_string(), isbn);
    }
    if let Some(abstract_text) = xml_text_first(record, &["abstract"]) {
        fields.insert("abstract".to_string(), abstract_text);
    }
    if let Some(language) = xml_text_first(record, &["language"]) {
        fields.insert("language".to_string(), language);
    }

    let mut reference = reference_from_fields(id, kind, &fields, now);
    let authors = xml_texts(record, "author")
        .into_iter()
        .flat_map(|value| parse_creators(&value))
        .collect::<Vec<_>>();
    if !authors.is_empty() {
        reference.creators = authors;
    }
    reference.metadata.source_format = Some("endnote_xml".to_string());
    reference.metadata.extra.push(KeyValue {
        key: "endnote_record_index".to_string(),
        value: index.to_string(),
    });
    Some(reference)
}

fn reference_from_identifier(
    format: ResearchImportFormat,
    value: &str,
    index: usize,
    now: &str,
) -> ReferenceItem {
    let normalized = normalize_identifier_value(format, value);
    let mut reference = reference_from_minimal_metadata(
        format!(
            "{}-{}",
            identifier_prefix(format),
            stable_text_id(&normalized)
        ),
        ReferenceKind::Unknown,
        identifier_title(format, &normalized),
        None,
        now,
    );
    match format {
        ResearchImportFormat::Doi => {
            reference.identifiers.doi = Some(normalized);
            reference.kind = ReferenceKind::JournalArticle;
        }
        ResearchImportFormat::Isbn => {
            reference.identifiers.isbn.push(normalized);
            reference.kind = ReferenceKind::Book;
        }
        ResearchImportFormat::Arxiv => {
            reference.identifiers.arxiv_id = Some(normalized);
            reference.kind = ReferenceKind::Preprint;
        }
        _ => {}
    }
    reference.metadata.source_format = Some(identifier_prefix(format).to_string());
    reference.metadata.extra.push(KeyValue {
        key: "identifier_import_index".to_string(),
        value: index.to_string(),
    });
    reference
}

fn parse_creators(value: &str) -> Vec<Creator> {
    value
        .split(" and ")
        .flat_map(|part| part.split(';'))
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|name| {
            let (family, given, literal, name_order) =
                if let Some((family, given)) = name.split_once(',') {
                    (
                        Some(family.trim().to_string()),
                        non_empty(given.trim()),
                        None,
                        CreatorNameOrder::FamilyFirst,
                    )
                } else {
                    let mut parts = name.split_whitespace().collect::<Vec<_>>();
                    if parts.len() > 1 {
                        let family = parts.pop().map(str::to_string);
                        (
                            family,
                            Some(parts.join(" ")),
                            None,
                            CreatorNameOrder::GivenFirst,
                        )
                    } else {
                        (
                            None,
                            None,
                            Some(name.to_string()),
                            CreatorNameOrder::Literal,
                        )
                    }
                };
            Creator {
                role: CreatorRole::Author,
                given,
                family,
                literal,
                transliteration: None,
                sort_key: None,
                name_order,
                locale: None,
                orcid: None,
                affiliation: None,
            }
        })
        .collect()
}

fn creator_from_csl_value(value: &Value) -> Option<Creator> {
    let object = value.as_object()?;
    Some(Creator {
        role: CreatorRole::Author,
        given: object
            .get("given")
            .and_then(Value::as_str)
            .map(str::to_string),
        family: object
            .get("family")
            .and_then(Value::as_str)
            .map(str::to_string),
        literal: object
            .get("literal")
            .and_then(Value::as_str)
            .map(str::to_string),
        transliteration: None,
        sort_key: None,
        name_order: CreatorNameOrder::LocaleDefault,
        locale: None,
        orcid: None,
        affiliation: None,
    })
}

fn find_matching_delimiter(text: &str, open: usize, close_char: char) -> Option<usize> {
    let open_char = text[open..].chars().next()?;
    let mut depth = 0u32;
    let mut in_quote = false;
    let mut escaped = false;
    for (offset, ch) in text[open..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quote = !in_quote;
        }
        if in_quote {
            continue;
        }
        if ch == open_char {
            depth += 1;
        } else if ch == close_char {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(open + offset);
            }
        }
    }
    None
}

fn split_top_level(value: &str, delimiter: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_quote = false;
    let mut escaped = false;
    for ch in value.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            current.push(ch);
            escaped = true;
            continue;
        }
        match ch {
            '"' => {
                in_quote = !in_quote;
                current.push(ch);
            }
            '{' | '(' if !in_quote => {
                depth += 1;
                current.push(ch);
            }
            '}' | ')' if !in_quote => {
                depth -= 1;
                current.push(ch);
            }
            ch if ch == delimiter && depth == 0 && !in_quote => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

fn clean_bib_value(value: &str) -> String {
    let mut value = value.trim().trim_end_matches(',').trim().to_string();
    loop {
        let trimmed = value.trim();
        let unwrapped = if (trimmed.starts_with('{') && trimmed.ends_with('}'))
            || (trimmed.starts_with('"') && trimmed.ends_with('"'))
        {
            Some(trimmed[1..trimmed.len().saturating_sub(1)].to_string())
        } else {
            None
        };
        if let Some(next) = unwrapped {
            value = next;
        } else {
            break;
        }
    }
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_csl_year(value: &Value) -> Option<u16> {
    value
        .get("date-parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.first())
        .and_then(Value::as_array)
        .and_then(|parts| parts.first())
        .and_then(Value::as_u64)
        .and_then(|year| u16::try_from(year).ok())
}

fn parse_year(value: &str) -> Option<u16> {
    value
        .split(|ch: char| !ch.is_ascii_digit())
        .find(|part| part.len() == 4)
        .and_then(|part| part.parse::<u16>().ok())
}

fn map_bibtex_kind(kind: &str) -> ReferenceKind {
    match kind.trim().to_ascii_lowercase().as_str() {
        "article" => ReferenceKind::JournalArticle,
        "inproceedings" | "conference" => ReferenceKind::ConferencePaper,
        "book" => ReferenceKind::Book,
        "inbook" | "incollection" => ReferenceKind::BookSection,
        "phdthesis" | "mastersthesis" | "thesis" => ReferenceKind::Thesis,
        "techreport" | "report" => ReferenceKind::Report,
        "misc" | "online" => ReferenceKind::WebPage,
        _ => ReferenceKind::Unknown,
    }
}

fn map_ris_kind(kind: &str) -> ReferenceKind {
    match kind.trim().to_ascii_uppercase().as_str() {
        "JOUR" | "JFULL" => ReferenceKind::JournalArticle,
        "CONF" | "CPAPER" => ReferenceKind::ConferencePaper,
        "BOOK" => ReferenceKind::Book,
        "CHAP" => ReferenceKind::BookSection,
        "THES" => ReferenceKind::Thesis,
        "RPRT" => ReferenceKind::Report,
        "DATA" => ReferenceKind::Dataset,
        "WEB" => ReferenceKind::WebPage,
        _ => ReferenceKind::Unknown,
    }
}

fn map_csl_kind(kind: &str) -> ReferenceKind {
    match kind {
        "article-journal" => ReferenceKind::JournalArticle,
        "paper-conference" => ReferenceKind::ConferencePaper,
        "book" => ReferenceKind::Book,
        "chapter" => ReferenceKind::BookSection,
        "thesis" => ReferenceKind::Thesis,
        "report" => ReferenceKind::Report,
        "dataset" => ReferenceKind::Dataset,
        "webpage" => ReferenceKind::WebPage,
        _ => ReferenceKind::Unknown,
    }
}

fn identifier_prefix(format: ResearchImportFormat) -> &'static str {
    match format {
        ResearchImportFormat::Doi => "doi",
        ResearchImportFormat::Isbn => "isbn",
        ResearchImportFormat::Arxiv => "arxiv",
        _ => "identifier",
    }
}

fn identifier_title(format: ResearchImportFormat, value: &str) -> String {
    match format {
        ResearchImportFormat::Doi => format!("DOI {value}"),
        ResearchImportFormat::Isbn => format!("ISBN {value}"),
        ResearchImportFormat::Arxiv => format!("arXiv {value}"),
        _ => value.to_string(),
    }
}

fn normalize_identifier_value(format: ResearchImportFormat, value: &str) -> String {
    let value = value.trim();
    match format {
        ResearchImportFormat::Doi => value
            .trim_start_matches("https://doi.org/")
            .trim_start_matches("http://doi.org/")
            .trim_start_matches("doi:")
            .trim()
            .to_ascii_lowercase(),
        ResearchImportFormat::Isbn => value
            .chars()
            .filter(|ch| ch.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_uppercase(),
        ResearchImportFormat::Arxiv => value
            .trim_start_matches("https://arxiv.org/abs/")
            .trim_start_matches("http://arxiv.org/abs/")
            .trim_start_matches("arXiv:")
            .trim_start_matches("arxiv:")
            .trim()
            .to_string(),
        _ => value.to_string(),
    }
}

fn map_endnote_kind(kind: &str) -> ReferenceKind {
    let value = kind.trim().to_ascii_lowercase();
    match value.as_str() {
        "17" => ReferenceKind::JournalArticle,
        "10" => ReferenceKind::Book,
        "12" => ReferenceKind::BookSection,
        "32" => ReferenceKind::Thesis,
        _ if value.contains("journal") => ReferenceKind::JournalArticle,
        _ if value.contains("conference") || value.contains("proceeding") => {
            ReferenceKind::ConferencePaper
        }
        _ if value.contains("chapter") || value.contains("book section") => {
            ReferenceKind::BookSection
        }
        _ if value.contains("book") => ReferenceKind::Book,
        _ if value.contains("thesis") || value.contains("dissertation") => ReferenceKind::Thesis,
        _ if value.contains("report") => ReferenceKind::Report,
        _ if value.contains("dataset") => ReferenceKind::Dataset,
        _ if value.contains("web") => ReferenceKind::WebPage,
        _ => ReferenceKind::Unknown,
    }
}

fn xml_text_first(text: &str, names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|name| xml_texts(text, name).into_iter().next())
}

fn xml_blocks(text: &str, element_name: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut cursor = 0usize;
    while let Some((open_start, open_end)) = find_xml_start(text, element_name, cursor) {
        let close_tag = format!("</{}>", element_name.to_ascii_lowercase());
        let lower = text.to_ascii_lowercase();
        let Some(close_relative) = lower[open_end + 1..].find(&close_tag) else {
            break;
        };
        let close_start = open_end + 1 + close_relative;
        blocks.push(text[open_end + 1..close_start].to_string());
        cursor = close_start + close_tag.len();
        if cursor <= open_start {
            break;
        }
    }
    blocks
}

fn xml_texts(text: &str, element_name: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut cursor = 0usize;
    while let Some((open_start, open_end)) = find_xml_start(text, element_name, cursor) {
        if text[open_start..=open_end].trim_end().ends_with("/>") {
            values.push(String::new());
            cursor = open_end + 1;
            continue;
        }
        let close_tag = format!("</{}>", element_name.to_ascii_lowercase());
        let lower = text.to_ascii_lowercase();
        let Some(close_relative) = lower[open_end + 1..].find(&close_tag) else {
            break;
        };
        let close_start = open_end + 1 + close_relative;
        let raw_value = strip_xml_tags(&text[open_end + 1..close_start]);
        values.push(xml_unescape_text(raw_value.trim()));
        cursor = close_start + close_tag.len();
    }
    values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect()
}

fn find_xml_start(text: &str, element_name: &str, from: usize) -> Option<(usize, usize)> {
    let lower = text.to_ascii_lowercase();
    let element = element_name.to_ascii_lowercase();
    let mut cursor = from;
    while let Some(relative_start) = lower[cursor..].find(&format!("<{element}")) {
        let open_start = cursor + relative_start;
        let after_name = open_start + element.len() + 1;
        let next = lower[after_name..].chars().next();
        if !next.is_some_and(|ch| ch == '>' || ch == '/' || ch.is_ascii_whitespace()) {
            cursor = after_name;
            continue;
        }
        let open_end = lower[after_name..].find('>')? + after_name;
        return Some((open_start, open_end));
    }
    None
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

fn xml_escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn xml_unescape_text(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&amp;", "&")
}

fn stable_text_id(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn non_empty(value: &str) -> Option<String> {
    (!value.trim().is_empty()).then(|| value.trim().to_string())
}
