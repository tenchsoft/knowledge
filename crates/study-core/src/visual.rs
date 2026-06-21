use serde::{Deserialize, Serialize};

use crate::{ContentLocale, CurriculumNodeId, LocalizedText, TextDirection};

crate::study_id_type!(LearningVisualId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LearningVisualSpec {
    pub id: LearningVisualId,
    pub node_id: CurriculumNodeId,
    pub kind: LearningVisualKind,
    pub title: LocalizedText,
    pub description: LocalizedText,
    pub renderer: VisualRenderer,
    pub playback: VisualPlayback,
    #[serde(default)]
    pub interactions: Vec<VisualInteraction>,
    pub accessibility: VisualAccessibility,
    #[serde(default)]
    pub locale: Option<ContentLocale>,
}

impl LearningVisualSpec {
    pub fn validate_for_release(&self) -> Result<(), String> {
        if self.title.value.trim().is_empty() {
            return Err("visual title is required".to_string());
        }
        if self.accessibility.alt_text.trim().is_empty()
            && self.accessibility.table_fallback_ref.is_none()
        {
            return Err("visual requires alt text or table fallback".to_string());
        }
        if self.playback.animated && !self.playback.reduced_motion_fallback {
            return Err("animated visuals require reduced-motion fallback".to_string());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningVisualKind {
    NumberLine,
    FunctionGraph,
    GeometryConstruction,
    MatrixTransformation,
    ProbabilitySimulation,
    MoleculeModel,
    OrganSystemModel,
    CellSimulation,
    ForceSimulation,
    FieldVisualization,
    OrbitalModel,
    ClimateTimeline,
    PhonemeMap,
    GrammarTree,
    ArgumentMap,
    StrokeOrder,
    TranslationAlignment,
    DialogueRoleplay,
    AlgorithmAnimation,
    MemoryModel,
    SystemTopology,
    Custom,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualRenderer {
    pub engine: VisualRendererEngine,
    pub spec_version: u32,
    pub scene_ref: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualRendererEngine {
    Tench2d,
    Tench3d,
    Plot,
    RichTextOverlay,
    CodeTrace,
    NativeWidget,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualPlayback {
    pub animated: bool,
    pub autoplay: bool,
    pub duration_ms: Option<u32>,
    pub timeline_position: f32,
    pub reduced_motion_fallback: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum VisualInteraction {
    PanZoom,
    Rotate3d,
    ScrubTimeline,
    DragPoint { point_id: String },
    ToggleLayer { layer_id: String },
    AdjustParameter { name: String, min: f64, max: f64 },
    SelectNode,
    EditConstruction,
    RunStep,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualAccessibility {
    pub alt_text: String,
    #[serde(default)]
    pub transcript: Option<String>,
    #[serde(default)]
    pub table_fallback_ref: Option<String>,
    #[serde(default)]
    pub keyboard_model: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualRuntimeState {
    pub visual_id: LearningVisualId,
    #[serde(default)]
    pub selected_id: Option<String>,
    #[serde(default)]
    pub active_layers: Vec<String>,
    #[serde(default)]
    pub parameter_values: Vec<VisualParameterValue>,
    pub playback: VisualPlayback,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LearningVisualDrawPlan {
    pub visual_id: LearningVisualId,
    pub renderer: VisualRenderer,
    pub frame: VisualFrame,
    #[serde(default)]
    pub commands: Vec<LearningVisualDrawCommand>,
    pub accessibility: VisualAccessibility,
    #[serde(default)]
    pub table_fallback: Vec<LearningVisualTableRow>,
    pub reduced_motion: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LearningVisualTableRow {
    pub label: String,
    #[serde(default)]
    pub cells: Vec<LearningVisualTableCell>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LearningVisualTableCell {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualFrame {
    pub timeline_position: f32,
    pub selected_id: Option<String>,
    #[serde(default)]
    pub active_layers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum LearningVisualDrawCommand {
    Axis2d {
        x_label: String,
        y_label: String,
    },
    Shape2d {
        id: String,
        role: String,
        progress: f32,
        selected: bool,
    },
    Shape3d {
        id: String,
        role: String,
        rotation: f32,
        visible: bool,
        selected: bool,
    },
    Layer {
        id: String,
        visible: bool,
    },
    TimelineCursor {
        position: f32,
    },
    ParameterControl {
        name: String,
        value: f64,
    },
    TextLabel {
        id: String,
        text: String,
    },
    LocalizedTextLabel {
        id: String,
        text: String,
        locale: ContentLocale,
        direction: TextDirection,
    },
}

impl VisualRuntimeState {
    pub fn apply_action(mut self, action: VisualRuntimeAction) -> Self {
        match action {
            VisualRuntimeAction::Step { delta_ms } => {
                advance_playback(&mut self.playback, delta_ms);
            }
            VisualRuntimeAction::Play => {
                self.playback.autoplay = true;
            }
            VisualRuntimeAction::Pause => {
                self.playback.autoplay = false;
            }
            VisualRuntimeAction::Replay => {
                self.playback.timeline_position = 0.0;
                self.playback.autoplay = true;
                self.selected_id = None;
            }
            VisualRuntimeAction::Reset => {
                self.playback.timeline_position = 0.0;
                self.playback.autoplay = false;
                self.selected_id = None;
            }
            VisualRuntimeAction::SetTimeline { position } => {
                self.playback.timeline_position = position.clamp(0.0, 1.0);
            }
            VisualRuntimeAction::ToggleLayer { layer_id } => {
                if let Some(index) = self.active_layers.iter().position(|id| id == &layer_id) {
                    self.active_layers.remove(index);
                } else {
                    self.active_layers.push(layer_id);
                }
            }
            VisualRuntimeAction::SetParameter { name, value } => {
                if let Some(parameter) = self
                    .parameter_values
                    .iter_mut()
                    .find(|parameter| parameter.name == name)
                {
                    parameter.value = value;
                } else {
                    self.parameter_values
                        .push(VisualParameterValue { name, value });
                }
            }
            VisualRuntimeAction::Select { id } => {
                self.selected_id = id;
            }
        }
        self
    }
}

pub fn build_learning_visual_draw_plan(
    spec: &LearningVisualSpec,
    state: &VisualRuntimeState,
    reduced_motion: bool,
) -> Result<LearningVisualDrawPlan, String> {
    spec.validate_for_release()?;
    if spec.id != state.visual_id {
        return Err(format!(
            "runtime state {} does not match visual {}",
            state.visual_id.as_str(),
            spec.id.as_str()
        ));
    }

    let position = if reduced_motion {
        1.0
    } else {
        state.playback.timeline_position.clamp(0.0, 1.0)
    };
    let mut commands = Vec::new();
    if spec.playback.animated {
        commands.push(LearningVisualDrawCommand::TimelineCursor { position });
    }
    for interaction in &spec.interactions {
        match interaction {
            VisualInteraction::ToggleLayer { layer_id } => {
                commands.push(LearningVisualDrawCommand::Layer {
                    id: layer_id.clone(),
                    visible: state.active_layers.contains(layer_id),
                });
            }
            VisualInteraction::AdjustParameter { name, .. } => {
                let value = state
                    .parameter_values
                    .iter()
                    .find(|parameter| parameter.name == *name)
                    .map(|parameter| parameter.value)
                    .unwrap_or_default();
                commands.push(LearningVisualDrawCommand::ParameterControl {
                    name: name.clone(),
                    value,
                });
            }
            _ => {}
        }
    }

    match spec.kind {
        LearningVisualKind::MoleculeModel
        | LearningVisualKind::OrganSystemModel
        | LearningVisualKind::OrbitalModel
        | LearningVisualKind::SystemTopology => {
            commands.push(LearningVisualDrawCommand::Shape3d {
                id: spec.id.as_str().to_string(),
                role: learning_visual_role(spec.kind).to_string(),
                rotation: position,
                visible: true,
                selected: state.selected_id.as_deref() == Some(spec.id.as_str()),
            });
        }
        LearningVisualKind::FunctionGraph
        | LearningVisualKind::NumberLine
        | LearningVisualKind::GeometryConstruction
        | LearningVisualKind::MatrixTransformation
        | LearningVisualKind::ProbabilitySimulation
        | LearningVisualKind::ForceSimulation
        | LearningVisualKind::FieldVisualization => {
            commands.push(LearningVisualDrawCommand::Axis2d {
                x_label: "x".to_string(),
                y_label: "y".to_string(),
            });
            commands.push(LearningVisualDrawCommand::Shape2d {
                id: spec.id.as_str().to_string(),
                role: learning_visual_role(spec.kind).to_string(),
                progress: position,
                selected: state.selected_id.as_deref() == Some(spec.id.as_str()),
            });
        }
        LearningVisualKind::CellSimulation
        | LearningVisualKind::ClimateTimeline
        | LearningVisualKind::PhonemeMap
        | LearningVisualKind::GrammarTree
        | LearningVisualKind::ArgumentMap
        | LearningVisualKind::StrokeOrder
        | LearningVisualKind::TranslationAlignment
        | LearningVisualKind::DialogueRoleplay
        | LearningVisualKind::AlgorithmAnimation
        | LearningVisualKind::MemoryModel
        | LearningVisualKind::Custom => {
            commands.push(LearningVisualDrawCommand::Shape2d {
                id: spec.id.as_str().to_string(),
                role: learning_visual_role(spec.kind).to_string(),
                progress: position,
                selected: state.selected_id.as_deref() == Some(spec.id.as_str()),
            });
        }
    }
    let label_id = format!("label-{}", spec.id.as_str());
    if let Some(locale) = spec.title.locale.clone() {
        commands.push(LearningVisualDrawCommand::LocalizedTextLabel {
            id: label_id,
            text: spec.title.value.clone(),
            direction: locale.direction,
            locale,
        });
    } else {
        commands.push(LearningVisualDrawCommand::TextLabel {
            id: label_id,
            text: spec.title.value.clone(),
        });
    }

    let table_fallback = learning_visual_table_fallback(&commands);

    Ok(LearningVisualDrawPlan {
        visual_id: spec.id.clone(),
        renderer: spec.renderer.clone(),
        frame: VisualFrame {
            timeline_position: position,
            selected_id: state.selected_id.clone(),
            active_layers: state.active_layers.clone(),
        },
        commands,
        accessibility: spec.accessibility.clone(),
        table_fallback,
        reduced_motion,
    })
}

pub fn learning_visual_table_fallback(
    commands: &[LearningVisualDrawCommand],
) -> Vec<LearningVisualTableRow> {
    commands
        .iter()
        .map(|command| match command {
            LearningVisualDrawCommand::Axis2d { x_label, y_label } => LearningVisualTableRow {
                label: "Axis".to_string(),
                cells: vec![table_cell("x", x_label), table_cell("y", y_label)],
            },
            LearningVisualDrawCommand::Shape2d {
                id,
                role,
                progress,
                selected,
            } => LearningVisualTableRow {
                label: role.clone(),
                cells: vec![
                    table_cell("id", id),
                    table_cell("progress", format!("{progress:.2}")),
                    table_cell("selected", selected),
                ],
            },
            LearningVisualDrawCommand::Shape3d {
                id,
                role,
                rotation,
                visible,
                selected,
            } => LearningVisualTableRow {
                label: role.clone(),
                cells: vec![
                    table_cell("id", id),
                    table_cell("rotation", format!("{rotation:.2}")),
                    table_cell("visible", visible),
                    table_cell("selected", selected),
                ],
            },
            LearningVisualDrawCommand::Layer { id, visible } => LearningVisualTableRow {
                label: id.clone(),
                cells: vec![table_cell("visible", visible)],
            },
            LearningVisualDrawCommand::TimelineCursor { position } => LearningVisualTableRow {
                label: "Timeline".to_string(),
                cells: vec![table_cell("position", format!("{position:.2}"))],
            },
            LearningVisualDrawCommand::ParameterControl { name, value } => LearningVisualTableRow {
                label: name.clone(),
                cells: vec![table_cell("value", format!("{value:.2}"))],
            },
            LearningVisualDrawCommand::TextLabel { id, text } => LearningVisualTableRow {
                label: text.clone(),
                cells: vec![table_cell("id", id), table_cell("text", text)],
            },
            LearningVisualDrawCommand::LocalizedTextLabel {
                id,
                text,
                locale,
                direction,
            } => LearningVisualTableRow {
                label: text.clone(),
                cells: vec![
                    table_cell("id", id),
                    table_cell("text", text),
                    table_cell("locale", locale.bcp47()),
                    table_cell("direction", format!("{direction:?}").to_ascii_lowercase()),
                ],
            },
        })
        .collect()
}

fn table_cell(key: impl Into<String>, value: impl ToString) -> LearningVisualTableCell {
    LearningVisualTableCell {
        key: key.into(),
        value: value.to_string(),
    }
}

fn learning_visual_role(kind: LearningVisualKind) -> &'static str {
    match kind {
        LearningVisualKind::NumberLine => "number line",
        LearningVisualKind::FunctionGraph => "function graph",
        LearningVisualKind::GeometryConstruction => "geometry construction",
        LearningVisualKind::MatrixTransformation => "matrix transformation",
        LearningVisualKind::ProbabilitySimulation => "probability simulation",
        LearningVisualKind::MoleculeModel => "molecule model",
        LearningVisualKind::OrganSystemModel => "organ system model",
        LearningVisualKind::CellSimulation => "cell simulation",
        LearningVisualKind::ForceSimulation => "force simulation",
        LearningVisualKind::FieldVisualization => "field visualization",
        LearningVisualKind::OrbitalModel => "orbital model",
        LearningVisualKind::ClimateTimeline => "climate timeline",
        LearningVisualKind::PhonemeMap => "phoneme map",
        LearningVisualKind::GrammarTree => "grammar tree",
        LearningVisualKind::ArgumentMap => "argument map",
        LearningVisualKind::StrokeOrder => "stroke order",
        LearningVisualKind::TranslationAlignment => "translation alignment",
        LearningVisualKind::DialogueRoleplay => "dialogue roleplay",
        LearningVisualKind::AlgorithmAnimation => "algorithm animation",
        LearningVisualKind::MemoryModel => "memory model",
        LearningVisualKind::SystemTopology => "system topology",
        LearningVisualKind::Custom => "custom visual",
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum VisualRuntimeAction {
    Step { delta_ms: u32 },
    Play,
    Pause,
    Replay,
    Reset,
    SetTimeline { position: f32 },
    ToggleLayer { layer_id: String },
    SetParameter { name: String, value: f64 },
    Select { id: Option<String> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualParameterValue {
    pub name: String,
    pub value: f64,
}

pub fn advance_playback(playback: &mut VisualPlayback, delta_ms: u32) {
    let Some(duration_ms) = playback.duration_ms else {
        return;
    };
    if duration_ms == 0 {
        playback.timeline_position = 1.0;
        return;
    }
    let delta = delta_ms as f32 / duration_ms as f32;
    playback.timeline_position = (playback.timeline_position + delta).clamp(0.0, 1.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animated_visual_requires_reduced_motion_fallback() {
        let visual = LearningVisualSpec {
            id: LearningVisualId::from("heart"),
            node_id: CurriculumNodeId::from("biology-heart"),
            kind: LearningVisualKind::OrganSystemModel,
            title: LocalizedText::plain("Heart"),
            description: LocalizedText::plain("Heart structure"),
            renderer: VisualRenderer {
                engine: VisualRendererEngine::Tench3d,
                spec_version: 1,
                scene_ref: "heart.scene.json".to_string(),
            },
            playback: VisualPlayback {
                animated: true,
                autoplay: true,
                duration_ms: Some(2000),
                timeline_position: 0.0,
                reduced_motion_fallback: false,
            },
            interactions: vec![
                VisualInteraction::Rotate3d,
                VisualInteraction::ScrubTimeline,
            ],
            accessibility: VisualAccessibility {
                alt_text: "Heart model".to_string(),
                transcript: None,
                table_fallback_ref: None,
                keyboard_model: vec!["tab".to_string()],
            },
            locale: None,
        };

        assert!(visual.validate_for_release().is_err());
    }

    #[test]
    fn runtime_action_updates_timeline_layer_and_parameter() {
        let state = VisualRuntimeState {
            visual_id: LearningVisualId::from("graph"),
            selected_id: None,
            active_layers: Vec::new(),
            parameter_values: Vec::new(),
            playback: VisualPlayback {
                animated: true,
                autoplay: false,
                duration_ms: Some(1000),
                timeline_position: 0.0,
                reduced_motion_fallback: true,
            },
        }
        .apply_action(VisualRuntimeAction::Step { delta_ms: 250 })
        .apply_action(VisualRuntimeAction::ToggleLayer {
            layer_id: "labels".to_string(),
        })
        .apply_action(VisualRuntimeAction::SetParameter {
            name: "slope".to_string(),
            value: 2.0,
        });

        assert_eq!(state.playback.timeline_position, 0.25);
        assert_eq!(state.active_layers, vec!["labels"]);
        assert_eq!(state.parameter_values[0].value, 2.0);
    }

    #[test]
    fn runtime_action_supports_play_pause_and_replay() {
        let state = VisualRuntimeState {
            visual_id: LearningVisualId::from("simulation"),
            selected_id: Some("point-a".to_string()),
            active_layers: Vec::new(),
            parameter_values: Vec::new(),
            playback: VisualPlayback {
                animated: true,
                autoplay: false,
                duration_ms: Some(1000),
                timeline_position: 0.7,
                reduced_motion_fallback: true,
            },
        }
        .apply_action(VisualRuntimeAction::Play)
        .apply_action(VisualRuntimeAction::Pause)
        .apply_action(VisualRuntimeAction::Replay);

        assert!(state.playback.autoplay);
        assert_eq!(state.playback.timeline_position, 0.0);
        assert_eq!(state.selected_id, None);
    }

    #[test]
    fn draw_plan_uses_runtime_state_and_reduced_motion() {
        let spec = LearningVisualSpec {
            id: LearningVisualId::from("heart"),
            node_id: CurriculumNodeId::from("biology-heart"),
            kind: LearningVisualKind::OrganSystemModel,
            title: LocalizedText::plain("Heart"),
            description: LocalizedText::plain("Heart structure"),
            renderer: VisualRenderer {
                engine: VisualRendererEngine::Tench3d,
                spec_version: 1,
                scene_ref: "heart.scene.json".to_string(),
            },
            playback: VisualPlayback {
                animated: true,
                autoplay: false,
                duration_ms: Some(2000),
                timeline_position: 0.0,
                reduced_motion_fallback: true,
            },
            interactions: vec![
                VisualInteraction::Rotate3d,
                VisualInteraction::ScrubTimeline,
                VisualInteraction::ToggleLayer {
                    layer_id: "labels".to_string(),
                },
            ],
            accessibility: VisualAccessibility {
                alt_text: "Heart model".to_string(),
                transcript: None,
                table_fallback_ref: Some("table://heart".to_string()),
                keyboard_model: vec!["tab".to_string(), "arrow-left/right".to_string()],
            },
            locale: None,
        };
        let state = VisualRuntimeState {
            visual_id: LearningVisualId::from("heart"),
            selected_id: Some("heart".to_string()),
            active_layers: vec!["labels".to_string()],
            parameter_values: Vec::new(),
            playback: VisualPlayback {
                animated: true,
                autoplay: false,
                duration_ms: Some(2000),
                timeline_position: 0.35,
                reduced_motion_fallback: true,
            },
        };

        let plan = build_learning_visual_draw_plan(&spec, &state, true).expect("draw plan");

        assert_eq!(plan.frame.timeline_position, 1.0);
        assert!(plan.commands.iter().any(|command| matches!(
            command,
            LearningVisualDrawCommand::Shape3d { selected: true, .. }
        )));
        assert!(plan.commands.iter().any(|command| matches!(
            command,
            LearningVisualDrawCommand::Layer { id, visible: true } if id == "labels"
        )));
        assert!(plan
            .table_fallback
            .iter()
            .any(|row| row.label == "organ system model"));
    }

    #[test]
    fn draw_plan_preserves_label_locale_and_direction() {
        let locale = ContentLocale::parse("ar-SA").expect("rtl locale");
        let spec = LearningVisualSpec {
            id: LearningVisualId::from("grammar"),
            node_id: CurriculumNodeId::from("language-grammar"),
            kind: LearningVisualKind::GrammarTree,
            title: LocalizedText::localized("شجرة الجملة", locale.clone()),
            description: LocalizedText::plain("Sentence tree"),
            renderer: VisualRenderer {
                engine: VisualRendererEngine::Tench2d,
                spec_version: 1,
                scene_ref: "grammar.scene.json".to_string(),
            },
            playback: VisualPlayback {
                animated: false,
                autoplay: false,
                duration_ms: None,
                timeline_position: 0.0,
                reduced_motion_fallback: true,
            },
            interactions: Vec::new(),
            accessibility: VisualAccessibility {
                alt_text: "Sentence tree".to_string(),
                transcript: None,
                table_fallback_ref: Some("table://grammar".to_string()),
                keyboard_model: vec!["tab".to_string()],
            },
            locale: Some(locale.clone()),
        };
        let state = VisualRuntimeState {
            visual_id: LearningVisualId::from("grammar"),
            selected_id: None,
            active_layers: Vec::new(),
            parameter_values: Vec::new(),
            playback: spec.playback.clone(),
        };

        let plan = build_learning_visual_draw_plan(&spec, &state, false).expect("draw plan");

        assert!(plan.commands.iter().any(|command| matches!(
            command,
            LearningVisualDrawCommand::LocalizedTextLabel {
                locale: command_locale,
                direction: TextDirection::Rtl,
                ..
            } if command_locale == &locale
        )));
        assert!(plan.table_fallback.iter().any(|row| row
            .cells
            .iter()
            .any(|cell| cell.key == "locale" && cell.value == "ar-SA")));
    }
}
