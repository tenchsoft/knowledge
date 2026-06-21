use serde::{Deserialize, Serialize};

/// Persisted study state for a single subject.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StudyState {
    pub subject_id: String,
    pub active_concept_id: String,
    pub streak: u32,
    pub total_sessions: u32,
    pub total_correct: u32,
    pub total_wrong: u32,
    pub total_seconds: u64,
    pub concept_progress: Vec<ConceptProgress>,
    pub review_queue: Vec<ReviewEntry>,
    pub spaced_repetition: Vec<SpacedRepetitionEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptProgress {
    pub concept_id: String,
    pub status: ConceptProgressStatus,
    pub attempts: u32,
    pub correct: u32,
    pub last_practiced_at: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConceptProgressStatus {
    Completed,
    #[default]
    InProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewEntry {
    pub id: String,
    pub problem_id: String,
    pub problem_text: String,
    pub problem_matrices: String,
    pub wrong_answer: String,
    pub correct_answer: String,
    pub cause_tag: String,
    pub related_concept: String,
    pub solution: String,
    pub reviewed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacedRepetitionEntry {
    pub concept_id: String,
    pub easiness_factor: f64,
    pub interval_days: u32,
    pub repetitions: u32,
    pub next_review_date: String,
}

#[derive(Debug, Clone)]
pub struct AnswerRecord {
    pub concept_id: String,
    pub correct: bool,
    pub problem_id: String,
    pub wrong_answer: String,
    pub correct_answer: String,
    pub cause_tag: String,
    pub related_concept: String,
    pub solution: String,
    pub problem_text: String,
    pub problem_matrices: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyStats {
    pub total_sessions: u32,
    pub total_correct: u32,
    pub total_wrong: u32,
    pub total_seconds: u64,
    pub streak: u32,
    pub concepts_completed: u32,
    pub concepts_in_progress: u32,
    pub pending_reviews: u32,
    pub weak_points: Vec<String>,
}

impl StudyState {
    pub fn record_answer(&mut self, record: AnswerRecord) {
        let now = chrono::Utc::now();
        if record.correct {
            self.total_correct += 1;
        } else {
            self.total_wrong += 1;
            self.review_queue.push(ReviewEntry {
                id: format!("r-{}", now.timestamp_millis()),
                problem_id: record.problem_id,
                problem_text: record.problem_text,
                problem_matrices: record.problem_matrices,
                wrong_answer: record.wrong_answer,
                correct_answer: record.correct_answer,
                cause_tag: record.cause_tag,
                related_concept: record.related_concept,
                solution: record.solution,
                reviewed: false,
            });
        }

        if let Some(progress) = self
            .concept_progress
            .iter_mut()
            .find(|progress| progress.concept_id == record.concept_id)
        {
            progress.attempts += 1;
            if record.correct {
                progress.correct += 1;
            }
            progress.last_practiced_at = Some(now.to_rfc3339());
        } else {
            self.concept_progress.push(ConceptProgress {
                concept_id: record.concept_id,
                status: if record.correct {
                    ConceptProgressStatus::Completed
                } else {
                    ConceptProgressStatus::InProgress
                },
                attempts: 1,
                correct: if record.correct { 1 } else { 0 },
                last_practiced_at: Some(now.to_rfc3339()),
            });
        }
    }

    pub fn update_spaced_repetition(&mut self, concept_id: String, quality: u32) {
        let now = chrono::Utc::now();
        let entry = self
            .spaced_repetition
            .iter_mut()
            .find(|entry| entry.concept_id == concept_id);

        match entry {
            Some(entry) => {
                let (easiness_factor, interval_days, repetitions) = tench_study_core::sm2(
                    quality,
                    entry.easiness_factor,
                    entry.interval_days,
                    entry.repetitions,
                );
                entry.easiness_factor = easiness_factor;
                entry.interval_days = interval_days;
                entry.repetitions = repetitions;
                entry.next_review_date = next_review_date(now, interval_days);
            }
            None => {
                let (easiness_factor, interval_days, repetitions) =
                    tench_study_core::sm2(quality, 2.5, 0, 0);
                self.spaced_repetition.push(SpacedRepetitionEntry {
                    concept_id,
                    easiness_factor,
                    interval_days,
                    repetitions,
                    next_review_date: next_review_date(now, interval_days),
                });
            }
        }
    }

    pub fn due_review_concepts(&self, now: &str) -> Vec<String> {
        self.spaced_repetition
            .iter()
            .filter(|entry| entry.next_review_date.as_str() <= now)
            .map(|entry| entry.concept_id.clone())
            .collect()
    }

    pub fn mark_reviewed(&mut self, review_id: &str) {
        if let Some(entry) = self
            .review_queue
            .iter_mut()
            .find(|entry| entry.id == review_id)
        {
            entry.reviewed = true;
        }
    }

    pub fn increment_session(&mut self) {
        self.total_sessions += 1;
    }

    pub fn add_elapsed_seconds(&mut self, seconds: u64) {
        self.total_seconds += seconds;
    }

    pub fn update_streak(&mut self, streak: u32) {
        self.streak = streak;
    }

    pub fn stats(&self) -> StudyStats {
        StudyStats {
            total_sessions: self.total_sessions,
            total_correct: self.total_correct,
            total_wrong: self.total_wrong,
            total_seconds: self.total_seconds,
            streak: self.streak,
            concepts_completed: self
                .concept_progress
                .iter()
                .filter(|concept| concept.status == ConceptProgressStatus::Completed)
                .count() as u32,
            concepts_in_progress: self
                .concept_progress
                .iter()
                .filter(|concept| concept.status == ConceptProgressStatus::InProgress)
                .count() as u32,
            pending_reviews: self
                .review_queue
                .iter()
                .filter(|review| !review.reviewed)
                .count() as u32,
            weak_points: self.weak_points(),
        }
    }

    fn weak_points(&self) -> Vec<String> {
        let mut cause_counts: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();
        for entry in &self.review_queue {
            if !entry.reviewed {
                *cause_counts.entry(entry.cause_tag.clone()).or_insert(0) += 1;
            }
        }
        let mut pairs: Vec<(String, u32)> = cause_counts.into_iter().collect();
        pairs.sort_by_key(|b| std::cmp::Reverse(b.1));
        pairs.into_iter().take(5).map(|(tag, _)| tag).collect()
    }
}

fn next_review_date(now: chrono::DateTime<chrono::Utc>, interval_days: u32) -> String {
    now.checked_add_signed(chrono::Duration::days(interval_days as i64))
        .unwrap_or(now)
        .to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concept_progress_status_reads_persisted_enum_strings() {
        let json = r#"{
            "concept_id": "math-algebra",
            "status": "completed",
            "attempts": 3,
            "correct": 2,
            "last_practiced_at": null
        }"#;

        let progress: ConceptProgress = serde_json::from_str(json).expect("progress");

        assert_eq!(progress.status, ConceptProgressStatus::Completed);
        assert_eq!(
            serde_json::to_value(progress.status).expect("status"),
            serde_json::json!("completed")
        );
    }

    #[test]
    fn stats_use_typed_progress_status_and_pending_reviews() {
        let mut state = StudyState {
            total_sessions: 4,
            total_correct: 7,
            total_wrong: 3,
            total_seconds: 600,
            streak: 2,
            ..StudyState::default()
        };
        state.concept_progress.push(ConceptProgress {
            concept_id: "math-algebra".to_string(),
            status: ConceptProgressStatus::Completed,
            attempts: 5,
            correct: 5,
            last_practiced_at: None,
        });
        state.concept_progress.push(ConceptProgress {
            concept_id: "science-heart".to_string(),
            status: ConceptProgressStatus::InProgress,
            attempts: 2,
            correct: 1,
            last_practiced_at: None,
        });
        state.review_queue = vec![
            review_entry("r1", "algebra", false),
            review_entry("r2", "algebra", false),
            review_entry("r3", "syntax", false),
            review_entry("r4", "resolved", true),
        ];

        let stats = state.stats();

        assert_eq!(stats.total_sessions, 4);
        assert_eq!(stats.concepts_completed, 1);
        assert_eq!(stats.concepts_in_progress, 1);
        assert_eq!(stats.pending_reviews, 3);
        assert_eq!(stats.weak_points, vec!["algebra", "syntax"]);
    }

    fn review_entry(id: &str, cause_tag: &str, reviewed: bool) -> ReviewEntry {
        ReviewEntry {
            id: id.to_string(),
            problem_id: format!("problem-{id}"),
            problem_text: "Question".to_string(),
            problem_matrices: String::new(),
            wrong_answer: "wrong".to_string(),
            correct_answer: "correct".to_string(),
            cause_tag: cause_tag.to_string(),
            related_concept: "concept".to_string(),
            solution: "solution".to_string(),
            reviewed,
        }
    }
}
