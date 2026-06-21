use std::collections::BTreeSet;

use tench_ui::prelude::Color;

use super::{ACCENT_EDITOR, STATUS_RUNNING, STATUS_WARNING};
use tench_ui::prelude::{Point, Rect};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Learn,
    Practice,
    Review,
}

impl Stage {
    pub fn label(self) -> &'static str {
        match self {
            Self::Learn => "learn",
            Self::Practice => "practice",
            Self::Review => "review",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::Learn => STATUS_RUNNING,
            Self::Practice => STATUS_WARNING,
            Self::Review => ACCENT_EDITOR,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConceptStatus {
    Completed,
    Active,
    InProgress,
    Warning,
}

#[derive(Clone, Debug)]
pub struct Concept {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub level: tench_study_core::EducationLevel,
    pub status: ConceptStatus,
    pub visual_count: usize,
    pub primary_visual_id: String,
    pub glossary_terms: Vec<GlossaryPreview>,
}

#[derive(Clone, Debug)]
pub struct GlossaryPreview {
    pub term: String,
    pub definition: String,
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub label: String,
    pub domain: tench_study_core::SubjectDomain,
    pub concepts: Vec<Concept>,
}

#[derive(Clone, Debug)]
pub struct Problem {
    pub concept_id: String,
    pub text: String,
    pub matrices: String,
    pub answer_key: tench_study_core::AnswerKey,
    pub answer: String,
    pub solution: String,
    pub cause_tag: String,
    pub related_concept: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProfileSetupStep {
    #[default]
    Identity,
    DomainLevel,
    Locale,
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StudyFocusTarget {
    None,
    SearchBox,
    NotesInput,
    TutorChat,
    ProfileLearnerId,
    ProfileDisplayName,
    AuthoringTitle,
    AuthoringBody,
}

#[derive(Clone, Debug)]
pub struct StudyNote {
    pub id: String,
    pub concept_id: String,
    pub text: String,
    pub created_at: String,
}

#[derive(Clone, Debug)]
pub struct StudyAchievement {
    pub id: String,
    pub label_key: String,
    pub description_key: String,
    pub unlocked: bool,
    pub progress: f32,
}

#[derive(Clone, Debug)]
pub struct StudyGoal {
    pub id: String,
    pub label_key: String,
    pub target: i32,
    pub current: i32,
    pub unit: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacedRepetitionRating {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Clone, Debug)]
pub struct SpacedRepetitionEntry {
    pub concept_id: String,
    pub easiness_factor: f64,
    pub interval_days: u32,
    pub repetitions: u32,
    pub next_review_date: String,
}

#[derive(Clone, Debug)]
pub struct ReviewItem {
    pub problem_text: String,
    pub wrong_answer: String,
    pub correct_answer: String,
    pub cause_tag: String,
    pub related_concept: String,
    pub solution: String,
    pub spaced_repetition: Option<SpacedRepetitionEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StudyViewportClass {
    Mobile,
    Tablet,
    Desktop,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileSetupState {
    pub learner_id: String,
    pub display_name: String,
    pub primary_locale: String,
    pub target_locales: Vec<String>,
    pub completed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudySelectionState {
    pub domain: tench_study_core::SubjectDomain,
    pub level: tench_study_core::EducationLevel,
    pub locale: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DailyStudyDashboard {
    pub due_review_count: usize,
    pub new_lesson_count: usize,
    pub current_streak: i32,
    pub minutes_today: i32,
    pub accuracy_percent: i32,
    pub recommended_concept_id: Option<String>,
    pub offline_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TouchReviewAction {
    Again,
    Hard,
    Good,
    Easy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TouchReviewState {
    pub enabled: bool,
    pub min_hit_size_px: u32,
    pub swipe_actions: Vec<TouchReviewAction>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BatchEditState {
    pub selected_concept_ids: BTreeSet<String>,
    pub selected_card_ids: BTreeSet<String>,
    pub pending_tags: Vec<String>,
    pub pending_status: Option<ConceptStatus>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StudyShortcutAction {
    StartOrSubmit,
    CycleStage,
    PreviousConcept,
    NextConcept,
    OpenStats,
    OpenReviewQueue,
    CloseModal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudyKeyboardShortcut {
    pub action: StudyShortcutAction,
    pub key: String,
    pub label_key: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StudyAccessibilityTarget {
    Header,
    Curriculum,
    LearnSurface,
    PracticeSurface,
    ReviewSurface,
    TutorPanel,
    StatsModal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudyAccessibilityLabel {
    pub target: StudyAccessibilityTarget,
    pub label_key: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OfflineAssetState {
    pub required_scene_refs: Vec<String>,
    pub missing_scene_refs: Vec<String>,
    pub cache_ready: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StudyUxAudit {
    pub missing_i18n_keys: Vec<String>,
    pub mock_content_removed: bool,
    pub accessibility_label_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StudyHit {
    Concept(usize, usize),
    ReviewQueue,
    Stats,
    StartPractice,
    SubmitAnswer,
    NextProblem,
    RetryAnswer,
    ReviewConcept,
    RevealHint(u8),
    CloseModal,
    StageClick,
    SearchBox,
    ToggleUnit(usize),
    BookmarkConcept,
    SkipProblem,
    PauseResume,
    SwipeAction(TouchReviewAction),
    RatingButton(SpacedRepetitionRating),
    VisualPlayPause,
    VisualScrubber(f64),
    VisualAutoplay,
    ProfileSetupNext,
    ProfileSetupBack,
    ProfileSetupFocusLearnerId,
    ProfileSetupFocusDisplayName,
    ProfileDomainSelect(usize),
    ProfileLevelSelect(usize),
    ProfileLocaleSelect(usize),
    NotesToggle,
    NotesInput,
    NotesSave,
    AuthoringNewCurriculum,
    AuthoringNewUnit,
    AuthoringNewConcept,
    AuthoringSaveDraft,
    ShortcutHelp,
    HintLevelKey(u8),
    HamburgerMenu,
    TutorChatSend,
    TutorChatInput,
    GlossaryExpand(usize),
    GlossarySearchToggle,
    GoalSetting,
    AchievementBadge(usize),
    MathPaletteToggle,
    MathSymbol(usize),
    HighContrastToggle,
    AuthoringTitleFocus,
    AuthoringBodyFocus,
    HamburgerMenuRow(Stage),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProfileField {
    #[default]
    LearnerId,
    DisplayName,
}

pub struct StudyState {
    pub active_subject: String,
    pub units: Vec<Unit>,
    pub problems: Vec<Problem>,
    pub profile_setup: ProfileSetupState,
    pub selection: StudySelectionState,
    pub dashboard: DailyStudyDashboard,
    pub viewport_class: StudyViewportClass,
    pub touch_review: TouchReviewState,
    pub batch_edit: BatchEditState,
    pub keyboard_shortcuts: Vec<StudyKeyboardShortcut>,
    pub accessibility_labels: Vec<StudyAccessibilityLabel>,
    pub offline_assets: OfflineAssetState,
    pub ux_audit: StudyUxAudit,
    pub stage: Stage,
    pub active_unit_idx: usize,
    pub active_concept_idx: usize,
    pub streak: i32,
    pub elapsed_seconds: i32,
    pub problem_index: usize,
    pub review_index: usize,
    pub hint_level: u8,
    pub feedback: Option<bool>,
    pub input_text: String,
    pub input_cursor_pos: usize,
    pub session_results: Vec<bool>,
    pub review_queue: Vec<ReviewItem>,
    pub show_result_modal: bool,
    pub show_stats_modal: bool,
    pub builtin_curriculum_count: usize,
    pub builtin_lesson_count: usize,
    pub builtin_visual_count: usize,
    pub builtin_glossary_count: usize,
    pub visual_specs: Vec<tench_study_core::LearningVisualSpec>,
    // Phase 1: Profile setup wizard
    pub show_profile_setup_modal: bool,
    pub profile_setup_step: ProfileSetupStep,
    pub wizard_learner_id: String,
    pub wizard_display_name: String,
    pub wizard_primary_locale: String,
    pub wizard_target_locales: Vec<String>,
    pub wizard_active_field: ProfileField,
    pub wizard_domain_idx: usize,
    pub wizard_level_idx: usize,
    pub wizard_locale_idx: usize,
    // Phase 2: Search/Notes/Bookmarks
    pub search_focused: bool,
    pub search_query: String,
    pub expanded_units: Vec<bool>,
    pub outline_scroll_offset: f64,
    pub bookmarked_concept_ids: BTreeSet<String>,
    pub show_notes_panel: bool,
    pub notes: Vec<StudyNote>,
    pub notes_input: String,
    pub focus_target: StudyFocusTarget,
    // Phase 3: Practice improvements
    pub session_paused: bool,
    pub show_math_palette: bool,
    // Phase 4: Mobile/touch
    pub swipe_start: Option<(Point, std::time::Instant)>,
    pub show_hamburger_menu: bool,
    // Phase 5: Stats/progress
    pub daily_accuracy_history: Vec<i32>,
    pub streak_calendar: Vec<bool>,
    pub goals: Vec<StudyGoal>,
    pub achievements: Vec<StudyAchievement>,
    pub show_goal_modal: bool,
    // Phase 6: Spaced repetition
    pub spaced_repetition_data: Vec<SpacedRepetitionEntry>,
    pub pending_rating: Option<SpacedRepetitionRating>,
    // Phase 7: Tutor expansion
    pub tutor_chat_input: String,
    pub tutor_chat_messages: Vec<TutorChatMessage>,
    pub expanded_glossary_idx: Option<usize>,
    pub glossary_search_focused: bool,
    pub glossary_search_query: String,
    // Phase 8: Visual interaction
    pub visual_playing: bool,
    pub visual_autoplay: bool,
    pub visual_timeline_position: f64,
    // Phase 9: Content authoring
    pub show_authoring_panel: bool,
    pub authoring_title: String,
    pub authoring_body: String,
    pub authoring_problem_text: String,
    pub authoring_problem_answer: String,
    // Phase 10: Shortcuts/accessibility
    pub show_shortcut_help: bool,
    pub focus_indicator: Option<Rect>,
    pub high_contrast_mode: bool,
    pub expanded_achievement_idx: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct TutorChatMessage {
    pub role: TutorChatRole,
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TutorChatRole {
    User,
    Assistant,
}

// Phase 3: Session auto-save/restore snapshot
#[derive(Clone, Debug)]
pub struct SessionSnapshot {
    pub stage: Stage,
    pub active_unit_idx: usize,
    pub active_concept_idx: usize,
    pub problem_index: usize,
    pub input_text: String,
    pub input_cursor_pos: usize,
    pub streak: i32,
    pub elapsed_seconds: i32,
    pub session_results: Vec<bool>,
}
