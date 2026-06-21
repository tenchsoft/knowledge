use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{
    build_learning_visual_draw_plan, builtin_assessments_for_all, builtin_content_coverage_report,
    builtin_curricula, builtin_practice_items_for_all, builtin_visual_specs,
    curriculum_pack_localization_report, glossary_terms_from_all_curricula, CurriculumPackDraft,
    EducationLevel, InstalledContentPackRegistry, LearnerProfile, LearnerProgress, SubjectDomain,
    VisualRuntimeState,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StudyReleaseReadinessInput {
    #[serde(default)]
    pub custom_drafts: Vec<CurriculumPackDraft>,
    #[serde(default)]
    pub installed_registry: InstalledContentPackRegistry,
    #[serde(default)]
    pub learner_profiles: Vec<LearnerProfile>,
    #[serde(default)]
    pub learner_progress: Vec<LearnerProgress>,
    #[serde(default)]
    pub i18n: Option<StudyI18nReadiness>,
    #[serde(default)]
    pub evidence: StudyReleaseEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyI18nReadiness {
    #[serde(default)]
    pub missing_keys: Vec<String>,
    #[serde(default)]
    pub fallback_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyReleaseEvidence {
    #[serde(default)]
    pub profile_round_trip_verified: bool,
    #[serde(default)]
    pub lesson_practice_review_exam_verified: bool,
    #[serde(default)]
    pub import_export_round_trip_verified: bool,
    #[serde(default)]
    pub ui_screenshots_verified: bool,
    #[serde(default)]
    pub keyboard_navigation_verified: bool,
    #[serde(default)]
    pub performance_targets_verified: bool,
    #[serde(default)]
    pub ai_disabled_smoke_tested: bool,
    #[serde(default)]
    pub engine_http_not_exposed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyReleaseReadinessReport {
    pub product_id: String,
    pub release_ready: bool,
    #[serde(default)]
    pub checks: Vec<StudyReleaseReadinessCheck>,
    #[serde(default)]
    pub blockers: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StudyReleaseReadinessCheck {
    pub category: String,
    pub code: String,
    pub status: StudyReleaseCheckStatus,
    pub message: String,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyReleaseCheckStatus {
    Pass,
    Warning,
    Fail,
    NotVerified,
}

pub fn study_release_readiness_report(
    input: &StudyReleaseReadinessInput,
) -> StudyReleaseReadinessReport {
    let mut checks = Vec::new();
    check_builtin_curriculum(&mut checks);
    check_custom_authoring(&mut checks, &input.custom_drafts);
    check_learning_engine(
        &mut checks,
        &input.learner_profiles,
        &input.learner_progress,
    );
    check_visual_learning(&mut checks);
    check_i18n(&mut checks, input.i18n.as_ref(), &input.custom_drafts);
    check_installed_registry(&mut checks, &input.installed_registry);
    check_study_evidence(&mut checks, &input.evidence);

    let blockers = checks
        .iter()
        .filter(|check| {
            matches!(
                check.status,
                StudyReleaseCheckStatus::Fail | StudyReleaseCheckStatus::NotVerified
            )
        })
        .map(|check| format!("{}:{}", check.category, check.code))
        .collect::<Vec<_>>();

    StudyReleaseReadinessReport {
        product_id: "tench-study".to_string(),
        release_ready: blockers.is_empty(),
        checks,
        blockers,
    }
}

fn check_builtin_curriculum(checks: &mut Vec<StudyReleaseReadinessCheck>) {
    let curricula = builtin_curricula();
    let domains = curricula
        .curricula
        .iter()
        .map(|curriculum| curriculum.domain.clone())
        .collect::<HashSet<_>>();
    let expected = HashSet::from([
        SubjectDomain::Mathematics,
        SubjectDomain::Science,
        SubjectDomain::Language,
        SubjectDomain::Programming,
    ]);
    push_check(
        checks,
        "built_in_curriculum",
        "product_scope_domains",
        domains == expected,
        "built-in scope exposes only Mathematics, Science, Languages, Programming",
        "built-in curriculum scope includes unexpected or missing subjects",
        domains
            .iter()
            .map(|domain| format!("{domain:?}").to_ascii_lowercase())
            .collect(),
    );

    let coverage = builtin_content_coverage_report();
    push_check(
        checks,
        "built_in_curriculum",
        "coverage_report",
        coverage.release_ready,
        "built-in content coverage report is release-ready",
        "built-in content coverage report has issues",
        coverage.issues.clone(),
    );

    let all_levels_present = coverage.subject_reports.iter().all(|subject| {
        EducationLevel::all().iter().all(|level| {
            subject
                .level_reports
                .iter()
                .any(|report| report.level == *level)
        })
    });
    push_check(
        checks,
        "built_in_curriculum",
        "kindergarten_to_graduate_levels",
        all_levels_present,
        "each built-in subject covers Kindergarten through Graduate",
        "one or more built-in subjects is missing an education level",
        Vec::new(),
    );
}

fn check_custom_authoring(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    drafts: &[CurriculumPackDraft],
) {
    if drafts.is_empty() {
        push_status(
            checks,
            "custom_curriculum_authoring",
            "custom_pack_smoke",
            StudyReleaseCheckStatus::NotVerified,
            "no custom curriculum draft was provided for authoring validation",
            Vec::new(),
        );
        return;
    }

    let mut issues = Vec::new();
    let mut localization_evidence = Vec::new();
    for draft in drafts {
        let validation = draft.validate_for_distribution();
        for issue in validation.issues {
            if issue.severity == crate::DraftValidationSeverity::Error {
                issues.push(format!("{}: {}", draft.id.as_str(), issue.message));
            }
        }
        let localization = curriculum_pack_localization_report(draft);
        localization_evidence.push(format!(
            "{} checked_fields={} missing={}",
            draft.id.as_str(),
            localization.checked_field_count,
            localization.missing.len()
        ));
    }
    push_check(
        checks,
        "custom_curriculum_authoring",
        "draft_validation",
        issues.is_empty(),
        "custom curriculum drafts produce valid pack records",
        "custom curriculum draft validation found release blockers",
        issues,
    );
    push_status(
        checks,
        "custom_curriculum_authoring",
        "localization_report",
        StudyReleaseCheckStatus::Pass,
        "custom curriculum localization report was generated".to_string(),
        localization_evidence,
    );
}

fn check_learning_engine(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    profiles: &[LearnerProfile],
    progress: &[LearnerProgress],
) {
    push_status(
        checks,
        "learning_engine",
        "profile_progress_data",
        if profiles.is_empty() || progress.is_empty() {
            StudyReleaseCheckStatus::NotVerified
        } else {
            StudyReleaseCheckStatus::Pass
        },
        "profile and learner progress data were provided for release validation".to_string(),
        vec![
            format!("profiles={}", profiles.len()),
            format!("progress_records={}", progress.len()),
        ],
    );
}

fn check_visual_learning(checks: &mut Vec<StudyReleaseReadinessCheck>) {
    let mut errors = Vec::new();
    let curricula = builtin_curricula();
    for curriculum in &curricula.curricula {
        for visual in builtin_visual_specs(curriculum) {
            let state = VisualRuntimeState {
                visual_id: visual.id.clone(),
                selected_id: None,
                active_layers: Vec::new(),
                parameter_values: Vec::new(),
                playback: visual.playback.clone(),
            };
            match build_learning_visual_draw_plan(&visual, &state, false) {
                Ok(plan) => {
                    if plan.table_fallback.is_empty() {
                        errors.push(format!("{} has empty table fallback", visual.id.as_str()));
                    }
                }
                Err(message) => errors.push(format!("{}: {message}", visual.id.as_str())),
            }
        }
    }
    push_check(
        checks,
        "visual_learning",
        "builtin_visual_draw_plans",
        errors.is_empty(),
        "built-in subject visuals render with accessibility/table fallback",
        "built-in subject visual draw plan validation failed",
        errors,
    );
    push_status(
        checks,
        "built_in_curriculum",
        "lesson_visual_practice_assessment_glossary_counts",
        StudyReleaseCheckStatus::Pass,
        "built-in generated content includes practice, assessments, and glossary".to_string(),
        vec![
            format!("practice_items={}", builtin_practice_items_for_all().len()),
            format!("assessments={}", builtin_assessments_for_all().len()),
            format!(
                "glossary_terms={}",
                glossary_terms_from_all_curricula(&builtin_curricula().curricula).len()
            ),
        ],
    );
}

fn check_i18n(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    i18n: Option<&StudyI18nReadiness>,
    drafts: &[CurriculumPackDraft],
) {
    let Some(i18n) = i18n else {
        push_status(
            checks,
            "all_language_support",
            "ui_i18n_coverage",
            StudyReleaseCheckStatus::NotVerified,
            "no UI i18n coverage report was provided",
            Vec::new(),
        );
        return;
    };
    let mut blockers = Vec::new();
    blockers.extend(i18n.missing_keys.iter().map(|key| format!("missing:{key}")));
    blockers.extend(
        i18n.fallback_keys
            .iter()
            .map(|key| format!("fallback:{key}")),
    );
    blockers.extend(drafts.iter().flat_map(|draft| {
        curriculum_pack_localization_report(draft)
            .missing
            .into_iter()
            .map(|gap| {
                format!(
                    "content:{}:{}:{}:{}",
                    gap.item_kind,
                    gap.item_id,
                    gap.field,
                    gap.locale.bcp47()
                )
            })
    }));
    push_check(
        checks,
        "all_language_support",
        "ui_and_content_i18n_coverage",
        blockers.is_empty(),
        "UI and provided content i18n coverage is complete",
        "UI or content i18n coverage still has gaps",
        blockers,
    );
}

fn check_installed_registry(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    registry: &InstalledContentPackRegistry,
) {
    let mut errors = Vec::new();
    for entry in &registry.entries {
        if entry.active_archive().is_none() {
            errors.push(format!(
                "pack {} active version {} is missing",
                entry.pack_id.as_str(),
                entry.active_version
            ));
        }
    }
    push_check(
        checks,
        "import_export",
        "installed_pack_registry",
        errors.is_empty(),
        "installed content pack registry active versions resolve",
        "installed content pack registry has broken active versions",
        errors,
    );
}

fn check_study_evidence(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    evidence: &StudyReleaseEvidence,
) {
    evidence_check(
        checks,
        "learning_engine",
        "profile_round_trip",
        evidence.profile_round_trip_verified,
        "profile create/open/save/reopen round trip was verified",
    );
    evidence_check(
        checks,
        "learning_engine",
        "lesson_practice_review_exam",
        evidence.lesson_practice_review_exam_verified,
        "lesson, practice, review, and exam flows were verified without AI",
    );
    evidence_check(
        checks,
        "import_export",
        "format_round_trip",
        evidence.import_export_round_trip_verified,
        "Anki/CSV/TSV/Markdown/content-pack/progress import-export round trips were verified",
    );
    evidence_check(
        checks,
        "ui_stack",
        "desktop_tablet_mobile_screenshots",
        evidence.ui_screenshots_verified,
        "desktop/tablet/mobile UI screenshots were checked for overlap",
    );
    evidence_check(
        checks,
        "ui_stack",
        "keyboard_navigation",
        evidence.keyboard_navigation_verified,
        "keyboard-only navigation was verified",
    );
    evidence_check(
        checks,
        "performance",
        "performance_targets",
        evidence.performance_targets_verified,
        "large curriculum/review/code grading/diagram performance targets were verified",
    );
    evidence_check(
        checks,
        "ai_separation",
        "ai_disabled_smoke",
        evidence.ai_disabled_smoke_tested,
        "non-AI Study workflows were verified with AI disabled",
    );
    evidence_check(
        checks,
        "ai_separation",
        "engine_http_not_exposed",
        evidence.engine_http_not_exposed,
        "product does not expose an Engine HTTP endpoint",
    );
}

fn evidence_check(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    category: &str,
    code: &str,
    verified: bool,
    message: &str,
) {
    push_status(
        checks,
        category,
        code,
        if verified {
            StudyReleaseCheckStatus::Pass
        } else {
            StudyReleaseCheckStatus::NotVerified
        },
        message.to_string(),
        Vec::new(),
    );
}

fn push_check(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    category: &str,
    code: &str,
    pass: bool,
    pass_message: &str,
    fail_message: &str,
    evidence: Vec<String>,
) {
    push_status(
        checks,
        category,
        code,
        if pass {
            StudyReleaseCheckStatus::Pass
        } else {
            StudyReleaseCheckStatus::Fail
        },
        if pass { pass_message } else { fail_message }.to_string(),
        evidence,
    );
}

fn push_status(
    checks: &mut Vec<StudyReleaseReadinessCheck>,
    category: &str,
    code: &str,
    status: StudyReleaseCheckStatus,
    message: impl Into<String>,
    evidence: Vec<String>,
) {
    checks.push(StudyReleaseReadinessCheck {
        category: category.to_string(),
        code: code.to_string(),
        status,
        message: message.into(),
        evidence,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readiness_report_passes_builtin_checks_but_requires_external_evidence_product_e2e() {
        let report = study_release_readiness_report(&StudyReleaseReadinessInput {
            custom_drafts: Vec::new(),
            installed_registry: InstalledContentPackRegistry::default(),
            learner_profiles: Vec::new(),
            learner_progress: Vec::new(),
            i18n: Some(StudyI18nReadiness {
                missing_keys: Vec::new(),
                fallback_keys: Vec::new(),
            }),
            evidence: StudyReleaseEvidence::default(),
        });

        assert!(!report.release_ready);
        assert!(report.checks.iter().any(|check| {
            check.category == "built_in_curriculum"
                && check.code == "coverage_report"
                && check.status == StudyReleaseCheckStatus::Pass
        }));
        assert!(report
            .blockers
            .iter()
            .any(|blocker| blocker.contains("custom_pack_smoke")));
        assert!(report
            .blockers
            .iter()
            .any(|blocker| blocker.contains("profile_progress_data")));
    }

    #[test]
    fn readiness_report_is_release_ready_with_complete_evidence_release_validation() {
        let locale = crate::ContentLocale::parse("en-US").expect("locale");
        let draft = custom_curriculum_draft_fixture(locale.clone());
        let profile = crate::LearnerProfile {
            id: crate::LearnerId::from("learner"),
            display_name: "Learner".to_string(),
            primary_locale: locale,
            target_locales: Vec::new(),
            accommodations: vec![crate::LearningAccommodation::KeyboardOnly],
        };
        let progress = crate::LearnerProgress {
            learner_id: profile.id.clone(),
            node_id: crate::CurriculumNodeId::from("node-1"),
            mastery: crate::MasteryState {
                score: 1.0,
                attempts: 1,
                correct: 1,
            },
            attempts: Vec::new(),
            review_state: crate::SpacedRepetitionState::default(),
        };

        let report = study_release_readiness_report(&StudyReleaseReadinessInput {
            custom_drafts: vec![draft],
            installed_registry: InstalledContentPackRegistry::default(),
            learner_profiles: vec![profile],
            learner_progress: vec![progress],
            i18n: Some(StudyI18nReadiness {
                missing_keys: Vec::new(),
                fallback_keys: Vec::new(),
            }),
            evidence: StudyReleaseEvidence {
                profile_round_trip_verified: true,
                lesson_practice_review_exam_verified: true,
                import_export_round_trip_verified: true,
                ui_screenshots_verified: true,
                keyboard_navigation_verified: true,
                performance_targets_verified: true,
                ai_disabled_smoke_tested: true,
                engine_http_not_exposed: true,
            },
        });

        assert!(
            report.release_ready,
            "unexpected release blockers: {:?}",
            report.blockers
        );
        assert!(report.blockers.is_empty());
    }

    fn custom_curriculum_draft_fixture(locale: crate::ContentLocale) -> crate::CurriculumPackDraft {
        let request = crate::CustomCurriculumDraftRequest {
            draft_id: crate::CurriculumPackDraftId::from("draft"),
            pack_id: crate::ContentPackId::from("pack"),
            curriculum_id: crate::CurriculumId::from("school-social"),
            domain_id: crate::CustomDomainId::from("social"),
            domain_label: "Social Studies".to_string(),
            title: crate::LocalizedStringSet::plain("Social Studies"),
            description: crate::LocalizedStringSet::plain("School-managed curriculum"),
            owner: crate::PackOwner {
                name: "school".to_string(),
                organization: None,
                contact: None,
                responsibility_statement: "school-owned custom curriculum".to_string(),
            },
            license: crate::LicenseInfo {
                name: "internal".to_string(),
                url: None,
                permits_redistribution: false,
            },
            update_policy: crate::PackUpdatePolicy {
                channel: crate::PackUpdateChannel::UserManaged,
                cadence: None,
                deprecation_policy: None,
                preserve_progress_on_stable_node_ids: true,
            },
            default_locale: locale,
            required_locales: Vec::new(),
            level_range: crate::LevelRange {
                start: crate::EducationLevel::Kindergarten,
                end: crate::EducationLevel::HighSchool,
            },
            version: "1.0.0".to_string(),
            integrity_hash: "hash".to_string(),
            created_at: "2026-05-04T00:00:00Z".to_string(),
        };
        let draft = crate::new_custom_curriculum_pack_draft(request);
        let draft = crate::add_lesson_to_curriculum_pack_draft(
            draft,
            crate::LessonDraftInput {
                lesson_id: crate::LessonId::from("lesson-1"),
                node_id: crate::CurriculumNodeId::from("node-1"),
                title: crate::LocalizedStringSet::plain("Local government"),
                summary: crate::LocalizedStringSet::plain("Local civics overview"),
                level: crate::EducationLevel::ElementaryUpper,
                strand: Some("civics".to_string()),
                objective: crate::LocalizedText::plain("Explain the role of local government"),
                taxonomy: crate::ObjectiveTaxonomy::Understand,
                blocks: vec![crate::LessonBlock::Paragraph {
                    text: crate::LocalizedText::plain(
                        "Cities and counties provide local services.",
                    ),
                }],
                visual_ids: vec![crate::LearningVisualId::from("visual-local-map")],
                glossary_terms: vec![crate::GlossaryTermId::from("term-local-government")],
                accessibility_summary: crate::LocalizedText::plain("Text lesson with map fallback"),
                reading_level: None,
                estimated_minutes: Some(15),
            },
        )
        .expect("add lesson");
        let draft = crate::add_learning_visual_to_curriculum_pack_draft(
            draft,
            crate::LearningVisualSpec {
                id: crate::LearningVisualId::from("visual-local-map"),
                node_id: crate::CurriculumNodeId::from("node-1"),
                kind: crate::LearningVisualKind::ArgumentMap,
                title: crate::LocalizedText::plain("Local services map"),
                description: crate::LocalizedText::plain("Map of local government services"),
                renderer: crate::VisualRenderer {
                    engine: crate::VisualRendererEngine::Tench2d,
                    spec_version: 1,
                    scene_ref: "custom/local-services".to_string(),
                },
                playback: crate::VisualPlayback {
                    animated: false,
                    autoplay: false,
                    duration_ms: None,
                    timeline_position: 0.0,
                    reduced_motion_fallback: true,
                },
                interactions: vec![crate::VisualInteraction::SelectNode],
                accessibility: crate::VisualAccessibility {
                    alt_text: "Text map of local government services".to_string(),
                    transcript: None,
                    table_fallback_ref: Some("tables/local-services".to_string()),
                    keyboard_model: vec!["select_service_node".to_string()],
                },
                locale: None,
            },
        )
        .expect("add visual");
        let draft = crate::add_glossary_term_to_curriculum_pack_draft(
            draft,
            crate::GlossaryTerm {
                id: crate::GlossaryTermId::from("term-local-government"),
                node_id: crate::CurriculumNodeId::from("node-1"),
                term: crate::LocalizedStringSet::plain("Local government"),
                definition: crate::LocalizedStringSet::plain(
                    "Government for a city, county, or town.",
                ),
                aliases: Vec::new(),
                related_term_ids: Vec::new(),
                subject_tags: vec!["social".to_string()],
            },
        )
        .expect("add glossary");
        let draft = crate::add_practice_item_to_curriculum_pack_draft(
            draft,
            crate::PracticeItem {
                id: crate::PracticeItemId::from("practice-local-government"),
                node_id: crate::CurriculumNodeId::from("node-1"),
                prompt: crate::LocalizedText::plain("Who provides local services?"),
                kind: crate::PracticeKind::ExactAnswer,
                answer_key: crate::AnswerKey::Exact {
                    value: "local government".to_string(),
                    case_sensitive: false,
                },
                explanation: crate::LocalizedText::plain(
                    "Local governments provide local services.",
                ),
                skills: Vec::new(),
                difficulty: Some(0.4),
            },
        )
        .expect("add practice item");
        crate::add_assessment_to_curriculum_pack_draft(
            draft,
            crate::AssessmentDraft {
                id: crate::AssessmentId::from("assessment-local-government"),
                title: crate::LocalizedStringSet::plain("Local government check"),
                kind: crate::AssessmentKind::Quiz,
                node_ids: vec![crate::CurriculumNodeId::from("node-1")],
                item_ids: vec![crate::PracticeItemId::from("practice-local-government")],
                time_limit_seconds: None,
                coverage_constraints: vec![crate::AssessmentCoverageConstraint {
                    label: "local-services".to_string(),
                    minimum_score: 0.8,
                }],
                report_template: crate::LocalizedText::plain("Report local government mastery."),
            },
        )
        .expect("add assessment")
    }
}
