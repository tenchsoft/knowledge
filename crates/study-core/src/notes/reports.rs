use std::collections::{BTreeMap, BTreeSet};

use crate::{
    AnswerKey, CurriculumNodeId, GradingResult, LearnerProgress, PracticeItem, PracticeItemId,
};

use super::{
    escape_csv, escape_csv_cell, sanitize_line, ExamBlueprint, ExamBuildReport, ExamQuestionResult,
    ExamQuestionReview, ExamReport, ExamResultReview, ExamSession, ExamTimingStatus, RubricScore,
    StudyExamReportExportFormat, StudyProgressExportFormat, StudyProgressReport,
    StudyProgressReportRow, StudyProgressReportView,
};

pub fn study_progress_report(
    progress: &[LearnerProgress],
    generated_at: impl Into<String>,
) -> StudyProgressReport {
    let generated_at = generated_at.into();
    let progress_count = progress.len() as u32;
    let total_mastery = progress
        .iter()
        .map(|progress| progress.mastery.score)
        .sum::<f32>();
    let total_attempts = progress
        .iter()
        .map(|progress| progress.mastery.attempts)
        .sum::<u32>();
    let total_correct = progress
        .iter()
        .map(|progress| progress.mastery.correct)
        .sum::<u32>();
    StudyProgressReport {
        generated_at,
        progress_count,
        average_mastery: if progress.is_empty() {
            0.0
        } else {
            total_mastery / progress.len() as f32
        },
        mastered_nodes: progress
            .iter()
            .filter(|progress| progress.mastery.score >= 0.85 && progress.mastery.attempts >= 3)
            .count() as u32,
        needs_practice_nodes: progress
            .iter()
            .filter(|progress| progress.mastery.score < 0.6 || progress.mastery.attempts == 0)
            .count() as u32,
        suspended_reviews: progress
            .iter()
            .filter(|progress| progress.review_state.suspended)
            .count() as u32,
        due_reviews: progress
            .iter()
            .filter(|progress| progress.review_state.due_on.is_some())
            .count() as u32,
        total_attempts,
        total_correct,
    }
}

pub fn export_study_progress_report(
    format: StudyProgressExportFormat,
    progress: &[LearnerProgress],
    generated_at: impl Into<String>,
) -> Result<String, String> {
    let report = study_progress_report(progress, generated_at);
    match format {
        StudyProgressExportFormat::Json => {
            serde_json::to_string_pretty(&report).map_err(|error| error.to_string())
        }
        StudyProgressExportFormat::Csv => Ok(format!(
            concat!(
                "generated_at,progress_count,average_mastery,mastered_nodes,needs_practice_nodes,",
                "suspended_reviews,due_reviews,total_attempts,total_correct\n",
                "{},{},{:.4},{},{},{},{},{},{}"
            ),
            escape_csv_cell(&report.generated_at),
            report.progress_count,
            report.average_mastery,
            report.mastered_nodes,
            report.needs_practice_nodes,
            report.suspended_reviews,
            report.due_reviews,
            report.total_attempts,
            report.total_correct
        )),
        StudyProgressExportFormat::Markdown => Ok(format!(
            concat!(
                "# Study Progress\n\n",
                "- Generated at: {generated_at}\n",
                "- Progress records: {progress_count}\n",
                "- Average mastery: {average_mastery:.1}%\n",
                "- Mastered nodes: {mastered_nodes}\n",
                "- Needs practice: {needs_practice_nodes}\n",
                "- Due reviews: {due_reviews}\n",
                "- Suspended reviews: {suspended_reviews}\n",
                "- Attempts: {total_correct}/{total_attempts} correct\n"
            ),
            generated_at = report.generated_at,
            progress_count = report.progress_count,
            average_mastery = report.average_mastery * 100.0,
            mastered_nodes = report.mastered_nodes,
            needs_practice_nodes = report.needs_practice_nodes,
            due_reviews = report.due_reviews,
            suspended_reviews = report.suspended_reviews,
            total_correct = report.total_correct,
            total_attempts = report.total_attempts
        )),
    }
}

pub fn export_study_exam_report(
    format: StudyExamReportExportFormat,
    session: &ExamSession,
    report: &ExamReport,
    review: Option<&ExamResultReview>,
) -> Result<String, String> {
    if session.id != report.session_id {
        return Err(format!(
            "exam report {} does not match session {}",
            report.session_id.as_str(),
            session.id.as_str()
        ));
    }
    if let Some(review) = review {
        if review.session_id != session.id {
            return Err(format!(
                "exam review {} does not match session {}",
                review.session_id.as_str(),
                session.id.as_str()
            ));
        }
    }

    match format {
        StudyExamReportExportFormat::Json => serde_json::to_string_pretty(&serde_json::json!({
            "session": session,
            "report": report,
            "review": review,
        }))
        .map_err(|error| error.to_string()),
        StudyExamReportExportFormat::Csv => {
            let mut lines = vec![
                "session_id,title,score,correct,total,item_id,node_id,result_score,correct,response"
                    .to_string(),
            ];
            if let Some(review) = review {
                for question in &review.questions {
                    lines.push(format!(
                        "\"{}\",\"{}\",{:.4},{},{},\"{}\",\"{}\",{:.4},{},\"{}\"",
                        escape_csv(&session.id.0),
                        escape_csv(&session.title.value),
                        report.score,
                        report.correct,
                        report.total,
                        escape_csv(question.item_id.as_str()),
                        escape_csv(question.node_id.as_str()),
                        question.result.score,
                        question.result.correct,
                        escape_csv(&question.response)
                    ));
                }
            } else {
                lines.push(format!(
                    "\"{}\",\"{}\",{:.4},{},{},\"\",\"\",0.0000,false,\"\"",
                    escape_csv(&session.id.0),
                    escape_csv(&session.title.value),
                    report.score,
                    report.correct,
                    report.total
                ));
            }
            Ok(lines.join("\n"))
        }
        StudyExamReportExportFormat::Markdown => {
            let mut markdown = format!(
                concat!(
                    "# {}\n\n",
                    "- Session: {}\n",
                    "- Score: {:.1}%\n",
                    "- Correct: {}/{}\n"
                ),
                session.title.value,
                session.id.as_str(),
                report.score * 100.0,
                report.correct,
                report.total
            );
            if !report.weak_item_ids.is_empty() {
                markdown.push_str(&format!(
                    "- Weak items: {}\n",
                    report
                        .weak_item_ids
                        .iter()
                        .map(PracticeItemId::as_str)
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if let Some(review) = review {
                markdown.push_str("\n## Review\n\n");
                for question in &review.questions {
                    markdown.push_str(&format!(
                        concat!(
                            "### {}\n\n",
                            "- Node: {}\n",
                            "- Score: {:.1}%\n",
                            "- Correct: {}\n",
                            "- Response: {}\n",
                            "- Expected: {}\n\n"
                        ),
                        question.prompt.value,
                        question.node_id.as_str(),
                        question.result.score * 100.0,
                        question.result.correct,
                        sanitize_line(&question.response),
                        sanitize_line(&question.expected)
                    ));
                }
            }
            Ok(markdown)
        }
    }
}

pub fn build_exam_session(
    blueprint: ExamBlueprint,
    items: &[PracticeItem],
) -> Result<ExamBuildReport, String> {
    if blueprint.item_count == 0 {
        return Err("exam blueprint requires at least one item".to_string());
    }
    let mut selected = Vec::<PracticeItemId>::new();
    let mut selected_set = BTreeSet::<PracticeItemId>::new();
    let mut coverage_issues = Vec::new();
    let item_by_node = items.iter().fold(
        BTreeMap::<CurriculumNodeId, Vec<&PracticeItem>>::new(),
        |mut map, item| {
            map.entry(item.node_id.clone()).or_default().push(item);
            map
        },
    );

    for constraint in &blueprint.coverage_constraints {
        let available = item_by_node
            .get(&constraint.node_id)
            .cloned()
            .unwrap_or_default();
        let max_items = constraint
            .max_items
            .unwrap_or(constraint.min_items)
            .max(constraint.min_items);
        let take = constraint
            .min_items
            .min(max_items)
            .min(available.len() as u32);
        if take < constraint.min_items {
            coverage_issues.push(format!(
                "node {} has {} available items but requires {}",
                constraint.node_id.as_str(),
                available.len(),
                constraint.min_items
            ));
        }
        for item in available.into_iter().take(take as usize) {
            if selected_set.insert(item.id.clone()) {
                selected.push(item.id.clone());
            }
        }
    }

    for item in items {
        if selected.len() >= blueprint.item_count as usize {
            break;
        }
        if selected_set.insert(item.id.clone()) {
            selected.push(item.id.clone());
        }
    }
    if selected.len() < blueprint.item_count as usize {
        coverage_issues.push(format!(
            "exam requested {} items but only {} unique items are available",
            blueprint.item_count,
            selected.len()
        ));
    }
    selected.truncate(blueprint.item_count as usize);

    let session = ExamSession {
        id: blueprint.id,
        learner_id: blueprint.learner_id,
        title: blueprint.title,
        item_ids: selected.clone(),
        submissions: Vec::new(),
        results: Vec::new(),
        time_limit_seconds: blueprint.time_limit_seconds,
        started_at: blueprint.started_at,
        completed_at: None,
    };

    Ok(ExamBuildReport {
        session,
        selected_item_ids: selected,
        coverage_issues,
    })
}

pub fn exam_timing_status(session: &ExamSession, elapsed_seconds: u32) -> ExamTimingStatus {
    let remaining_seconds = session
        .time_limit_seconds
        .map(|limit| limit.saturating_sub(elapsed_seconds));
    ExamTimingStatus {
        elapsed_seconds,
        remaining_seconds,
        expired: session
            .time_limit_seconds
            .map(|limit| elapsed_seconds >= limit)
            .unwrap_or(false),
    }
}

pub fn grade_exam_session(
    session: ExamSession,
    items: &[PracticeItem],
    completed_at: impl Into<String>,
) -> Result<(ExamSession, ExamReport), String> {
    grade_exam_session_with_rubrics(session, items, &[], completed_at)
}

pub fn grade_exam_session_with_rubrics(
    mut session: ExamSession,
    items: &[PracticeItem],
    rubric_scores: &[RubricScore],
    completed_at: impl Into<String>,
) -> Result<(ExamSession, ExamReport), String> {
    let rubric_scores = rubric_scores
        .iter()
        .map(|score| (&score.item_id, score))
        .collect::<BTreeMap<_, _>>();
    let mut results = Vec::new();
    for submission in &session.submissions {
        let item = items
            .iter()
            .find(|item| item.id == submission.item_id)
            .ok_or_else(|| format!("practice item {} not found", submission.item_id.as_str()))?;
        let result = if let Some(rubric_score) = rubric_scores.get(&item.id) {
            rubric_grading_result(item, rubric_score)?
        } else {
            item.grade(submission)
        };
        results.push(ExamQuestionResult {
            item_id: item.id.clone(),
            result,
        });
    }
    session.results = results;
    session.completed_at = Some(completed_at.into());
    let total = session.results.len() as u32;
    let correct = session
        .results
        .iter()
        .filter(|result| result.result.correct)
        .count() as u32;
    let weak_item_ids = session
        .results
        .iter()
        .filter(|result| !result.result.correct)
        .map(|result| result.item_id.clone())
        .collect::<Vec<_>>();
    let report = ExamReport {
        session_id: session.id.clone(),
        score: if total == 0 {
            0.0
        } else {
            correct as f32 / total as f32
        },
        correct,
        total,
        weak_item_ids,
    };
    Ok((session, report))
}

pub fn build_exam_result_review(
    session: &ExamSession,
    items: &[PracticeItem],
) -> Result<ExamResultReview, String> {
    let mut questions = Vec::new();
    let mut weak_node_ids = Vec::new();
    let mut mastered_node_ids = Vec::new();
    for result in &session.results {
        let item = items
            .iter()
            .find(|item| item.id == result.item_id)
            .ok_or_else(|| format!("practice item {} not found", result.item_id.as_str()))?;
        let response = session
            .submissions
            .iter()
            .find(|submission| submission.item_id == item.id)
            .map(|submission| submission.response.clone())
            .unwrap_or_default();
        if result.result.score >= 0.85 {
            push_unique_node(&mut mastered_node_ids, item.node_id.clone());
        } else if result.result.score < 0.6 {
            push_unique_node(&mut weak_node_ids, item.node_id.clone());
        }
        questions.push(ExamQuestionReview {
            item_id: item.id.clone(),
            node_id: item.node_id.clone(),
            prompt: item.prompt.clone(),
            response,
            expected: answer_key_label(&item.answer_key),
            explanation: item.explanation.clone(),
            result: result.result.clone(),
        });
    }
    Ok(ExamResultReview {
        session_id: session.id.clone(),
        questions,
        weak_node_ids,
        mastered_node_ids,
    })
}

pub fn build_study_progress_report_view(
    progress: &[LearnerProgress],
    generated_at: impl Into<String>,
) -> StudyProgressReportView {
    let report = study_progress_report(progress, generated_at);
    let mut rows = progress
        .iter()
        .map(|progress| {
            let status = if progress.mastery.score >= 0.85 && progress.mastery.attempts >= 3 {
                "mastered"
            } else if progress.mastery.score < 0.6 || progress.mastery.attempts == 0 {
                "needs_practice"
            } else {
                "in_progress"
            };
            StudyProgressReportRow {
                node_id: progress.node_id.clone(),
                mastery_percent: progress.mastery.score * 100.0,
                attempts: progress.mastery.attempts,
                correct: progress.mastery.correct,
                review_due: progress
                    .review_state
                    .due_on
                    .as_ref()
                    .map(|date| date.0.clone()),
                status: status.to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.node_id.cmp(&right.node_id));
    StudyProgressReportView { report, rows }
}

fn rubric_grading_result(
    item: &PracticeItem,
    rubric_score: &RubricScore,
) -> Result<GradingResult, String> {
    let AnswerKey::Rubric { max_score, .. } = &item.answer_key else {
        return Err(format!(
            "rubric score supplied for non-rubric item {}",
            item.id.as_str()
        ));
    };
    if !rubric_score.score.is_finite() || rubric_score.score < 0.0 {
        return Err(format!("invalid rubric score for {}", item.id.as_str()));
    }
    let score = if *max_score <= 0.0 {
        0.0
    } else {
        (rubric_score.score / *max_score).clamp(0.0, 1.0)
    };
    Ok(GradingResult {
        correct: score >= 0.8,
        score,
        feedback_code: rubric_score.feedback_code.clone(),
    })
}

fn answer_key_label(key: &AnswerKey) -> String {
    match key {
        AnswerKey::Exact { value, .. } => value.clone(),
        AnswerKey::Numeric { value, tolerance } => format!("{value} +/- {tolerance}"),
        AnswerKey::MultipleChoice { option_id } => option_id.clone(),
        AnswerKey::Cloze { accepted } => accepted.join(" | "),
        AnswerKey::Rubric {
            rubric_id,
            max_score,
        } => format!("{rubric_id} / {max_score}"),
    }
}

fn push_unique_node(values: &mut Vec<CurriculumNodeId>, value: CurriculumNodeId) {
    if !values.contains(&value) {
        values.push(value);
    }
}
