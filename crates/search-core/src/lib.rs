use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchDomain {
    Files,
    Documents,
    Code,
    Media,
    Notes,
    EngineModels,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub domains: Vec<SearchDomain>,
    pub tags: Vec<String>,
    pub limit: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub domain: SearchDomain,
    pub title: String,
    pub snippet: String,
    pub score: f32,
    pub location: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IndexStats {
    pub indexed_items: u64,
    pub pending_items: u64,
    pub failed_items: u64,
    pub updated_at: Option<String>,
}

/// Normalize user-visible text for local search matching.
///
/// This intentionally stays dependency-free. It performs Unicode case folding
/// through `char::to_lowercase`, collapses whitespace, skips marks that are
/// normally optional for search in Latin/Hebrew/Arabic-family scripts, folds
/// common Latin diacritics, and converts full-width ASCII to ASCII. CJK, Thai,
/// Indic scripts, RTL base letters, and other scripts are preserved.
pub fn normalize_search_text(value: &str) -> String {
    let mut out = String::new();
    let mut pending_space = false;

    for lower in value.trim().chars().flat_map(char::to_lowercase) {
        if lower.is_whitespace() {
            pending_space = true;
            continue;
        }
        if is_combining_mark(lower) {
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        append_folded_char(&mut out, lower);
        pending_space = false;
    }

    out
}

fn append_folded_char(out: &mut String, ch: char) {
    if ('\u{ff01}'..='\u{ff5e}').contains(&ch) {
        if let Some(ascii) = char::from_u32(ch as u32 - 0xfee0) {
            out.extend(ascii.to_lowercase());
            return;
        }
    }

    match ch {
        'á' | 'à' | 'ả' | 'ạ' | 'â' | 'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' | 'ä' | 'ã' | 'å' | 'ā' | 'ă'
        | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' | 'ą' | 'ǎ' | 'ª' => out.push('a'),
        'æ' => out.push_str("ae"),
        'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => out.push('c'),
        'ď' | 'đ' => out.push('d'),
        'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' | 'ê' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' | 'ë' | 'ē' | 'ĕ' | 'ė'
        | 'ę' | 'ě' => out.push('e'),
        'ƒ' => out.push('f'),
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => out.push('g'),
        'ĥ' | 'ħ' => out.push('h'),
        'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' | 'î' | 'ï' | 'ī' | 'ĭ' | 'į' | 'ı' => {
            out.push('i')
        }
        'ĵ' => out.push('j'),
        'ķ' | 'ĸ' => out.push('k'),
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => out.push('l'),
        'ñ' | 'ń' | 'ņ' | 'ň' | 'ŉ' => out.push('n'),
        'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' | 'ô' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' | 'ơ' | 'ớ' | 'ờ' | 'ở'
        | 'ỡ' | 'ợ' | 'ö' | 'ō' | 'ŏ' | 'ő' | 'ø' | 'º' => out.push('o'),
        'œ' => out.push_str("oe"),
        'ŕ' | 'ŗ' | 'ř' => out.push('r'),
        'ś' | 'ŝ' | 'ş' | 'š' | 'ſ' => out.push('s'),
        'ß' => out.push_str("ss"),
        'ţ' | 'ť' | 'ŧ' | 'þ' => out.push('t'),
        'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' | 'ư' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' | 'û' | 'ü' | 'ū' | 'ŭ'
        | 'ů' | 'ű' | 'ų' => out.push('u'),
        'ŵ' => out.push('w'),
        'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' | 'ÿ' | 'ŷ' => out.push('y'),
        'ź' | 'ż' | 'ž' => out.push('z'),
        _ => out.push(ch),
    }
}

fn is_combining_mark(ch: char) -> bool {
    matches!(
        ch as u32,
        0x0300..=0x036f
            | 0x1ab0..=0x1aff
            | 0x1dc0..=0x1dff
            | 0x20d0..=0x20ff
            | 0xfe20..=0xfe2f
            | 0x0591..=0x05bd
            | 0x05bf
            | 0x05c1..=0x05c2
            | 0x05c4..=0x05c5
            | 0x05c7
            | 0x0610..=0x061a
            | 0x064b..=0x065f
            | 0x0670
            | 0x06d6..=0x06dc
            | 0x06df..=0x06e8
            | 0x06ea..=0x06ed
            | 0x0730..=0x074a
            | 0x07a6..=0x07b0
            | 0x07eb..=0x07f3
            | 0x08d3..=0x08ff
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_normalization_folds_common_diacritics_and_width() {
        assert_eq!(normalize_search_text("  Café  ＡI  "), "cafe ai");
        assert_eq!(normalize_search_text("straße"), "strasse");
        assert_eq!(normalize_search_text("cafe\u{301}"), "cafe");
        assert_eq!(normalize_search_text("Tế bào học"), "te bao hoc");
    }

    #[test]
    fn search_normalization_preserves_non_latin_scripts() {
        assert_eq!(normalize_search_text("심장 구조"), "심장 구조");
        assert_eq!(normalize_search_text("قلب"), "قلب");
        assert_eq!(normalize_search_text("心臓 構造"), "心臓 構造");
        assert_eq!(normalize_search_text("หัวใจ"), "หัวใจ");
        assert_eq!(normalize_search_text("हृदय"), "हृदय");
    }

    #[test]
    fn search_normalization_ignores_rtl_vocalization_marks() {
        assert_eq!(normalize_search_text("قَلْب"), "قلب");
        assert_eq!(normalize_search_text("שָׁלוֹם"), "שלום");
    }
}
