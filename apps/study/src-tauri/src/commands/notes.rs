#[tauri::command]
pub fn create_study_note(
    id: tench_study_core::StudyNoteId,
    learner_id: tench_study_core::LearnerId,
    node_id: tench_study_core::CurriculumNodeId,
    title: tench_study_core::LocalizedText,
    body_plain_text: String,
    now: String,
) -> Result<tench_study_core::StudyNote, String> {
    tench_study_core::create_study_note(id, learner_id, node_id, title, body_plain_text, now)
}

#[tauri::command]
pub fn create_study_card_from_note(
    id: tench_study_core::StudyCardId,
    deck_id: tench_study_core::StudyDeckId,
    note: tench_study_core::StudyNote,
    kind: tench_study_core::StudyCardKind,
    front: tench_study_core::LocalizedText,
    back: tench_study_core::LocalizedText,
    now: String,
) -> Result<tench_study_core::StudyCard, String> {
    tench_study_core::create_card_from_note(id, deck_id, &note, kind, front, back, now)
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub fn create_study_image_occlusion_card_from_note(
    id: tench_study_core::StudyCardId,
    deck_id: tench_study_core::StudyDeckId,
    note: tench_study_core::StudyNote,
    front: tench_study_core::LocalizedText,
    back: tench_study_core::LocalizedText,
    image_ref: String,
    occlusions: Vec<tench_study_core::ImageOcclusionMask>,
    now: String,
) -> Result<tench_study_core::StudyCard, String> {
    tench_study_core::create_image_occlusion_card_from_note(
        id, deck_id, &note, front, back, image_ref, occlusions, now,
    )
}

#[tauri::command]
pub fn create_study_audio_card_from_note(
    id: tench_study_core::StudyCardId,
    deck_id: tench_study_core::StudyDeckId,
    note: tench_study_core::StudyNote,
    front: tench_study_core::LocalizedText,
    back: tench_study_core::LocalizedText,
    audio_ref: String,
    now: String,
) -> Result<tench_study_core::StudyCard, String> {
    tench_study_core::create_audio_card_from_note(id, deck_id, &note, front, back, audio_ref, now)
}

#[tauri::command]
pub fn create_study_code_card_from_note(
    id: tench_study_core::StudyCardId,
    deck_id: tench_study_core::StudyDeckId,
    note: tench_study_core::StudyNote,
    front: tench_study_core::LocalizedText,
    back: tench_study_core::LocalizedText,
    code: tench_study_core::StudyCardCodePayload,
    now: String,
) -> Result<tench_study_core::StudyCard, String> {
    tench_study_core::create_code_card_from_note(id, deck_id, &note, front, back, code, now)
}

#[tauri::command]
pub fn extract_study_cloze_cards_from_note(
    deck_id: tench_study_core::StudyDeckId,
    note: tench_study_core::StudyNote,
    now: String,
) -> Vec<tench_study_core::StudyCard> {
    tench_study_core::extract_cloze_cards_from_note(deck_id, &note, now)
}

#[tauri::command]
pub fn export_study_cards(
    format: tench_study_core::StudyCardExchangeFormat,
    cards: Vec<tench_study_core::StudyCard>,
) -> Result<String, String> {
    tench_study_core::export_study_cards(format, &cards)
}

#[tauri::command]
pub fn import_study_cards(
    format: tench_study_core::StudyCardExchangeFormat,
    deck_id: tench_study_core::StudyDeckId,
    node_id: tench_study_core::CurriculumNodeId,
    locale: Option<tench_study_core::ContentLocale>,
    text: String,
    now: String,
) -> Result<Vec<tench_study_core::StudyCard>, String> {
    tench_study_core::import_study_cards(format, deck_id, node_id, locale, &text, now)
}

#[tauri::command]
pub fn find_duplicate_study_cards(
    cards: Vec<tench_study_core::StudyCard>,
) -> Vec<tench_study_core::StudyDuplicateCandidate> {
    tench_study_core::find_duplicate_study_cards(&cards)
}

#[tauri::command]
pub fn cleanup_imported_study_cards(
    existing_cards: Vec<tench_study_core::StudyCard>,
    imported_cards: Vec<tench_study_core::StudyCard>,
) -> tench_study_core::StudyCardImportCleanupReport {
    tench_study_core::cleanup_imported_study_cards(&existing_cards, imported_cards)
}

#[tauri::command]
pub fn export_study_cards_anki_package_zip(
    cards: Vec<tench_study_core::StudyCard>,
) -> Result<Vec<u8>, String> {
    tench_study_core::export_study_cards_anki_package_zip(&cards)
}

#[tauri::command]
pub fn export_study_cards_anki_apkg(
    cards: Vec<tench_study_core::StudyCard>,
) -> Result<Vec<u8>, String> {
    tench_study_core::export_study_cards_anki_apkg(&cards)
}

#[tauri::command]
pub fn import_study_cards_anki_package_zip(
    bytes: Vec<u8>,
    deck_id: tench_study_core::StudyDeckId,
    node_id: tench_study_core::CurriculumNodeId,
    locale: Option<tench_study_core::ContentLocale>,
    now: String,
) -> Result<Vec<tench_study_core::StudyCard>, String> {
    tench_study_core::import_study_cards_anki_package_zip(&bytes, deck_id, node_id, locale, now)
}

#[tauri::command]
pub fn import_study_cards_anki_apkg(
    bytes: Vec<u8>,
    deck_id: tench_study_core::StudyDeckId,
    node_id: tench_study_core::CurriculumNodeId,
    locale: Option<tench_study_core::ContentLocale>,
    now: String,
) -> Result<Vec<tench_study_core::StudyCard>, String> {
    tench_study_core::import_study_cards_anki_apkg(&bytes, deck_id, node_id, locale, now)
}

#[tauri::command]
pub fn export_study_notes(
    format: tench_study_core::StudyNoteExchangeFormat,
    notes: Vec<tench_study_core::StudyNote>,
) -> Result<String, String> {
    tench_study_core::export_study_notes(format, &notes)
}

#[tauri::command]
pub fn import_study_notes(
    format: tench_study_core::StudyNoteExchangeFormat,
    learner_id: tench_study_core::LearnerId,
    node_id: tench_study_core::CurriculumNodeId,
    locale: Option<tench_study_core::ContentLocale>,
    text: String,
    now: String,
) -> Result<Vec<tench_study_core::StudyNote>, String> {
    tench_study_core::import_study_notes(format, learner_id, node_id, locale, &text, now)
}

#[tauri::command]
pub fn find_duplicate_study_notes(
    notes: Vec<tench_study_core::StudyNote>,
) -> Vec<tench_study_core::StudyDuplicateCandidate> {
    tench_study_core::find_duplicate_study_notes(&notes)
}

#[tauri::command]
pub fn cleanup_imported_study_notes(
    existing_notes: Vec<tench_study_core::StudyNote>,
    imported_notes: Vec<tench_study_core::StudyNote>,
) -> tench_study_core::StudyNoteImportCleanupReport {
    tench_study_core::cleanup_imported_study_notes(&existing_notes, imported_notes)
}

#[tauri::command]
pub fn export_study_progress_report(
    format: tench_study_core::StudyProgressExportFormat,
    progress: Vec<tench_study_core::LearnerProgress>,
    generated_at: String,
) -> Result<String, String> {
    tench_study_core::export_study_progress_report(format, &progress, generated_at)
}

#[tauri::command]
pub fn build_study_progress_report_view(
    progress: Vec<tench_study_core::LearnerProgress>,
    generated_at: String,
) -> tench_study_core::StudyProgressReportView {
    tench_study_core::build_study_progress_report_view(&progress, generated_at)
}
