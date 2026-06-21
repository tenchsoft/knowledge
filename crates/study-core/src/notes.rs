mod anki;
mod reports;
mod types;

pub use anki::{
    export_study_cards_anki_apkg, export_study_cards_anki_package_zip,
    export_study_cards_anki_tench_package_zip, import_study_cards_anki_apkg,
    import_study_cards_anki_package_zip, import_study_cards_anki_tench_package_zip,
};
pub use reports::{
    build_exam_result_review, build_exam_session, build_study_progress_report_view,
    exam_timing_status, export_study_exam_report, export_study_progress_report, grade_exam_session,
    grade_exam_session_with_rubrics, study_progress_report,
};
pub use types::*;

use tench_document_core::TenchDocument;
use tench_search_core::normalize_search_text;

use crate::{ContentLocale, CurriculumNodeId, LearnerId, LocalizedText};

pub fn create_study_note(
    id: StudyNoteId,
    learner_id: LearnerId,
    node_id: CurriculumNodeId,
    title: LocalizedText,
    body_plain_text: impl AsRef<str>,
    now: impl Into<String>,
) -> Result<StudyNote, String> {
    if title.value.trim().is_empty() {
        return Err("note title is required".to_string());
    }
    let now = now.into();
    let mut document = TenchDocument::plain_text(body_plain_text.as_ref());
    document.metadata.title = title.value.clone();
    document.metadata.created_at = Some(now.clone());
    document.metadata.updated_at = Some(now.clone());
    Ok(StudyNote {
        id,
        learner_id,
        node_id,
        title,
        document,
        tags: Vec::new(),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn create_card_from_note(
    id: StudyCardId,
    deck_id: StudyDeckId,
    note: &StudyNote,
    kind: StudyCardKind,
    front: LocalizedText,
    back: LocalizedText,
    now: impl Into<String>,
) -> Result<StudyCard, String> {
    if front.value.trim().is_empty() || back.value.trim().is_empty() {
        return Err("card front and back are required".to_string());
    }
    let now = now.into();
    Ok(StudyCard {
        id,
        deck_id,
        node_id: note.node_id.clone(),
        source_note_id: Some(note.id.clone()),
        kind,
        front,
        back,
        tags: note.tags.clone(),
        media: StudyCardMedia::default(),
        created_at: now.clone(),
        updated_at: now,
    })
}

// Card creation requires many fields from the note and caller context.
#[allow(clippy::too_many_arguments)]
pub fn create_image_occlusion_card_from_note(
    id: StudyCardId,
    deck_id: StudyDeckId,
    note: &StudyNote,
    front: LocalizedText,
    back: LocalizedText,
    image_ref: String,
    occlusions: Vec<ImageOcclusionMask>,
    now: impl Into<String>,
) -> Result<StudyCard, String> {
    if image_ref.trim().is_empty() {
        return Err("image occlusion card requires an image reference".to_string());
    }
    if occlusions.is_empty() {
        return Err("image occlusion card requires at least one mask".to_string());
    }
    for mask in &occlusions {
        validate_occlusion_mask(mask)?;
    }
    let mut card = create_card_from_note(
        id,
        deck_id,
        note,
        StudyCardKind::ImageOcclusion,
        front,
        back,
        now,
    )?;
    card.media.image_ref = Some(image_ref);
    card.media.occlusions = occlusions;
    Ok(card)
}

pub fn create_audio_card_from_note(
    id: StudyCardId,
    deck_id: StudyDeckId,
    note: &StudyNote,
    front: LocalizedText,
    back: LocalizedText,
    audio_ref: String,
    now: impl Into<String>,
) -> Result<StudyCard, String> {
    if audio_ref.trim().is_empty() {
        return Err("audio card requires an audio reference".to_string());
    }
    let mut card =
        create_card_from_note(id, deck_id, note, StudyCardKind::Audio, front, back, now)?;
    card.media.audio_ref = Some(audio_ref);
    Ok(card)
}

pub fn create_code_card_from_note(
    id: StudyCardId,
    deck_id: StudyDeckId,
    note: &StudyNote,
    front: LocalizedText,
    back: LocalizedText,
    code: StudyCardCodePayload,
    now: impl Into<String>,
) -> Result<StudyCard, String> {
    if code.language.trim().is_empty() || code.code.trim().is_empty() {
        return Err("code card requires language and code".to_string());
    }
    let mut card = create_card_from_note(id, deck_id, note, StudyCardKind::Code, front, back, now)?;
    card.media.code = Some(code);
    Ok(card)
}

pub fn extract_cloze_cards_from_note(
    deck_id: StudyDeckId,
    note: &StudyNote,
    now: impl Into<String>,
) -> Vec<StudyCard> {
    let now = now.into();
    let text = note.document.to_plain_text();
    let mut cards = Vec::new();
    let mut cursor = 0;
    let mut index = 0;
    while let Some(start) = text[cursor..].find("{{c") {
        let start = cursor + start;
        let Some(split) = text[start..].find("::") else {
            break;
        };
        let answer_start = start + split + 2;
        let Some(end) = text[answer_start..].find("}}") else {
            break;
        };
        let answer_end = answer_start + end;
        let answer = text[answer_start..answer_end].trim();
        if !answer.is_empty() {
            let mut prompt = text.clone();
            prompt.replace_range(start..answer_end + 2, "[...]");
            cards.push(StudyCard {
                id: StudyCardId::from(format!("{}-cloze-{index}", note.id.as_str())),
                deck_id: deck_id.clone(),
                node_id: note.node_id.clone(),
                source_note_id: Some(note.id.clone()),
                kind: StudyCardKind::Cloze,
                front: LocalizedText::plain(prompt),
                back: LocalizedText::plain(answer),
                tags: note.tags.clone(),
                media: StudyCardMedia::default(),
                created_at: now.clone(),
                updated_at: now.clone(),
            });
            index += 1;
        }
        cursor = answer_end + 2;
    }
    cards
}

pub fn export_study_cards(
    format: StudyCardExchangeFormat,
    cards: &[StudyCard],
) -> Result<String, String> {
    match format {
        StudyCardExchangeFormat::Json => {
            serde_json::to_string_pretty(cards).map_err(|error| error.to_string())
        }
        StudyCardExchangeFormat::AnkiTsv => Ok(cards
            .iter()
            .map(|card| {
                format!(
                    "{}\t{}",
                    sanitize_line(&card.front.value),
                    sanitize_line(&card.back.value)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")),
        StudyCardExchangeFormat::Tsv => Ok(cards
            .iter()
            .map(|card| {
                format!(
                    "{}\t{}\t{}",
                    sanitize_line(&card.front.value),
                    sanitize_line(&card.back.value),
                    sanitize_line(&card.tags.join(","))
                )
            })
            .collect::<Vec<_>>()
            .join("\n")),
        StudyCardExchangeFormat::Csv => Ok(cards
            .iter()
            .map(|card| {
                format!(
                    "\"{}\",\"{}\"",
                    escape_csv(&card.front.value),
                    escape_csv(&card.back.value)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")),
        StudyCardExchangeFormat::Markdown => Ok(cards
            .iter()
            .map(|card| format!("## {}\n\n{}\n", card.front.value, card.back.value))
            .collect::<Vec<_>>()
            .join("\n")),
    }
}

pub fn import_study_cards(
    format: StudyCardExchangeFormat,
    deck_id: StudyDeckId,
    node_id: CurriculumNodeId,
    locale: Option<ContentLocale>,
    text: &str,
    now: impl Into<String>,
) -> Result<Vec<StudyCard>, String> {
    let now = now.into();
    match format {
        StudyCardExchangeFormat::Json => {
            serde_json::from_str(text).map_err(|error| error.to_string())
        }
        StudyCardExchangeFormat::AnkiTsv | StudyCardExchangeFormat::Tsv => Ok(text
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let (front, back) = line.split_once('\t')?;
                let mut card =
                    imported_card(&deck_id, &node_id, locale.clone(), index, front, back, &now);
                if format == StudyCardExchangeFormat::Tsv {
                    card.tags = line.split('\t').nth(2).map(split_tags).unwrap_or_default();
                }
                Some(card)
            })
            .collect()),
        StudyCardExchangeFormat::Csv => Ok(text
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let columns = parse_simple_csv_line(line);
                (columns.len() >= 2).then(|| {
                    imported_card(
                        &deck_id,
                        &node_id,
                        locale.clone(),
                        index,
                        &columns[0],
                        &columns[1],
                        &now,
                    )
                })
            })
            .collect()),
        StudyCardExchangeFormat::Markdown => {
            let mut cards = Vec::new();
            let mut front = None;
            let mut back = Vec::new();
            for line in text.lines() {
                if let Some(title) = line.strip_prefix("## ") {
                    if let Some(previous_front) = front.take() {
                        cards.push(imported_card(
                            &deck_id,
                            &node_id,
                            locale.clone(),
                            cards.len(),
                            previous_front,
                            back.join("\n"),
                            &now,
                        ));
                        back.clear();
                    }
                    front = Some(title.trim().to_string());
                } else if front.is_some() {
                    back.push(line.to_string());
                }
            }
            if let Some(previous_front) = front {
                cards.push(imported_card(
                    &deck_id,
                    &node_id,
                    locale,
                    cards.len(),
                    previous_front,
                    back.join("\n"),
                    &now,
                ));
            }
            Ok(cards)
        }
    }
}

pub fn find_duplicate_study_cards(cards: &[StudyCard]) -> Vec<StudyDuplicateCandidate> {
    let mut candidates = Vec::new();
    for (index, card) in cards.iter().enumerate() {
        let key = study_card_duplicate_key(card);
        if let Some(existing) = cards[..index]
            .iter()
            .find(|existing| study_card_duplicate_key(existing) == key)
        {
            candidates.push(StudyDuplicateCandidate {
                existing_id: existing.id.as_str().to_string(),
                duplicate_id: card.id.as_str().to_string(),
                reason: "same normalized card front, back, deck, node, and kind".to_string(),
                confidence: 100,
            });
        }
    }
    candidates
}

pub fn cleanup_imported_study_cards(
    existing_cards: &[StudyCard],
    imported_cards: Vec<StudyCard>,
) -> StudyCardImportCleanupReport {
    let mut kept = Vec::new();
    let mut duplicates = Vec::new();
    let mut known = existing_cards
        .iter()
        .map(|card| (study_card_duplicate_key(card), card.id.as_str().to_string()))
        .collect::<Vec<_>>();

    for card in imported_cards {
        let key = study_card_duplicate_key(&card);
        if let Some((_, existing_id)) = known.iter().find(|(known_key, _)| *known_key == key) {
            duplicates.push(StudyDuplicateCandidate {
                existing_id: existing_id.clone(),
                duplicate_id: card.id.as_str().to_string(),
                reason: "same normalized card front, back, deck, node, and kind".to_string(),
                confidence: 100,
            });
            continue;
        }

        known.push((key, card.id.as_str().to_string()));
        kept.push(card);
    }

    StudyCardImportCleanupReport {
        removed_count: duplicates.len() as u32,
        cards: kept,
        duplicates,
    }
}

pub fn find_duplicate_study_notes(notes: &[StudyNote]) -> Vec<StudyDuplicateCandidate> {
    let mut candidates = Vec::new();
    for (index, note) in notes.iter().enumerate() {
        let key = study_note_duplicate_key(note);
        if let Some(existing) = notes[..index]
            .iter()
            .find(|existing| study_note_duplicate_key(existing) == key)
        {
            candidates.push(StudyDuplicateCandidate {
                existing_id: existing.id.as_str().to_string(),
                duplicate_id: note.id.as_str().to_string(),
                reason: "same normalized note title, body, learner, and node".to_string(),
                confidence: 100,
            });
        }
    }
    candidates
}

pub fn cleanup_imported_study_notes(
    existing_notes: &[StudyNote],
    imported_notes: Vec<StudyNote>,
) -> StudyNoteImportCleanupReport {
    let mut kept = Vec::new();
    let mut duplicates = Vec::new();
    let mut known = existing_notes
        .iter()
        .map(|note| (study_note_duplicate_key(note), note.id.as_str().to_string()))
        .collect::<Vec<_>>();

    for note in imported_notes {
        let key = study_note_duplicate_key(&note);
        if let Some((_, existing_id)) = known.iter().find(|(known_key, _)| *known_key == key) {
            duplicates.push(StudyDuplicateCandidate {
                existing_id: existing_id.clone(),
                duplicate_id: note.id.as_str().to_string(),
                reason: "same normalized note title, body, learner, and node".to_string(),
                confidence: 100,
            });
            continue;
        }

        known.push((key, note.id.as_str().to_string()));
        kept.push(note);
    }

    StudyNoteImportCleanupReport {
        removed_count: duplicates.len() as u32,
        notes: kept,
        duplicates,
    }
}

pub fn export_study_notes(
    format: StudyNoteExchangeFormat,
    notes: &[StudyNote],
) -> Result<String, String> {
    match format {
        StudyNoteExchangeFormat::Json => {
            serde_json::to_string_pretty(notes).map_err(|error| error.to_string())
        }
        StudyNoteExchangeFormat::PlainText => Ok(notes
            .iter()
            .map(|note| {
                format!(
                    "{}\n{}\n",
                    sanitize_line(&note.title.value),
                    note.document.to_plain_text()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")),
        StudyNoteExchangeFormat::Markdown => Ok(notes
            .iter()
            .map(|note| {
                let locale = note
                    .title
                    .locale
                    .as_ref()
                    .map(ContentLocale::bcp47)
                    .unwrap_or_default();
                format!(
                    concat!(
                        "# {title}\n",
                        "<!-- tench-study-note id=\"{id}\" node=\"{node}\" locale=\"{locale}\" tags=\"{tags}\" -->\n\n",
                        "{body}\n"
                    ),
                    title = note.title.value,
                    id = note.id.as_str(),
                    node = note.node_id.as_str(),
                    locale = locale,
                    tags = note.tags.join(","),
                    body = note.document.to_plain_text()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")),
    }
}

pub fn import_study_notes(
    format: StudyNoteExchangeFormat,
    learner_id: LearnerId,
    node_id: CurriculumNodeId,
    locale: Option<ContentLocale>,
    text: &str,
    now: impl Into<String>,
) -> Result<Vec<StudyNote>, String> {
    let now = now.into();
    match format {
        StudyNoteExchangeFormat::Json => {
            serde_json::from_str(text).map_err(|error| error.to_string())
        }
        StudyNoteExchangeFormat::PlainText => {
            let title = text.lines().next().unwrap_or("Imported note").trim();
            let title = if title.is_empty() {
                "Imported note"
            } else {
                title
            };
            Ok(vec![imported_note(
                &learner_id,
                &node_id,
                locale,
                0,
                title,
                text,
                &now,
            )?])
        }
        StudyNoteExchangeFormat::Markdown => {
            import_markdown_notes(&learner_id, &node_id, locale, text, &now)
        }
    }
}

fn imported_card(
    deck_id: &StudyDeckId,
    node_id: &CurriculumNodeId,
    locale: Option<ContentLocale>,
    index: usize,
    front: impl AsRef<str>,
    back: impl AsRef<str>,
    now: &str,
) -> StudyCard {
    StudyCard {
        id: StudyCardId::from(format!("imported-card-{index}")),
        deck_id: deck_id.clone(),
        node_id: node_id.clone(),
        source_note_id: None,
        kind: StudyCardKind::Basic,
        front: LocalizedText {
            value: front.as_ref().trim().to_string(),
            locale: locale.clone(),
            source_locale: None,
            machine_translated: false,
        },
        back: LocalizedText {
            value: back.as_ref().trim().to_string(),
            locale,
            source_locale: None,
            machine_translated: false,
        },
        tags: Vec::new(),
        media: StudyCardMedia::default(),
        created_at: now.to_string(),
        updated_at: now.to_string(),
    }
}

fn study_card_duplicate_key(card: &StudyCard) -> String {
    format!(
        "{}|{}|{:?}|{}|{}|{}|{}|{}|{}",
        card.deck_id.as_str(),
        card.node_id.as_str(),
        card.kind,
        normalize_search_text(&card.front.value),
        normalize_search_text(&card.back.value),
        card.media.image_ref.as_deref().unwrap_or_default(),
        card.media.audio_ref.as_deref().unwrap_or_default(),
        card.media
            .code
            .as_ref()
            .map(|code| format!("{}:{}", code.language, normalize_search_text(&code.code)))
            .unwrap_or_default(),
        card.media.occlusions.len()
    )
}

fn study_note_duplicate_key(note: &StudyNote) -> String {
    format!(
        "{}|{}|{}|{}",
        note.learner_id.as_str(),
        note.node_id.as_str(),
        normalize_search_text(&note.title.value),
        normalize_search_text(&note.document.to_plain_text())
    )
}

fn validate_occlusion_mask(mask: &ImageOcclusionMask) -> Result<(), String> {
    if mask.id.trim().is_empty() {
        return Err("image occlusion mask requires an id".to_string());
    }
    if !mask.x.is_finite()
        || !mask.y.is_finite()
        || !mask.width.is_finite()
        || !mask.height.is_finite()
    {
        return Err(format!(
            "image occlusion mask {} has invalid geometry",
            mask.id
        ));
    }
    if mask.width <= 0.0 || mask.height <= 0.0 {
        return Err(format!(
            "image occlusion mask {} requires positive size",
            mask.id
        ));
    }
    Ok(())
}

fn imported_note(
    learner_id: &LearnerId,
    node_id: &CurriculumNodeId,
    locale: Option<ContentLocale>,
    index: usize,
    title: impl AsRef<str>,
    body: impl AsRef<str>,
    now: &str,
) -> Result<StudyNote, String> {
    create_study_note(
        StudyNoteId::from(format!("imported-note-{index}")),
        learner_id.clone(),
        node_id.clone(),
        LocalizedText {
            value: title.as_ref().trim().to_string(),
            locale,
            source_locale: None,
            machine_translated: false,
        },
        body.as_ref(),
        now.to_string(),
    )
}

fn import_markdown_notes(
    learner_id: &LearnerId,
    node_id: &CurriculumNodeId,
    locale: Option<ContentLocale>,
    text: &str,
    now: &str,
) -> Result<Vec<StudyNote>, String> {
    let mut notes = Vec::new();
    let mut title = None::<String>;
    let mut body = Vec::<String>::new();

    for line in text.lines() {
        if let Some(next_title) = line.strip_prefix("# ") {
            if let Some(previous_title) = title.take() {
                notes.push(imported_note(
                    learner_id,
                    node_id,
                    locale.clone(),
                    notes.len(),
                    previous_title,
                    body.join("\n"),
                    now,
                )?);
                body.clear();
            }
            title = Some(next_title.trim().to_string());
        } else if title.is_some() && !line.trim_start().starts_with("<!-- tench-study-note") {
            body.push(line.to_string());
        }
    }

    if let Some(previous_title) = title {
        notes.push(imported_note(
            learner_id,
            node_id,
            locale,
            notes.len(),
            previous_title,
            body.join("\n"),
            now,
        )?);
    }

    Ok(notes)
}

fn sanitize_line(value: &str) -> String {
    value.replace(['\n', '\t'], " ")
}

fn escape_csv(value: &str) -> String {
    value.replace('"', "\"\"").replace('\n', " ")
}

fn escape_csv_cell(value: &str) -> String {
    let escaped = value.replace('"', "\"\"");
    if escaped.contains(',') || escaped.contains('"') || escaped.contains('\n') {
        format!("\"{escaped}\"")
    } else {
        escaped
    }
}

fn split_tags(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_simple_csv_line(line: &str) -> Vec<String> {
    let mut columns = Vec::new();
    let mut current = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                current.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                columns.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    columns.push(current.trim().to_string());
    columns
}

#[cfg(test)]
mod tests;
