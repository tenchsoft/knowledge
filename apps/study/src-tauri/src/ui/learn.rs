use tench_ui::parley::FontWeight;
use tench_ui::prelude::*;
use tench_ui::render::Painter;

use super::curriculum::StudyRegions;
use super::state::{
    StudyFocusTarget, StudyState, ACCENT_STUDY, NEUTRAL_100, NEUTRAL_300, NEUTRAL_400, NEUTRAL_500,
    NEUTRAL_600, NEUTRAL_700, NEUTRAL_800, STATUS_ERROR, STATUS_RUNNING,
};

pub fn paint_learn_surface(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    let surface = regions.surface;
    p.fill_rect(surface, NEUTRAL_800);
    let x = surface.x0 + 32.0;
    let y = surface.y0 + 34.0;
    p.draw_text(
        &state.active_concept().label,
        x,
        y,
        NEUTRAL_100,
        18.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &state.active_concept().summary,
        x,
        y + 38.0,
        NEUTRAL_100,
        16.0,
        FontWeight::NORMAL,
        false,
    );
    let definition = Rect::new(x, y + 70.0, surface.x1 - 32.0, y + 154.0);
    p.fill_rounded_rect(definition, NEUTRAL_700, 4.0);
    p.fill_rect(
        Rect::new(
            definition.x0,
            definition.y0,
            definition.x0 + 3.0,
            definition.y1,
        ),
        ACCENT_STUDY,
    );
    p.draw_text(
        t("study.learn.definition"),
        definition.x0 + 16.0,
        definition.y0 + 25.0,
        ACCENT_STUDY,
        11.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &format!(
            "{} {}",
            state.active_concept().visual_count,
            t("study.learn.visuals_available")
        ),
        definition.x0 + 16.0,
        definition.y0 + 56.0,
        NEUTRAL_100,
        13.0,
        FontWeight::NORMAL,
        false,
    );

    let example = Rect::new(x, y + 176.0, surface.x1 - 32.0, y + 260.0);
    p.fill_rounded_rect(example, NEUTRAL_700, 6.0);
    p.draw_text(
        t("study.learn.example"),
        example.x0 + 16.0,
        example.y0 + 25.0,
        NEUTRAL_300,
        11.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        &format!(
            "{} {} {} {}",
            state.active_concept().label,
            t("study.learn.example_prefix"),
            state.active_unit().label,
            t("study.learn.example_suffix")
        ),
        example.x0 + 16.0,
        example.y0 + 56.0,
        NEUTRAL_100,
        13.0,
        FontWeight::NORMAL,
        false,
    );

    let check = Rect::new(x, y + 292.0, surface.x1 - 32.0, y + 390.0);
    p.fill_rounded_rect(check, NEUTRAL_700, 8.0);
    p.stroke_rounded_rect(check, NEUTRAL_600, 1.0, 8.0);
    p.draw_text(
        t("study.learn.quick_check"),
        check.x0 + 16.0,
        check.y0 + 25.0,
        STATUS_RUNNING,
        11.0,
        FontWeight::BOLD,
        false,
    );
    p.draw_text(
        t("study.learn.quick_question"),
        check.x0 + 16.0,
        check.y0 + 54.0,
        NEUTRAL_100,
        13.0,
        FontWeight::BOLD,
        false,
    );

    let visual_rect = Rect::new(x, y + 406.0, surface.x1 - 32.0, y + 520.0);
    paint_active_visual_surface(p, state, visual_rect, t("study.learn.visual"));

    // Phase 8: Visual play/pause and timeline controls
    let play_btn = Rect::new(x, y + 528.0, x + 32.0, y + 548.0);
    p.stroke_rounded_rect(play_btn, NEUTRAL_500, 1.0, 4.0);
    p.draw_text(
        if state.visual_playing { "||" } else { ">" },
        play_btn.x0 + 10.0,
        play_btn.y0 + 14.0,
        if state.visual_playing {
            ACCENT_STUDY
        } else {
            NEUTRAL_100
        },
        10.0,
        FontWeight::BOLD,
        false,
    );

    // Phase 8: Timeline scrubber
    let scrubber = Rect::new(x + 40.0, y + 534.0, surface.x1 - 100.0, y + 542.0);
    p.fill_rounded_rect(scrubber, NEUTRAL_600, 2.0);
    let filled_w = scrubber.width() * state.visual_timeline_position;
    if filled_w > 0.0 {
        p.fill_rounded_rect(
            Rect::new(
                scrubber.x0,
                scrubber.y0,
                scrubber.x0 + filled_w,
                scrubber.y1,
            ),
            ACCENT_STUDY,
            2.0,
        );
    }

    // Phase 8: Autoplay toggle
    let autoplay_btn = Rect::new(surface.x1 - 92.0, y + 528.0, surface.x1 - 32.0, y + 548.0);
    p.stroke_rounded_rect(autoplay_btn, NEUTRAL_500, 1.0, 4.0);
    p.draw_text(
        if state.visual_autoplay {
            "Auto: ON"
        } else {
            "Auto: OFF"
        },
        autoplay_btn.x0 + 6.0,
        autoplay_btn.y0 + 14.0,
        if state.visual_autoplay {
            ACCENT_STUDY
        } else {
            NEUTRAL_400
        },
        9.0,
        FontWeight::NORMAL,
        false,
    );

    let start = start_practice_rect(surface);
    p.fill_rounded_rect(start, ACCENT_STUDY, 6.0);
    p.draw_text(
        t("study.learn.start_practice"),
        start.x0 + 16.0,
        start.y0 + 24.0,
        NEUTRAL_800,
        13.0,
        FontWeight::BOLD,
        false,
    );

    let review_concept = Rect::new(start.x1 + 10.0, start.y0, start.x1 + 134.0, start.y1);
    p.fill_rounded_rect(review_concept, NEUTRAL_700, 6.0);
    p.stroke_rounded_rect(review_concept, ACCENT_STUDY, 1.0, 6.0);
    p.draw_text(
        t("study.learn.review_concept"),
        review_concept.x0 + 14.0,
        review_concept.y0 + 24.0,
        NEUTRAL_100,
        13.0,
        FontWeight::NORMAL,
        false,
    );
}

pub fn start_practice_rect(surface: Rect) -> Rect {
    Rect::new(
        surface.x0 + 32.0,
        surface.y0 + 536.0,
        surface.x0 + 148.0,
        surface.y0 + 572.0,
    )
}

pub fn review_concept_rect(surface: Rect) -> Rect {
    let start = start_practice_rect(surface);
    Rect::new(start.x1 + 10.0, start.y0, start.x1 + 134.0, start.y1)
}

/// Phase 8: Timeline scrubber rect for hit testing and automation
pub fn scrubber_rect(surface: Rect) -> Rect {
    let x = surface.x0 + 32.0;
    let y = surface.y0 + 34.0;
    Rect::new(x + 40.0, y + 534.0, surface.x1 - 100.0, y + 542.0)
}

fn paint_active_visual_surface(p: &mut Painter<'_>, state: &StudyState, rect: Rect, label: &str) {
    let Some(plan) = state.active_visual_draw_plan() else {
        p.fill_rounded_rect(rect, NEUTRAL_700, 6.0);
        p.stroke_rounded_rect(rect, NEUTRAL_600, 1.0, 6.0);
        p.draw_text(
            label,
            rect.x0 + 12.0,
            rect.y0 + 23.0,
            ACCENT_STUDY,
            11.0,
            FontWeight::BOLD,
            false,
        );
        return;
    };
    let viewport = VisualSurfaceViewport {
        timeline_position: plan.frame.timeline_position,
        reduced_motion: plan.reduced_motion,
        ..VisualSurfaceViewport::default()
    };
    let surface = VisualSurface::new(visual_surface_commands(&plan))
        .with_viewport(viewport)
        .with_accessibility_summary(plan.accessibility.alt_text);
    surface.paint_in_rect(p, rect, &study_visual_theme());
    p.draw_text(
        label,
        rect.x0 + 12.0,
        rect.y0 + 20.0,
        ACCENT_STUDY,
        11.0,
        FontWeight::BOLD,
        false,
    );
}

fn visual_surface_commands(
    plan: &tench_study_core::LearningVisualDrawPlan,
) -> Vec<VisualSurfaceCommand> {
    let mut label_row = 0.0;
    plan.commands
        .iter()
        .map(|command| match command {
            tench_study_core::LearningVisualDrawCommand::Axis2d { x_label, y_label } => {
                VisualSurfaceCommand {
                    id: "axis".to_string(),
                    kind: VisualSurfaceCommandKind::Axis2d {
                        x_label: x_label.clone(),
                        y_label: y_label.clone(),
                    },
                    label: None,
                    color: NEUTRAL_300,
                }
            }
            tench_study_core::LearningVisualDrawCommand::Shape2d {
                id,
                role,
                progress,
                selected,
            } => VisualSurfaceCommand {
                id: id.clone(),
                kind: VisualSurfaceCommandKind::Shape2d {
                    unit_rect: Rect::new(0.18, 0.30, 0.82, 0.66),
                    progress: *progress,
                    selected: *selected,
                },
                label: Some(role.clone()),
                color: ACCENT_STUDY,
            },
            tench_study_core::LearningVisualDrawCommand::Shape3d {
                id,
                role,
                rotation,
                selected,
                ..
            } => VisualSurfaceCommand {
                id: id.clone(),
                kind: VisualSurfaceCommandKind::Shape3dProxy {
                    unit_rect: Rect::new(0.34, 0.24, 0.66, 0.72),
                    rotation: *rotation,
                    selected: *selected,
                },
                label: Some(role.clone()),
                color: ACCENT_STUDY,
            },
            tench_study_core::LearningVisualDrawCommand::TimelineCursor { position } => {
                VisualSurfaceCommand {
                    id: "timeline-cursor".to_string(),
                    kind: VisualSurfaceCommandKind::TimelineCursor {
                        position: *position,
                    },
                    label: None,
                    color: ACCENT_STUDY,
                }
            }
            tench_study_core::LearningVisualDrawCommand::ParameterControl { name, value } => {
                VisualSurfaceCommand {
                    id: format!("parameter-{name}"),
                    kind: VisualSurfaceCommandKind::ParameterMarker {
                        unit_track: Rect::new(0.18, 0.78, 0.82, 0.82),
                        value: (*value / 3.0).clamp(0.0, 1.0),
                    },
                    label: Some(name.clone()),
                    color: STATUS_RUNNING,
                }
            }
            tench_study_core::LearningVisualDrawCommand::TextLabel { id, text } => {
                label_row += 0.07;
                VisualSurfaceCommand {
                    id: id.clone(),
                    kind: VisualSurfaceCommandKind::TextLabel {
                        unit_position: Point::new(0.12, 0.20 + label_row),
                        text: text.clone(),
                    },
                    label: None,
                    color: NEUTRAL_100,
                }
            }
            tench_study_core::LearningVisualDrawCommand::LocalizedTextLabel {
                id, text, ..
            } => {
                label_row += 0.07;
                VisualSurfaceCommand {
                    id: id.clone(),
                    kind: VisualSurfaceCommandKind::TextLabel {
                        unit_position: Point::new(0.12, 0.20 + label_row),
                        text: text.clone(),
                    },
                    label: None,
                    color: NEUTRAL_100,
                }
            }
            tench_study_core::LearningVisualDrawCommand::Layer { id, visible } => {
                label_row += 0.07;
                VisualSurfaceCommand {
                    id: format!("layer-{id}"),
                    kind: VisualSurfaceCommandKind::TextLabel {
                        unit_position: Point::new(0.12, 0.20 + label_row),
                        text: format!("{id}: {visible}"),
                    },
                    label: None,
                    color: NEUTRAL_300,
                }
            }
        })
        .collect()
}

fn study_visual_theme() -> Theme {
    Theme {
        background: NEUTRAL_700,
        surface: NEUTRAL_600,
        primary: ACCENT_STUDY,
        secondary: NEUTRAL_300,
        on_background: NEUTRAL_100,
        on_surface: NEUTRAL_100,
        on_primary: NEUTRAL_800,
        error: STATUS_ERROR,
        border: NEUTRAL_600,
        disabled: NEUTRAL_500,
        font_size: 12.0,
        font_size_small: 10.0,
        font_size_large: 15.0,
        spacing: 8.0,
        spacing_small: 4.0,
        spacing_large: 12.0,
        border_radius: 6.0,
        button_height: 32.0,
        input_height: 30.0,
    }
}

// Phase 2: Notes panel overlay in Learn mode

pub fn paint_notes_panel(
    p: &mut Painter<'_>,
    state: &StudyState,
    regions: &StudyRegions,
    i18n: &tench_app_core::I18nCatalog,
) {
    let t = |key| crate::i18n::resolve(i18n, key);
    let surface = regions.surface;
    // Draw notes panel as an overlay on the right side of the surface
    let panel_w = 280.0_f64.min(surface.width() * 0.4);
    let panel = Rect::new(surface.x1 - panel_w, surface.y0, surface.x1, surface.y1);
    // Dim the background slightly
    p.fill_rect(panel, Color::rgba8(0x1A, 0x1A, 0x1A, 240));
    p.draw_line(
        Point::new(panel.x0, panel.y0),
        Point::new(panel.x0, panel.y1),
        NEUTRAL_600,
        1.0,
    );

    p.draw_text(
        t("study.notes.title"),
        panel.x0 + 16.0,
        panel.y0 + 24.0,
        NEUTRAL_100,
        14.0,
        FontWeight::BOLD,
        false,
    );

    // Notes input
    let input_rect = Rect::new(
        panel.x0 + 12.0,
        panel.y0 + 40.0,
        panel.x1 - 12.0,
        panel.y0 + 72.0,
    );
    p.fill_rounded_rect(input_rect, NEUTRAL_700, 6.0);
    let border_color = if state.focus_target == StudyFocusTarget::NotesInput {
        ACCENT_STUDY
    } else {
        NEUTRAL_500
    };
    p.stroke_rounded_rect(input_rect, border_color, 1.0, 6.0);
    if state.notes_input.is_empty() {
        p.draw_text(
            t("study.notes.placeholder"),
            input_rect.x0 + 8.0,
            input_rect.y0 + 19.0,
            NEUTRAL_500,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    } else {
        p.draw_text(
            &state.notes_input,
            input_rect.x0 + 8.0,
            input_rect.y0 + 19.0,
            NEUTRAL_100,
            12.0,
            FontWeight::NORMAL,
            false,
        );
    }

    // Save button
    let save_btn = Rect::new(
        panel.x1 - 80.0,
        panel.y0 + 78.0,
        panel.x1 - 12.0,
        panel.y0 + 104.0,
    );
    p.fill_rounded_rect(save_btn, ACCENT_STUDY, 6.0);
    p.draw_text(
        t("study.notes.save"),
        save_btn.x0 + 14.0,
        save_btn.y0 + 17.0,
        NEUTRAL_800,
        11.0,
        FontWeight::BOLD,
        false,
    );

    // Existing notes list
    let notes = state
        .notes
        .iter()
        .filter(|note| note.concept_id == state.active_concept().id);
    for (idx, note) in notes.enumerate() {
        let y = panel.y0 + 118.0 + idx as f64 * 48.0;
        if y + 48.0 > panel.y1 {
            break;
        }
        let note_rect = Rect::new(panel.x0 + 12.0, y, panel.x1 - 12.0, y + 44.0);
        p.fill_rounded_rect(note_rect, NEUTRAL_700, 4.0);
        p.draw_text(
            &note.text,
            note_rect.x0 + 8.0,
            note_rect.y0 + 17.0,
            NEUTRAL_100,
            11.0,
            FontWeight::NORMAL,
            false,
        );
        p.draw_text(
            &note.created_at,
            note_rect.x0 + 8.0,
            note_rect.y0 + 33.0,
            NEUTRAL_500,
            9.0,
            FontWeight::NORMAL,
            false,
        );
    }
}

pub(crate) fn notes_panel_rect(regions: &StudyRegions) -> Rect {
    let surface = regions.surface;
    let panel_w = 280.0_f64.min(surface.width() * 0.4);
    Rect::new(surface.x1 - panel_w, surface.y0, surface.x1, surface.y1)
}

pub(crate) fn notes_input_rect(regions: &StudyRegions) -> Rect {
    let panel = notes_panel_rect(regions);
    Rect::new(
        panel.x0 + 12.0,
        panel.y0 + 40.0,
        panel.x1 - 12.0,
        panel.y0 + 72.0,
    )
}

pub(crate) fn notes_save_rect(regions: &StudyRegions) -> Rect {
    let panel = notes_panel_rect(regions);
    Rect::new(
        panel.x1 - 80.0,
        panel.y0 + 78.0,
        panel.x1 - 12.0,
        panel.y0 + 104.0,
    )
}

pub(crate) fn note_row_rect(regions: &StudyRegions, row: usize) -> Rect {
    let panel = notes_panel_rect(regions);
    let y = panel.y0 + 118.0 + row as f64 * 48.0;
    Rect::new(panel.x0 + 12.0, y, panel.x1 - 12.0, y + 44.0)
}
