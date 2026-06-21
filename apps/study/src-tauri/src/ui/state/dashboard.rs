use super::builders::offline_asset_state;
use super::*;
use tench_ui::prelude::Size;

impl StudyState {
    pub fn update_viewport(&mut self, size: Size) {
        self.viewport_class = if size.width < 700.0 {
            StudyViewportClass::Mobile
        } else if size.width < 1100.0 {
            StudyViewportClass::Tablet
        } else {
            StudyViewportClass::Desktop
        };
        self.touch_review.enabled = self.viewport_class == StudyViewportClass::Mobile;
        self.touch_review.min_hit_size_px = if self.touch_review.enabled { 48 } else { 44 };
    }

    pub fn toggle_batch_concept_selection(&mut self, concept_id: impl Into<String>) {
        let concept_id = concept_id.into();
        if !self.batch_edit.selected_concept_ids.remove(&concept_id) {
            self.batch_edit.selected_concept_ids.insert(concept_id);
        }
    }

    pub fn clear_batch_selection(&mut self) {
        self.batch_edit.selected_concept_ids.clear();
        self.batch_edit.selected_card_ids.clear();
        self.batch_edit.pending_tags.clear();
        self.batch_edit.pending_status = None;
    }

    pub fn apply_batch_concept_status(&mut self, status: ConceptStatus) {
        let selected = self.batch_edit.selected_concept_ids.clone();
        for concept in self
            .units
            .iter_mut()
            .flat_map(|unit| unit.concepts.iter_mut())
            .filter(|concept| selected.contains(&concept.id))
        {
            concept.status = status;
        }
        self.batch_edit.pending_status = Some(status);
    }

    pub fn refresh_daily_dashboard(&mut self) {
        let recommended_concept_id = self
            .review_queue
            .first()
            .and_then(|item| {
                self.problems
                    .iter()
                    .find(|problem| problem.related_concept == item.related_concept)
            })
            .map(|problem| problem.concept_id.clone())
            .or_else(|| {
                self.units
                    .get(self.active_unit_idx)
                    .and_then(|unit| unit.concepts.get(self.active_concept_idx))
                    .map(|concept| concept.id.clone())
            });
        self.dashboard = DailyStudyDashboard {
            due_review_count: self.review_queue.len(),
            new_lesson_count: self.total_problems(),
            current_streak: self.streak,
            minutes_today: self.elapsed_seconds / 60,
            accuracy_percent: self.accuracy(),
            recommended_concept_id,
            offline_ready: self.offline_assets.cache_ready,
        };
    }

    pub fn refresh_offline_asset_state(&mut self) {
        self.offline_assets = offline_asset_state(&self.visual_specs);
        self.refresh_daily_dashboard();
    }

    pub fn refresh_ux_audit(&mut self, catalog: &tench_app_core::I18nCatalog) {
        let mut required = crate::i18n::study_i18n_required_keys();
        required.extend(
            self.keyboard_shortcuts
                .iter()
                .map(|shortcut| shortcut.label_key.clone()),
        );
        required.extend(
            self.accessibility_labels
                .iter()
                .map(|label| label.label_key.clone()),
        );
        required.sort();
        required.dedup();
        let coverage = catalog.coverage_report(&required);
        let mut missing_i18n_keys = coverage.missing_keys;
        missing_i18n_keys.extend(coverage.fallback_keys);
        missing_i18n_keys.sort();
        missing_i18n_keys.dedup();
        self.ux_audit = StudyUxAudit {
            missing_i18n_keys,
            mock_content_removed: self.mock_content_removed(),
            accessibility_label_count: self.accessibility_labels.len(),
        };
    }

    pub fn mock_content_removed(&self) -> bool {
        self.streak == 0
            && self.session_results.is_empty()
            && self.review_queue.is_empty()
            && self
                .problems
                .iter()
                .all(|problem| problem.answer != "core idea")
    }
}
