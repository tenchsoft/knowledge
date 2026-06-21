use super::*;

pub fn render_pdf_page(
    request: PdfRenderRequest,
    document_text: Option<&PdfDocumentText>,
) -> RenderedPdfPage {
    let (width, height) = pdf_render_dimensions(request.zoom, request.max_dimension_px);
    let mut pixels = vec![255u8; (width * height * 4) as usize];
    let theme = request.theme.unwrap_or(PdfReaderTheme::Paper);
    let (r, g, b) = match theme {
        PdfReaderTheme::Paper => (248, 249, 251),
        PdfReaderTheme::Dark => (32, 36, 46),
        PdfReaderTheme::Sepia => (250, 244, 224),
    };
    fill_rgba(&mut pixels, r, g, b);
    draw_page_frame(&mut pixels, width, height, theme);

    let page_text = document_text
        .and_then(|document| document.pages.iter().find(|page| page.page == request.page))
        .or_else(|| document_text.and_then(|document| document.pages.first()))
        .map(|page| page.text.trim())
        .filter(|text| !text.is_empty());
    let render_quality = if let Some(text) = page_text {
        draw_pdf_text_preview(&mut pixels, width, height, theme, text);
        PdfRenderQuality::TextPreview
    } else {
        draw_pdf_document_shell(&mut pixels, width, height, theme, request.page);
        PdfRenderQuality::DocumentShell
    };
    let cache_key = PdfCacheKey {
        attachment_id: request.attachment_id.clone(),
        attachment_hash: request.attachment_hash,
        page: Some(request.page),
        annotation_updated_at: None,
        kind: PdfCacheKind::RenderedPageBitmap,
    };
    let accessibility_summary = match render_quality {
        PdfRenderQuality::NativeBitmap => format!("PDF page {} rendered bitmap.", request.page),
        PdfRenderQuality::TextPreview => format!(
            "PDF page {} rendered as text preview from extracted document text.",
            request.page
        ),
        PdfRenderQuality::DocumentShell => format!(
            "PDF page {} opened without extractable text for this page.",
            request.page
        ),
    };
    RenderedPdfPage {
        attachment_id: request.attachment_id,
        page: request.page,
        width_px: width,
        height_px: height,
        pixel_format: PdfPixelFormat::Rgba8,
        pixels,
        cache_key,
        render_quality,
        accessibility_summary,
    }
}

pub fn render_pdf_page_from_bytes(
    request: PdfRenderRequest,
    bytes: &[u8],
    locale: Option<crate::ResearchLocale>,
) -> Result<RenderedPdfPage, String> {
    let open = inspect_pdf_document_bytes(request.attachment_id.clone(), bytes);
    match open.metadata.status {
        PdfDocumentStatus::Ready | PdfDocumentStatus::ImageOnly => {}
        PdfDocumentStatus::Encrypted => return Err("encrypted PDF render blocked".into()),
        PdfDocumentStatus::Corrupt => return Err("corrupt PDF cannot be rendered".into()),
        PdfDocumentStatus::Unsupported => return Err("unsupported PDF format".into()),
    }
    let text = extract_pdf_literal_text(request.attachment_id.clone(), bytes, locale)?;
    Ok(render_pdf_page(request, Some(&text)))
}

pub fn pdf_page_cache_window(current_page: u32, page_count: u32, radius: u32) -> Vec<u32> {
    if page_count == 0 {
        return Vec::new();
    }
    let current_page = current_page.clamp(1, page_count);
    let start = current_page.saturating_sub(radius).max(1);
    let end = current_page.saturating_add(radius).min(page_count);
    (start..=end).collect()
}

pub fn build_pdf_thumbnail_strip(
    request: PdfThumbnailRequest,
    document_text: Option<&PdfDocumentText>,
) -> PdfThumbnailStrip {
    let pages = pdf_page_cache_window(request.current_page, request.page_count, request.radius);
    let selected_page = request.current_page.clamp(1, request.page_count.max(1));
    let thumbnails = pages
        .iter()
        .map(|page| {
            let rendered = render_pdf_page(
                PdfRenderRequest {
                    attachment_id: request.attachment_id.clone(),
                    attachment_hash: request.attachment_hash.clone(),
                    page: *page,
                    zoom: 0.25,
                    max_dimension_px: request.max_dimension_px,
                    theme: request.theme,
                },
                document_text,
            );
            PdfThumbnail {
                page: *page,
                width_px: rendered.width_px,
                height_px: rendered.height_px,
                pixel_format: rendered.pixel_format,
                pixels: rendered.pixels,
                cache_key: PdfCacheKey {
                    kind: PdfCacheKind::Thumbnail,
                    ..rendered.cache_key
                },
                selected: *page == selected_page,
                accessibility_summary: format!(
                    "PDF page {} thumbnail{}.",
                    page,
                    if *page == selected_page {
                        " selected"
                    } else {
                        ""
                    }
                ),
            }
        })
        .collect();

    PdfThumbnailStrip {
        attachment_id: request.attachment_id,
        current_page: selected_page,
        page_count: request.page_count,
        pages,
        thumbnails,
    }
}

pub fn pdf_render_job_descriptor(
    request: &PdfRenderRequest,
    batch_id: impl Into<String>,
) -> JobDescriptor {
    let mut descriptor = crate::research_job_descriptor(
        format!(
            "pdf-render-{}-{}",
            stable_cache_token(request.attachment_id.as_str()),
            request.page
        ),
        crate::ResearchJobKind::RenderPdfPage,
        JobState::Queued,
        batch_id,
    );
    descriptor.payload = json!({
        "batch_id": descriptor.payload["batch_id"].clone(),
        "attachment_id": request.attachment_id.as_str(),
        "attachment_hash": &request.attachment_hash,
        "page": request.page,
        "zoom": request.zoom,
        "max_dimension_px": request.max_dimension_px,
        "theme": request.theme
    });
    descriptor
}

fn pdf_render_dimensions(zoom: f32, max_dimension_px: u32) -> (u32, u32) {
    let max_dimension = max_dimension_px.max(64) as f32;
    let width = (612.0 * zoom.max(0.1)).max(64.0);
    let height = (792.0 * zoom.max(0.1)).max(64.0);
    let scale = (max_dimension / width.max(height)).min(1.0);
    (
        (width * scale).round().max(64.0) as u32,
        (height * scale).round().max(64.0) as u32,
    )
}

fn fill_rgba(pixels: &mut [u8], r: u8, g: u8, b: u8) {
    for pixel in pixels.chunks_exact_mut(4) {
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
        pixel[3] = 255;
    }
}

fn draw_page_frame(pixels: &mut [u8], width: u32, height: u32, theme: PdfReaderTheme) {
    let color = match theme {
        PdfReaderTheme::Paper | PdfReaderTheme::Sepia => (196, 202, 211, 255),
        PdfReaderTheme::Dark => (82, 91, 110, 255),
    };
    draw_rect_outline(pixels, width, height, 2, 2, width - 4, height - 4, color);
}

fn draw_pdf_document_shell(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    theme: PdfReaderTheme,
    page: u32,
) {
    let ink = pdf_ink_color(theme);
    let margin = (width / 10).max(8);
    let top = (height / 8).max(10);
    for row in 0..5 {
        let y = top + row * 18;
        let line_width = width.saturating_sub(margin * 2 + row * 9).max(12);
        draw_filled_rect(pixels, width, height, margin, y, line_width, 4, ink);
    }
    draw_preview_token(
        pixels,
        width,
        height,
        margin,
        height.saturating_sub(top + 16),
        &format!("page {page}"),
        theme,
    );
}

fn draw_pdf_text_preview(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    theme: PdfReaderTheme,
    text: &str,
) {
    let margin_x = (width / 12).max(8);
    let mut y = (height / 12).max(8);
    let line_height = 12;
    let max_lines = ((height.saturating_sub(y * 2)) / line_height).max(1) as usize;
    for line in wrap_preview_text(text, 72).into_iter().take(max_lines) {
        draw_preview_token(pixels, width, height, margin_x, y, &line, theme);
        y += line_height;
    }
}

fn wrap_preview_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let next_len =
            current.chars().count() + word.chars().count() + usize::from(!current.is_empty());
        if next_len > max_chars && !current.is_empty() {
            lines.push(current);
            current = String::new();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn draw_preview_token(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    text: &str,
    theme: PdfReaderTheme,
) {
    let ink = pdf_ink_color(theme);
    let mut cursor_x = x;
    let max_x = width.saturating_sub(x.max(4));
    for ch in text.chars() {
        if cursor_x >= max_x {
            break;
        }
        if ch.is_whitespace() {
            cursor_x += 4;
            continue;
        }
        let char_width = if ch.is_ascii_punctuation() { 3 } else { 5 };
        draw_hashed_glyph(pixels, width, height, cursor_x, y, char_width, 8, ch, ink);
        cursor_x += char_width + 2;
    }
}

// Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
#[allow(clippy::too_many_arguments)]
fn draw_hashed_glyph(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    glyph_width: u32,
    glyph_height: u32,
    ch: char,
    color: (u8, u8, u8, u8),
) {
    let mut hash = ch as u32;
    hash ^= hash.rotate_left(13);
    hash = hash.wrapping_mul(0x45d9f3b);
    for dy in 0..glyph_height {
        for dx in 0..glyph_width {
            let edge = dy == 0 || dy + 1 == glyph_height || dx == 0 || dx + 1 == glyph_width;
            let bit = (hash >> ((dx + dy * glyph_width) % 24)) & 1 == 1;
            if edge || bit {
                set_pixel(pixels, width, height, x + dx, y + dy, color);
            }
        }
    }
}

// Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
#[allow(clippy::too_many_arguments)]
fn draw_rect_outline(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    rect_width: u32,
    rect_height: u32,
    color: (u8, u8, u8, u8),
) {
    if rect_width == 0 || rect_height == 0 {
        return;
    }
    draw_filled_rect(pixels, width, height, x, y, rect_width, 1, color);
    draw_filled_rect(
        pixels,
        width,
        height,
        x,
        y + rect_height.saturating_sub(1),
        rect_width,
        1,
        color,
    );
    draw_filled_rect(pixels, width, height, x, y, 1, rect_height, color);
    draw_filled_rect(
        pixels,
        width,
        height,
        x + rect_width.saturating_sub(1),
        y,
        1,
        rect_height,
        color,
    );
}

// Pixel-level drawing primitives use many coordinate/size/color parameters by nature.
#[allow(clippy::too_many_arguments)]
fn draw_filled_rect(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    rect_width: u32,
    rect_height: u32,
    color: (u8, u8, u8, u8),
) {
    for py in y..(y + rect_height).min(height) {
        for px in x..(x + rect_width).min(width) {
            set_pixel(pixels, width, height, px, py, color);
        }
    }
}

fn set_pixel(pixels: &mut [u8], width: u32, height: u32, x: u32, y: u32, color: (u8, u8, u8, u8)) {
    if x >= width || y >= height {
        return;
    }
    let index = ((y * width + x) * 4) as usize;
    if index + 3 >= pixels.len() {
        return;
    }
    pixels[index] = color.0;
    pixels[index + 1] = color.1;
    pixels[index + 2] = color.2;
    pixels[index + 3] = color.3;
}

fn pdf_ink_color(theme: PdfReaderTheme) -> (u8, u8, u8, u8) {
    match theme {
        PdfReaderTheme::Paper => (48, 54, 66, 255),
        PdfReaderTheme::Dark => (224, 229, 238, 255),
        PdfReaderTheme::Sepia => (82, 65, 42, 255),
    }
}
