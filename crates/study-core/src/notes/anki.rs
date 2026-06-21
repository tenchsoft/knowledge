use rusqlite::{params, Connection};
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use crate::{ContentLocale, CurriculumNodeId};

use super::{
    export_study_cards, import_study_cards, imported_card, StudyCard, StudyCardExchangeFormat,
    StudyCardId, StudyDeckId,
};

pub fn export_study_cards_anki_package_zip(cards: &[StudyCard]) -> Result<Vec<u8>, String> {
    export_study_cards_anki_apkg(cards)
}

pub fn export_study_cards_anki_apkg(cards: &[StudyCard]) -> Result<Vec<u8>, String> {
    let collection_path = write_anki_collection_sqlite(cards)?;
    let collection_bytes = fs::read(&collection_path).map_err(|error| error.to_string())?;
    let _ = fs::remove_file(&collection_path);

    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    writer
        .start_file("collection.anki2", options)
        .map_err(|error| error.to_string())?;
    writer
        .write_all(&collection_bytes)
        .map_err(|error| error.to_string())?;
    writer
        .start_file("media", options)
        .map_err(|error| error.to_string())?;
    writer.write_all(b"{}").map_err(|error| error.to_string())?;
    writer
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| error.to_string())
}

pub fn export_study_cards_anki_tench_package_zip(cards: &[StudyCard]) -> Result<Vec<u8>, String> {
    let cards_tsv = export_study_cards(StudyCardExchangeFormat::AnkiTsv, cards)?;
    let manifest = serde_json::json!({
        "schema_version": 1,
        "format": "tench-study-anki-package",
        "card_count": cards.len(),
    });
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    writer
        .start_file("cards.tsv", options)
        .map_err(|error| error.to_string())?;
    writer
        .write_all(cards_tsv.as_bytes())
        .map_err(|error| error.to_string())?;
    writer
        .start_file("manifest.json", options)
        .map_err(|error| error.to_string())?;
    writer
        .write_all(manifest.to_string().as_bytes())
        .map_err(|error| error.to_string())?;
    writer
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| error.to_string())
}

pub fn import_study_cards_anki_package_zip(
    bytes: &[u8],
    deck_id: StudyDeckId,
    node_id: CurriculumNodeId,
    locale: Option<ContentLocale>,
    now: impl Into<String>,
) -> Result<Vec<StudyCard>, String> {
    let now = now.into();
    import_study_cards_anki_apkg(
        bytes,
        deck_id.clone(),
        node_id.clone(),
        locale.clone(),
        now.clone(),
    )
    .or_else(|_| import_study_cards_anki_tench_package_zip(bytes, deck_id, node_id, locale, now))
}

pub fn import_study_cards_anki_apkg(
    bytes: &[u8],
    deck_id: StudyDeckId,
    node_id: CurriculumNodeId,
    locale: Option<ContentLocale>,
    now: impl Into<String>,
) -> Result<Vec<StudyCard>, String> {
    let collection_bytes = read_anki_collection_from_package(bytes)?;
    let collection_path = temp_anki_collection_path("import");
    fs::write(&collection_path, collection_bytes).map_err(|error| error.to_string())?;
    let cards = read_anki_collection_sqlite(&collection_path, deck_id, node_id, locale, now);
    let _ = fs::remove_file(&collection_path);
    cards
}

pub fn import_study_cards_anki_tench_package_zip(
    bytes: &[u8],
    deck_id: StudyDeckId,
    node_id: CurriculumNodeId,
    locale: Option<ContentLocale>,
    now: impl Into<String>,
) -> Result<Vec<StudyCard>, String> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|error| error.to_string())?;
    crate::storage::check_study_archive_limits(&mut archive)?;
    let mut cards_tsv = String::new();
    archive
        .by_name("cards.tsv")
        .map_err(|error| format!("missing cards.tsv: {error}"))?
        .read_to_string(&mut cards_tsv)
        .map_err(|error| error.to_string())?;
    import_study_cards(
        StudyCardExchangeFormat::AnkiTsv,
        deck_id,
        node_id,
        locale,
        &cards_tsv,
        now,
    )
}

fn write_anki_collection_sqlite(cards: &[StudyCard]) -> Result<PathBuf, String> {
    let path = temp_anki_collection_path("export");
    let connection = Connection::open(&path).map_err(|error| error.to_string())?;
    create_anki_schema(&connection)?;
    insert_anki_collection_metadata(&connection)?;

    let deck_id = anki_deck_id();
    let model_id = anki_basic_model_id();
    let now_seconds = anki_now_seconds();
    let base_id = anki_now_millis();
    for (index, card) in cards.iter().enumerate() {
        let note_id = base_id + index as i64 + 1;
        let card_id = base_id + cards.len() as i64 + index as i64 + 1;
        let fields = format!(
            "{}\u{1f}{}",
            anki_field_text(&card.front.value),
            anki_field_text(&card.back.value)
        );
        let tags = anki_tags(&card.tags);
        connection
            .execute(
                concat!(
                    "insert into notes ",
                    "(id, guid, mid, mod, usn, tags, flds, sfld, csum, flags, data) ",
                    "values (?1, ?2, ?3, ?4, -1, ?5, ?6, ?7, ?8, 0, '')"
                ),
                params![
                    note_id,
                    format!("tench{note_id}"),
                    model_id,
                    now_seconds,
                    tags,
                    fields,
                    anki_field_text(&card.front.value),
                    anki_sort_checksum(&card.front.value),
                ],
            )
            .map_err(|error| error.to_string())?;
        connection
            .execute(
                concat!(
                    "insert into cards ",
                    "(id, nid, did, ord, mod, usn, type, queue, due, ivl, factor, reps, lapses, ",
                    "left, odue, odid, flags, data) ",
                    "values (?1, ?2, ?3, 0, ?4, -1, 0, 0, ?5, 0, 0, 0, 0, 0, 0, 0, 0, '')"
                ),
                params![card_id, note_id, deck_id, now_seconds, index as i64 + 1],
            )
            .map_err(|error| error.to_string())?;
    }
    drop(connection);
    Ok(path)
}

fn create_anki_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(concat!(
            "pragma journal_mode=off;",
            "create table col (",
            "id integer primary key, crt integer not null, mod integer not null, ",
            "scm integer not null, ver integer not null, dty integer not null, ",
            "usn integer not null, ls integer not null, conf text not null, ",
            "models text not null, decks text not null, dconf text not null, tags text not null);",
            "create table notes (",
            "id integer primary key, guid text not null, mid integer not null, ",
            "mod integer not null, usn integer not null, tags text not null, ",
            "flds text not null, sfld text not null, csum integer not null, ",
            "flags integer not null, data text not null);",
            "create table cards (",
            "id integer primary key, nid integer not null, did integer not null, ",
            "ord integer not null, mod integer not null, usn integer not null, ",
            "type integer not null, queue integer not null, due integer not null, ",
            "ivl integer not null, factor integer not null, reps integer not null, ",
            "lapses integer not null, left integer not null, odue integer not null, ",
            "odid integer not null, flags integer not null, data text not null);",
            "create table revlog (",
            "id integer primary key, cid integer not null, usn integer not null, ",
            "ease integer not null, ivl integer not null, lastIvl integer not null, ",
            "factor integer not null, time integer not null, type integer not null);",
            "create table graves (",
            "usn integer not null, oid integer not null, type integer not null);",
            "create index ix_notes_usn on notes (usn);",
            "create index ix_cards_usn on cards (usn);",
            "create index ix_cards_nid on cards (nid);",
            "create index ix_cards_sched on cards (did, queue, due);"
        ))
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn insert_anki_collection_metadata(connection: &Connection) -> Result<(), String> {
    let deck_id = anki_deck_id();
    let model_id = anki_basic_model_id();
    let now_seconds = anki_now_seconds();
    let now_millis = anki_now_millis();
    let mut decks = serde_json::Map::new();
    decks.insert(
        deck_id.to_string(),
        serde_json::json!({
            "id": deck_id,
            "mod": now_seconds,
            "name": "Tench Study",
            "usn": -1,
            "lrnToday": [0, 0],
            "revToday": [0, 0],
            "newToday": [0, 0],
            "timeToday": [0, 0],
            "conf": 1,
            "desc": "Exported from Tench Study",
            "dyn": 0,
            "collapsed": false,
            "browserCollapsed": false,
            "extendNew": 0,
            "extendRev": 0
        }),
    );
    let mut models = serde_json::Map::new();
    models.insert(
        model_id.to_string(),
        serde_json::json!({
            "id": model_id,
            "name": "Tench Basic",
            "type": 0,
            "mod": now_seconds,
            "usn": -1,
            "sortf": 0,
            "did": deck_id,
            "tmpls": [{
                "name": "Card 1",
                "ord": 0,
                "qfmt": "{{Front}}",
                "afmt": "{{FrontSide}}<hr id=answer>{{Back}}",
                "did": null,
                "bqfmt": "",
                "bafmt": ""
            }],
            "flds": [
                {"name": "Front", "ord": 0, "sticky": false, "rtl": false, "font": "Arial", "size": 20},
                {"name": "Back", "ord": 1, "sticky": false, "rtl": false, "font": "Arial", "size": 20}
            ],
            "css": ".card { font-family: arial; font-size: 20px; text-align: center; color: black; background-color: white; }",
            "req": [[0, "all", [0]]],
            "tags": []
        }),
    );
    let conf = serde_json::json!({
        "nextPos": 1,
        "estTimes": true,
        "activeDecks": [deck_id],
        "sortType": "noteFld",
        "timeLim": 0
    });
    let mut dconf = serde_json::Map::new();
    dconf.insert(
        "1".to_string(),
        serde_json::json!({
            "id": 1,
            "name": "Default",
            "mod": now_seconds,
            "usn": -1,
            "maxTaken": 60,
            "autoplay": true,
            "timer": 0,
            "replayq": true,
            "new": {
                "delays": [1.0, 10.0],
                "ints": [1, 4, 7],
                "initialFactor": 2500,
                "perDay": 20,
                "order": 1,
                "bury": true
            },
            "rev": {
                "perDay": 200,
                "ease4": 1.3,
                "fuzz": 0.05,
                "minSpace": 1,
                "ivlFct": 1.0,
                "maxIvl": 36500,
                "bury": true
            },
            "lapse": {
                "delays": [10.0],
                "mult": 0.0,
                "minInt": 1,
                "leechFails": 8,
                "leechAction": 0
            }
        }),
    );
    connection
        .execute(
            concat!(
                "insert into col ",
                "(id, crt, mod, scm, ver, dty, usn, ls, conf, models, decks, dconf, tags) ",
                "values (1, ?1, ?2, ?3, 11, 0, -1, 0, ?4, ?5, ?6, ?7, '{}')"
            ),
            params![
                now_seconds,
                now_seconds,
                now_millis,
                conf.to_string(),
                serde_json::Value::Object(models).to_string(),
                serde_json::Value::Object(decks).to_string(),
                serde_json::Value::Object(dconf).to_string(),
            ],
        )
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn read_anki_collection_from_package(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|error| error.to_string())?;
    crate::storage::check_study_archive_limits(&mut archive)?;
    for name in ["collection.anki2", "collection.anki21"] {
        if let Ok(mut collection) = archive.by_name(name) {
            let mut output = Vec::new();
            collection
                .read_to_end(&mut output)
                .map_err(|error| error.to_string())?;
            return Ok(output);
        }
    }
    Err("missing collection.anki2 in Anki package".to_string())
}

fn read_anki_collection_sqlite(
    path: &PathBuf,
    deck_id: StudyDeckId,
    node_id: CurriculumNodeId,
    locale: Option<ContentLocale>,
    now: impl Into<String>,
) -> Result<Vec<StudyCard>, String> {
    let now = now.into();
    let connection = Connection::open(path).map_err(|error| error.to_string())?;
    let mut statement = connection
        .prepare("select id, flds, tags from notes order by id")
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .map_err(|error| error.to_string())?;
    let mut cards = Vec::new();
    for row in rows {
        let (note_id, fields, tags) = row.map_err(|error| error.to_string())?;
        let mut split = fields.split('\u{1f}');
        let front = split.next().unwrap_or_default().trim();
        let back = split.next().unwrap_or_default().trim();
        if front.is_empty() && back.is_empty() {
            continue;
        }
        let mut card = imported_card(
            &deck_id,
            &node_id,
            locale.clone(),
            cards.len(),
            front,
            back,
            &now,
        );
        card.id = StudyCardId::from(format!("anki-card-{note_id}"));
        card.tags = tags
            .split_whitespace()
            .filter(|tag| !tag.is_empty())
            .map(str::to_string)
            .collect();
        cards.push(card);
    }
    Ok(cards)
}

fn temp_anki_collection_path(prefix: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "tench-study-{prefix}-{}-{}.anki2",
        std::process::id(),
        anki_now_millis()
    ))
}

fn anki_deck_id() -> i64 {
    1_766_000_000_001
}

fn anki_basic_model_id() -> i64 {
    1_766_000_000_002
}

fn anki_now_seconds() -> i64 {
    anki_now_millis() / 1000
}

fn anki_now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

fn anki_field_text(value: &str) -> String {
    value.replace(['\u{1f}', '\0'], " ")
}

fn anki_tags(tags: &[String]) -> String {
    if tags.is_empty() {
        String::new()
    } else {
        format!(" {} ", tags.join(" "))
    }
}

fn anki_sort_checksum(value: &str) -> i64 {
    let mut hash: u32 = 2_166_136_261;
    for byte in value.as_bytes() {
        hash ^= u32::from(*byte);
        hash = hash.wrapping_mul(16_777_619);
    }
    i64::from(hash & 0x7fff_ffff)
}
