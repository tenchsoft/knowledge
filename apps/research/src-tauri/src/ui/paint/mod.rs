mod center;
mod header;
mod inspector;
mod left;
mod overlays;

use super::{research_regions, ResearchApp, HEADER_H};
use tench_ui::prelude::*;

impl ResearchApp {
    pub(super) fn paint_impl(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let size = ctx.size();
        let theme = ctx.theme();
        let mut p = Painter::new(scene);
        let regions = research_regions(size);

        // Background
        p.fill_background(size, theme.background);

        let header_h = HEADER_H;
        let right_w = regions.right.width();
        let spacing = theme.spacing;
        let spacing_large = theme.spacing_large;

        self.paint_header(&mut p, size, theme, regions);

        self.paint_left_panel(&mut p, size, theme, regions, spacing, spacing_large);

        let Some(paper) = self.paint_center_panel(&mut p, theme, regions, spacing, spacing_large)
        else {
            return;
        };
        let center_right = regions.center.x1;

        // Center-right separator
        if right_w >= 160.0 {
            p.draw_line(
                Point::new(center_right, header_h),
                Point::new(center_right, size.height),
                theme.border,
                1.0,
            );
            self.paint_inspector_panel(
                &mut p,
                inspector::InspectorPanelContext {
                    size,
                    theme,
                    center_right,
                    right_w,
                    spacing,
                    spacing_large,
                    paper,
                },
            );
        } else {
            return;
        }

        self.paint_overlays(&mut p, size, theme);
    }
}
