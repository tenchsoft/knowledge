use serde::{Deserialize, Serialize};

pub const CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION: u32 = 1;

crate::research_id_type!(LibraryId);
crate::research_id_type!(ReferenceId);
crate::research_id_type!(AttachmentId);
crate::research_id_type!(AnnotationId);
crate::research_id_type!(ResearchNoteId);
crate::research_id_type!(ResearchCollectionId);
crate::research_id_type!(ResearchTagId);
crate::research_id_type!(Citekey);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(pub String);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchLocale {
    pub language: String,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub direction: TextDirection,
}

impl ResearchLocale {
    pub fn parse(tag: &str) -> Option<Self> {
        let mut parts = tag.split('-').filter(|part| !part.is_empty());
        let language = parts.next()?.to_ascii_lowercase();
        if !(2..=8).contains(&language.len())
            || !language.chars().all(|ch| ch.is_ascii_alphabetic())
        {
            return None;
        }

        let mut script = None;
        let mut region = None;
        for part in parts {
            if part.len() == 4 && part.chars().all(|ch| ch.is_ascii_alphabetic()) {
                let mut chars = part.chars();
                let first = chars.next()?.to_ascii_uppercase();
                let rest = chars.as_str().to_ascii_lowercase();
                script = Some(format!("{first}{rest}"));
            } else if (part.len() == 2 && part.chars().all(|ch| ch.is_ascii_alphabetic()))
                || (part.len() == 3 && part.chars().all(|ch| ch.is_ascii_digit()))
            {
                region = Some(part.to_ascii_uppercase());
            }
        }

        let direction = if script.as_deref().is_some_and(is_rtl_script)
            || (script.is_none() && is_default_rtl_language(&language))
        {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        };

        Some(Self {
            language,
            script,
            region,
            direction,
        })
    }

    pub fn bcp47(&self) -> String {
        let mut tag = self.language.clone();
        if let Some(script) = &self.script {
            tag.push('-');
            tag.push_str(script);
        }
        if let Some(region) = &self.region {
            tag.push('-');
            tag.push_str(region);
        }
        tag
    }
}

fn is_rtl_script(script: &str) -> bool {
    matches!(
        script,
        "Adlm" | "Arab" | "Aran" | "Hebr" | "Mand" | "Nkoo" | "Rohg" | "Samr" | "Syrc" | "Thaa"
    )
}

fn is_default_rtl_language(language: &str) -> bool {
    matches!(
        language,
        "ar" | "arc"
            | "bcc"
            | "bqi"
            | "ckb"
            | "dv"
            | "fa"
            | "glk"
            | "he"
            | "ks"
            | "ku"
            | "mzn"
            | "nqo"
            | "pnb"
            | "ps"
            | "sd"
            | "ug"
            | "ur"
            | "yi"
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
    Auto,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalizedField {
    pub value: String,
    #[serde(default)]
    pub locale: Option<ResearchLocale>,
    #[serde(default)]
    pub original: bool,
}

impl LocalizedField {
    pub fn plain(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            locale: None,
            original: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DateParts {
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    #[serde(default)]
    pub raw: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchLibrary {
    pub id: LibraryId,
    pub name: String,
    pub root_dir: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub schema_version: u32,
    pub settings: LibrarySettings,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LibrarySettings {
    pub default_locale: ResearchLocale,
    pub citation_locale: ResearchLocale,
    pub default_citation_style: String,
    pub attachment_policy: AttachmentStoragePolicy,
}

impl Default for LibrarySettings {
    fn default() -> Self {
        let locale = ResearchLocale::parse("en-US").expect("default locale");
        Self {
            default_locale: locale.clone(),
            citation_locale: locale,
            default_citation_style: "apa".to_string(),
            attachment_policy: AttachmentStoragePolicy::CopyIntoLibrary,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentStoragePolicy {
    CopyIntoLibrary,
    LinkOriginal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceItem {
    pub id: ReferenceId,
    pub kind: ReferenceKind,
    pub title: LocalizedField,
    #[serde(default)]
    pub subtitle: Option<LocalizedField>,
    #[serde(default)]
    pub creators: Vec<Creator>,
    pub issued: DateParts,
    #[serde(default)]
    pub abstract_text: Option<LocalizedField>,
    #[serde(default)]
    pub language: Option<ResearchLocale>,
    #[serde(default)]
    pub venue: Option<PublicationVenue>,
    pub identifiers: ReferenceIdentifiers,
    #[serde(default)]
    pub urls: Vec<ReferenceUrl>,
    #[serde(default)]
    pub collections: Vec<ResearchCollectionId>,
    #[serde(default)]
    pub tags: Vec<ResearchTagId>,
    pub status: crate::ReadingStatus,
    pub favorite: bool,
    #[serde(default)]
    pub rating: Option<u8>,
    #[serde(default)]
    pub citekey: Option<Citekey>,
    #[serde(default)]
    pub citekey_locked: bool,
    pub metadata: ReferenceMetadata,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl ReferenceItem {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.value.trim().is_empty() {
            return Err("reference title is required".to_string());
        }
        if let Some(rating) = self.rating {
            if rating > 5 {
                return Err("rating must be between 0 and 5".to_string());
            }
        }
        if self
            .creators
            .iter()
            .any(|creator| !creator.has_display_name())
        {
            return Err("creator requires given/family or literal name".to_string());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceKind {
    JournalArticle,
    ConferencePaper,
    Book,
    BookSection,
    Thesis,
    Report,
    Preprint,
    WebPage,
    Dataset,
    Patent,
    Presentation,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Creator {
    pub role: CreatorRole,
    #[serde(default)]
    pub given: Option<String>,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub literal: Option<String>,
    #[serde(default)]
    pub transliteration: Option<String>,
    #[serde(default)]
    pub sort_key: Option<String>,
    #[serde(default)]
    pub name_order: CreatorNameOrder,
    #[serde(default)]
    pub locale: Option<ResearchLocale>,
    #[serde(default)]
    pub orcid: Option<String>,
    #[serde(default)]
    pub affiliation: Option<String>,
}

impl Creator {
    pub fn has_display_name(&self) -> bool {
        self.literal
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
            || self
                .given
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
            || self
                .family
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
    }

    pub fn sort_name(&self) -> String {
        if let Some(sort_key) = &self.sort_key {
            return sort_key.clone();
        }
        if let Some(transliteration) = &self.transliteration {
            return transliteration.clone();
        }
        if let Some(literal) = &self.literal {
            return literal.clone();
        }
        match (&self.family, &self.given, self.name_order) {
            (Some(family), Some(given), CreatorNameOrder::FamilyFirst) => {
                format!("{family} {given}")
            }
            (Some(family), Some(given), _) => format!("{given} {family}"),
            (Some(family), None, _) => family.clone(),
            (None, Some(given), _) => given.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatorRole {
    Author,
    Editor,
    Translator,
    Advisor,
    Contributor,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreatorNameOrder {
    #[default]
    LocaleDefault,
    GivenFirst,
    FamilyFirst,
    Literal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicationVenue {
    pub name: LocalizedField,
    #[serde(default)]
    pub volume: Option<String>,
    #[serde(default)]
    pub issue: Option<String>,
    #[serde(default)]
    pub pages: Option<String>,
    #[serde(default)]
    pub publisher: Option<LocalizedField>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceIdentifiers {
    #[serde(default)]
    pub doi: Option<String>,
    #[serde(default)]
    pub isbn: Vec<String>,
    #[serde(default)]
    pub issn: Vec<String>,
    #[serde(default)]
    pub arxiv_id: Option<String>,
    #[serde(default)]
    pub pmid: Option<String>,
    #[serde(default)]
    pub pmcid: Option<String>,
    #[serde(default)]
    pub semantic_scholar_id: Option<String>,
    #[serde(default)]
    pub custom: Vec<KeyValue>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReferenceUrl {
    pub url: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub accessed_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceMetadata {
    #[serde(default)]
    pub source_format: Option<String>,
    #[serde(default)]
    pub imported_from: Option<String>,
    #[serde(default)]
    pub locale_hints: Vec<ResearchLocale>,
    #[serde(default)]
    pub extra: Vec<KeyValue>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub id: AttachmentId,
    pub reference_id: ReferenceId,
    pub kind: AttachmentKind,
    pub title: String,
    pub stored_path: String,
    #[serde(default)]
    pub original_path: Option<String>,
    pub mime_type: String,
    pub size_bytes: u64,
    pub content_hash: String,
    #[serde(default)]
    pub page_count: Option<u32>,
    pub text_indexed: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentKind {
    Pdf,
    Snapshot,
    Image,
    Supplementary,
    Link,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PdfAnnotation {
    pub id: AnnotationId,
    pub attachment_id: AttachmentId,
    pub reference_id: ReferenceId,
    pub kind: PdfAnnotationKind,
    pub page: u32,
    #[serde(default)]
    pub rects: Vec<PageRect>,
    pub color: ColorRgba,
    #[serde(default)]
    pub selected_text: Option<String>,
    #[serde(default)]
    pub note_markdown: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PdfAnnotationKind {
    Highlight,
    Underline,
    Strikeout,
    Note,
    Drawing,
    Bookmark,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchNote {
    pub id: ResearchNoteId,
    #[serde(default)]
    pub reference_id: Option<ReferenceId>,
    #[serde(default)]
    pub annotation_id: Option<AnnotationId>,
    pub title: String,
    pub body_markdown: String,
    #[serde(default)]
    pub tags: Vec<ResearchTagId>,
    #[serde(default)]
    pub backlinks: Vec<ResearchNoteId>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchCollection {
    pub id: ResearchCollectionId,
    #[serde(default)]
    pub parent_id: Option<ResearchCollectionId>,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub sort_order: i64,
    #[serde(default)]
    pub rules: Option<SmartCollectionRule>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchTag {
    pub id: ResearchTagId,
    pub label: String,
    pub color: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SmartCollectionRule {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub tags: Vec<ResearchTagId>,
    #[serde(default)]
    pub status: Option<crate::ReadingStatus>,
    #[serde(default)]
    pub year_range: Option<(u16, u16)>,
    #[serde(default)]
    pub has_attachment: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchSnapshotV2 {
    pub library: ResearchLibrary,
    #[serde(default)]
    pub references: Vec<ReferenceItem>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub annotations: Vec<PdfAnnotation>,
    #[serde(default)]
    pub notes: Vec<ResearchNote>,
    #[serde(default)]
    pub collections: Vec<ResearchCollection>,
    #[serde(default)]
    pub tags: Vec<ResearchTag>,
}

pub fn new_research_library_snapshot(
    id: LibraryId,
    name: impl Into<String>,
    root_dir: impl Into<String>,
    locale: ResearchLocale,
    now: Timestamp,
) -> ResearchSnapshotV2 {
    ResearchSnapshotV2 {
        library: ResearchLibrary {
            id,
            name: name.into(),
            root_dir: root_dir.into(),
            created_at: now.clone(),
            updated_at: now,
            schema_version: CURRENT_RESEARCH_LIBRARY_SCHEMA_VERSION,
            settings: LibrarySettings {
                default_locale: locale.clone(),
                citation_locale: locale,
                default_citation_style: "apa".to_string(),
                attachment_policy: AttachmentStoragePolicy::CopyIntoLibrary,
            },
        },
        references: Vec::new(),
        attachments: Vec::new(),
        annotations: Vec::new(),
        notes: Vec::new(),
        collections: Vec::new(),
        tags: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bcp47_locale_and_direction() {
        let locale = ResearchLocale::parse("ar-Arab-EG").expect("locale");

        assert_eq!(locale.language, "ar");
        assert_eq!(locale.script.as_deref(), Some("Arab"));
        assert_eq!(locale.region.as_deref(), Some("EG"));
        assert_eq!(locale.direction, TextDirection::Rtl);
        assert_eq!(locale.bcp47(), "ar-Arab-EG");
    }

    #[test]
    fn locale_direction_uses_script_before_language_default() {
        let latin_kurdish = ResearchLocale::parse("ku-Latn-TR").expect("locale");
        let arabic_azerbaijani = ResearchLocale::parse("az-Arab-IR").expect("locale");
        let dhivehi = ResearchLocale::parse("dv-MV").expect("locale");
        let yiddish = ResearchLocale::parse("yi").expect("locale");

        assert_eq!(latin_kurdish.direction, TextDirection::Ltr);
        assert_eq!(arabic_azerbaijani.direction, TextDirection::Rtl);
        assert_eq!(dhivehi.direction, TextDirection::Rtl);
        assert_eq!(yiddish.direction, TextDirection::Rtl);
    }

    #[test]
    fn rejects_reference_without_title() {
        let reference = ReferenceItem {
            id: ReferenceId::from("ref_1"),
            kind: ReferenceKind::JournalArticle,
            title: LocalizedField::plain(""),
            subtitle: None,
            creators: Vec::new(),
            issued: DateParts {
                year: Some(2026),
                month: None,
                day: None,
                raw: None,
            },
            abstract_text: None,
            language: None,
            venue: None,
            identifiers: ReferenceIdentifiers::default(),
            urls: Vec::new(),
            collections: Vec::new(),
            tags: Vec::new(),
            status: crate::ReadingStatus::Unread,
            favorite: false,
            rating: None,
            citekey: None,
            citekey_locked: false,
            metadata: ReferenceMetadata::default(),
            created_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
            updated_at: Timestamp("2026-05-04T00:00:00Z".to_string()),
        };

        assert!(reference.validate().is_err());
    }

    #[test]
    fn creator_sort_name_uses_sort_key_then_transliteration() {
        let creator = Creator {
            role: CreatorRole::Author,
            given: Some("길동".to_string()),
            family: Some("홍".to_string()),
            literal: None,
            transliteration: Some("Hong Gildong".to_string()),
            sort_key: Some("Hong, Gildong".to_string()),
            name_order: CreatorNameOrder::FamilyFirst,
            locale: ResearchLocale::parse("ko-KR"),
            orcid: None,
            affiliation: None,
        };

        assert_eq!(creator.sort_name(), "Hong, Gildong");
    }

    #[test]
    fn new_library_snapshot_has_schema_and_locale_settings() {
        let locale = ResearchLocale::parse("ko-KR").expect("locale");
        let snapshot = new_research_library_snapshot(
            LibraryId::from("lib"),
            "Library",
            "/tmp/lib",
            locale,
            Timestamp("2026-05-04T00:00:00Z".to_string()),
        );

        assert_eq!(snapshot.library.schema_version, 1);
        assert_eq!(snapshot.library.settings.default_locale.bcp47(), "ko-KR");
        assert!(snapshot.references.is_empty());
    }
}
