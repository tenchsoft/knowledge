use super::*;
use crate::{
    AnswerKey, ContentLocale, ContentPackId, CurriculumAuthority, CurriculumGraph, CurriculumId,
    EducationLevel, LevelRange, PackIntegrity, PracticeItemId, PracticeKind, SubjectDomain,
    VisualAccessibility, VisualInteraction, VisualPlayback, VisualRenderer, VisualRendererEngine,
};

#[test]
fn custom_domain_requires_custom_authority_and_license() {
    let locale = ContentLocale::parse("en-US").unwrap();
    let draft = CurriculumPackDraft {
        id: CurriculumPackDraftId::from("draft"),
        owner: PackOwner {
            name: "".to_string(),
            organization: None,
            contact: None,
            responsibility_statement: "owner is responsible".to_string(),
        },
        license: LicenseInfo {
            name: "".to_string(),
            url: None,
            permits_redistribution: false,
        },
        update_policy: PackUpdatePolicy {
            channel: PackUpdateChannel::Unspecified,
            cadence: None,
            deprecation_policy: None,
            preserve_progress_on_stable_node_ids: true,
        },
        curriculum: Curriculum {
            id: CurriculumId::from("social"),
            domain: SubjectDomain::Custom {
                id: crate::CustomDomainId::from("social"),
                label: "Social Studies".to_string(),
                owner: "school".to_string(),
            },
            title: LocalizedStringSet::plain("Social Studies"),
            description: LocalizedStringSet::plain("Custom curriculum"),
            locale: locale.clone(),
            supported_locales: vec![locale.clone()],
            authority: CurriculumAuthority {
                owner: "school".to_string(),
                source_url: None,
                version: None,
                custom: false,
            },
            level_range: LevelRange {
                start: EducationLevel::Kindergarten,
                end: EducationLevel::HighSchool,
            },
            graph: CurriculumGraph::default(),
            standards: Vec::new(),
            metadata: Default::default(),
        },
        manifest: ContentPackManifest {
            id: ContentPackId::from("pack"),
            title: "Social".to_string(),
            curriculum_id: CurriculumId::from("social"),
            version: "0.1".to_string(),
            default_locale: locale.clone(),
            required_locales: vec![locale.clone()],
            provided_locales: vec![locale],
            lessons: Vec::new(),
            assets: Vec::new(),
            visuals: Vec::new(),
            practice_items: Vec::new(),
            assessments: Vec::new(),
            glossary_terms: Vec::new(),
            integrity: PackIntegrity {
                algorithm: "sha256".to_string(),
                content_hash: "".to_string(),
            },
        },
        lessons: Vec::new(),
        problems: Vec::new(),
        visuals: Vec::new(),
        assessments: Vec::new(),
        glossary: Vec::new(),
    };
    let encoded = export_curriculum_pack_draft_json(&draft).expect("export draft");
    let decoded = import_curriculum_pack_draft_json(&encoded).expect("import draft");

    let report = decoded.validate_for_distribution();

    assert!(!report.is_valid());
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.code == "custom_authority_required"));
}

#[test]
fn custom_curriculum_draft_adds_lesson_and_publishes() {
    let locale = ContentLocale::parse("en-US").unwrap();
    let request = CustomCurriculumDraftRequest {
        draft_id: CurriculumPackDraftId::from("draft"),
        pack_id: ContentPackId::from("pack"),
        curriculum_id: CurriculumId::from("school-social"),
        domain_id: crate::CustomDomainId::from("social"),
        domain_label: "Social Studies".to_string(),
        title: LocalizedStringSet::plain("Social Studies"),
        description: LocalizedStringSet::plain("School-managed curriculum"),
        owner: PackOwner {
            name: "school".to_string(),
            organization: None,
            contact: None,
            responsibility_statement: "school-owned custom curriculum".to_string(),
        },
        license: LicenseInfo {
            name: "internal".to_string(),
            url: None,
            permits_redistribution: false,
        },
        update_policy: PackUpdatePolicy {
            channel: PackUpdateChannel::UserManaged,
            cadence: None,
            deprecation_policy: None,
            preserve_progress_on_stable_node_ids: true,
        },
        default_locale: locale,
        required_locales: Vec::new(),
        level_range: LevelRange {
            start: EducationLevel::Kindergarten,
            end: EducationLevel::HighSchool,
        },
        version: "1.0.0".to_string(),
        integrity_hash: "hash".to_string(),
        created_at: "2026-05-04T00:00:00Z".to_string(),
    };
    let draft = new_custom_curriculum_pack_draft(request);
    let draft = add_lesson_to_curriculum_pack_draft(
        draft,
        LessonDraftInput {
            lesson_id: LessonId::from("lesson-1"),
            node_id: CurriculumNodeId::from("node-1"),
            title: LocalizedStringSet::plain("Local government"),
            summary: LocalizedStringSet::plain("Local civics overview"),
            level: EducationLevel::ElementaryUpper,
            strand: Some("civics".to_string()),
            objective: LocalizedText::plain("Explain the role of local government"),
            taxonomy: ObjectiveTaxonomy::Understand,
            blocks: vec![LessonBlock::Paragraph {
                text: LocalizedText::plain("Cities and counties provide local services."),
            }],
            visual_ids: vec![LearningVisualId::from("visual-local-map")],
            glossary_terms: vec![GlossaryTermId::from("term-local-government")],
            accessibility_summary: LocalizedText::plain("Text lesson with map fallback"),
            reading_level: None,
            estimated_minutes: Some(15),
        },
    )
    .expect("add lesson");
    let draft = add_learning_visual_to_curriculum_pack_draft(
        draft,
        LearningVisualSpec {
            id: LearningVisualId::from("visual-local-map"),
            node_id: CurriculumNodeId::from("node-1"),
            kind: crate::LearningVisualKind::ArgumentMap,
            title: LocalizedText::plain("Local services map"),
            description: LocalizedText::plain("Map of local government services"),
            renderer: VisualRenderer {
                engine: VisualRendererEngine::Tench2d,
                spec_version: 1,
                scene_ref: "custom/local-services".to_string(),
            },
            playback: VisualPlayback {
                animated: false,
                autoplay: false,
                duration_ms: None,
                timeline_position: 0.0,
                reduced_motion_fallback: true,
            },
            interactions: vec![VisualInteraction::SelectNode],
            accessibility: VisualAccessibility {
                alt_text: "Text map of local government services".to_string(),
                transcript: None,
                table_fallback_ref: Some("tables/local-services".to_string()),
                keyboard_model: vec!["select_service_node".to_string()],
            },
            locale: None,
        },
    )
    .expect("add visual");
    let draft = add_glossary_term_to_curriculum_pack_draft(
        draft,
        GlossaryTerm {
            id: GlossaryTermId::from("term-local-government"),
            node_id: CurriculumNodeId::from("node-1"),
            term: LocalizedStringSet::plain("Local government"),
            definition: LocalizedStringSet::plain("Government for a city, county, or town."),
            aliases: Vec::new(),
            related_term_ids: Vec::new(),
            subject_tags: vec!["social".to_string()],
        },
    )
    .expect("add glossary");
    let draft = add_practice_item_to_curriculum_pack_draft(
        draft,
        PracticeItem {
            id: PracticeItemId::from("practice-local-government"),
            node_id: CurriculumNodeId::from("node-1"),
            prompt: LocalizedText::plain("Who provides local services?"),
            kind: PracticeKind::ExactAnswer,
            answer_key: AnswerKey::Exact {
                value: "local government".to_string(),
                case_sensitive: false,
            },
            explanation: LocalizedText::plain("Local governments provide local services."),
            skills: Vec::new(),
            difficulty: Some(0.4),
        },
    )
    .expect("add practice item");
    let draft = add_assessment_to_curriculum_pack_draft(
        draft,
        AssessmentDraft {
            id: AssessmentId::from("assessment-local-government"),
            title: LocalizedStringSet::plain("Local government check"),
            kind: AssessmentKind::Quiz,
            node_ids: vec![CurriculumNodeId::from("node-1")],
            item_ids: vec![PracticeItemId::from("practice-local-government")],
            time_limit_seconds: None,
            coverage_constraints: vec![AssessmentCoverageConstraint {
                label: "local-services".to_string(),
                minimum_score: 0.8,
            }],
            report_template: LocalizedText::plain("Report local government mastery."),
        },
    )
    .expect("add assessment");

    let preview = preview_curriculum_pack_draft(&draft, None);
    assert_eq!(preview.lesson_count, 1);
    assert_eq!(preview.practice_count, 1);
    assert_eq!(preview.visual_count, 1);
    assert_eq!(preview.assessment_count, 1);
    assert_eq!(preview.glossary_count, 1);
    assert!(preview.validation.is_valid());

    let published =
        publish_curriculum_pack_draft(draft, "2026-05-04T01:00:00Z".to_string()).expect("publish");

    assert_eq!(published.lessons.len(), 1);
    assert!(matches!(
        published.curriculum.domain,
        SubjectDomain::Custom { .. }
    ));
    assert_eq!(published.manifest.visuals.len(), 1);
    assert_eq!(published.manifest.practice_items.len(), 1);
    assert_eq!(published.manifest.assessments.len(), 1);
    assert_eq!(published.manifest.glossary_terms.len(), 1);
}

#[test]
fn localization_report_marks_missing_required_content_locale() {
    let en = ContentLocale::parse("en-US").unwrap();
    let ko = ContentLocale::parse("ko-KR").unwrap();
    let draft = new_custom_curriculum_pack_draft(CustomCurriculumDraftRequest {
        draft_id: CurriculumPackDraftId::from("draft"),
        pack_id: ContentPackId::from("pack"),
        curriculum_id: CurriculumId::from("custom"),
        domain_id: crate::CustomDomainId::from("custom"),
        domain_label: "Custom".to_string(),
        title: LocalizedStringSet::plain("Custom curriculum"),
        description: LocalizedStringSet::plain("Custom description"),
        owner: PackOwner {
            name: "school".to_string(),
            organization: None,
            contact: None,
            responsibility_statement: "school-owned custom curriculum".to_string(),
        },
        license: LicenseInfo {
            name: "internal".to_string(),
            url: None,
            permits_redistribution: false,
        },
        update_policy: PackUpdatePolicy {
            channel: PackUpdateChannel::UserManaged,
            cadence: None,
            deprecation_policy: None,
            preserve_progress_on_stable_node_ids: true,
        },
        default_locale: en,
        required_locales: vec![ko.clone()],
        level_range: LevelRange {
            start: EducationLevel::Kindergarten,
            end: EducationLevel::HighSchool,
        },
        version: "1.0.0".to_string(),
        integrity_hash: "hash".to_string(),
        created_at: "2026-05-04T00:00:00Z".to_string(),
    });

    let report = curriculum_pack_localization_report(&draft);
    let validation = draft.validate_for_distribution();

    assert!(!report.release_ready);
    assert!(report
        .missing
        .iter()
        .any(|gap| gap.field == "title" && gap.locale == ko));
    assert!(validation
        .issues
        .iter()
        .any(|issue| issue.code == "localization_missing"));
}
