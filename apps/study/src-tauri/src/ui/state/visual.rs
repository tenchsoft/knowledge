use super::*;

impl StudyState {
    pub fn toggle_visual_play(&mut self) {
        self.visual_playing = !self.visual_playing;
    }

    pub fn toggle_visual_autoplay(&mut self) {
        self.visual_autoplay = !self.visual_autoplay;
    }

    pub fn set_visual_timeline(&mut self, position: f64) {
        self.visual_timeline_position = position.clamp(0.0, 1.0);
    }

    pub fn active_visual_runtime_state(&self) -> tench_study_core::VisualRuntimeState {
        let concept = self.active_concept();
        tench_study_core::VisualRuntimeState {
            visual_id: tench_study_core::LearningVisualId::from(concept.primary_visual_id.clone()),
            selected_id: Some(concept.id.clone()),
            active_layers: vec!["labels".to_string(), "explanation".to_string()],
            parameter_values: vec![tench_study_core::VisualParameterValue {
                name: "hint_level".to_string(),
                value: f64::from(self.hint_level),
            }],
            playback: tench_study_core::VisualPlayback {
                animated: concept.visual_count > 0,
                autoplay: false,
                duration_ms: Some(3000),
                timeline_position: (self.hint_level as f32 / 3.0).clamp(0.0, 1.0),
                reduced_motion_fallback: true,
            },
        }
    }

    pub fn active_visual_draw_plan(&self) -> Option<tench_study_core::LearningVisualDrawPlan> {
        let state = self.active_visual_runtime_state();
        let spec = self
            .visual_specs
            .iter()
            .find(|spec| spec.id == state.visual_id)?;
        tench_study_core::build_learning_visual_draw_plan(spec, &state, false).ok()
    }
}
