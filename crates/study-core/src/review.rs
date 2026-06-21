use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacedRepetitionState {
    pub easiness_factor: f64,
    pub interval_days: u32,
    pub repetitions: u32,
    #[serde(default)]
    pub fsrs_stability: Option<f64>,
    #[serde(default)]
    pub fsrs_difficulty: Option<f64>,
    #[serde(default)]
    pub due_on: Option<ReviewDate>,
    #[serde(default)]
    pub last_reviewed_at: Option<ReviewDate>,
    #[serde(default)]
    pub suspended: bool,
    #[serde(default)]
    pub buried_until: Option<ReviewDate>,
}

impl Default for SpacedRepetitionState {
    fn default() -> Self {
        Self {
            easiness_factor: 2.5,
            interval_days: 0,
            repetitions: 0,
            fsrs_stability: None,
            fsrs_difficulty: None,
            due_on: None,
            last_reviewed_at: None,
            suspended: false,
            buried_until: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReviewDate(pub String);

impl ReviewDate {
    pub fn parse(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        let date = value.split('T').next().unwrap_or("").to_string();
        let Some((year, month, day)) = parse_ymd(&date) else {
            return Err(format!("invalid review date {value}"));
        };
        Ok(Self(format!("{year:04}-{month:02}-{day:02}")))
    }

    pub fn add_days(&self, days: u32) -> Self {
        let Some((year, month, day)) = parse_ymd(&self.0) else {
            return self.clone();
        };
        let serial = days_from_civil(year, month, day) + i64::from(days);
        let (year, month, day) = civil_from_days(serial);
        Self(format!("{year:04}-{month:02}-{day:02}"))
    }

    pub fn days_until(&self, other: &ReviewDate) -> i64 {
        let Some((year, month, day)) = parse_ymd(&self.0) else {
            return 0;
        };
        let Some((other_year, other_month, other_day)) = parse_ymd(&other.0) else {
            return 0;
        };
        days_from_civil(other_year, other_month, other_day) - days_from_civil(year, month, day)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRating {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReviewQueuePolicy {
    pub today: ReviewDate,
    pub daily_limit: u16,
    #[serde(default)]
    pub include_suspended: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledReviewItem {
    pub queue_item_id: crate::ReviewQueueItemId,
    pub node_id: crate::CurriculumNodeId,
    pub practice_item_id: crate::PracticeItemId,
    pub due_on: Option<ReviewDate>,
    pub overdue_days: i64,
    pub priority: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewQueuePlan {
    #[serde(default)]
    pub due: Vec<ScheduledReviewItem>,
    pub buried_count: u32,
    pub suspended_count: u32,
    pub limited_by_daily_limit: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReviewStats {
    pub due_count: u32,
    pub overdue_count: u32,
    pub buried_count: u32,
    pub suspended_count: u32,
    pub average_easiness_factor: f64,
}

impl ReviewRating {
    pub fn quality(self) -> u32 {
        match self {
            Self::Again => 1,
            Self::Hard => 3,
            Self::Good => 4,
            Self::Easy => 5,
        }
    }
}

/// SM-2 spaced repetition algorithm.
///
/// `quality` is 0-5 (0 = complete blackout, 5 = perfect).
/// Returns (easiness_factor, interval_days, repetitions).
pub fn sm2(quality: u32, ef: f64, interval: u32, repetitions: u32) -> (f64, u32, u32) {
    let q = quality.min(5) as f64;
    let new_ef = (ef + 0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02)).max(1.3);

    if quality < 3 {
        return (new_ef, 1, 0);
    }

    let new_rep = repetitions + 1;
    let new_interval = match new_rep {
        1 => 1,
        2 => 6,
        _ => ((interval as f64) * new_ef).round() as u32,
    };

    (new_ef, new_interval.max(1), new_rep)
}

pub fn schedule_review(
    state: SpacedRepetitionState,
    rating: ReviewRating,
) -> SpacedRepetitionState {
    let (easiness_factor, interval_days, repetitions) = sm2(
        rating.quality(),
        state.easiness_factor,
        state.interval_days,
        state.repetitions,
    );
    SpacedRepetitionState {
        easiness_factor,
        interval_days,
        repetitions,
        fsrs_stability: state.fsrs_stability,
        fsrs_difficulty: state.fsrs_difficulty,
        due_on: state.due_on,
        last_reviewed_at: state.last_reviewed_at,
        suspended: state.suspended,
        buried_until: state.buried_until,
    }
}

pub fn schedule_review_at(
    state: SpacedRepetitionState,
    rating: ReviewRating,
    reviewed_on: ReviewDate,
) -> SpacedRepetitionState {
    let mut next = schedule_review(state, rating);
    next.last_reviewed_at = Some(reviewed_on.clone());
    next.due_on = Some(reviewed_on.add_days(next.interval_days));
    next.buried_until = None;
    next
}

pub fn apply_fsrs_hint(
    mut state: SpacedRepetitionState,
    stability: f64,
    difficulty: f64,
) -> SpacedRepetitionState {
    state.fsrs_stability = Some(stability.max(0.0));
    state.fsrs_difficulty = Some(difficulty.clamp(1.0, 10.0));
    state
}

pub fn bury_review_until(
    mut state: SpacedRepetitionState,
    buried_until: ReviewDate,
) -> SpacedRepetitionState {
    state.buried_until = Some(buried_until);
    state
}

pub fn set_review_suspended(
    mut state: SpacedRepetitionState,
    suspended: bool,
) -> SpacedRepetitionState {
    state.suspended = suspended;
    if !suspended {
        state.buried_until = None;
    }
    state
}

pub fn build_review_queue(
    progress: &[crate::LearnerProgress],
    review_items: &[crate::ReviewQueueItem],
    policy: &ReviewQueuePolicy,
) -> ReviewQueuePlan {
    let mut due = Vec::new();
    let mut buried_count = 0;
    let mut suspended_count = 0;
    let progress_by_node = progress
        .iter()
        .map(|progress| (&progress.node_id, &progress.review_state))
        .collect::<HashMap<_, _>>();

    for item in review_items.iter().filter(|item| !item.reviewed) {
        let state = progress_by_node.get(&item.node_id).copied();
        let default_state;
        let state = match state {
            Some(state) => state,
            None => {
                default_state = SpacedRepetitionState::default();
                &default_state
            }
        };

        if state.suspended && !policy.include_suspended {
            suspended_count += 1;
            continue;
        }
        if state
            .buried_until
            .as_ref()
            .is_some_and(|date| date > &policy.today)
        {
            buried_count += 1;
            continue;
        }
        let overdue_days = state
            .due_on
            .as_ref()
            .map(|due_on| due_on.days_until(&policy.today).max(0))
            .unwrap_or(0);
        let due_now = state
            .due_on
            .as_ref()
            .is_none_or(|due_on| due_on <= &policy.today);
        if due_now {
            let priority = overdue_days as f32
                + match item.rating {
                    ReviewRating::Again => 4.0,
                    ReviewRating::Hard => 3.0,
                    ReviewRating::Good => 2.0,
                    ReviewRating::Easy => 1.0,
                };
            due.push(ScheduledReviewItem {
                queue_item_id: item.id.clone(),
                node_id: item.node_id.clone(),
                practice_item_id: item.practice_item_id.clone(),
                due_on: state.due_on.clone(),
                overdue_days,
                priority,
            });
        }
    }

    due.sort_by(|a, b| {
        b.priority
            .partial_cmp(&a.priority)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.queue_item_id.cmp(&b.queue_item_id))
    });
    let limited_by_daily_limit = due.len() > usize::from(policy.daily_limit);
    due.truncate(usize::from(policy.daily_limit));

    ReviewQueuePlan {
        due,
        buried_count,
        suspended_count,
        limited_by_daily_limit,
    }
}

pub fn review_stats(
    progress: &[crate::LearnerProgress],
    review_items: &[crate::ReviewQueueItem],
    today: &ReviewDate,
) -> ReviewStats {
    let policy = ReviewQueuePolicy {
        today: today.clone(),
        daily_limit: u16::MAX,
        include_suspended: false,
    };
    let queue = build_review_queue(progress, review_items, &policy);
    let overdue_count = queue
        .due
        .iter()
        .filter(|item| item.overdue_days > 0)
        .count() as u32;
    let average_easiness_factor = if progress.is_empty() {
        0.0
    } else {
        progress
            .iter()
            .map(|progress| progress.review_state.easiness_factor)
            .sum::<f64>()
            / progress.len() as f64
    };
    ReviewStats {
        due_count: queue.due.len() as u32,
        overdue_count,
        buried_count: queue.buried_count,
        suspended_count: queue.suspended_count,
        average_easiness_factor,
    }
}

fn parse_ymd(value: &str) -> Option<(i32, u32, u32)> {
    let mut parts = value.split('-');
    let year = parts.next()?.parse().ok()?;
    let month = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some((year, month, day))
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let year = year - (month <= 2) as i32;
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = month as i32;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day as i32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    i64::from(era * 146097 + doe - 719468)
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = year + (month <= 2) as i64;
    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sm2_first_correct() {
        let (ef, interval, rep) = sm2(4, 2.5, 0, 0);
        assert!(ef > 1.3);
        assert_eq!(interval, 1);
        assert_eq!(rep, 1);
    }

    #[test]
    fn test_sm2_failed_resets() {
        let (ef, interval, rep) = sm2(1, 2.5, 6, 2);
        assert!(ef >= 1.3);
        assert_eq!(interval, 1);
        assert_eq!(rep, 0);
    }

    #[test]
    fn schedule_review_at_sets_due_date_and_fsrs_hint() {
        let today = ReviewDate::parse("2026-05-04T12:00:00Z").expect("date");
        let state = apply_fsrs_hint(SpacedRepetitionState::default(), 2.0, 6.0);

        let next = schedule_review_at(state, ReviewRating::Good, today);

        assert_eq!(next.due_on, Some(ReviewDate("2026-05-05".to_string())));
        assert_eq!(next.fsrs_stability, Some(2.0));
        assert_eq!(next.fsrs_difficulty, Some(6.0));
    }

    #[test]
    fn queue_builder_respects_daily_limit_bury_and_suspend() {
        let today = ReviewDate::parse("2026-05-04").expect("date");
        let progress = vec![
            progress(
                "node-a",
                schedule_review_at(
                    SpacedRepetitionState::default(),
                    ReviewRating::Good,
                    ReviewDate::parse("2026-05-01").unwrap(),
                ),
            ),
            progress(
                "node-b",
                bury_review_until(
                    SpacedRepetitionState {
                        due_on: Some(ReviewDate::parse("2026-05-01").unwrap()),
                        ..SpacedRepetitionState::default()
                    },
                    ReviewDate::parse("2026-05-06").unwrap(),
                ),
            ),
            progress(
                "node-c",
                set_review_suspended(
                    SpacedRepetitionState {
                        due_on: Some(ReviewDate::parse("2026-05-01").unwrap()),
                        ..SpacedRepetitionState::default()
                    },
                    true,
                ),
            ),
        ];
        let items = vec![
            review_item("item-a", "node-a", ReviewRating::Again),
            review_item("item-b", "node-b", ReviewRating::Again),
            review_item("item-c", "node-c", ReviewRating::Again),
        ];

        let plan = build_review_queue(
            &progress,
            &items,
            &ReviewQueuePolicy {
                today,
                daily_limit: 1,
                include_suspended: false,
            },
        );

        assert_eq!(plan.due.len(), 1);
        assert_eq!(plan.buried_count, 1);
        assert_eq!(plan.suspended_count, 1);
        assert!(!plan.limited_by_daily_limit);
    }

    #[test]
    fn review_stats_counts_overdue_items() {
        let today = ReviewDate::parse("2026-05-04").expect("date");
        let progress = vec![progress(
            "node-a",
            SpacedRepetitionState {
                due_on: Some(ReviewDate::parse("2026-05-01").unwrap()),
                ..SpacedRepetitionState::default()
            },
        )];
        let items = vec![review_item("item-a", "node-a", ReviewRating::Hard)];

        let stats = review_stats(&progress, &items, &today);

        assert_eq!(stats.due_count, 1);
        assert_eq!(stats.overdue_count, 1);
        assert!(stats.average_easiness_factor > 0.0);
    }

    #[test]
    fn queue_builder_handles_large_review_sets_with_daily_limit() {
        let today = ReviewDate::parse("2026-05-04").expect("date");
        let progress = (0..50_000)
            .map(|index| {
                progress(
                    &format!("node-{index}"),
                    SpacedRepetitionState {
                        due_on: Some(ReviewDate::parse("2026-05-01").unwrap()),
                        ..SpacedRepetitionState::default()
                    },
                )
            })
            .collect::<Vec<_>>();
        let items = (0..50_000)
            .map(|index| {
                review_item(
                    &format!("item-{index}"),
                    &format!("node-{index}"),
                    ReviewRating::Good,
                )
            })
            .collect::<Vec<_>>();

        let plan = build_review_queue(
            &progress,
            &items,
            &ReviewQueuePolicy {
                today,
                daily_limit: 50,
                include_suspended: false,
            },
        );

        assert_eq!(plan.due.len(), 50);
        assert!(plan.limited_by_daily_limit);
        assert_eq!(plan.due[0].overdue_days, 3);
    }

    fn progress(node_id: &str, review_state: SpacedRepetitionState) -> crate::LearnerProgress {
        crate::LearnerProgress {
            learner_id: crate::LearnerId::from("learner"),
            node_id: crate::CurriculumNodeId::from(node_id),
            mastery: crate::MasteryState::default(),
            attempts: Vec::new(),
            review_state,
        }
    }

    fn review_item(id: &str, node_id: &str, rating: ReviewRating) -> crate::ReviewQueueItem {
        crate::ReviewQueueItem {
            id: crate::ReviewQueueItemId::from(id),
            node_id: crate::CurriculumNodeId::from(node_id),
            practice_item_id: crate::PracticeItemId::from(format!("practice-{id}")),
            wrong_answer: "wrong".to_string(),
            correct_answer: "correct".to_string(),
            cause_tag: "concept".to_string(),
            explanation: crate::LocalizedText::plain("explanation"),
            rating,
            reviewed: false,
        }
    }
}
