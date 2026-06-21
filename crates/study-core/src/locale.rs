use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ContentLocale {
    pub language: String,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub direction: TextDirection,
}

impl ContentLocale {
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
    Auto,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalizedText {
    pub value: String,
    #[serde(default)]
    pub locale: Option<ContentLocale>,
    #[serde(default)]
    pub source_locale: Option<ContentLocale>,
    #[serde(default)]
    pub machine_translated: bool,
}

impl LocalizedText {
    pub fn plain(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            locale: None,
            source_locale: None,
            machine_translated: false,
        }
    }

    pub fn localized(value: impl Into<String>, locale: ContentLocale) -> Self {
        Self {
            value: value.into(),
            locale: Some(locale),
            source_locale: None,
            machine_translated: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalizedStringSet {
    pub default: LocalizedText,
    #[serde(default)]
    pub translations: Vec<LocalizedText>,
}

impl LocalizedStringSet {
    pub fn plain(value: impl Into<String>) -> Self {
        Self {
            default: LocalizedText::plain(value),
            translations: Vec::new(),
        }
    }

    pub fn has_locale(&self, locale: &ContentLocale) -> bool {
        self.default.locale.as_ref() == Some(locale)
            || self
                .translations
                .iter()
                .any(|translation| translation.locale.as_ref() == Some(locale))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocaleCoverage {
    pub required: Vec<ContentLocale>,
    pub available: Vec<ContentLocale>,
    pub missing: Vec<ContentLocale>,
}

pub fn compute_locale_coverage(
    required: &[ContentLocale],
    available: &[ContentLocale],
) -> LocaleCoverage {
    let missing = required
        .iter()
        .filter(|locale| !available.contains(locale))
        .cloned()
        .collect();

    LocaleCoverage {
        required: required.to_vec(),
        available: available.to_vec(),
        missing,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_locale_and_detects_direction() {
        let locale = ContentLocale::parse("ur-Arab-PK").expect("locale");

        assert_eq!(locale.language, "ur");
        assert_eq!(locale.script.as_deref(), Some("Arab"));
        assert_eq!(locale.region.as_deref(), Some("PK"));
        assert_eq!(locale.direction, TextDirection::Rtl);
        assert_eq!(locale.bcp47(), "ur-Arab-PK");
    }

    #[test]
    fn locale_direction_uses_script_before_language_default() {
        let latin_kurdish = ContentLocale::parse("ku-Latn-TR").expect("locale");
        let arabic_azerbaijani = ContentLocale::parse("az-Arab-IR").expect("locale");
        let dhivehi = ContentLocale::parse("dv-MV").expect("locale");
        let yiddish = ContentLocale::parse("yi").expect("locale");

        assert_eq!(latin_kurdish.direction, TextDirection::Ltr);
        assert_eq!(arabic_azerbaijani.direction, TextDirection::Rtl);
        assert_eq!(dhivehi.direction, TextDirection::Rtl);
        assert_eq!(yiddish.direction, TextDirection::Rtl);
    }

    #[test]
    fn coverage_reports_missing_locales() {
        let en = ContentLocale::parse("en-US").unwrap();
        let ko = ContentLocale::parse("ko-KR").unwrap();
        let coverage = compute_locale_coverage(&[en.clone(), ko.clone()], &[en]);

        assert_eq!(coverage.missing, vec![ko]);
    }
}
