use super::*;

pub fn inspect_pdf_document_bytes(attachment_id: AttachmentId, bytes: &[u8]) -> PdfOpenResult {
    let header = String::from_utf8_lossy(bytes.get(..bytes.len().min(16)).unwrap_or_default());
    let pdf_version = header
        .strip_prefix("%PDF-")
        .and_then(|rest| rest.split_whitespace().next())
        .map(str::to_string);
    let mut status = if pdf_version.is_some() {
        PdfDocumentStatus::Ready
    } else if bytes.is_empty() {
        PdfDocumentStatus::Corrupt
    } else {
        PdfDocumentStatus::Unsupported
    };
    if bytes
        .windows(b"/Encrypt".len())
        .any(|window| window == b"/Encrypt")
    {
        status = PdfDocumentStatus::Encrypted;
    }
    let text_extractable = status == PdfDocumentStatus::Ready && has_literal_pdf_text(bytes);
    if status == PdfDocumentStatus::Ready && !text_extractable {
        status = PdfDocumentStatus::ImageOnly;
    }
    let metadata = PdfDocumentMetadata {
        attachment_id,
        status,
        title: find_pdf_info_string(bytes, "/Title"),
        author: find_pdf_info_string(bytes, "/Author"),
        page_count: count_pdf_pages(bytes).filter(|count| *count > 0),
        pdf_version,
        text_extractable,
    };
    let warning = match metadata.status {
        PdfDocumentStatus::Encrypted => Some("PDF is encrypted; open requires credentials.".into()),
        PdfDocumentStatus::Unsupported => Some("File does not look like a PDF document.".into()),
        PdfDocumentStatus::Corrupt => Some("PDF file is empty or corrupt.".into()),
        PdfDocumentStatus::ImageOnly => {
            Some("PDF has no directly extractable text; OCR is unavailable in the current reader pipeline.".into())
        }
        PdfDocumentStatus::Ready => None,
    };
    PdfOpenResult { metadata, warning }
}

pub fn extract_pdf_literal_text(
    attachment_id: AttachmentId,
    bytes: &[u8],
    locale: Option<crate::ResearchLocale>,
) -> Result<PdfDocumentText, String> {
    let open = inspect_pdf_document_bytes(attachment_id.clone(), bytes);
    match open.metadata.status {
        PdfDocumentStatus::Ready | PdfDocumentStatus::ImageOnly => {}
        PdfDocumentStatus::Encrypted => return Err("encrypted PDF text extraction blocked".into()),
        PdfDocumentStatus::Corrupt => return Err("corrupt PDF cannot be extracted".into()),
        PdfDocumentStatus::Unsupported => return Err("unsupported PDF format".into()),
    }
    let text = extract_literal_strings(bytes).join(" ");
    if text.trim().is_empty() {
        return Ok(PdfDocumentText {
            attachment_id,
            pages: Vec::new(),
        });
    }
    Ok(PdfDocumentText {
        attachment_id,
        pages: vec![PdfPageText {
            page: 1,
            text,
            locale,
        }],
    })
}

fn count_pdf_pages(bytes: &[u8]) -> Option<u32> {
    if !bytes.starts_with(b"%PDF-") {
        return None;
    }
    let mut count = 0u32;
    let needle = b"/Type";
    let mut cursor = 0usize;
    while let Some(index) = find_bytes(&bytes[cursor..], needle) {
        let absolute = cursor + index + needle.len();
        let rest = bytes.get(absolute..absolute + 16).unwrap_or_default();
        let trimmed = trim_ascii_start(rest);
        if trimmed.starts_with(b"/Page")
            && !trimmed
                .get(5)
                .is_some_and(|value| value.is_ascii_alphabetic())
        {
            count += 1;
        }
        cursor = absolute;
    }
    Some(count)
}

fn has_literal_pdf_text(bytes: &[u8]) -> bool {
    bytes.windows(2).any(|window| window == b"BT")
        && bytes.windows(2).any(|window| window == b"ET")
        && !extract_literal_strings(bytes).is_empty()
}

fn find_pdf_info_string(bytes: &[u8], key: &str) -> Option<String> {
    let key = key.as_bytes();
    let start = find_bytes(bytes, key)? + key.len();
    let rest = trim_ascii_start(bytes.get(start..)?);
    if !rest.starts_with(b"(") {
        return None;
    }
    extract_parenthesized(rest).map(|(value, _)| value)
}

fn extract_literal_strings(bytes: &[u8]) -> Vec<String> {
    let mut values = Vec::new();
    let mut cursor = 0usize;
    while let Some(relative) = bytes[cursor..].iter().position(|byte| *byte == b'(') {
        let start = cursor + relative;
        if let Some((value, consumed)) = extract_parenthesized(&bytes[start..]) {
            if value
                .chars()
                .any(|ch| !ch.is_control() && !ch.is_whitespace())
            {
                values.push(value);
            }
            cursor = start + consumed;
        } else {
            break;
        }
    }
    values
}

fn extract_parenthesized(bytes: &[u8]) -> Option<(String, usize)> {
    if !bytes.starts_with(b"(") {
        return None;
    }
    let mut output = Vec::new();
    let mut escaped = false;
    let mut depth = 0u32;
    for (index, byte) in bytes.iter().enumerate() {
        if index == 0 {
            depth = 1;
            continue;
        }
        if escaped {
            output.push(match byte {
                b'n' => b'\n',
                b'r' => b'\r',
                b't' => b'\t',
                other => *other,
            });
            escaped = false;
            continue;
        }
        match byte {
            b'\\' => escaped = true,
            b'(' => {
                depth += 1;
                output.push(*byte);
            }
            b')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some((String::from_utf8_lossy(&output).to_string(), index + 1));
                }
                output.push(*byte);
            }
            other => output.push(*other),
        }
    }
    None
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn trim_ascii_start(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    &bytes[start..]
}
