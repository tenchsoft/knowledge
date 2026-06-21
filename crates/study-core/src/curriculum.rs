use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::{ContentLocale, LearningVisualId, LocalizedStringSet, LocalizedText};

crate::study_id_type!(CurriculumId);
crate::study_id_type!(CurriculumNodeId);
crate::study_id_type!(LearningObjectiveId);
crate::study_id_type!(SkillId);
crate::study_id_type!(StandardId);
crate::study_id_type!(RubricId);
crate::study_id_type!(CustomDomainId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Curriculum {
    pub id: CurriculumId,
    pub domain: SubjectDomain,
    pub title: LocalizedStringSet,
    pub description: LocalizedStringSet,
    pub locale: ContentLocale,
    #[serde(default)]
    pub supported_locales: Vec<ContentLocale>,
    pub authority: CurriculumAuthority,
    pub level_range: LevelRange,
    pub graph: CurriculumGraph,
    #[serde(default)]
    pub standards: Vec<CurriculumStandard>,
    #[serde(default)]
    pub metadata: CurriculumMetadata,
}

impl Curriculum {
    pub fn validate(&self) -> CurriculumValidationReport {
        self.graph.validate()
    }

    pub fn supports_locale(&self, locale: &ContentLocale) -> bool {
        self.locale == *locale || self.supported_locales.contains(locale)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum SubjectDomain {
    Mathematics,
    Science,
    Language,
    Programming,
    Custom {
        id: CustomDomainId,
        label: String,
        owner: String,
    },
}

impl SubjectDomain {
    pub fn product_supported(&self) -> bool {
        matches!(
            self,
            Self::Mathematics | Self::Science | Self::Language | Self::Programming
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurriculumAuthority {
    pub owner: String,
    #[serde(default)]
    pub source_url: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    pub custom: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EducationLevel {
    Kindergarten,
    ElementaryLower,
    ElementaryUpper,
    MiddleSchool,
    HighSchool,
    UndergraduateLower,
    UndergraduateUpper,
    GraduateMasters,
    GraduateDoctoral,
}

impl EducationLevel {
    pub fn all() -> &'static [EducationLevel] {
        &[
            Self::Kindergarten,
            Self::ElementaryLower,
            Self::ElementaryUpper,
            Self::MiddleSchool,
            Self::HighSchool,
            Self::UndergraduateLower,
            Self::UndergraduateUpper,
            Self::GraduateMasters,
            Self::GraduateDoctoral,
        ]
    }

    pub fn ordinal(self) -> u8 {
        match self {
            Self::Kindergarten => 0,
            Self::ElementaryLower => 1,
            Self::ElementaryUpper => 2,
            Self::MiddleSchool => 3,
            Self::HighSchool => 4,
            Self::UndergraduateLower => 5,
            Self::UndergraduateUpper => 6,
            Self::GraduateMasters => 7,
            Self::GraduateDoctoral => 8,
        }
    }

    pub fn stable_id(self) -> &'static str {
        match self {
            Self::Kindergarten => "kindergarten",
            Self::ElementaryLower => "elementary-lower",
            Self::ElementaryUpper => "elementary-upper",
            Self::MiddleSchool => "middle-school",
            Self::HighSchool => "high-school",
            Self::UndergraduateLower => "undergraduate-lower",
            Self::UndergraduateUpper => "undergraduate-upper",
            Self::GraduateMasters => "graduate-masters",
            Self::GraduateDoctoral => "graduate-doctoral",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Kindergarten => "Kindergarten",
            Self::ElementaryLower => "Elementary lower",
            Self::ElementaryUpper => "Elementary upper",
            Self::MiddleSchool => "Middle school",
            Self::HighSchool => "High school",
            Self::UndergraduateLower => "Undergraduate lower",
            Self::UndergraduateUpper => "Undergraduate upper",
            Self::GraduateMasters => "Graduate masters",
            Self::GraduateDoctoral => "Graduate doctoral",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LevelRange {
    pub start: EducationLevel,
    pub end: EducationLevel,
}

impl LevelRange {
    pub fn all_supported() -> Self {
        Self {
            start: EducationLevel::Kindergarten,
            end: EducationLevel::GraduateDoctoral,
        }
    }

    pub fn contains(self, level: EducationLevel) -> bool {
        self.start.ordinal() <= level.ordinal() && level.ordinal() <= self.end.ordinal()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurriculumGraph {
    #[serde(default)]
    pub nodes: Vec<CurriculumNode>,
    #[serde(default)]
    pub edges: Vec<CurriculumEdge>,
}

impl CurriculumGraph {
    pub fn add_node(&mut self, node: CurriculumNode) -> Result<(), String> {
        if self.nodes.iter().any(|existing| existing.id == node.id) {
            return Err(format!("duplicate node id {}", node.id.as_str()));
        }
        self.nodes.push(node);
        Ok(())
    }

    pub fn add_edge(&mut self, edge: CurriculumEdge) -> Result<(), String> {
        let ids: HashSet<CurriculumNodeId> =
            self.nodes.iter().map(|node| node.id.clone()).collect();
        if !ids.contains(&edge.from) {
            return Err(format!("edge source {} does not exist", edge.from.as_str()));
        }
        if !ids.contains(&edge.to) {
            return Err(format!("edge target {} does not exist", edge.to.as_str()));
        }
        self.edges.push(edge);
        let report = self.validate();
        if report.is_valid() {
            Ok(())
        } else {
            let cycle = report
                .issues
                .iter()
                .find(|issue| issue.code == "prerequisite_cycle")
                .map(|issue| issue.message.clone());
            if let Some(message) = cycle {
                self.edges.pop();
                Err(message)
            } else {
                Ok(())
            }
        }
    }

    pub fn orphan_nodes(&self) -> Vec<CurriculumNodeId> {
        let connected = self
            .edges
            .iter()
            .flat_map(|edge| [&edge.from, &edge.to])
            .cloned()
            .collect::<HashSet<_>>();
        self.nodes
            .iter()
            .filter(|node| {
                matches!(
                    node.kind,
                    CurriculumNodeKind::Lesson
                        | CurriculumNodeKind::PracticeSet
                        | CurriculumNodeKind::Assessment
                        | CurriculumNodeKind::Project
                        | CurriculumNodeKind::Lab
                ) && !connected.contains(&node.id)
            })
            .map(|node| node.id.clone())
            .collect()
    }

    pub fn validate(&self) -> CurriculumValidationReport {
        let mut issues = Vec::new();
        let mut seen = HashSet::new();
        for node in &self.nodes {
            if node.title.default.value.trim().is_empty() {
                issues.push(CurriculumValidationIssue::new(
                    CurriculumIssueSeverity::Error,
                    "node_title_required",
                    format!("node {} requires a title", node.id.as_str()),
                ));
            }
            if !seen.insert(node.id.clone()) {
                issues.push(CurriculumValidationIssue::new(
                    CurriculumIssueSeverity::Error,
                    "duplicate_node_id",
                    format!("duplicate node id {}", node.id.as_str()),
                ));
            }
            if node.objectives.is_empty() && matches!(node.kind, CurriculumNodeKind::Lesson) {
                issues.push(CurriculumValidationIssue::new(
                    CurriculumIssueSeverity::Warning,
                    "lesson_without_objective",
                    format!("lesson {} has no objective", node.id.as_str()),
                ));
            }
        }

        let ids: HashSet<CurriculumNodeId> =
            self.nodes.iter().map(|node| node.id.clone()).collect();
        for edge in &self.edges {
            if !ids.contains(&edge.from) {
                issues.push(CurriculumValidationIssue::new(
                    CurriculumIssueSeverity::Error,
                    "missing_edge_source",
                    format!("edge source {} does not exist", edge.from.as_str()),
                ));
            }
            if !ids.contains(&edge.to) {
                issues.push(CurriculumValidationIssue::new(
                    CurriculumIssueSeverity::Error,
                    "missing_edge_target",
                    format!("edge target {} does not exist", edge.to.as_str()),
                ));
            }
        }

        if has_prerequisite_cycle(&self.edges) {
            issues.push(CurriculumValidationIssue::new(
                CurriculumIssueSeverity::Error,
                "prerequisite_cycle",
                "curriculum prerequisites must be acyclic".to_string(),
            ));
        }

        for node_id in self.orphan_nodes() {
            issues.push(CurriculumValidationIssue::new(
                CurriculumIssueSeverity::Warning,
                "orphan_curriculum_node",
                format!(
                    "node {} is not connected to the curriculum graph",
                    node_id.as_str()
                ),
            ));
        }

        CurriculumValidationReport { issues }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurriculumNode {
    pub id: CurriculumNodeId,
    pub kind: CurriculumNodeKind,
    pub title: LocalizedStringSet,
    pub summary: LocalizedStringSet,
    pub level: EducationLevel,
    #[serde(default)]
    pub strand: Option<String>,
    #[serde(default)]
    pub objectives: Vec<LearningObjective>,
    #[serde(default)]
    pub skills: Vec<SkillId>,
    #[serde(default)]
    pub standards: Vec<StandardId>,
    #[serde(default)]
    pub visuals: Vec<LearningVisualId>,
    #[serde(default)]
    pub estimated_minutes: Option<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurriculumNodeKind {
    Program,
    Course,
    Unit,
    Module,
    Lesson,
    PracticeSet,
    Assessment,
    Project,
    Lab,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurriculumEdge {
    pub from: CurriculumNodeId,
    pub to: CurriculumNodeId,
    pub relation: CurriculumEdgeKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurriculumEdgeKind {
    Contains,
    Prerequisite,
    Reinforces,
    Remediates,
    Extends,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LearningObjective {
    pub id: LearningObjectiveId,
    pub statement: LocalizedText,
    pub taxonomy: ObjectiveTaxonomy,
    #[serde(default)]
    pub measurable: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectiveTaxonomy {
    Remember,
    Understand,
    Apply,
    Analyze,
    Evaluate,
    Create,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurriculumStandard {
    pub id: StandardId,
    pub label: String,
    pub description: LocalizedText,
    #[serde(default)]
    pub jurisdiction: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rubric {
    pub id: RubricId,
    pub title: LocalizedStringSet,
    pub levels: Vec<RubricLevel>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RubricLevel {
    pub score: u8,
    pub label: LocalizedText,
    pub descriptor: LocalizedText,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct CurriculumMetadata {
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub source_notes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurriculumValidationReport {
    pub issues: Vec<CurriculumValidationIssue>,
}

impl CurriculumValidationReport {
    pub fn is_valid(&self) -> bool {
        !self
            .issues
            .iter()
            .any(|issue| issue.severity == CurriculumIssueSeverity::Error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CurriculumValidationIssue {
    pub severity: CurriculumIssueSeverity,
    pub code: String,
    pub message: String,
}

impl CurriculumValidationIssue {
    pub fn new(
        severity: CurriculumIssueSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurriculumIssueSeverity {
    Info,
    Warning,
    Error,
}

fn has_prerequisite_cycle(edges: &[CurriculumEdge]) -> bool {
    let mut adjacency: HashMap<&CurriculumNodeId, Vec<&CurriculumNodeId>> = HashMap::new();
    for edge in edges {
        if edge.relation == CurriculumEdgeKind::Prerequisite {
            adjacency.entry(&edge.from).or_default().push(&edge.to);
        }
    }

    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for node in adjacency.keys() {
        if visit_cycle(node, &adjacency, &mut visiting, &mut visited) {
            return true;
        }
    }
    false
}

fn visit_cycle<'a>(
    node: &'a CurriculumNodeId,
    adjacency: &HashMap<&'a CurriculumNodeId, Vec<&'a CurriculumNodeId>>,
    visiting: &mut HashSet<&'a CurriculumNodeId>,
    visited: &mut HashSet<&'a CurriculumNodeId>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node) {
        return true;
    }
    if let Some(next_nodes) = adjacency.get(node) {
        for next in next_nodes {
            if visit_cycle(next, adjacency, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(node);
    visited.insert(node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str) -> CurriculumNode {
        CurriculumNode {
            id: CurriculumNodeId::from(id),
            kind: CurriculumNodeKind::Lesson,
            title: LocalizedStringSet::plain(id),
            summary: LocalizedStringSet::plain("summary"),
            level: EducationLevel::HighSchool,
            strand: None,
            objectives: vec![LearningObjective {
                id: LearningObjectiveId::from(format!("obj_{id}")),
                statement: LocalizedText::plain("understand"),
                taxonomy: ObjectiveTaxonomy::Understand,
                measurable: true,
            }],
            skills: Vec::new(),
            standards: Vec::new(),
            visuals: Vec::new(),
            estimated_minutes: Some(20),
        }
    }

    #[test]
    fn validates_missing_edge_endpoint() {
        let graph = CurriculumGraph {
            nodes: vec![node("a")],
            edges: vec![CurriculumEdge {
                from: CurriculumNodeId::from("a"),
                to: CurriculumNodeId::from("b"),
                relation: CurriculumEdgeKind::Prerequisite,
            }],
        };

        let report = graph.validate();

        assert!(!report.is_valid());
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "missing_edge_target"));
    }

    #[test]
    fn detects_prerequisite_cycle() {
        let graph = CurriculumGraph {
            nodes: vec![node("a"), node("b")],
            edges: vec![
                CurriculumEdge {
                    from: CurriculumNodeId::from("a"),
                    to: CurriculumNodeId::from("b"),
                    relation: CurriculumEdgeKind::Prerequisite,
                },
                CurriculumEdge {
                    from: CurriculumNodeId::from("b"),
                    to: CurriculumNodeId::from("a"),
                    relation: CurriculumEdgeKind::Prerequisite,
                },
            ],
        };

        let report = graph.validate();

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "prerequisite_cycle"));
    }

    #[test]
    fn graph_editor_rejects_duplicate_and_detects_orphans() {
        let mut graph = CurriculumGraph::default();

        graph.add_node(node("a")).expect("add node");
        assert!(graph.add_node(node("a")).is_err());
        assert_eq!(graph.orphan_nodes(), vec![CurriculumNodeId::from("a")]);
    }

    #[test]
    fn graph_editor_rejects_cycle_on_edge_add() {
        let mut graph = CurriculumGraph {
            nodes: vec![node("a"), node("b")],
            edges: vec![CurriculumEdge {
                from: CurriculumNodeId::from("a"),
                to: CurriculumNodeId::from("b"),
                relation: CurriculumEdgeKind::Prerequisite,
            }],
        };

        let result = graph.add_edge(CurriculumEdge {
            from: CurriculumNodeId::from("b"),
            to: CurriculumNodeId::from("a"),
            relation: CurriculumEdgeKind::Prerequisite,
        });

        assert!(result.is_err());
        assert_eq!(graph.edges.len(), 1);
    }
}
