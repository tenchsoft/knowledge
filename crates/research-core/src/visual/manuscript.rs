use super::*;

use crate::{
    manuscript::{
        run_non_ai_writing_checks, OutlineSection, ResearchManuscript, SectionStatus,
        WritingCheckSeverity,
    },
    ReferenceId, ResearchNoteId,
};

pub fn build_manuscript_readiness_dashboard_visual(
    id: VisualSpecId,
    manuscript: &ResearchManuscript,
) -> ResearchVisualSpec {
    let checks = run_non_ai_writing_checks(manuscript);
    let error_count = checks
        .iter()
        .filter(|check| check.severity == WritingCheckSeverity::Error)
        .count();
    let warning_count = checks
        .iter()
        .filter(|check| check.severity == WritingCheckSeverity::Warning)
        .count();
    let total_sections = count_sections(&manuscript.outline.sections);
    let completed_sections =
        count_sections_with_status(&manuscript.outline.sections, SectionStatus::Complete);
    let readiness = if total_sections == 0 {
        0.0
    } else {
        completed_sections as f32 / total_sections as f32
    };

    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::ResultComparisonChart,
        title: VisualLocalizedText {
            value: "Manuscript readiness dashboard".to_string(),
            locale: Some(manuscript.locale.clone()),
        },
        data_query: VisualQuery {
            library_id: manuscript.library_id.as_str().to_string(),
            reference_ids: manuscript.citation_state.bibliography.reference_ids.clone(),
            note_ids: collect_section_notes(&manuscript.outline.sections),
            filters: vec![
                VisualFilter {
                    field: "sections_total".to_string(),
                    value: total_sections.to_string(),
                },
                VisualFilter {
                    field: "sections_complete".to_string(),
                    value: completed_sections.to_string(),
                },
                VisualFilter {
                    field: "checks_error".to_string(),
                    value: error_count.to_string(),
                },
                VisualFilter {
                    field: "checks_warning".to_string(),
                    value: warning_count.to_string(),
                },
                VisualFilter {
                    field: "readiness_score".to_string(),
                    value: format!("{readiness:.3}"),
                },
            ],
            aggregation: Some(VisualAggregation {
                group_by: "manuscript_section_status".to_string(),
                metric: "readiness_score".to_string(),
                limit: Some(100),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::X,
                field: "section_status".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
            VisualEncoding {
                channel: VisualChannel::Y,
                field: "readiness_score".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Zero,
            },
            VisualEncoding {
                channel: VisualChannel::Color,
                field: "check_severity".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::ShowUnknown,
            },
        ],
        state: VisualState::default(),
        animation: Some(VisualAnimation {
            layout_transition: true,
            filter_transition: true,
            timeline_playback: false,
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![VisualInteraction::Select, VisualInteraction::Pan, VisualInteraction::Zoom],
        accessibility: VisualAccessibility {
            summary: format!(
                "Manuscript has {completed_sections} of {total_sections} sections complete, {error_count} errors, and {warning_count} warnings."
            ),
            table_fallback_ref: Some(format!(
                "table://manuscripts/{}/readiness",
                manuscript.id.as_str()
            )),
            screen_reader_label: Some("Manuscript readiness dashboard".to_string()),
        },
        source: VisualSource::UserAuthored,
        manual_data: None,
    }
}

pub fn build_manuscript_citation_density_visual(
    id: VisualSpecId,
    manuscript: &ResearchManuscript,
) -> ResearchVisualSpec {
    let cited_references = collect_section_references(&manuscript.outline.sections);
    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::EvidenceMatrix,
        title: VisualLocalizedText {
            value: "Citation density by section".to_string(),
            locale: Some(manuscript.locale.clone()),
        },
        data_query: VisualQuery {
            library_id: manuscript.library_id.as_str().to_string(),
            reference_ids: cited_references,
            note_ids: collect_section_notes(&manuscript.outline.sections),
            filters: Vec::new(),
            aggregation: Some(VisualAggregation {
                group_by: "section_id".to_string(),
                metric: "citation_count".to_string(),
                limit: Some(250),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::X,
                field: "section_id".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Hide,
            },
            VisualEncoding {
                channel: VisualChannel::Y,
                field: "citation_count".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Zero,
            },
            VisualEncoding {
                channel: VisualChannel::Opacity,
                field: "unresolved_citation_count".to_string(),
                metric_source: Some(MetricSource::LocalLibrary),
                missing_behavior: MissingDataBehavior::Zero,
            },
        ],
        state: VisualState::default(),
        animation: Some(VisualAnimation {
            layout_transition: true,
            filter_transition: false,
            timeline_playback: false,
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![
            VisualInteraction::Select,
            VisualInteraction::Pan,
            VisualInteraction::Zoom,
        ],
        accessibility: VisualAccessibility {
            summary: format!(
                "Citation density view for {} cited references.",
                manuscript.citation_state.citations.len()
            ),
            table_fallback_ref: Some(format!(
                "table://manuscripts/{}/citation-density",
                manuscript.id.as_str()
            )),
            screen_reader_label: Some("Citation density by section".to_string()),
        },
        source: VisualSource::UserAuthored,
        manual_data: None,
    }
}

pub fn build_manuscript_claim_evidence_visual(
    id: VisualSpecId,
    manuscript: &ResearchManuscript,
) -> ResearchVisualSpec {
    ResearchVisualSpec {
        id,
        kind: ResearchVisualKind::ClaimEvidenceGraph,
        title: VisualLocalizedText {
            value: "Claim evidence map".to_string(),
            locale: Some(manuscript.locale.clone()),
        },
        data_query: VisualQuery {
            library_id: manuscript.library_id.as_str().to_string(),
            reference_ids: collect_section_references(&manuscript.outline.sections),
            note_ids: collect_section_notes(&manuscript.outline.sections),
            filters: Vec::new(),
            aggregation: Some(VisualAggregation {
                group_by: "outline_section".to_string(),
                metric: "linked_evidence_count".to_string(),
                limit: Some(500),
            }),
        },
        encodings: vec![
            VisualEncoding {
                channel: VisualChannel::Size,
                field: "linked_evidence_count".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Zero,
            },
            VisualEncoding {
                channel: VisualChannel::Label,
                field: "section_title".to_string(),
                metric_source: Some(MetricSource::UserAuthored),
                missing_behavior: MissingDataBehavior::Hide,
            },
        ],
        state: VisualState::default(),
        animation: Some(VisualAnimation {
            layout_transition: true,
            filter_transition: true,
            timeline_playback: false,
            selection_focus: true,
            reduced_motion_fallback: true,
        }),
        interactions: vec![
            VisualInteraction::Select,
            VisualInteraction::Pan,
            VisualInteraction::Zoom,
            VisualInteraction::ExpandCluster,
        ],
        accessibility: VisualAccessibility {
            summary: format!(
                "Claim evidence graph for {} outline sections.",
                count_sections(&manuscript.outline.sections)
            ),
            table_fallback_ref: Some(format!(
                "table://manuscripts/{}/claim-evidence",
                manuscript.id.as_str()
            )),
            screen_reader_label: Some("Claim evidence map".to_string()),
        },
        source: VisualSource::UserAuthored,
        manual_data: None,
    }
}

fn count_sections(sections: &[OutlineSection]) -> usize {
    sections
        .iter()
        .map(|section| 1 + count_sections(&section.children))
        .sum()
}

fn count_sections_with_status(sections: &[OutlineSection], status: SectionStatus) -> usize {
    sections
        .iter()
        .map(|section| {
            usize::from(section.status == status)
                + count_sections_with_status(&section.children, status)
        })
        .sum()
}

fn collect_section_references(sections: &[OutlineSection]) -> Vec<ReferenceId> {
    let mut references = Vec::new();
    collect_section_references_into(sections, &mut references);
    references
}

fn collect_section_references_into(sections: &[OutlineSection], references: &mut Vec<ReferenceId>) {
    for section in sections {
        for reference_id in &section.cited_references {
            if !references.contains(reference_id) {
                references.push(reference_id.clone());
            }
        }
        collect_section_references_into(&section.children, references);
    }
}

fn collect_section_notes(sections: &[OutlineSection]) -> Vec<ResearchNoteId> {
    let mut notes = Vec::new();
    collect_section_notes_into(sections, &mut notes);
    notes
}

fn collect_section_notes_into(sections: &[OutlineSection], notes: &mut Vec<ResearchNoteId>) {
    for section in sections {
        for note_id in &section.source_notes {
            if !notes.contains(note_id) {
                notes.push(note_id.clone());
            }
        }
        collect_section_notes_into(&section.children, notes);
    }
}
