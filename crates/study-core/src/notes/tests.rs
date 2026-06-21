use std::io::Cursor;

use super::*;
use crate::{
    AnswerKey, AnswerSubmission, AttemptId, LearnerProgress, MasteryState, PracticeItem,
    PracticeItemId, PracticeKind, ReviewDate, SpacedRepetitionState,
};
use zip::ZipArchive;

fn note() -> StudyNote {
    create_study_note(
        StudyNoteId::from("note_1"),
        LearnerId::from("learner"),
        CurriculumNodeId::from("node"),
        LocalizedText::plain("Note"),
        "The heart has {{c1::four chambers}}.",
        "2026-05-04T00:00:00Z",
    )
    .expect("note")
}

fn exact_item(id: &str, node_id: &str, prompt: &str, answer: &str) -> PracticeItem {
    PracticeItem {
        id: PracticeItemId::from(id),
        node_id: CurriculumNodeId::from(node_id),
        prompt: LocalizedText::plain(prompt),
        kind: PracticeKind::ExactAnswer,
        answer_key: AnswerKey::Exact {
            value: answer.to_string(),
            case_sensitive: false,
        },
        explanation: LocalizedText::plain("Review the worked solution."),
        skills: Vec::new(),
        difficulty: None,
    }
}

#[test]
fn notes_use_tench_document_and_cards_link_to_source() {
    let note = note();
    let card = create_card_from_note(
        StudyCardId::from("card_1"),
        StudyDeckId::from("deck"),
        &note,
        StudyCardKind::Basic,
        LocalizedText::plain("Question"),
        LocalizedText::plain("Answer"),
        "2026-05-04T00:01:00Z",
    )
    .expect("card");

    assert!(note.document.to_plain_text().contains("heart"));
    assert_eq!(card.source_note_id.as_ref(), Some(&note.id));
}

#[test]
fn cloze_cards_are_extracted_from_note_text() {
    let cards =
        extract_cloze_cards_from_note(StudyDeckId::from("deck"), &note(), "2026-05-04T00:01:00Z");

    assert_eq!(cards.len(), 1);
    assert!(cards[0].front.value.contains("[...]"));
    assert_eq!(cards[0].back.value, "four chambers");
}

#[test]
fn rich_card_media_payloads_round_trip_through_json() {
    let note = note();
    let image_card = create_image_occlusion_card_from_note(
        StudyCardId::from("image-card"),
        StudyDeckId::from("deck"),
        &note,
        LocalizedText::plain("Name the hidden chamber"),
        LocalizedText::plain("Left ventricle"),
        "assets/heart.png".to_string(),
        vec![ImageOcclusionMask {
            id: "mask-left-ventricle".to_string(),
            x: 0.24,
            y: 0.32,
            width: 0.18,
            height: 0.22,
            label: Some("left ventricle".to_string()),
        }],
        "2026-05-04T00:02:00Z",
    )
    .expect("image card");
    let audio_card = create_audio_card_from_note(
        StudyCardId::from("audio-card"),
        StudyDeckId::from("deck"),
        &note,
        LocalizedText::plain("Repeat the phrase"),
        LocalizedText::plain("bonjour"),
        "assets/bonjour.ogg".to_string(),
        "2026-05-04T00:03:00Z",
    )
    .expect("audio card");
    let code_card = create_code_card_from_note(
        StudyCardId::from("code-card"),
        StudyDeckId::from("deck"),
        &note,
        LocalizedText::plain("Trace the loop"),
        LocalizedText::plain("prints 0 1 2"),
        StudyCardCodePayload {
            language: "rust".to_string(),
            code: "for i in 0..3 { println!(\"{i}\"); }".to_string(),
            expected_output: Some("0\n1\n2".to_string()),
        },
        "2026-05-04T00:04:00Z",
    )
    .expect("code card");

    let json = export_study_cards(
        StudyCardExchangeFormat::Json,
        &[image_card, audio_card, code_card],
    )
    .expect("json export");
    let imported = import_study_cards(
        StudyCardExchangeFormat::Json,
        StudyDeckId::from("deck"),
        CurriculumNodeId::from("node"),
        None,
        &json,
        "2026-05-04T00:05:00Z",
    )
    .expect("json import");

    assert_eq!(imported[0].kind, StudyCardKind::ImageOcclusion);
    assert_eq!(imported[0].media.occlusions.len(), 1);
    assert_eq!(
        imported[1].media.audio_ref.as_deref(),
        Some("assets/bonjour.ogg")
    );
    assert_eq!(
        imported[2]
            .media
            .code
            .as_ref()
            .map(|code| code.language.as_str()),
        Some("rust")
    );
}

#[test]
fn cards_round_trip_through_tsv_and_csv() {
    let note = note();
    let mut card = create_card_from_note(
        StudyCardId::from("card_1"),
        StudyDeckId::from("deck"),
        &note,
        StudyCardKind::Basic,
        LocalizedText::plain("Front"),
        LocalizedText::plain("Back"),
        "2026-05-04T00:01:00Z",
    )
    .expect("card");
    card.tags = vec!["math".to_string(), "review".to_string()];
    let tsv = export_study_cards(
        StudyCardExchangeFormat::AnkiTsv,
        std::slice::from_ref(&card),
    )
    .expect("export tsv");
    let cards = import_study_cards(
        StudyCardExchangeFormat::AnkiTsv,
        StudyDeckId::from("deck"),
        CurriculumNodeId::from("node"),
        None,
        &tsv,
        "2026-05-04T00:02:00Z",
    )
    .expect("import tsv");

    assert_eq!(cards[0].front.value, "Front");
    assert_eq!(cards[0].back.value, "Back");

    let csv = export_study_cards(StudyCardExchangeFormat::Csv, &[card]).expect("export csv");
    assert!(csv.contains("\"Front\",\"Back\""));
}

#[test]
fn cards_round_trip_through_anki_apkg_package() {
    let note = note();
    let mut card = create_card_from_note(
        StudyCardId::from("card_1"),
        StudyDeckId::from("deck"),
        &note,
        StudyCardKind::Basic,
        LocalizedText::plain("Front"),
        LocalizedText::plain("Back"),
        "2026-05-04T00:01:00Z",
    )
    .expect("card");
    card.tags = vec!["science".to_string(), "diagram".to_string()];

    let bytes = export_study_cards_anki_package_zip(&[card]).expect("export package");
    let mut archive = ZipArchive::new(Cursor::new(bytes.clone())).expect("zip");
    assert!(archive.by_name("collection.anki2").is_ok());
    assert!(archive.by_name("media").is_ok());
    let imported = import_study_cards_anki_package_zip(
        &bytes,
        StudyDeckId::from("deck"),
        CurriculumNodeId::from("node"),
        None,
        "2026-05-04T00:02:00Z",
    )
    .expect("import package");

    assert!(bytes.starts_with(b"PK"));
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].front.value, "Front");
    assert_eq!(imported[0].back.value, "Back");
    assert_eq!(imported[0].tags, vec!["science", "diagram"]);
}

#[test]
fn cards_round_trip_through_legacy_tench_anki_package_zip() {
    let note = note();
    let card = create_card_from_note(
        StudyCardId::from("card_1"),
        StudyDeckId::from("deck"),
        &note,
        StudyCardKind::Basic,
        LocalizedText::plain("Legacy Front"),
        LocalizedText::plain("Legacy Back"),
        "2026-05-04T00:01:00Z",
    )
    .expect("card");

    let bytes = export_study_cards_anki_tench_package_zip(&[card]).expect("export package");
    let imported = import_study_cards_anki_package_zip(
        &bytes,
        StudyDeckId::from("deck"),
        CurriculumNodeId::from("node"),
        None,
        "2026-05-04T00:02:00Z",
    )
    .expect("import package");

    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].front.value, "Legacy Front");
    assert_eq!(imported[0].back.value, "Legacy Back");
}

#[test]
fn cards_export_import_generic_tsv_with_tags() {
    let note = note();
    let mut card = create_card_from_note(
        StudyCardId::from("card_1"),
        StudyDeckId::from("deck"),
        &note,
        StudyCardKind::Basic,
        LocalizedText::plain("Front"),
        LocalizedText::plain("Back"),
        "2026-05-04T00:01:00Z",
    )
    .expect("card");
    card.tags = vec!["science".to_string(), "diagram".to_string()];

    let tsv = export_study_cards(StudyCardExchangeFormat::Tsv, &[card]).expect("export tsv");
    let imported = import_study_cards(
        StudyCardExchangeFormat::Tsv,
        StudyDeckId::from("deck"),
        CurriculumNodeId::from("node"),
        None,
        &tsv,
        "2026-05-04T00:02:00Z",
    )
    .expect("import tsv");

    assert_eq!(imported[0].front.value, "Front");
    assert_eq!(
        imported[0].tags,
        vec!["science".to_string(), "diagram".to_string()]
    );
}

#[test]
fn imported_card_cleanup_removes_existing_and_batch_duplicates() {
    let existing = create_card_from_note(
        StudyCardId::from("existing"),
        StudyDeckId::from("deck"),
        &note(),
        StudyCardKind::Basic,
        LocalizedText::plain("Café term"),
        LocalizedText::plain("coffee"),
        "2026-05-04T00:00:00Z",
    )
    .expect("existing card");
    let imported = import_study_cards(
        StudyCardExchangeFormat::Tsv,
        StudyDeckId::from("deck"),
        CurriculumNodeId::from("node"),
        None,
        "Cafe term\tcoffee\nNew term\tanswer\nNew term\tanswer",
        "2026-05-04T00:01:00Z",
    )
    .expect("import cards");

    let duplicates = find_duplicate_study_cards(&imported);
    let report = cleanup_imported_study_cards(&[existing], imported);

    assert_eq!(duplicates.len(), 1);
    assert_eq!(report.cards.len(), 1);
    assert_eq!(report.cards[0].front.value, "New term");
    assert_eq!(report.removed_count, 2);
    assert_eq!(report.duplicates[0].existing_id, "existing");
    assert_eq!(report.duplicates[1].existing_id, "imported-card-1");
}

#[test]
fn imported_note_cleanup_removes_existing_and_batch_duplicates() {
    let existing = create_study_note(
        StudyNoteId::from("existing-note"),
        LearnerId::from("learner"),
        CurriculumNodeId::from("node"),
        LocalizedText::plain("Café note"),
        "Practice pronunciation.",
        "2026-05-04T00:00:00Z",
    )
    .expect("existing note");
    let imported = vec![
        create_study_note(
            StudyNoteId::from("imported-note-0"),
            LearnerId::from("learner"),
            CurriculumNodeId::from("node"),
            LocalizedText::plain("Cafe note"),
            "Practice pronunciation.",
            "2026-05-04T00:01:00Z",
        )
        .expect("duplicate existing"),
        create_study_note(
            StudyNoteId::from("imported-note-1"),
            LearnerId::from("learner"),
            CurriculumNodeId::from("node"),
            LocalizedText::plain("Unique note"),
            "Unique body.",
            "2026-05-04T00:01:00Z",
        )
        .expect("unique"),
        create_study_note(
            StudyNoteId::from("imported-note-2"),
            LearnerId::from("learner"),
            CurriculumNodeId::from("node"),
            LocalizedText::plain("Unique note"),
            "Unique body.",
            "2026-05-04T00:01:00Z",
        )
        .expect("duplicate imported"),
    ];

    let duplicates = find_duplicate_study_notes(&imported);
    let report = cleanup_imported_study_notes(&[existing], imported);

    assert_eq!(duplicates.len(), 1);
    assert_eq!(report.notes.len(), 1);
    assert_eq!(report.notes[0].title.value, "Unique note");
    assert_eq!(report.removed_count, 2);
    assert_eq!(report.duplicates[0].existing_id, "existing-note");
    assert_eq!(report.duplicates[1].existing_id, "imported-note-1");
}

#[test]
fn notes_round_trip_through_markdown() {
    let mut note = note();
    note.tags = vec!["biology".to_string()];

    let markdown = export_study_notes(StudyNoteExchangeFormat::Markdown, &[note])
        .expect("export markdown notes");
    let imported = import_study_notes(
        StudyNoteExchangeFormat::Markdown,
        LearnerId::from("learner"),
        CurriculumNodeId::from("node"),
        ContentLocale::parse("ko-KR"),
        &markdown,
        "2026-05-04T00:02:00Z",
    )
    .expect("import markdown notes");

    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].title.value, "Note");
    assert!(imported[0].document.to_plain_text().contains("heart"));
    assert_eq!(
        imported[0].title.locale.as_ref().map(ContentLocale::bcp47),
        Some("ko-KR".to_string())
    );
}

#[test]
fn progress_report_exports_markdown_and_csv() {
    let mut mastery = MasteryState::default();
    mastery.update(true);
    mastery.update(true);
    mastery.update(true);
    let progress = LearnerProgress {
        learner_id: LearnerId::from("learner"),
        node_id: CurriculumNodeId::from("node"),
        mastery,
        attempts: Vec::new(),
        review_state: SpacedRepetitionState {
            due_on: Some(ReviewDate::parse("2026-05-04").expect("date")),
            ..SpacedRepetitionState::default()
        },
    };

    let markdown = export_study_progress_report(
        StudyProgressExportFormat::Markdown,
        std::slice::from_ref(&progress),
        "2026-05-04T00:03:00Z",
    )
    .expect("export progress markdown");
    let csv = export_study_progress_report(
        StudyProgressExportFormat::Csv,
        &[progress],
        "2026-05-04T00:03:00Z",
    )
    .expect("export progress csv");

    assert!(markdown.contains("Average mastery: 100.0%"));
    assert!(csv.contains("progress_count"));
    assert!(csv.ends_with(",1,0,0,1,3,3"));
}

#[test]
fn exam_session_grades_submissions_and_reports_weak_items() {
    let item = exact_item("item_1", "node", "2+2", "4");
    let session = ExamSession {
        id: ExamSessionId::from("exam"),
        learner_id: LearnerId::from("learner"),
        title: LocalizedText::plain("Exam"),
        item_ids: vec![item.id.clone()],
        submissions: vec![AnswerSubmission {
            attempt_id: AttemptId::from("attempt"),
            item_id: item.id.clone(),
            response: "5".to_string(),
            locale: None,
            submitted_at: None,
        }],
        results: Vec::new(),
        time_limit_seconds: Some(60),
        started_at: Some("2026-05-04T00:00:00Z".to_string()),
        completed_at: None,
    };

    let (_, report) =
        grade_exam_session(session, &[item], "2026-05-04T00:01:00Z").expect("grade exam");

    assert_eq!(report.total, 1);
    assert_eq!(report.correct, 0);
    assert_eq!(report.weak_item_ids, vec![PracticeItemId::from("item_1")]);
}

#[test]
fn exam_builder_respects_coverage_and_time_limit() {
    let items = vec![
        exact_item("algebra_1", "math-algebra", "x + 1 = 3", "2"),
        exact_item("algebra_2", "math-algebra", "2x = 10", "5"),
        exact_item("biology_1", "science-biology", "Heart chambers", "4"),
    ];
    let blueprint = ExamBlueprint {
        id: ExamSessionId::from("exam"),
        learner_id: LearnerId::from("learner"),
        title: LocalizedText::plain("Coverage exam"),
        item_count: 3,
        coverage_constraints: vec![
            ExamCoverageConstraint {
                node_id: CurriculumNodeId::from("math-algebra"),
                min_items: 2,
                max_items: Some(2),
            },
            ExamCoverageConstraint {
                node_id: CurriculumNodeId::from("science-biology"),
                min_items: 1,
                max_items: None,
            },
        ],
        time_limit_seconds: Some(60),
        started_at: Some("2026-05-04T09:00:00Z".to_string()),
    };

    let report = build_exam_session(blueprint, &items).expect("build exam");
    let timing = exam_timing_status(&report.session, 45);
    let expired = exam_timing_status(&report.session, 60);

    assert_eq!(
        report.selected_item_ids,
        vec![
            PracticeItemId::from("algebra_1"),
            PracticeItemId::from("algebra_2"),
            PracticeItemId::from("biology_1"),
        ]
    );
    assert!(report.coverage_issues.is_empty());
    assert_eq!(timing.remaining_seconds, Some(15));
    assert!(!timing.expired);
    assert_eq!(expired.remaining_seconds, Some(0));
    assert!(expired.expired);
}

#[test]
fn exam_builder_reports_unmet_objective_coverage() {
    let items = vec![exact_item("algebra_1", "math-algebra", "x + 1 = 3", "2")];
    let blueprint = ExamBlueprint {
        id: ExamSessionId::from("exam"),
        learner_id: LearnerId::from("learner"),
        title: LocalizedText::plain("Coverage exam"),
        item_count: 2,
        coverage_constraints: vec![ExamCoverageConstraint {
            node_id: CurriculumNodeId::from("science-biology"),
            min_items: 1,
            max_items: None,
        }],
        time_limit_seconds: None,
        started_at: None,
    };

    let report = build_exam_session(blueprint, &items).expect("build exam");

    assert_eq!(
        report.selected_item_ids,
        vec![PracticeItemId::from("algebra_1")]
    );
    assert_eq!(report.coverage_issues.len(), 2);
    assert!(report.coverage_issues[0].contains("science-biology"));
    assert!(report.coverage_issues[1].contains("only 1 unique items"));
}

#[test]
fn rubric_exam_scoring_and_review_surface_results() {
    let proof_item = PracticeItem {
        id: PracticeItemId::from("proof_1"),
        node_id: CurriculumNodeId::from("math-proof"),
        prompt: LocalizedText::plain("Prove the identity."),
        kind: PracticeKind::ProofStep,
        answer_key: AnswerKey::Rubric {
            rubric_id: "proof-rubric".to_string(),
            max_score: 4.0,
        },
        explanation: LocalizedText::plain("Use algebraic equivalence step by step."),
        skills: Vec::new(),
        difficulty: Some(0.7),
    };
    let recall_item = exact_item("recall_1", "science-biology", "Heart chambers", "4");
    let session = ExamSession {
        id: ExamSessionId::from("exam"),
        learner_id: LearnerId::from("learner"),
        title: LocalizedText::plain("Mixed exam"),
        item_ids: vec![proof_item.id.clone(), recall_item.id.clone()],
        submissions: vec![
            AnswerSubmission {
                attempt_id: AttemptId::from("attempt_proof"),
                item_id: proof_item.id.clone(),
                response: "structured proof".to_string(),
                locale: None,
                submitted_at: None,
            },
            AnswerSubmission {
                attempt_id: AttemptId::from("attempt_recall"),
                item_id: recall_item.id.clone(),
                response: "2".to_string(),
                locale: None,
                submitted_at: None,
            },
        ],
        results: Vec::new(),
        time_limit_seconds: Some(120),
        started_at: Some("2026-05-04T09:00:00Z".to_string()),
        completed_at: None,
    };

    let items = vec![proof_item, recall_item];
    let (graded, report) = grade_exam_session_with_rubrics(
        session,
        &items,
        &[RubricScore {
            item_id: PracticeItemId::from("proof_1"),
            score: 3.6,
            feedback_code: Some("clear_reasoning".to_string()),
        }],
        "2026-05-04T09:02:00Z",
    )
    .expect("grade with rubric");
    let review = build_exam_result_review(&graded, &items).expect("review");

    assert_eq!(report.total, 2);
    assert_eq!(report.correct, 1);
    assert_eq!(report.score, 0.5);
    assert_eq!(
        graded.results[0].result.feedback_code.as_deref(),
        Some("clear_reasoning")
    );
    assert_eq!(
        review.mastered_node_ids,
        vec![CurriculumNodeId::from("math-proof")]
    );
    assert_eq!(
        review.weak_node_ids,
        vec![CurriculumNodeId::from("science-biology")]
    );
    assert_eq!(review.questions[0].expected, "proof-rubric / 4");
}

#[test]
fn exam_report_exports_markdown_csv_and_json() {
    let item = exact_item("item_1", "math-arithmetic", "2+2", "4");
    let session = ExamSession {
        id: ExamSessionId::from("exam"),
        learner_id: LearnerId::from("learner"),
        title: LocalizedText::plain("Arithmetic exam"),
        item_ids: vec![item.id.clone()],
        submissions: vec![AnswerSubmission {
            attempt_id: AttemptId::from("attempt"),
            item_id: item.id.clone(),
            response: "4".to_string(),
            locale: None,
            submitted_at: None,
        }],
        results: Vec::new(),
        time_limit_seconds: None,
        started_at: None,
        completed_at: None,
    };
    let (graded, report) =
        grade_exam_session(session, std::slice::from_ref(&item), "2026-05-04T09:01:00Z")
            .expect("grade");
    let review = build_exam_result_review(&graded, std::slice::from_ref(&item)).expect("review");

    let markdown = export_study_exam_report(
        StudyExamReportExportFormat::Markdown,
        &graded,
        &report,
        Some(&review),
    )
    .expect("markdown");
    let csv = export_study_exam_report(
        StudyExamReportExportFormat::Csv,
        &graded,
        &report,
        Some(&review),
    )
    .expect("csv");
    let json = export_study_exam_report(
        StudyExamReportExportFormat::Json,
        &graded,
        &report,
        Some(&review),
    )
    .expect("json");

    assert!(markdown.contains("Arithmetic exam"));
    assert!(csv.contains("session_id,title,score"));
    assert!(json.contains("\"report\""));
    assert!(json.contains("\"review\""));
}

#[test]
fn progress_report_view_is_local_rows_without_cloud() {
    let mut mastered = MasteryState::default();
    mastered.update(true);
    mastered.update(true);
    mastered.update(true);
    let mut in_progress = MasteryState::default();
    in_progress.update(true);
    in_progress.update(false);
    in_progress.update(true);
    let needs_practice = MasteryState::default();
    let progress = vec![
        LearnerProgress {
            learner_id: LearnerId::from("learner"),
            node_id: CurriculumNodeId::from("node-b"),
            mastery: mastered,
            attempts: Vec::new(),
            review_state: SpacedRepetitionState::default(),
        },
        LearnerProgress {
            learner_id: LearnerId::from("learner"),
            node_id: CurriculumNodeId::from("node-a"),
            mastery: needs_practice,
            attempts: Vec::new(),
            review_state: SpacedRepetitionState {
                due_on: Some(ReviewDate::parse("2026-05-05").expect("date")),
                ..SpacedRepetitionState::default()
            },
        },
        LearnerProgress {
            learner_id: LearnerId::from("learner"),
            node_id: CurriculumNodeId::from("node-c"),
            mastery: in_progress,
            attempts: Vec::new(),
            review_state: SpacedRepetitionState::default(),
        },
    ];

    let view = build_study_progress_report_view(&progress, "2026-05-04T10:00:00Z");

    assert_eq!(view.report.progress_count, 3);
    assert_eq!(view.rows[0].node_id, CurriculumNodeId::from("node-a"));
    assert_eq!(view.rows[0].status, "needs_practice");
    assert_eq!(view.rows[0].review_due.as_deref(), Some("2026-05-05"));
    assert_eq!(view.rows[1].status, "mastered");
    assert_eq!(view.rows[2].status, "in_progress");
}
