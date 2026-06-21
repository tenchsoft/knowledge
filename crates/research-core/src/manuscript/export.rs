use std::io::Cursor;

use tench_document_core::{BlockNode, InlineNode};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::ReferenceItem;

use super::{ResearchManuscript, WritingExportFormat};

pub(super) fn render_manuscript_markdown(manuscript: &ResearchManuscript) -> String {
    let mut out = String::new();
    out.push_str("# ");
    out.push_str(&manuscript.title.value);
    out.push_str("\n\n");
    for block in &manuscript.document.content {
        append_block_markdown(&mut out, block);
        if !out.ends_with("\n\n") {
            out.push_str("\n\n");
        }
    }
    if !manuscript
        .citation_state
        .bibliography
        .rendered
        .trim()
        .is_empty()
    {
        out.push_str("## References\n\n");
        out.push_str(&manuscript.citation_state.bibliography.rendered);
        out.push('\n');
    }
    out
}

pub(super) fn render_manuscript_html(manuscript: &ResearchManuscript) -> String {
    let mut out = String::new();
    out.push_str("<article>\n<h1>");
    out.push_str(&escape_html(&manuscript.title.value));
    out.push_str("</h1>\n");
    for block in &manuscript.document.content {
        append_block_html(&mut out, block);
    }
    if !manuscript
        .citation_state
        .bibliography
        .rendered
        .trim()
        .is_empty()
    {
        out.push_str("<section><h2>References</h2><pre>");
        out.push_str(&escape_html(
            &manuscript.citation_state.bibliography.rendered,
        ));
        out.push_str("</pre></section>\n");
    }
    out.push_str("</article>\n");
    out
}

pub(super) fn render_manuscript_latex(manuscript: &ResearchManuscript) -> String {
    let mut out = String::new();
    out.push_str("\\section*{");
    out.push_str(&escape_latex(&manuscript.title.value));
    out.push_str("}\n\n");
    for block in &manuscript.document.content {
        append_block_latex(&mut out, block);
    }
    if !manuscript
        .citation_state
        .bibliography
        .rendered
        .trim()
        .is_empty()
    {
        out.push_str("\\section*{References}\n\\begin{verbatim}\n");
        out.push_str(&manuscript.citation_state.bibliography.rendered);
        out.push_str("\n\\end{verbatim}\n");
    }
    out
}

pub(super) fn render_manuscript_archive_manifest(manuscript: &ResearchManuscript) -> String {
    format!(
        "manuscript_id: {}\ntitle: {}\nassets: {}\ncitations: {}\ncross_references: {}\n",
        manuscript.id.as_str(),
        manuscript.title.value,
        manuscript.assets.len(),
        manuscript.citation_state.citations.len(),
        manuscript.cross_references.len()
    )
}

pub(super) fn render_manuscript_archive_bytes(
    manuscript: &ResearchManuscript,
    references: &[ReferenceItem],
) -> Result<Vec<u8>, String> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "manifest.txt",
        &render_manuscript_archive_manifest(manuscript),
        options,
    )
    .map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "manuscript.md",
        &render_manuscript_markdown(manuscript),
        options,
    )
    .map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "bibliography.txt",
        &manuscript.citation_state.bibliography.rendered,
        options,
    )
    .map_err(|error| error.to_string())?;
    let references_json =
        serde_json::to_string_pretty(references).map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "references.json",
        &references_json,
        options,
    )
    .map_err(|error| error.to_string())?;
    let assets_json =
        serde_json::to_string_pretty(&manuscript.assets).map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(&mut writer, "assets.json", &assets_json, options)
        .map_err(|error| error.to_string())?;
    let cross_references_json = serde_json::to_string_pretty(&manuscript.cross_references)
        .map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "cross-references.json",
        &cross_references_json,
        options,
    )
    .map_err(|error| error.to_string())?;
    writer
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| error.to_string())
}

pub(super) fn render_manuscript_docx_bytes(
    manuscript: &ResearchManuscript,
) -> Result<Vec<u8>, String> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "[Content_Types].xml",
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#,
        options,
    )
    .map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "_rels/.rels",
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#,
        options,
    )
    .map_err(|error| error.to_string())?;
    tench_office_io::zip_util::write_zip_file(
        &mut writer,
        "word/document.xml",
        &render_manuscript_docx_document_xml(manuscript),
        options,
    )
    .map_err(|error| error.to_string())?;
    writer
        .finish()
        .map(|cursor| cursor.into_inner())
        .map_err(|error| error.to_string())
}

fn render_manuscript_docx_document_xml(manuscript: &ResearchManuscript) -> String {
    let mut body = String::new();
    body.push_str(&docx_paragraph(&manuscript.title.value, true));
    for block in &manuscript.document.content {
        for line in block_to_plain_text(block).lines() {
            if !line.trim().is_empty() {
                body.push_str(&docx_paragraph(line.trim(), false));
            }
        }
    }
    if !manuscript
        .citation_state
        .bibliography
        .rendered
        .trim()
        .is_empty()
    {
        body.push_str(&docx_paragraph("References", true));
        for line in manuscript.citation_state.bibliography.rendered.lines() {
            body.push_str(&docx_paragraph(line, false));
        }
    }
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    {body}
    <w:sectPr/>
  </w:body>
</w:document>"#
    )
}

fn docx_paragraph(text: &str, bold: bool) -> String {
    let run_props = if bold { "<w:rPr><w:b/></w:rPr>" } else { "" };
    format!(
        "<w:p><w:r>{run_props}<w:t xml:space=\"preserve\">{}</w:t></w:r></w:p>",
        escape_xml(text)
    )
}

pub(super) fn render_manuscript_pdf_bytes(manuscript: &ResearchManuscript) -> Vec<u8> {
    let text = render_manuscript_markdown(manuscript);
    let escaped = escape_pdf_literal(&text);
    let stream = format!("BT /F1 12 Tf 72 740 Td 14 TL ({escaped}) Tj ET");
    let objects = [
        "1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj".to_string(),
        "2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj".to_string(),
        "3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >> endobj".to_string(),
        "4 0 obj << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> endobj".to_string(),
        format!(
            "5 0 obj << /Length {} >> stream\n{}\nendstream endobj",
            stream.len(),
            stream
        ),
    ];
    let mut out = String::from("%PDF-1.4\n");
    let mut offsets = Vec::new();
    for object in &objects {
        offsets.push(out.len());
        out.push_str(object);
        out.push('\n');
    }
    let xref_offset = out.len();
    out.push_str(&format!(
        "xref\n0 {}\n0000000000 65535 f \n",
        objects.len() + 1
    ));
    for offset in offsets {
        out.push_str(&format!("{offset:010} 00000 n \n"));
    }
    out.push_str(&format!(
        "trailer << /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
        objects.len() + 1,
        xref_offset
    ));
    out.into_bytes()
}

fn append_block_markdown(out: &mut String, block: &BlockNode) {
    match block {
        BlockNode::Paragraph { content, .. } => {
            out.push_str(&inline_nodes_to_text(content));
            out.push('\n');
        }
        BlockNode::Heading { level, content, .. } => {
            out.push_str(&"#".repeat((*level).clamp(1, 6) as usize));
            out.push(' ');
            out.push_str(&inline_nodes_to_text(content));
            out.push('\n');
        }
        BlockNode::BulletList { items } => {
            for item in items {
                out.push_str("- ");
                out.push_str(&inline_nodes_to_text(&item.content));
                out.push('\n');
            }
        }
        BlockNode::OrderedList { items, start } => {
            for (index, item) in items.iter().enumerate() {
                out.push_str(&(start + index as u32).to_string());
                out.push_str(". ");
                out.push_str(&inline_nodes_to_text(&item.content));
                out.push('\n');
            }
        }
        BlockNode::TaskList { items } => {
            for item in items {
                out.push_str(if item.checked { "- [x] " } else { "- [ ] " });
                out.push_str(&inline_nodes_to_text(&item.content));
                out.push('\n');
            }
        }
        BlockNode::BlockQuote { content } => {
            for child in content {
                out.push_str("> ");
                append_block_markdown(out, child);
            }
        }
        BlockNode::CodeBlock { language, code } => {
            out.push_str("```");
            out.push_str(language.as_deref().unwrap_or_default());
            out.push('\n');
            out.push_str(code);
            out.push_str("\n```\n");
        }
        BlockNode::Table { rows } => {
            for row in rows {
                out.push('|');
                for cell in &row.cells {
                    out.push(' ');
                    out.push_str(
                        &cell
                            .content
                            .iter()
                            .map(block_to_plain_text)
                            .collect::<Vec<_>>()
                            .join(" "),
                    );
                    out.push_str(" |");
                }
                out.push('\n');
            }
        }
        BlockNode::HorizontalRule => out.push_str("---\n"),
        BlockNode::Image { alt, .. } => {
            out.push_str("![");
            out.push_str(alt.as_deref().unwrap_or("image"));
            out.push_str("]\n");
        }
        BlockNode::PageBreak => out.push_str("\\pagebreak\n"),
        BlockNode::Footnote { .. } => out.push_str("[footnote]\n"),
    }
}

fn append_block_html(out: &mut String, block: &BlockNode) {
    match block {
        BlockNode::Heading { level, content, .. } => {
            let level = (*level).clamp(1, 6);
            out.push_str(&format!(
                "<h{level}>{}</h{level}>\n",
                escape_html(&inline_nodes_to_text(content))
            ));
        }
        BlockNode::Paragraph { content, .. } => {
            out.push_str("<p>");
            out.push_str(&escape_html(&inline_nodes_to_text(content)));
            out.push_str("</p>\n");
        }
        other => {
            out.push_str("<pre>");
            out.push_str(&escape_html(&block_to_plain_text(other)));
            out.push_str("</pre>\n");
        }
    }
}

fn append_block_latex(out: &mut String, block: &BlockNode) {
    match block {
        BlockNode::Heading { level, content, .. } => {
            let command = if *level <= 2 {
                "section"
            } else if *level == 3 {
                "subsection"
            } else {
                "paragraph"
            };
            out.push('\\');
            out.push_str(command);
            out.push_str("*{");
            out.push_str(&escape_latex(&inline_nodes_to_text(content)));
            out.push_str("}\n\n");
        }
        BlockNode::Paragraph { content, .. } => {
            out.push_str(&escape_latex(&inline_nodes_to_text(content)));
            out.push_str("\n\n");
        }
        other => {
            out.push_str("\\begin{verbatim}\n");
            out.push_str(&block_to_plain_text(other));
            out.push_str("\n\\end{verbatim}\n");
        }
    }
}

pub(super) fn block_to_plain_text(block: &BlockNode) -> String {
    match block {
        BlockNode::Paragraph { content, .. } | BlockNode::Heading { content, .. } => {
            inline_nodes_to_text(content)
        }
        BlockNode::BulletList { items } | BlockNode::OrderedList { items, .. } => items
            .iter()
            .map(|item| inline_nodes_to_text(&item.content))
            .collect::<Vec<_>>()
            .join("\n"),
        BlockNode::TaskList { items } => items
            .iter()
            .map(|item| inline_nodes_to_text(&item.content))
            .collect::<Vec<_>>()
            .join("\n"),
        BlockNode::BlockQuote { content } => content
            .iter()
            .map(block_to_plain_text)
            .collect::<Vec<_>>()
            .join("\n"),
        BlockNode::CodeBlock { code, .. } => code.clone(),
        BlockNode::Table { rows } => rows
            .iter()
            .map(|row| {
                row.cells
                    .iter()
                    .map(|cell| {
                        cell.content
                            .iter()
                            .map(block_to_plain_text)
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect::<Vec<_>>()
                    .join("\t")
            })
            .collect::<Vec<_>>()
            .join("\n"),
        BlockNode::HorizontalRule => String::new(),
        BlockNode::Image { alt, .. } => alt.clone().unwrap_or_default(),
        BlockNode::PageBreak => String::new(),
        BlockNode::Footnote { .. } => String::new(),
    }
}

pub(super) fn inline_nodes_to_text(nodes: &[InlineNode]) -> String {
    nodes
        .iter()
        .map(|node| match node {
            InlineNode::Text { text, .. } => text.clone(),
            InlineNode::HardBreak => "\n".to_string(),
            InlineNode::InlineImage { alt, .. } => alt.clone().unwrap_or_default(),
            InlineNode::Link { text, href, .. } => {
                if text.is_empty() {
                    href.clone()
                } else {
                    text.clone()
                }
            }
        })
        .collect()
}

pub(super) fn manuscript_export_file_name(
    manuscript: &ResearchManuscript,
    format: WritingExportFormat,
) -> String {
    let stem = manuscript
        .title
        .value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    let stem = if stem.is_empty() {
        "manuscript".to_string()
    } else {
        stem
    };
    format!("{stem}.{}", manuscript_export_extension(format))
}

fn manuscript_export_extension(format: WritingExportFormat) -> &'static str {
    match format {
        WritingExportFormat::Docx => "docx",
        WritingExportFormat::Pdf => "pdf",
        WritingExportFormat::Markdown => "md",
        WritingExportFormat::Html => "html",
        WritingExportFormat::Latex => "tex",
        WritingExportFormat::Archive => "zip",
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_xml(value: &str) -> String {
    escape_html(value).replace('\'', "&apos;")
}

fn escape_pdf_literal(value: &str) -> String {
    value
        .lines()
        .take(48)
        .collect::<Vec<_>>()
        .join("\\n")
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

fn escape_latex(value: &str) -> String {
    value
        .replace('\\', "\\textbackslash{}")
        .replace('&', "\\&")
        .replace('%', "\\%")
        .replace('$', "\\$")
        .replace('#', "\\#")
        .replace('_', "\\_")
        .replace('{', "\\{")
        .replace('}', "\\}")
}
