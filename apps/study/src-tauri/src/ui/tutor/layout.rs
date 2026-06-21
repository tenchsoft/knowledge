use tench_ui::prelude::{Rect, Size};

pub fn modal_close_rect(size: Size) -> Rect {
    let modal = modal_rect(size);
    Rect::new(
        modal.x1 - 44.0,
        modal.y0 + 14.0,
        modal.x1 - 16.0,
        modal.y0 + 42.0,
    )
}

pub(crate) fn modal_rect(size: Size) -> Rect {
    let w = 420.0_f64.min(size.width - 48.0).max(280.0);
    let h = 260.0_f64.min(size.height - 48.0).max(200.0);
    let x = (size.width - w) / 2.0;
    let y = (size.height - h) / 2.0;
    Rect::new(x, y, x + w, y + h)
}
