use serde::{Deserialize, Serialize};
use tench_job_core::JobDescriptor;
use tench_shared_types::EngineRequest;

use crate::{AttachmentId, ReferenceId, ResearchLocale, ResearchNoteId, Timestamp};

crate::research_id_type!(VisualSpecId);
crate::research_id_type!(VisualDraftId);
crate::research_id_type!(EngineRunId);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchVisualSpec {
    pub id: VisualSpecId,
    pub kind: ResearchVisualKind,
    pub title: VisualLocalizedText,
    pub data_query: VisualQuery,
    #[serde(default)]
    pub encodings: Vec<VisualEncoding>,
    pub state: VisualState,
    #[serde(default)]
    pub animation: Option<VisualAnimation>,
    #[serde(default)]
    pub interactions: Vec<VisualInteraction>,
    pub accessibility: VisualAccessibility,
    pub source: VisualSource,
    #[serde(default)]
    pub manual_data: Option<ResearchVisualManualData>,
}

impl ResearchVisualSpec {
    pub fn validate_for_non_ai_release(&self) -> Result<(), String> {
        if self.title.value.trim().is_empty() {
            return Err("visual title is required".to_string());
        }
        if self.accessibility.table_fallback_ref.is_none()
            && self.accessibility.summary.trim().is_empty()
        {
            return Err("visual requires table fallback or textual summary".to_string());
        }
        if let Some(manual_data) = &self.manual_data {
            manual_data.validate_for_kind(self.kind)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResearchVisualKind {
    BarChart,
    Histogram,
    Timeline,
    InfluenceGraph,
    KeywordMap,
    EvidenceMatrix,
    PdfOverlay,
    AnnotationHeatmap,
    PaperAnalysisMap,
    MethodFlow,
    ClaimEvidenceGraph,
    ExperimentTimeline,
    ResultComparisonChart,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualLocalizedText {
    pub value: String,
    #[serde(default)]
    pub locale: Option<ResearchLocale>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResearchVisualManualData {
    #[serde(default)]
    pub nodes: Vec<ManualVisualNode>,
    #[serde(default)]
    pub edges: Vec<ManualVisualEdge>,
    #[serde(default)]
    pub cells: Vec<ManualVisualCell>,
    #[serde(default)]
    pub events: Vec<ManualVisualEvent>,
}

impl ResearchVisualManualData {
    pub fn validate_for_kind(&self, kind: ResearchVisualKind) -> Result<(), String> {
        for node in &self.nodes {
            if node.id.trim().is_empty() || node.label.trim().is_empty() {
                return Err("manual visual nodes require id and label".to_string());
            }
            if !node.weight.is_finite() {
                return Err(format!("manual visual node {} has invalid weight", node.id));
            }
        }
        for edge in &self.edges {
            if edge.from.trim().is_empty() || edge.to.trim().is_empty() {
                return Err("manual visual edges require from and to ids".to_string());
            }
            if !edge.strength.is_finite() {
                return Err(format!(
                    "manual visual edge {} -> {} has invalid strength",
                    edge.from, edge.to
                ));
            }
            if !self.nodes.is_empty()
                && (!self.nodes.iter().any(|node| node.id == edge.from)
                    || !self.nodes.iter().any(|node| node.id == edge.to))
            {
                return Err(format!(
                    "manual visual edge {} -> {} references missing node",
                    edge.from, edge.to
                ));
            }
        }
        for cell in &self.cells {
            if cell.row.trim().is_empty() || cell.column.trim().is_empty() {
                return Err("manual visual cells require row and column labels".to_string());
            }
            if !cell.value.is_finite() {
                return Err(format!(
                    "manual visual cell {} / {} has invalid value",
                    cell.row, cell.column
                ));
            }
        }
        for event in &self.events {
            if event.id.trim().is_empty() || event.label.trim().is_empty() {
                return Err("manual visual events require id and label".to_string());
            }
            if !event.position.is_finite() {
                return Err(format!(
                    "manual visual event {} has invalid position",
                    event.id
                ));
            }
        }
        match kind {
            ResearchVisualKind::PaperAnalysisMap
            | ResearchVisualKind::MethodFlow
            | ResearchVisualKind::ClaimEvidenceGraph
                if self.nodes.is_empty() =>
            {
                return Err("manual graph visual requires nodes".to_string());
            }
            ResearchVisualKind::EvidenceMatrix | ResearchVisualKind::ResultComparisonChart
                if self.cells.is_empty() =>
            {
                return Err("manual matrix visual requires cells".to_string());
            }
            ResearchVisualKind::ExperimentTimeline | ResearchVisualKind::Timeline
                if self.events.is_empty() =>
            {
                return Err("manual timeline visual requires events".to_string());
            }
            _ => {}
        }
        Ok(())
    }
}

fn default_manual_weight() -> f32 {
    1.0
}

fn default_manual_strength() -> f32 {
    1.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualVisualNode {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default = "default_manual_weight")]
    pub weight: f32,
    #[serde(default)]
    pub reference_id: Option<ReferenceId>,
    #[serde(default)]
    pub note_id: Option<ResearchNoteId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualVisualEdge {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default = "default_manual_strength")]
    pub strength: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualVisualCell {
    pub row: String,
    pub column: String,
    pub value: f32,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualVisualEvent {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub year: Option<i32>,
    #[serde(default)]
    pub position: f32,
    #[serde(default)]
    pub reference_id: Option<ReferenceId>,
    #[serde(default)]
    pub note_id: Option<ResearchNoteId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualQuery {
    pub library_id: String,
    #[serde(default)]
    pub reference_ids: Vec<ReferenceId>,
    #[serde(default)]
    pub note_ids: Vec<ResearchNoteId>,
    #[serde(default)]
    pub filters: Vec<VisualFilter>,
    #[serde(default)]
    pub aggregation: Option<VisualAggregation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualFilter {
    pub field: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualAggregation {
    pub group_by: String,
    pub metric: String,
    #[serde(default)]
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualEncoding {
    pub channel: VisualChannel,
    pub field: String,
    #[serde(default)]
    pub metric_source: Option<MetricSource>,
    #[serde(default)]
    pub missing_behavior: MissingDataBehavior,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualChannel {
    X,
    Y,
    Color,
    Size,
    Shape,
    Label,
    Opacity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricSource {
    LocalLibrary,
    ImportedMetadata,
    UserAuthored,
    ExternalImported,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingDataBehavior {
    #[default]
    Hide,
    ShowUnknown,
    Zero,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VisualState {
    #[serde(default)]
    pub selected_id: Option<String>,
    #[serde(default)]
    pub active_filters: Vec<VisualFilter>,
    pub viewport: ViewportTransform,
    #[serde(default)]
    pub hovered_id: Option<String>,
    #[serde(default)]
    pub expanded_clusters: Vec<String>,
    #[serde(default)]
    pub timeline_range: Option<(i32, i32)>,
}

impl VisualState {
    pub fn apply_action(mut self, action: ResearchVisualAction) -> Self {
        match action {
            ResearchVisualAction::Select { id } => {
                self.selected_id = id;
            }
            ResearchVisualAction::Hover { id } => {
                self.hovered_id = id;
            }
            ResearchVisualAction::SetViewport { pan_x, pan_y, zoom } => {
                self.viewport = ViewportTransform {
                    pan_x,
                    pan_y,
                    zoom: zoom.clamp(0.05, 64.0),
                };
            }
            ResearchVisualAction::AddFilter { field, value } => {
                self.active_filters.push(VisualFilter { field, value });
            }
            ResearchVisualAction::ClearFilters => {
                self.active_filters.clear();
            }
            ResearchVisualAction::ToggleCluster { id } => {
                if let Some(index) = self
                    .expanded_clusters
                    .iter()
                    .position(|cluster| cluster == &id)
                {
                    self.expanded_clusters.remove(index);
                } else {
                    self.expanded_clusters.push(id);
                }
            }
            ResearchVisualAction::SetTimelineRange { start, end } => {
                self.timeline_range = Some((start.min(end), start.max(end)));
            }
        }
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum ResearchVisualAction {
    Select { id: Option<String> },
    Hover { id: Option<String> },
    SetViewport { pan_x: f32, pan_y: f32, zoom: f32 },
    AddFilter { field: String, value: String },
    ClearFilters,
    ToggleCluster { id: String },
    SetTimelineRange { start: i32, end: i32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewportTransform {
    pub pan_x: f32,
    pub pan_y: f32,
    pub zoom: f32,
}

impl Default for ViewportTransform {
    fn default() -> Self {
        Self {
            pan_x: 0.0,
            pan_y: 0.0,
            zoom: 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VisualAnimation {
    #[serde(default)]
    pub layout_transition: bool,
    #[serde(default)]
    pub filter_transition: bool,
    #[serde(default)]
    pub timeline_playback: bool,
    #[serde(default)]
    pub selection_focus: bool,
    #[serde(default)]
    pub reduced_motion_fallback: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualInteraction {
    Select,
    Pan,
    Zoom,
    DragNode,
    ExpandCluster,
    ScrubTimeline,
    EditNode,
    EditEdge,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VisualAccessibility {
    pub summary: String,
    #[serde(default)]
    pub table_fallback_ref: Option<String>,
    #[serde(default)]
    pub screen_reader_label: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualSource {
    UserAuthored,
    MetadataDerived,
    CitationDerived,
    AnnotationDerived,
    LlmDerivedDraft,
    Imported,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiVisualDraft {
    pub id: VisualDraftId,
    pub source_reference_id: ReferenceId,
    #[serde(default)]
    pub source_attachment_id: Option<AttachmentId>,
    #[serde(default)]
    pub source_ranges: Vec<SourceRange>,
    pub visual_spec: ResearchVisualSpec,
    #[serde(default)]
    pub confidence: Option<f32>,
    #[serde(default)]
    pub warnings: Vec<AiVisualWarning>,
    pub created_by: EngineRunId,
    pub status: DraftStatus,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AiVisualRequest {
    pub source_reference_id: ReferenceId,
    #[serde(default)]
    pub source_attachment_id: Option<AttachmentId>,
    #[serde(default)]
    pub source_ranges: Vec<SourceRange>,
    pub requested_kind: ResearchVisualKind,
    pub prompt: String,
    pub library_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiVisualEngineRequestPlan {
    pub request_id: String,
    pub draft_id: VisualDraftId,
    pub source_reference_id: ReferenceId,
    #[serde(default)]
    pub source_attachment_id: Option<AttachmentId>,
    #[serde(default)]
    pub source_ranges: Vec<SourceRange>,
    pub requested_kind: ResearchVisualKind,
    pub model: String,
    pub context_preview: String,
    pub requires_user_approval: bool,
    pub job: JobDescriptor,
    pub engine_request: EngineRequest,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchVisualDrawPlan {
    pub visual_id: VisualSpecId,
    pub kind: ResearchVisualKind,
    pub title: String,
    pub viewport: ViewportTransform,
    #[serde(default)]
    pub commands: Vec<ResearchVisualDrawCommand>,
    pub accessibility_summary: String,
    pub table_fallback_ref: Option<String>,
    #[serde(default)]
    pub table_fallback: Vec<ResearchVisualTableRow>,
    pub reduced_motion: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchVisualTableRow {
    pub label: String,
    #[serde(default)]
    pub cells: Vec<ResearchVisualTableCell>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResearchVisualTableCell {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResearchVisualAggregationBundle {
    pub library_id: String,
    pub reference_count: usize,
    pub included_reference_count: usize,
    pub omitted_reference_count: usize,
    #[serde(default)]
    pub visuals: Vec<ResearchVisualSpec>,
    #[serde(default)]
    pub summary_rows: Vec<ResearchVisualTableRow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum ResearchVisualDrawCommand {
    TimelineAxis {
        start_year: i32,
        end_year: i32,
    },
    TimelineBin {
        id: String,
        year: i32,
        count: u32,
        selected: bool,
    },
    GraphNode {
        id: String,
        label: String,
        radius: f32,
        selected: bool,
        hovered: bool,
    },
    GraphEdge {
        from: String,
        to: String,
        strength: f32,
    },
    MatrixCell {
        row: String,
        column: String,
        value: f32,
        selected: bool,
    },
    OverlayRegion {
        id: String,
        label: String,
        opacity: f32,
    },
    TextLabel {
        id: String,
        label: String,
    },
}

impl AiVisualDraft {
    pub fn can_commit_to_canonical_content(&self) -> bool {
        matches!(self.status, DraftStatus::Accepted | DraftStatus::Edited)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceRange {
    pub kind: SourceRangeKind,
    #[serde(default)]
    pub page: Option<u32>,
    #[serde(default)]
    pub start: Option<u32>,
    #[serde(default)]
    pub end: Option<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceRangeKind {
    PdfText,
    NoteText,
    ManuscriptText,
    Figure,
    Table,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AiVisualWarning {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftStatus {
    Proposed,
    Accepted,
    Rejected,
    Edited,
}
