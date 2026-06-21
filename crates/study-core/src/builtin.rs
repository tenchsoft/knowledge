use serde::{Deserialize, Serialize};

use crate::{
    glossary_terms_from_curriculum, language_track, mathematics_track, programming_track,
    science_track, AnswerKey, AssessmentCoverageConstraint, AssessmentDraft, AssessmentId,
    AssessmentKind, ContentLocale, Curriculum, CurriculumAuthority, CurriculumEdge,
    CurriculumEdgeKind, CurriculumGraph, CurriculumId, CurriculumMetadata, CurriculumNode,
    CurriculumNodeId, CurriculumNodeKind, EducationLevel, LearningObjective, LearningObjectiveId,
    LearningVisualId, LearningVisualKind, LearningVisualSpec, LevelRange, LocalizedStringSet,
    LocalizedText, ObjectiveTaxonomy, PracticeItem, PracticeItemId, PracticeKind, SubjectDomain,
    SubjectTrackDefinition, VisualAccessibility, VisualInteraction, VisualPlayback, VisualRenderer,
    VisualRendererEngine,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuiltinCurriculumSet {
    pub curricula: Vec<Curriculum>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuiltinContentCoverageReport {
    pub release_ready: bool,
    #[serde(default)]
    pub subject_reports: Vec<SubjectContentCoverageReport>,
    #[serde(default)]
    pub issues: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubjectContentCoverageReport {
    pub domain: SubjectDomain,
    #[serde(default)]
    pub level_reports: Vec<LevelContentCoverageReport>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LevelContentCoverageReport {
    pub level: EducationLevel,
    pub lesson_count: usize,
    pub visual_count: usize,
    pub practice_count: usize,
    pub assessment_count: usize,
    pub glossary_count: usize,
    pub release_ready: bool,
}

impl BuiltinCurriculumSet {
    pub fn production_scope() -> Self {
        Self {
            curricula: vec![
                curriculum_from_track(mathematics_track()),
                curriculum_from_track(science_track()),
                curriculum_from_track(language_track()),
                curriculum_from_track(programming_track()),
            ],
        }
    }

    pub fn by_domain(&self, domain: &SubjectDomain) -> Option<&Curriculum> {
        self.curricula
            .iter()
            .find(|curriculum| &curriculum.domain == domain)
    }
}

pub fn builtin_curricula() -> BuiltinCurriculumSet {
    BuiltinCurriculumSet::production_scope()
}

pub fn builtin_visual_specs(curriculum: &Curriculum) -> Vec<LearningVisualSpec> {
    curriculum
        .graph
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, CurriculumNodeKind::Lesson))
        .flat_map(|node| {
            node.visuals
                .iter()
                .enumerate()
                .map(|(index, visual_id)| visual_spec_for_node(curriculum, node, visual_id, index))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn builtin_visual_specs_for_all() -> Vec<LearningVisualSpec> {
    builtin_curricula()
        .curricula
        .iter()
        .flat_map(builtin_visual_specs)
        .collect()
}

pub fn builtin_practice_items(curriculum: &Curriculum) -> Vec<PracticeItem> {
    curriculum
        .graph
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, CurriculumNodeKind::Lesson))
        .map(|node| {
            let objective = node
                .objectives
                .first()
                .map(|objective| objective.statement.value.clone())
                .unwrap_or_else(|| format!("Apply {}", node.title.default.value));
            PracticeItem {
                id: PracticeItemId::from(format!("practice-{}", node.id.as_str())),
                node_id: node.id.clone(),
                prompt: LocalizedText::plain(format!(
                    "Name the lesson objective for {}.",
                    node.title.default.value
                )),
                kind: PracticeKind::Cloze,
                answer_key: AnswerKey::Cloze {
                    accepted: vec![objective.clone(), node.title.default.value.clone()],
                },
                explanation: LocalizedText::plain(format!(
                    "{}\n{}",
                    objective, node.summary.default.value
                )),
                skills: node.skills.clone(),
                difficulty: Some(level_difficulty(node.level)),
            }
        })
        .collect()
}

pub fn builtin_practice_items_for_all() -> Vec<PracticeItem> {
    builtin_curricula()
        .curricula
        .iter()
        .flat_map(builtin_practice_items)
        .collect()
}

pub fn builtin_assessments(curriculum: &Curriculum) -> Vec<AssessmentDraft> {
    EducationLevel::all()
        .iter()
        .filter_map(|level| {
            let lesson_nodes = curriculum
                .graph
                .nodes
                .iter()
                .filter(|node| {
                    node.level == *level && matches!(node.kind, CurriculumNodeKind::Lesson)
                })
                .collect::<Vec<_>>();
            if lesson_nodes.is_empty() {
                return None;
            }
            let level_id = level.stable_id();
            Some(AssessmentDraft {
                id: AssessmentId::from(format!("assessment-{}-{level_id}", curriculum.id.as_str())),
                title: LocalizedStringSet::plain(format!(
                    "{} {} mastery check",
                    curriculum.title.default.value,
                    level.label()
                )),
                kind: AssessmentKind::MasteryCheckpoint,
                node_ids: lesson_nodes.iter().map(|node| node.id.clone()).collect(),
                item_ids: lesson_nodes
                    .iter()
                    .map(|node| PracticeItemId::from(format!("practice-{}", node.id.as_str())))
                    .collect(),
                time_limit_seconds: None,
                coverage_constraints: vec![AssessmentCoverageConstraint {
                    label: format!("{}-{level_id}-mastery", curriculum.id.as_str()),
                    minimum_score: 0.8,
                }],
                report_template: LocalizedText::plain(format!(
                    "Report mastery for {} {}.",
                    curriculum.title.default.value,
                    level.label()
                )),
            })
        })
        .collect()
}

pub fn builtin_assessments_for_all() -> Vec<AssessmentDraft> {
    builtin_curricula()
        .curricula
        .iter()
        .flat_map(builtin_assessments)
        .collect()
}

pub fn builtin_content_coverage_report() -> BuiltinContentCoverageReport {
    let mut subject_reports = Vec::new();
    let mut issues = Vec::new();
    for curriculum in builtin_curricula().curricula {
        let practice_items = builtin_practice_items(&curriculum);
        let visual_specs = builtin_visual_specs(&curriculum);
        let assessments = builtin_assessments(&curriculum);
        let glossary_terms = glossary_terms_from_curriculum(&curriculum);
        let mut level_reports = Vec::new();

        for level in EducationLevel::all() {
            let lesson_ids = curriculum
                .graph
                .nodes
                .iter()
                .filter(|node| {
                    node.level == *level && matches!(node.kind, CurriculumNodeKind::Lesson)
                })
                .map(|node| node.id.clone())
                .collect::<Vec<_>>();
            let lesson_count = lesson_ids.len();
            let visual_count = visual_specs
                .iter()
                .filter(|visual| lesson_ids.contains(&visual.node_id))
                .count();
            let practice_count = practice_items
                .iter()
                .filter(|item| lesson_ids.contains(&item.node_id))
                .count();
            let assessment_count = assessments
                .iter()
                .filter(|assessment| assessment.node_ids.iter().any(|id| lesson_ids.contains(id)))
                .count();
            let glossary_count = glossary_terms
                .iter()
                .filter(|term| lesson_ids.contains(&term.node_id))
                .count();
            let release_ready = lesson_count > 0
                && visual_count >= lesson_count
                && practice_count >= lesson_count
                && assessment_count > 0
                && glossary_count >= lesson_count;
            if !release_ready {
                issues.push(format!(
                    "{:?} {:?} coverage incomplete: lessons={lesson_count}, visuals={visual_count}, practice={practice_count}, assessments={assessment_count}, glossary={glossary_count}",
                    curriculum.domain, level
                ));
            }
            level_reports.push(LevelContentCoverageReport {
                level: *level,
                lesson_count,
                visual_count,
                practice_count,
                assessment_count,
                glossary_count,
                release_ready,
            });
        }

        subject_reports.push(SubjectContentCoverageReport {
            domain: curriculum.domain,
            level_reports,
        });
    }

    BuiltinContentCoverageReport {
        release_ready: issues.is_empty(),
        subject_reports,
        issues,
    }
}

fn curriculum_from_track(track: SubjectTrackDefinition) -> Curriculum {
    let locale = ContentLocale::parse("en-US").expect("built-in locale");
    let root_id = CurriculumNodeId::from(format!("{}-program", track.id));
    let mut nodes = vec![CurriculumNode {
        id: root_id.clone(),
        kind: CurriculumNodeKind::Program,
        title: LocalizedStringSet::plain(track.title.clone()),
        summary: LocalizedStringSet::plain(format!(
            "{} curriculum from kindergarten through graduate study",
            track.title
        )),
        level: EducationLevel::Kindergarten,
        strand: None,
        objectives: Vec::new(),
        skills: Vec::new(),
        standards: Vec::new(),
        visuals: Vec::new(),
        estimated_minutes: None,
    }];
    let mut edges = Vec::new();
    let mut previous_course = None;

    for level in EducationLevel::all() {
        let course_id = CurriculumNodeId::from(format!("{}-{}", track.id, level.stable_id()));
        nodes.push(CurriculumNode {
            id: course_id.clone(),
            kind: CurriculumNodeKind::Course,
            title: LocalizedStringSet::plain(format!("{} {}", track.title, level.label())),
            summary: LocalizedStringSet::plain(format!(
                "{} scope for {} learners",
                track.title,
                level.label()
            )),
            level: *level,
            strand: None,
            objectives: Vec::new(),
            skills: Vec::new(),
            standards: Vec::new(),
            visuals: Vec::new(),
            estimated_minutes: None,
        });
        edges.push(CurriculumEdge {
            from: root_id.clone(),
            to: course_id.clone(),
            relation: CurriculumEdgeKind::Contains,
        });
        if let Some(previous) = previous_course {
            edges.push(CurriculumEdge {
                from: previous,
                to: course_id.clone(),
                relation: CurriculumEdgeKind::Prerequisite,
            });
        }
        previous_course = Some(course_id.clone());

        for strand in track.strands.iter().filter(|strand| {
            strand.level_start.ordinal() <= level.ordinal()
                && level.ordinal() <= strand.level_end.ordinal()
        }) {
            let lesson_id =
                CurriculumNodeId::from(format!("{}-{}-{}", track.id, level.stable_id(), strand.id));
            nodes.push(CurriculumNode {
                id: lesson_id.clone(),
                kind: CurriculumNodeKind::Lesson,
                title: LocalizedStringSet::plain(format!("{}: {}", level.label(), strand.title)),
                summary: LocalizedStringSet::plain(strand.required_content.join(", ")),
                level: *level,
                strand: Some(strand.id.clone()),
                objectives: vec![LearningObjective {
                    id: LearningObjectiveId::from(format!("objective-{}", lesson_id.as_str())),
                    statement: LocalizedText::plain(format!(
                        "Understand and apply {} for {}",
                        strand.title,
                        level.label()
                    )),
                    taxonomy: ObjectiveTaxonomy::Apply,
                    measurable: true,
                }],
                skills: Vec::new(),
                standards: Vec::new(),
                visuals: strand
                    .visual_expectations
                    .iter()
                    .enumerate()
                    .map(|(index, _)| {
                        LearningVisualId::from(format!("visual-{}-{}", lesson_id.as_str(), index))
                    })
                    .collect(),
                estimated_minutes: Some(30),
            });
            edges.push(CurriculumEdge {
                from: course_id.clone(),
                to: lesson_id,
                relation: CurriculumEdgeKind::Contains,
            });
        }
    }

    Curriculum {
        id: CurriculumId::from(format!("builtin-{}", track.id)),
        domain: track.domain,
        title: LocalizedStringSet::plain(track.title.clone()),
        description: LocalizedStringSet::plain(format!(
            "Built-in {} curriculum covering all supported education levels",
            track.title
        )),
        locale: locale.clone(),
        supported_locales: vec![locale],
        authority: CurriculumAuthority {
            owner: "Tench".to_string(),
            source_url: None,
            version: Some("0.1.0".to_string()),
            custom: false,
        },
        level_range: LevelRange::all_supported(),
        graph: CurriculumGraph { nodes, edges },
        standards: Vec::new(),
        metadata: CurriculumMetadata {
            created_at: None,
            updated_at: None,
            source_notes: vec![
                "Built-in scope is generated from subject track definitions.".to_string(),
            ],
        },
    }
}

fn visual_spec_for_node(
    curriculum: &Curriculum,
    node: &CurriculumNode,
    visual_id: &LearningVisualId,
    index: usize,
) -> LearningVisualSpec {
    let kind = visual_kind_for(curriculum, node, index);
    let animated = matches!(
        kind,
        LearningVisualKind::FunctionGraph
            | LearningVisualKind::MatrixTransformation
            | LearningVisualKind::ProbabilitySimulation
            | LearningVisualKind::OrganSystemModel
            | LearningVisualKind::CellSimulation
            | LearningVisualKind::ForceSimulation
            | LearningVisualKind::FieldVisualization
            | LearningVisualKind::OrbitalModel
            | LearningVisualKind::StrokeOrder
            | LearningVisualKind::TranslationAlignment
            | LearningVisualKind::DialogueRoleplay
            | LearningVisualKind::AlgorithmAnimation
            | LearningVisualKind::MemoryModel
            | LearningVisualKind::SystemTopology
    );

    LearningVisualSpec {
        id: visual_id.clone(),
        node_id: node.id.clone(),
        kind,
        title: LocalizedText::plain(format!("{} visual {}", node.title.default.value, index + 1)),
        description: LocalizedText::plain(format!(
            "Interactive visual for {}",
            node.summary.default.value
        )),
        renderer: VisualRenderer {
            engine: renderer_for(kind),
            spec_version: 1,
            scene_ref: format!(
                "builtin://{}/{}",
                curriculum.id.as_str(),
                visual_id.as_str()
            ),
        },
        playback: VisualPlayback {
            animated,
            autoplay: false,
            duration_ms: animated.then_some(3000),
            timeline_position: 0.0,
            reduced_motion_fallback: true,
        },
        interactions: interactions_for(kind),
        accessibility: VisualAccessibility {
            alt_text: format!("Text equivalent for {}", node.title.default.value),
            transcript: animated
                .then(|| format!("Step-by-step explanation for {}", node.title.default.value)),
            table_fallback_ref: Some(format!("table://{}", visual_id.as_str())),
            keyboard_model: vec![
                "tab".to_string(),
                "enter".to_string(),
                "arrow_keys".to_string(),
            ],
        },
        locale: Some(curriculum.locale.clone()),
    }
}

fn visual_kind_for(
    curriculum: &Curriculum,
    node: &CurriculumNode,
    index: usize,
) -> LearningVisualKind {
    let strand = node.strand.as_deref().unwrap_or_default();
    match &curriculum.domain {
        SubjectDomain::Mathematics => match strand {
            "number-sense" => LearningVisualKind::NumberLine,
            "algebra" => LearningVisualKind::FunctionGraph,
            "geometry" => LearningVisualKind::GeometryConstruction,
            "calculus-analysis" if index == 2 => LearningVisualKind::FieldVisualization,
            "calculus-analysis" => LearningVisualKind::FunctionGraph,
            "probability-statistics" => LearningVisualKind::ProbabilitySimulation,
            "linear-discrete" => LearningVisualKind::MatrixTransformation,
            _ => LearningVisualKind::FunctionGraph,
        },
        SubjectDomain::Science => match strand {
            "life-science" if index == 1 => LearningVisualKind::CellSimulation,
            "life-science" => LearningVisualKind::OrganSystemModel,
            "physical-science" if index == 1 => LearningVisualKind::MoleculeModel,
            "physical-science" if index == 2 => LearningVisualKind::FieldVisualization,
            "physical-science" => LearningVisualKind::ForceSimulation,
            "earth-space" => LearningVisualKind::OrbitalModel,
            "scientific-practice" => LearningVisualKind::ClimateTimeline,
            _ => LearningVisualKind::MoleculeModel,
        },
        SubjectDomain::Language => match strand {
            "literacy" if index == 1 => LearningVisualKind::GrammarTree,
            "literacy" => LearningVisualKind::PhonemeMap,
            "writing" if index == 2 => LearningVisualKind::ClimateTimeline,
            "writing" => LearningVisualKind::ArgumentMap,
            "second-language" if index == 0 => LearningVisualKind::PhonemeMap,
            "second-language" if index == 1 => LearningVisualKind::DialogueRoleplay,
            "second-language" if index == 2 => LearningVisualKind::GrammarTree,
            "second-language" if index == 3 => LearningVisualKind::TranslationAlignment,
            "second-language" if index == 4 => LearningVisualKind::StrokeOrder,
            "second-language" => LearningVisualKind::DialogueRoleplay,
            _ => LearningVisualKind::GrammarTree,
        },
        SubjectDomain::Programming => match strand {
            "computational-thinking" => LearningVisualKind::AlgorithmAnimation,
            "software-development" if index == 0 => LearningVisualKind::AlgorithmAnimation,
            "software-development" if index == 1 => LearningVisualKind::MemoryModel,
            "software-development" => LearningVisualKind::AlgorithmAnimation,
            "advanced-computing" => LearningVisualKind::SystemTopology,
            _ => LearningVisualKind::AlgorithmAnimation,
        },
        SubjectDomain::Custom { .. } => LearningVisualKind::Custom,
    }
}

fn renderer_for(kind: LearningVisualKind) -> VisualRendererEngine {
    match kind {
        LearningVisualKind::OrganSystemModel
        | LearningVisualKind::MoleculeModel
        | LearningVisualKind::OrbitalModel => VisualRendererEngine::Tench3d,
        LearningVisualKind::FunctionGraph
        | LearningVisualKind::ProbabilitySimulation
        | LearningVisualKind::ClimateTimeline => VisualRendererEngine::Plot,
        LearningVisualKind::StrokeOrder | LearningVisualKind::TranslationAlignment => {
            VisualRendererEngine::RichTextOverlay
        }
        LearningVisualKind::AlgorithmAnimation
        | LearningVisualKind::MemoryModel
        | LearningVisualKind::SystemTopology => VisualRendererEngine::CodeTrace,
        _ => VisualRendererEngine::Tench2d,
    }
}

fn interactions_for(kind: LearningVisualKind) -> Vec<VisualInteraction> {
    match kind {
        LearningVisualKind::OrganSystemModel
        | LearningVisualKind::MoleculeModel
        | LearningVisualKind::OrbitalModel => {
            vec![
                VisualInteraction::Rotate3d,
                VisualInteraction::ScrubTimeline,
            ]
        }
        LearningVisualKind::FunctionGraph
        | LearningVisualKind::GeometryConstruction
        | LearningVisualKind::MatrixTransformation
        | LearningVisualKind::ProbabilitySimulation => vec![
            VisualInteraction::PanZoom,
            VisualInteraction::ScrubTimeline,
            VisualInteraction::AdjustParameter {
                name: "parameter".to_string(),
                min: 0.0,
                max: 1.0,
            },
        ],
        LearningVisualKind::StrokeOrder | LearningVisualKind::TranslationAlignment => vec![
            VisualInteraction::ScrubTimeline,
            VisualInteraction::DragPoint {
                point_id: "learner-anchor".to_string(),
            },
        ],
        LearningVisualKind::AlgorithmAnimation
        | LearningVisualKind::MemoryModel
        | LearningVisualKind::SystemTopology => {
            vec![VisualInteraction::RunStep, VisualInteraction::ScrubTimeline]
        }
        _ => vec![
            VisualInteraction::PanZoom,
            VisualInteraction::ToggleLayer {
                layer_id: "labels".to_string(),
            },
        ],
    }
}

fn level_difficulty(level: EducationLevel) -> f32 {
    let max = EducationLevel::all()
        .last()
        .map(|level| level.ordinal())
        .unwrap_or(1) as f32;
    (level.ordinal() as f32 / max).clamp(0.1, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_curricula_cover_four_product_domains() {
        let set = builtin_curricula();

        assert_eq!(set.curricula.len(), 4);
        assert!(set.by_domain(&SubjectDomain::Mathematics).is_some());
        assert!(set.by_domain(&SubjectDomain::Science).is_some());
        assert!(set.by_domain(&SubjectDomain::Language).is_some());
        assert!(set.by_domain(&SubjectDomain::Programming).is_some());
    }

    #[test]
    fn every_builtin_curriculum_has_each_level_and_release_ready_lessons() {
        for curriculum in builtin_curricula().curricula {
            let report = curriculum.validate();
            assert!(
                report.is_valid(),
                "invalid curriculum {:?}: {:?}",
                curriculum.domain,
                report.issues
            );

            for level in EducationLevel::all() {
                assert!(
                    curriculum
                        .graph
                        .nodes
                        .iter()
                        .any(|node| node.level == *level
                            && matches!(node.kind, CurriculumNodeKind::Course)),
                    "missing course for {:?} in {:?}",
                    level,
                    curriculum.domain
                );
                assert!(
                    curriculum
                        .graph
                        .nodes
                        .iter()
                        .any(|node| node.level == *level
                            && matches!(node.kind, CurriculumNodeKind::Lesson)
                            && !node.objectives.is_empty()
                            && !node.visuals.is_empty()),
                    "missing lesson with objective/visual for {:?} in {:?}",
                    level,
                    curriculum.domain
                );
            }
        }
    }

    #[test]
    fn every_builtin_visual_id_has_release_valid_spec() {
        for curriculum in builtin_curricula().curricula {
            let specs = builtin_visual_specs(&curriculum);
            let spec_ids = specs
                .iter()
                .map(|spec| spec.id.clone())
                .collect::<std::collections::HashSet<_>>();
            for node in curriculum
                .graph
                .nodes
                .iter()
                .filter(|node| matches!(node.kind, CurriculumNodeKind::Lesson))
            {
                for visual_id in &node.visuals {
                    assert!(spec_ids.contains(visual_id));
                }
            }
            for spec in specs {
                spec.validate_for_release().expect("valid visual spec");
            }
        }
    }

    #[test]
    fn builtin_visuals_cover_subject_benchmark_kinds() {
        let curricula = builtin_curricula();
        let expected = [
            (
                SubjectDomain::Mathematics,
                vec![
                    LearningVisualKind::NumberLine,
                    LearningVisualKind::FunctionGraph,
                    LearningVisualKind::GeometryConstruction,
                    LearningVisualKind::ProbabilitySimulation,
                    LearningVisualKind::MatrixTransformation,
                ],
            ),
            (
                SubjectDomain::Science,
                vec![
                    LearningVisualKind::OrganSystemModel,
                    LearningVisualKind::CellSimulation,
                    LearningVisualKind::MoleculeModel,
                    LearningVisualKind::ForceSimulation,
                    LearningVisualKind::FieldVisualization,
                    LearningVisualKind::OrbitalModel,
                ],
            ),
            (
                SubjectDomain::Language,
                vec![
                    LearningVisualKind::PhonemeMap,
                    LearningVisualKind::GrammarTree,
                    LearningVisualKind::DialogueRoleplay,
                    LearningVisualKind::TranslationAlignment,
                    LearningVisualKind::StrokeOrder,
                ],
            ),
            (
                SubjectDomain::Programming,
                vec![
                    LearningVisualKind::AlgorithmAnimation,
                    LearningVisualKind::MemoryModel,
                    LearningVisualKind::SystemTopology,
                ],
            ),
        ];

        for (domain, kinds) in expected {
            let curriculum = curricula.by_domain(&domain).expect("curriculum");
            let actual = builtin_visual_specs(curriculum)
                .into_iter()
                .map(|spec| spec.kind)
                .collect::<std::collections::HashSet<_>>();
            for kind in kinds {
                assert!(
                    actual.contains(&kind),
                    "missing {:?} visual kind for {:?}",
                    kind,
                    domain
                );
            }
        }
    }

    #[test]
    fn builtin_practice_items_cover_every_lesson() {
        for curriculum in builtin_curricula().curricula {
            let lesson_count = curriculum
                .graph
                .nodes
                .iter()
                .filter(|node| matches!(node.kind, CurriculumNodeKind::Lesson))
                .count();
            let items = builtin_practice_items(&curriculum);

            assert_eq!(items.len(), lesson_count);
            for item in items {
                assert!(!item.prompt.value.trim().is_empty());
                assert!(!item.explanation.value.trim().is_empty());
                assert!(item.difficulty.is_some());
            }
        }
    }

    #[test]
    fn builtin_assessments_and_coverage_report_are_release_ready() {
        let report = builtin_content_coverage_report();

        assert!(report.release_ready, "{:?}", report.issues);
        assert_eq!(report.subject_reports.len(), 4);
        for subject in &report.subject_reports {
            assert_eq!(subject.level_reports.len(), EducationLevel::all().len());
            for level in &subject.level_reports {
                assert!(level.release_ready, "{:?} {:?}", subject.domain, level);
                assert!(level.lesson_count > 0);
                assert!(level.visual_count >= level.lesson_count);
                assert!(level.practice_count >= level.lesson_count);
                assert!(level.assessment_count > 0);
                assert!(level.glossary_count >= level.lesson_count);
            }
        }

        for curriculum in builtin_curricula().curricula {
            let assessments = builtin_assessments(&curriculum);
            assert_eq!(assessments.len(), EducationLevel::all().len());
            for assessment in assessments {
                assert!(!assessment.node_ids.is_empty());
                assert_eq!(assessment.node_ids.len(), assessment.item_ids.len());
                assert!(!assessment.coverage_constraints.is_empty());
            }
        }
    }
}
