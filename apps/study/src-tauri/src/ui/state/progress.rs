use super::*;

impl StudyState {
    pub fn update_goals_progress(&mut self) {
        let session_len = self.session_results.len() as i32;
        let elapsed = self.elapsed_seconds;
        let accuracy = self.accuracy();
        for goal in &mut self.goals {
            match goal.id.as_str() {
                "daily-problems" => {
                    goal.current = session_len;
                }
                "daily-minutes" => {
                    goal.current = elapsed / 60;
                }
                "daily-accuracy" => {
                    goal.current = accuracy;
                }
                _ => {}
            }
        }
    }

    pub fn check_achievements(&mut self) {
        let streak = self.streak;
        let session_len = self.session_results.len() as i32;
        for achievement in &mut self.achievements {
            match achievement.id.as_str() {
                "first-session" if session_len > 0 => {
                    achievement.unlocked = true;
                    achievement.progress = 1.0;
                }
                "streak-10" => {
                    achievement.progress = (streak as f32 / 10.0).min(1.0);
                    achievement.unlocked = streak >= 10;
                }
                "problems-100" => {
                    achievement.progress = (session_len as f32 / 100.0).min(1.0);
                    achievement.unlocked = session_len >= 100;
                }
                _ => {}
            }
        }
    }

    pub fn apply_spaced_repetition_rating(&mut self, rating: SpacedRepetitionRating) {
        self.pending_rating = Some(rating);
        let concept_id = self.active_concept().id.clone();
        let existing = self
            .spaced_repetition_data
            .iter()
            .position(|e| e.concept_id == concept_id);

        let idx = if let Some(idx) = existing {
            idx
        } else {
            self.spaced_repetition_data.push(SpacedRepetitionEntry {
                concept_id: concept_id.clone(),
                easiness_factor: 2.5,
                interval_days: 0,
                repetitions: 0,
                next_review_date: String::new(),
            });
            self.spaced_repetition_data.len() - 1
        };

        let quality = match rating {
            SpacedRepetitionRating::Again => 0,
            SpacedRepetitionRating::Hard => 3,
            SpacedRepetitionRating::Good => 4,
            SpacedRepetitionRating::Easy => 5,
        };

        let entry = &mut self.spaced_repetition_data[idx];
        if quality < 3 {
            entry.repetitions = 0;
            entry.interval_days = 1;
        } else {
            if entry.repetitions == 0 {
                entry.interval_days = 1;
            } else if entry.repetitions == 1 {
                entry.interval_days = 6;
            } else {
                entry.interval_days =
                    (entry.interval_days as f64 * entry.easiness_factor).round() as u32;
            }
            entry.repetitions += 1;
        }

        let ef = entry.easiness_factor
            + (0.1 - (5 - quality) as f64 * (0.08 + (5 - quality) as f64 * 0.02));
        entry.easiness_factor = ef.clamp(1.3, 10.0);
    }

    pub fn sort_review_queue_by_due_date(&mut self) {
        // Sort review queue by spaced repetition due date (items without SR data first)
        self.review_queue.sort_by(|a, b| {
            let a_due = a
                .spaced_repetition
                .as_ref()
                .map(|e| e.next_review_date.as_str())
                .unwrap_or("");
            let b_due = b
                .spaced_repetition
                .as_ref()
                .map(|e| e.next_review_date.as_str())
                .unwrap_or("");
            a_due.cmp(b_due)
        });
    }

    pub fn retry_review_item(&mut self) {
        if self.stage != Stage::Review {
            return;
        }
        // Phase 6: Allow retrying the current review item
        self.review_index = self.review_index.saturating_sub(1).max(1);
    }

    pub fn concept_retention_score(&self, concept_id: &str) -> f64 {
        // Phase 6: Return retention score for a concept based on spaced repetition data
        self.spaced_repetition_data
            .iter()
            .find(|entry| entry.concept_id == concept_id)
            .map(|entry| {
                // Higher easiness factor = better retention
                // More repetitions = better retention
                let ef_score = (entry.easiness_factor - 1.3) / 8.7; // normalized 0-1
                let rep_score = (entry.repetitions as f64 / 10.0).min(1.0);
                (ef_score + rep_score) / 2.0
            })
            .unwrap_or(0.0)
    }

    pub fn weak_points(&self) -> Vec<String> {
        let mut output = Vec::new();
        for item in &self.review_queue {
            if !output.contains(&item.cause_tag) {
                output.push(item.cause_tag.clone());
            }
            if output.len() == 5 {
                break;
            }
        }
        output
    }

    pub fn accuracy(&self) -> i32 {
        if self.session_results.is_empty() {
            return 0;
        }
        let correct = self
            .session_results
            .iter()
            .filter(|correct| **correct)
            .count() as i32;
        correct * 100 / self.session_results.len() as i32
    }
}
