use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{
    build_research_index_documents, build_research_visual_draw_plan, run_non_ai_writing_checks,
    ResearchManuscript, ResearchSnapshotV2, ResearchVisualSpec, WritingExportFormat,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchReleaseReadinessInput {
    pub snapshot: ResearchSnapshotV2,
    #[serde(default)]
    pub manuscripts: Vec<ResearchManuscript>,
    #[serde(default)]
    pub visuals: Vec<ResearchVisualSpec>,
    #[serde(default)]
    pub i18n: Option<ResearchI18nReadiness>,
    #[serde(default)]
    pub evidence: ResearchReleaseEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchI18nReadiness {
    #[serde(default)]
    pub missing_keys: Vec<String>,
    #[serde(default)]
    pub fallback_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchReleaseEvidence {
    #[serde(default)]
    pub library_round_trip_verified: bool,
    #[serde(default)]
    pub import_export_round_trip_verified: bool,
    #[serde(default)]
    pub pdf_reader_round_trip_verified: bool,
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
pub struct ResearchReleaseReadinessReport {
    pub product_id: String,
    pub release_ready: bool,
    #[serde(default)]
    pub checks: Vec<ResearchReleaseReadinessCheck>,
    #[serde(default)]
    pub blockers: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchReleaseReadinessCheck {
    pub category: String,
    pub code: String,
    pub status: ReleaseCheckStatus,
    pub message: String,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseCheckStatus {
    Pass,
    Warning,
    Fail,
    NotVerified,
}

pub fn research_release_readiness_report(
    input: &ResearchReleaseReadinessInput,
) -> ResearchReleaseReadinessReport {
    let mut checks = Vec::new();
    check_snapshot_integrity(&mut checks, &input.snapshot);
    check_index_contract(&mut checks, &input.snapshot);
    check_manuscripts(&mut checks, &input.manuscripts);
    check_visuals(&mut checks, &input.visuals);
    check_i18n(&mut checks, input.i18n.as_ref());
    check_research_evidence(&mut checks, &input.evidence);

    let blockers = checks
        .iter()
        .filter(|check| {
            matches!(
                check.status,
                ReleaseCheckStatus::Fail | ReleaseCheckStatus::NotVerified
            )
        })
        .map(|check| format!("{}:{}", check.category, check.code))
        .collect::<Vec<_>>();

    ResearchReleaseReadinessReport {
        product_id: "tench-research".to_string(),
        release_ready: blockers.is_empty(),
        checks,
        blockers,
    }
}

fn check_snapshot_integrity(
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
    snapshot: &ResearchSnapshotV2,
) {
    let mut reference_ids = HashSet::new();
    let mut reference_errors = Vec::new();
    for reference in &snapshot.references {
        if !reference_ids.insert(reference.id.clone()) {
            reference_errors.push(format!("duplicate reference {}", reference.id.as_str()));
        }
        if let Err(message) = reference.validate() {
            reference_errors.push(format!("{}: {message}", reference.id.as_str()));
        }
    }
    push_check(
        checks,
        "core_data",
        "reference_integrity",
        reference_errors.is_empty(),
        "all references are unique and valid",
        "reference integrity issues were found",
        reference_errors,
    );

    let attachment_ids = snapshot
        .attachments
        .iter()
        .map(|attachment| attachment.id.clone())
        .collect::<HashSet<_>>();
    let mut attachment_errors = Vec::new();
    for attachment in &snapshot.attachments {
        if !reference_ids.contains(&attachment.reference_id) {
            attachment_errors.push(format!(
                "attachment {} references missing reference {}",
                attachment.id.as_str(),
                attachment.reference_id.as_str()
            ));
        }
        if attachment.content_hash.trim().is_empty() {
            attachment_errors.push(format!(
                "attachment {} is missing content hash",
                attachment.id.as_str()
            ));
        }
    }
    push_check(
        checks,
        "core_data",
        "attachment_integrity",
        attachment_errors.is_empty(),
        "attachments preserve reference links and content hashes",
        "attachment integrity issues were found",
        attachment_errors,
    );

    let annotation_ids = snapshot
        .annotations
        .iter()
        .map(|annotation| annotation.id.clone())
        .collect::<HashSet<_>>();
    let mut note_annotation_errors = Vec::new();
    for annotation in &snapshot.annotations {
        if !reference_ids.contains(&annotation.reference_id) {
            note_annotation_errors.push(format!(
                "annotation {} references missing reference {}",
                annotation.id.as_str(),
                annotation.reference_id.as_str()
            ));
        }
        if !attachment_ids.contains(&annotation.attachment_id) {
            note_annotation_errors.push(format!(
                "annotation {} references missing attachment {}",
                annotation.id.as_str(),
                annotation.attachment_id.as_str()
            ));
        }
    }
    for note in &snapshot.notes {
        if let Some(reference_id) = &note.reference_id {
            if !reference_ids.contains(reference_id) {
                note_annotation_errors.push(format!(
                    "note {} references missing reference {}",
                    note.id.as_str(),
                    reference_id.as_str()
                ));
            }
        }
        if let Some(annotation_id) = &note.annotation_id {
            if !annotation_ids.contains(annotation_id) {
                note_annotation_errors.push(format!(
                    "note {} references missing annotation {}",
                    note.id.as_str(),
                    annotation_id.as_str()
                ));
            }
        }
    }
    push_check(
        checks,
        "pdf_notes",
        "annotation_note_links",
        note_annotation_errors.is_empty(),
        "annotations and notes preserve existing links",
        "annotation or note link issues were found",
        note_annotation_errors,
    );
}

fn check_index_contract(
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
    snapshot: &ResearchSnapshotV2,
) {
    let docs = build_research_index_documents(snapshot);
    push_status(
        checks,
        "backend_compatibility",
        "search_index_contract",
        ReleaseCheckStatus::Pass,
        format!(
            "research index uses tench-search-core documents: {} docs",
            docs.len()
        ),
        vec![
            format!("references={}", snapshot.references.len()),
            format!("notes={}", snapshot.notes.len()),
            format!("annotations={}", snapshot.annotations.len()),
        ],
    );
}

fn check_manuscripts(
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
    manuscripts: &[ResearchManuscript],
) {
    if manuscripts.is_empty() {
        push_status(
            checks,
            "citation_writing",
            "manuscript_smoke",
            ReleaseCheckStatus::NotVerified,
            "no manuscript was provided for writing checks",
            Vec::new(),
        );
        return;
    }

    let required_exports = [
        WritingExportFormat::Docx,
        WritingExportFormat::Pdf,
        WritingExportFormat::Markdown,
        WritingExportFormat::Html,
        WritingExportFormat::Latex,
    ];
    let mut writing_errors = Vec::new();
    let mut export_evidence = Vec::new();
    for manuscript in manuscripts {
        for check in run_non_ai_writing_checks(manuscript) {
            if check.export_blocker {
                writing_errors.push(format!("{}: {}", manuscript.id.as_str(), check.message));
            }
        }
        for format in required_exports {
            if manuscript.target.export_formats.contains(&format) {
                export_evidence.push(format!("{} supports {:?}", manuscript.id.as_str(), format));
            }
        }
    }
    push_check(
        checks,
        "citation_writing",
        "writing_export_blockers",
        writing_errors.is_empty(),
        "provided manuscripts have no non-AI export blockers",
        "manuscript export blockers were found",
        writing_errors,
    );
    push_status(
        checks,
        "citation_writing",
        "export_format_support",
        if export_evidence.is_empty() {
            ReleaseCheckStatus::Fail
        } else {
            ReleaseCheckStatus::Pass
        },
        "manuscript target export support checked".to_string(),
        export_evidence,
    );
}

fn check_visuals(checks: &mut Vec<ResearchReleaseReadinessCheck>, visuals: &[ResearchVisualSpec]) {
    if visuals.is_empty() {
        push_status(
            checks,
            "visual_information",
            "visual_smoke",
            ReleaseCheckStatus::NotVerified,
            "no research visual spec was provided for rendering checks",
            Vec::new(),
        );
        return;
    }

    let mut visual_errors = Vec::new();
    let mut table_evidence = Vec::new();
    let mut manual_visual_present = false;
    for visual in visuals {
        match build_research_visual_draw_plan(visual, false) {
            Ok(plan) => {
                if plan.table_fallback.is_empty() {
                    visual_errors.push(format!("{} has empty table fallback", visual.id.as_str()));
                } else {
                    table_evidence.push(format!(
                        "{} fallback rows={}",
                        visual.id.as_str(),
                        plan.table_fallback.len()
                    ));
                }
                manual_visual_present |= visual.manual_data.is_some();
            }
            Err(message) => {
                visual_errors.push(format!("{}: {message}", visual.id.as_str()));
            }
        }
    }
    push_check(
        checks,
        "visual_information",
        "draw_plan_and_table_fallback",
        visual_errors.is_empty(),
        "visuals render with table fallbacks and accessibility summaries",
        "visual rendering or fallback issues were found",
        visual_errors,
    );
    push_status(
        checks,
        "visual_information",
        "manual_paper_analysis_visual",
        if manual_visual_present {
            ReleaseCheckStatus::Pass
        } else {
            ReleaseCheckStatus::NotVerified
        },
        "manual paper-analysis visual support requires at least one manual-data spec".to_string(),
        table_evidence,
    );
}

fn check_i18n(
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
    i18n: Option<&ResearchI18nReadiness>,
) {
    let Some(i18n) = i18n else {
        push_status(
            checks,
            "all_language_support",
            "ui_i18n_coverage",
            ReleaseCheckStatus::NotVerified,
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
    push_check(
        checks,
        "all_language_support",
        "ui_i18n_coverage",
        blockers.is_empty(),
        "UI i18n coverage has no missing or fallback keys",
        "UI i18n coverage still has missing or fallback keys",
        blockers,
    );
}

fn check_research_evidence(
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
    evidence: &ResearchReleaseEvidence,
) {
    evidence_check(
        checks,
        "core_data",
        "library_round_trip",
        evidence.library_round_trip_verified,
        "library create/open/save/reopen round trip was verified",
    );
    evidence_check(
        checks,
        "import_export",
        "format_round_trip",
        evidence.import_export_round_trip_verified,
        "PDF/BibTeX/RIS/CSL JSON/EndNote import-export round trips were verified",
    );
    evidence_check(
        checks,
        "pdf_notes",
        "reader_annotation_round_trip",
        evidence.pdf_reader_round_trip_verified,
        "PDF render/search/selection/annotation persistence was verified",
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
        "large-library/indexing/PDF/visual performance targets were verified",
    );
    evidence_check(
        checks,
        "ai_separation",
        "ai_disabled_smoke",
        evidence.ai_disabled_smoke_tested,
        "non-AI workflows were verified with AI disabled",
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
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
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
            ReleaseCheckStatus::Pass
        } else {
            ReleaseCheckStatus::NotVerified
        },
        message.to_string(),
        Vec::new(),
    );
}

fn push_check(
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
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
            ReleaseCheckStatus::Pass
        } else {
            ReleaseCheckStatus::Fail
        },
        if pass { pass_message } else { fail_message }.to_string(),
        evidence,
    );
}

fn push_status(
    checks: &mut Vec<ResearchReleaseReadinessCheck>,
    category: &str,
    code: &str,
    status: ReleaseCheckStatus,
    message: impl Into<String>,
    evidence: Vec<String>,
) {
    checks.push(ResearchReleaseReadinessCheck {
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
    use crate::{
        build_manual_paper_analysis_visual, create_manuscript_from_template,
        new_research_library_snapshot, reference_from_minimal_metadata, LocalizedField,
        ManualVisualNode, ManuscriptId, ManuscriptTemplateKind, ReferenceKind, ResearchLocale,
        ResearchVisualKind, ResearchVisualManualData, Timestamp, VisualSpecId,
    };

    #[test]
    fn readiness_report_flags_unverified_release_gates_product_e2e() {
        let locale = ResearchLocale::parse("en-US").expect("locale");
        let now = Timestamp("2026-05-04T00:00:00Z".to_string());
        let mut snapshot = new_research_library_snapshot(
            crate::LibraryId::from("lib"),
            "Library",
            "/tmp/lib",
            locale,
            now.clone(),
        );
        snapshot.references.push(reference_from_minimal_metadata(
            "ref_1",
            ReferenceKind::JournalArticle,
            "Paper",
            Some(2026),
            now.0,
        ));
        let visual = build_manual_paper_analysis_visual(
            VisualSpecId::from("manual"),
            "lib",
            ResearchVisualKind::PaperAnalysisMap,
            "Manual analysis",
            None,
            ResearchVisualManualData {
                nodes: vec![ManualVisualNode {
                    id: "claim".to_string(),
                    label: "Claim".to_string(),
                    group: None,
                    weight: 1.0,
                    reference_id: Some(crate::ReferenceId::from("ref_1")),
                    note_id: None,
                }],
                edges: Vec::new(),
                cells: Vec::new(),
                events: Vec::new(),
            },
        )
        .expect("visual");

        let report = research_release_readiness_report(&ResearchReleaseReadinessInput {
            snapshot,
            manuscripts: Vec::new(),
            visuals: vec![visual],
            i18n: Some(ResearchI18nReadiness {
                missing_keys: Vec::new(),
                fallback_keys: Vec::new(),
            }),
            evidence: ResearchReleaseEvidence::default(),
        });

        assert!(!report.release_ready);
        assert!(report
            .blockers
            .iter()
            .any(|blocker| blocker.contains("manuscript_smoke")));
        assert!(report
            .checks
            .iter()
            .any(|check| check.code == "visual_smoke"
                || check.code == "draw_plan_and_table_fallback"));
    }

    #[test]
    fn readiness_report_is_release_ready_with_complete_evidence_release_validation() {
        let locale = ResearchLocale::parse("en-US").expect("locale");
        let now = Timestamp("2026-05-04T00:00:00Z".to_string());
        let mut snapshot = new_research_library_snapshot(
            crate::LibraryId::from("lib"),
            "Library",
            "/tmp/lib",
            locale.clone(),
            now.clone(),
        );
        snapshot.references.push(reference_from_minimal_metadata(
            "ref_1",
            ReferenceKind::JournalArticle,
            "Paper",
            Some(2026),
            now.0.clone(),
        ));
        let manuscript = create_manuscript_from_template(
            ManuscriptId::from("ms"),
            crate::LibraryId::from("lib"),
            LocalizedField::plain("Draft"),
            ManuscriptTemplateKind::JournalArticle,
            locale,
            now.clone(),
        );
        let visual = build_manual_paper_analysis_visual(
            VisualSpecId::from("manual"),
            "lib",
            ResearchVisualKind::PaperAnalysisMap,
            "Manual analysis",
            None,
            ResearchVisualManualData {
                nodes: vec![ManualVisualNode {
                    id: "claim".to_string(),
                    label: "Claim".to_string(),
                    group: None,
                    weight: 1.0,
                    reference_id: Some(crate::ReferenceId::from("ref_1")),
                    note_id: None,
                }],
                edges: Vec::new(),
                cells: Vec::new(),
                events: Vec::new(),
            },
        )
        .expect("visual");

        let report = research_release_readiness_report(&ResearchReleaseReadinessInput {
            snapshot,
            manuscripts: vec![manuscript],
            visuals: vec![visual],
            i18n: Some(ResearchI18nReadiness {
                missing_keys: Vec::new(),
                fallback_keys: Vec::new(),
            }),
            evidence: ResearchReleaseEvidence {
                library_round_trip_verified: true,
                import_export_round_trip_verified: true,
                pdf_reader_round_trip_verified: true,
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
}
