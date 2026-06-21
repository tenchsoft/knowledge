pub const INSPECTOR_TAB_COUNT: usize = 6;

pub fn hit_test_tab(x: f64, y: f64, panel_x: f64, header_h: f64, spacing: f64) -> Option<usize> {
    let tab_y0 = header_h + 16.0 - 10.0;
    let tab_y1 = header_h + 16.0 + 8.0;
    if y < tab_y0 || y > tab_y1 {
        return None;
    }
    let tab = ((x - panel_x - spacing) / 44.0) as usize;
    (tab < INSPECTOR_TAB_COUNT).then_some(tab)
}
