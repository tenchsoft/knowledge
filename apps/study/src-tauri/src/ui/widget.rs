use super::state::{
    ProfileField, ProfileSetupStep, SpacedRepetitionRating, Stage, StudyHit, TouchReviewAction,
};
use super::*;
use tench_ui::prelude::*;

impl Widget for StudyApp {
    fn measure(&mut self, _ctx: &mut MeasureCtx<'_>, _axis: Axis, available: f64) -> f64 {
        available
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx<'_>, size: Size) {
        self.size = size;
        self.state.update_viewport(size);
    }

    fn paint(&mut self, ctx: &mut PaintCtx<'_>, scene: &mut Scene) {
        let regions = curriculum::regions(ctx.size());
        let mut painter = Painter::new(scene);
        let hc = self.state.high_contrast_mode;

        // Automatic session auto-save: increment timer and save every 30 seconds
        if self.state.stage == Stage::Practice || self.state.stage == Stage::Review {
            self.state.elapsed_seconds += 1;
            if self.state.elapsed_seconds % 30 == 0 {
                let _snapshot = self.state.auto_save_session();
                // In production, snapshot would be persisted to storage here.
            }
        }

        // Phase 1: Profile setup wizard overlay (shown when not completed)
        if self.state.show_profile_setup_modal {
            tutor::paint_profile_setup_wizard(&mut painter, &self.state, ctx.size(), &self.i18n);
            return;
        }

        curriculum::paint_shell(&mut painter, &self.state, &regions, &self.i18n);
        curriculum::paint_outline(&mut painter, &self.state, &regions, &self.i18n);

        // Phase 2: Notes panel overlay in Learn mode
        if self.state.show_notes_panel && self.state.stage == Stage::Learn {
            learn::paint_notes_panel(&mut painter, &self.state, &regions, &self.i18n);
        }

        // Phase 9: Authoring panel overlay
        if self.state.show_authoring_panel {
            tutor::paint_authoring_panel(&mut painter, &self.state, ctx.size(), &self.i18n);
            return;
        }

        match self.state.stage {
            Stage::Learn => {
                learn::paint_learn_surface(&mut painter, &self.state, &regions, &self.i18n)
            }
            Stage::Practice => {
                practice::paint_practice_surface(&mut painter, &self.state, &regions, &self.i18n)
            }
            Stage::Review => {
                practice::paint_review_surface(&mut painter, &self.state, &regions, &self.i18n)
            }
        }
        tutor::paint_tutor_panel(&mut painter, &self.state, &regions, &self.i18n);
        tutor::paint_modals(&mut painter, &self.state, ctx.size(), &self.i18n);

        // Phase 10: Shortcut help modal
        if self.state.show_shortcut_help {
            tutor::paint_shortcut_help_modal(&mut painter, &self.state, ctx.size(), &self.i18n);
        }

        // Phase 5: Goal setting modal
        if self.state.show_goal_modal {
            tutor::paint_goal_modal(&mut painter, &self.state, ctx.size(), &self.i18n);
        }

        // Phase 4: Mobile hamburger menu
        if self.state.show_hamburger_menu {
            curriculum::paint_hamburger_menu(&mut painter, &self.state, &regions, &self.i18n);
        }

        // Phase 10: Focus indicator
        if self.state.focus_target != state::StudyFocusTarget::None {
            let focus_rect = match self.state.focus_target {
                state::StudyFocusTarget::SearchBox => regions.outline,
                _ => regions.surface,
            };
            let indicator_color = state::accent_color(hc);
            painter.stroke_rounded_rect(focus_rect, indicator_color, 2.0, 4.0);
        }
    }

    fn on_pointer_event(&mut self, ctx: &mut EventCtx<'_>, event: &PointerEvent) {
        let regions = curriculum::regions(self.size);

        match event {
            PointerEvent::Down(event) => {
                // Phase 4: Record swipe start position on mobile
                if self.state.touch_review.enabled {
                    self.state.swipe_start = Some((event.pos, std::time::Instant::now()));
                }

                // Phase 1: Profile setup wizard hit testing
                if self.state.show_profile_setup_modal {
                    match tutor::hit_test_profile_setup(
                        self.state.profile_setup_step,
                        self.size,
                        event.pos,
                    ) {
                        Some(StudyHit::ProfileSetupNext) => {
                            self.state.advance_profile_step();
                        }
                        Some(StudyHit::ProfileSetupBack) => {
                            self.state.go_back_profile_step();
                        }
                        Some(StudyHit::ProfileSetupFocusLearnerId) => {
                            self.state.wizard_active_field = ProfileField::LearnerId;
                        }
                        Some(StudyHit::ProfileSetupFocusDisplayName) => {
                            self.state.wizard_active_field = ProfileField::DisplayName;
                        }
                        Some(StudyHit::ProfileDomainSelect(idx)) => {
                            self.state.wizard_domain_idx = idx;
                        }
                        Some(StudyHit::ProfileLevelSelect(idx)) => {
                            self.state.wizard_level_idx = idx;
                        }
                        Some(StudyHit::ProfileLocaleSelect(idx)) => {
                            self.state.wizard_locale_idx = idx;
                        }
                        _ => {}
                    }
                    ctx.request_paint();
                    return;
                }

                // Phase 9: Authoring panel hit testing
                if self.state.show_authoring_panel {
                    match tutor::hit_test_authoring(self.size, &self.state, event.pos) {
                        Some(StudyHit::CloseModal) => {
                            self.state.show_authoring_panel = false;
                        }
                        Some(StudyHit::AuthoringTitleFocus) => {
                            self.state.focus_target = state::StudyFocusTarget::AuthoringTitle;
                        }
                        Some(StudyHit::AuthoringBodyFocus) => {
                            self.state.focus_target = state::StudyFocusTarget::AuthoringBody;
                        }
                        Some(StudyHit::AuthoringNewCurriculum) => {
                            self.state.create_new_curriculum();
                        }
                        Some(StudyHit::AuthoringNewUnit) => {
                            self.state.create_new_unit();
                        }
                        Some(StudyHit::AuthoringNewConcept) => {
                            self.state.create_new_concept();
                        }
                        Some(StudyHit::AuthoringSaveDraft) => {
                            self.state.save_authoring_draft();
                        }
                        _ => {}
                    }
                    ctx.request_paint();
                    return;
                }

                match curriculum::hit_test(&regions, &self.state, event.pos) {
                    Some(StudyHit::Concept(unit, concept)) => {
                        self.state.select_concept(unit, concept);
                    }
                    Some(StudyHit::ReviewQueue) => self.state.open_review_queue(),
                    Some(StudyHit::Stats) => self.state.open_stats(),
                    Some(StudyHit::StartPractice) => self.state.start_practice(),
                    Some(StudyHit::SubmitAnswer) => self.state.submit_answer(),
                    Some(StudyHit::NextProblem) => self.state.next_problem(),
                    Some(StudyHit::RetryAnswer) => self.state.retry_answer(),
                    Some(StudyHit::ReviewConcept) => self.state.stage = Stage::Learn,
                    // Phase 11: Stage pill click cycles stage
                    Some(StudyHit::StageClick) => self.state.cycle_stage(false),
                    Some(StudyHit::RevealHint(level)) => self.state.reveal_hint(level),
                    Some(StudyHit::CloseModal) => self.state.close_modals(),
                    // Phase 2: Search box focus
                    Some(StudyHit::SearchBox) => {
                        self.state.search_focused = true;
                        self.state.focus_target = state::StudyFocusTarget::SearchBox;
                    }
                    // Phase 2: Unit expand/collapse
                    Some(StudyHit::ToggleUnit(unit_idx)) => {
                        self.state.toggle_unit_expand(unit_idx);
                    }
                    // Phase 2: Bookmark toggle
                    Some(StudyHit::BookmarkConcept) => self.state.toggle_bookmark(),
                    // Phase 2: Notes toggle
                    Some(StudyHit::NotesToggle) => self.state.toggle_notes_panel(),
                    Some(StudyHit::NotesInput) => {
                        self.state.search_focused = false;
                        self.state.glossary_search_focused = false;
                        self.state.focus_target = state::StudyFocusTarget::NotesInput;
                    }
                    Some(StudyHit::NotesSave) => self.state.save_note(),
                    // Phase 3: Skip problem
                    Some(StudyHit::SkipProblem) => self.state.skip_problem(),
                    // Phase 3: Pause/resume
                    Some(StudyHit::PauseResume) => self.state.toggle_pause(),
                    // Phase 4: Swipe action (touch review)
                    Some(StudyHit::SwipeAction(action)) => {
                        self.state.apply_spaced_repetition_rating(match action {
                            TouchReviewAction::Again => SpacedRepetitionRating::Again,
                            TouchReviewAction::Hard => SpacedRepetitionRating::Hard,
                            TouchReviewAction::Good => SpacedRepetitionRating::Good,
                            TouchReviewAction::Easy => SpacedRepetitionRating::Easy,
                        });
                    }
                    // Phase 6: Spaced repetition rating
                    Some(StudyHit::RatingButton(rating)) => {
                        self.state.apply_spaced_repetition_rating(rating);
                    }
                    // Phase 7: Tutor chat send
                    Some(StudyHit::TutorChatInput) => {
                        self.state.search_focused = false;
                        self.state.glossary_search_focused = false;
                        self.state.focus_target = state::StudyFocusTarget::TutorChat;
                    }
                    Some(StudyHit::TutorChatSend) => self.state.send_tutor_chat(),
                    // Phase 7: Glossary expand
                    Some(StudyHit::GlossaryExpand(idx)) => {
                        self.state.toggle_glossary_expand(idx);
                    }
                    Some(StudyHit::GlossarySearchToggle) => {
                        self.state.glossary_search_focused = !self.state.glossary_search_focused;
                    }
                    // Phase 8: Visual controls
                    Some(StudyHit::VisualPlayPause) => self.state.toggle_visual_play(),
                    Some(StudyHit::VisualAutoplay) => self.state.toggle_visual_autoplay(),
                    // Phase 9: Authoring
                    Some(StudyHit::AuthoringNewCurriculum) => {
                        self.state.create_new_curriculum();
                    }
                    Some(StudyHit::AuthoringNewUnit) => self.state.create_new_unit(),
                    Some(StudyHit::AuthoringNewConcept) => self.state.create_new_concept(),
                    Some(StudyHit::AuthoringSaveDraft) => self.state.save_authoring_draft(),
                    Some(StudyHit::AuthoringTitleFocus) => {
                        self.state.focus_target = state::StudyFocusTarget::AuthoringTitle;
                    }
                    Some(StudyHit::AuthoringBodyFocus) => {
                        self.state.focus_target = state::StudyFocusTarget::AuthoringBody;
                    }
                    // Phase 4: Hamburger menu row click
                    Some(StudyHit::HamburgerMenuRow(stage)) => {
                        self.state.stage = stage;
                        self.state.show_hamburger_menu = false;
                    }
                    // Phase 10: Shortcut help
                    Some(StudyHit::ShortcutHelp) => self.state.toggle_shortcut_help(),
                    Some(StudyHit::HintLevelKey(level)) => self.state.reveal_hint(level),
                    // Phase 3: Math palette
                    Some(StudyHit::MathPaletteToggle) => self.state.toggle_math_palette(),
                    Some(StudyHit::MathSymbol(idx)) => {
                        let symbols = ["^", "sqrt(", "frac(", "pi", "alpha", "beta", "inf", "sum("];
                        if let Some(symbol) = symbols.get(idx) {
                            self.state.insert_math_symbol(symbol);
                        }
                    }
                    // Phase 4: Hamburger menu
                    Some(StudyHit::HamburgerMenu) => self.state.toggle_hamburger_menu(),
                    // Phase 5: Goal setting
                    Some(StudyHit::GoalSetting) => {
                        self.state.show_goal_modal = !self.state.show_goal_modal;
                    }
                    // Phase 10: High contrast toggle
                    Some(StudyHit::HighContrastToggle) => {
                        self.state.toggle_high_contrast();
                    }
                    Some(StudyHit::AchievementBadge(idx)) => {
                        self.state.expanded_achievement_idx =
                            if self.state.expanded_achievement_idx == Some(idx) {
                                None
                            } else {
                                Some(idx)
                            };
                    }
                    // Phase 8: Visual scrubber
                    Some(StudyHit::VisualScrubber(position)) => {
                        self.state.set_visual_timeline(position);
                    }
                    // Hits handled by other panels (profile setup, etc.)
                    Some(StudyHit::ProfileSetupNext)
                    | Some(StudyHit::ProfileSetupBack)
                    | Some(StudyHit::ProfileSetupFocusLearnerId)
                    | Some(StudyHit::ProfileSetupFocusDisplayName)
                    | Some(StudyHit::ProfileDomainSelect(_))
                    | Some(StudyHit::ProfileLevelSelect(_))
                    | Some(StudyHit::ProfileLocaleSelect(_)) => {}
                    None => {
                        // Click outside search box defocuses it
                        if self.state.search_focused {
                            self.state.search_focused = false;
                            self.state.focus_target = state::StudyFocusTarget::None;
                        }
                    }
                }
            }
            // Phase 4: Pointer Move/Up for swipe recognition
            PointerEvent::Move(event) => {
                if self.state.swipe_start.is_some() {
                    // Tracking swipe in progress
                }
                let _ = event;
            }
            PointerEvent::Up(event) => {
                if let Some((start_pos, start_time)) = self.state.swipe_start.take() {
                    let dx = event.pos.x - start_pos.x;
                    let dy = event.pos.y - start_pos.y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    let elapsed = start_time.elapsed().as_millis() as f64;
                    let speed = distance / elapsed.max(1.0);

                    // Swipe thresholds: min distance 50px, min speed 0.3 px/ms
                    if distance >= 50.0 && speed >= 0.3 && self.state.stage == Stage::Review {
                        let dx_abs = dx.abs();
                        let dy_abs = dy.abs();
                        let actions = &self.state.touch_review.swipe_actions;
                        let action = if dx_abs > dy_abs {
                            // Horizontal swipe: right = first, left = second
                            if dx > 0.0 {
                                actions.first()
                            } else {
                                actions.get(1)
                            }
                        } else {
                            // Vertical swipe: up = third, down = fourth
                            if dy < 0.0 {
                                actions.get(2)
                            } else {
                                actions.get(3)
                            }
                        };
                        if let Some(action) = action {
                            let rating = match action {
                                TouchReviewAction::Again => SpacedRepetitionRating::Again,
                                TouchReviewAction::Hard => SpacedRepetitionRating::Hard,
                                TouchReviewAction::Good => SpacedRepetitionRating::Good,
                                TouchReviewAction::Easy => SpacedRepetitionRating::Easy,
                            };
                            self.state.apply_spaced_repetition_rating(rating);
                        }
                    }
                }
            }
            _ => {}
        }
        ctx.request_paint();
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx<'_>, event: &TextEvent) {
        let TextEvent::Keyboard(event) = event else {
            return;
        };
        if !event.is_pressed {
            return;
        }

        // Phase 1: Profile setup wizard keyboard input
        if self.state.show_profile_setup_modal {
            match &event.logical_key {
                LogicalKey::Named(NamedKey::Enter) => self.state.advance_profile_step(),
                LogicalKey::Named(NamedKey::Escape) => self.state.go_back_profile_step(),
                LogicalKey::Named(NamedKey::Tab) => {
                    self.state.wizard_active_field = match self.state.wizard_active_field {
                        ProfileField::LearnerId => ProfileField::DisplayName,
                        ProfileField::DisplayName => ProfileField::LearnerId,
                    };
                }
                LogicalKey::Named(NamedKey::Backspace) => match self.state.profile_setup_step {
                    ProfileSetupStep::Identity => match self.state.wizard_active_field {
                        ProfileField::LearnerId => {
                            self.state.wizard_learner_id.pop();
                        }
                        ProfileField::DisplayName => {
                            self.state.wizard_display_name.pop();
                        }
                    },
                    ProfileSetupStep::DomainLevel | ProfileSetupStep::Locale => {}
                    ProfileSetupStep::Done => {}
                },
                LogicalKey::Character(ch) => match self.state.profile_setup_step {
                    ProfileSetupStep::Identity => match self.state.wizard_active_field {
                        ProfileField::LearnerId => {
                            self.state.wizard_learner_id.push_str(ch);
                        }
                        ProfileField::DisplayName => {
                            self.state.wizard_display_name.push_str(ch);
                        }
                    },
                    ProfileSetupStep::DomainLevel | ProfileSetupStep::Locale => {}
                    ProfileSetupStep::Done => {}
                },
                _ => return,
            }
            ctx.request_paint();
            return;
        }

        // Phase 9: Authoring panel keyboard input
        if self.state.show_authoring_panel {
            match self.state.focus_target {
                state::StudyFocusTarget::AuthoringTitle => {
                    match &event.logical_key {
                        LogicalKey::Named(NamedKey::Escape) => {
                            self.state.focus_target = state::StudyFocusTarget::None;
                        }
                        LogicalKey::Named(NamedKey::Tab) => {
                            self.state.focus_target = state::StudyFocusTarget::AuthoringBody;
                        }
                        LogicalKey::Named(NamedKey::Backspace) => {
                            self.state.authoring_title.pop();
                        }
                        LogicalKey::Character(ch)
                            if !event.modifiers.control && !event.modifiers.alt =>
                        {
                            self.state.authoring_title.push_str(ch);
                        }
                        _ => return,
                    }
                    ctx.request_paint();
                    return;
                }
                state::StudyFocusTarget::AuthoringBody => {
                    match &event.logical_key {
                        LogicalKey::Named(NamedKey::Escape) => {
                            self.state.focus_target = state::StudyFocusTarget::None;
                        }
                        LogicalKey::Named(NamedKey::Tab) => {
                            self.state.focus_target = state::StudyFocusTarget::AuthoringTitle;
                        }
                        LogicalKey::Named(NamedKey::Backspace) => {
                            self.state.authoring_body.pop();
                        }
                        LogicalKey::Named(NamedKey::Enter) => {
                            self.state.authoring_body.push('\n');
                        }
                        LogicalKey::Character(ch)
                            if !event.modifiers.control && !event.modifiers.alt =>
                        {
                            self.state.authoring_body.push_str(ch);
                        }
                        _ => return,
                    }
                    ctx.request_paint();
                    return;
                }
                _ => {}
            }
        }

        // Phase 2: Route keyboard input to search box when focused
        if self.state.search_focused && self.state.stage != Stage::Practice {
            match &event.logical_key {
                LogicalKey::Named(NamedKey::Escape) => {
                    self.state.search_focused = false;
                    self.state.focus_target = state::StudyFocusTarget::None;
                }
                LogicalKey::Named(NamedKey::Backspace) => {
                    self.state.search_query.pop();
                }
                LogicalKey::Named(NamedKey::Enter) => {
                    // Jump to first search result
                    let matches = self.state.search_matches(&self.state.search_query);
                    if let Some((unit, concept)) = matches.first() {
                        self.state.select_concept(*unit, *concept);
                    }
                }
                LogicalKey::Character(ch) => {
                    self.state.search_query.push_str(ch);
                }
                _ => return,
            }
            ctx.request_paint();
            return;
        }

        // Phase 2: Notes input when notes panel is active
        if self.state.show_notes_panel
            && self.state.focus_target == state::StudyFocusTarget::NotesInput
        {
            match &event.logical_key {
                LogicalKey::Named(NamedKey::Escape) => {
                    self.state.focus_target = state::StudyFocusTarget::None;
                }
                LogicalKey::Named(NamedKey::Backspace) => {
                    self.state.notes_input.pop();
                }
                LogicalKey::Named(NamedKey::Enter) => {
                    self.state.save_note();
                }
                LogicalKey::Character(ch) => {
                    self.state.notes_input.push_str(ch);
                }
                _ => return,
            }
            ctx.request_paint();
            return;
        }

        // Phase 7: Tutor chat input when focused
        if self.state.focus_target == state::StudyFocusTarget::TutorChat {
            match &event.logical_key {
                LogicalKey::Named(NamedKey::Escape) => {
                    self.state.focus_target = state::StudyFocusTarget::None;
                }
                LogicalKey::Named(NamedKey::Backspace) => {
                    self.state.tutor_chat_input.pop();
                }
                LogicalKey::Named(NamedKey::Enter) => {
                    self.state.send_tutor_chat();
                }
                LogicalKey::Character(ch) => {
                    self.state.tutor_chat_input.push_str(ch);
                }
                _ => return,
            }
            ctx.request_paint();
            return;
        }

        // Phase 7: Glossary search input when focused
        if self.state.glossary_search_focused {
            match &event.logical_key {
                LogicalKey::Named(NamedKey::Escape) => {
                    self.state.glossary_search_focused = false;
                }
                LogicalKey::Named(NamedKey::Backspace) => {
                    self.state.glossary_search_query.pop();
                }
                LogicalKey::Character(ch) => {
                    self.state.glossary_search_query.push_str(ch);
                }
                _ => return,
            }
            ctx.request_paint();
            return;
        }

        match &event.logical_key {
            LogicalKey::Named(NamedKey::Escape) => self.state.close_modals(),
            // Phase 10: Space only triggers primary action when NOT in Practice mode
            // (to allow space in answer text)
            LogicalKey::Named(NamedKey::Enter) => {
                self.state.activate_primary_keyboard_action();
            }
            LogicalKey::Named(NamedKey::Space) => {
                if self.state.stage == Stage::Practice {
                    // Phase 10: Space inserts space in practice input
                    self.state.insert_char_at_cursor(" ");
                } else {
                    self.state.activate_primary_keyboard_action();
                }
            }
            LogicalKey::Named(NamedKey::Tab) => self.state.cycle_stage(event.modifiers.shift),
            LogicalKey::Named(NamedKey::ArrowUp) => {
                if self.state.stage == Stage::Practice {
                    self.state.move_cursor(-1);
                } else {
                    self.state.move_concept(-1);
                }
            }
            LogicalKey::Named(NamedKey::ArrowDown) => {
                if self.state.stage == Stage::Practice {
                    self.state.move_cursor(1);
                } else {
                    self.state.move_concept(1);
                }
            }
            // Phase 3: Arrow left/right for cursor movement in Practice mode
            LogicalKey::Named(NamedKey::ArrowLeft) => {
                if self.state.stage == Stage::Practice {
                    self.state.move_cursor(-1);
                } else {
                    self.state.stage = Stage::Learn;
                    self.state.feedback = None;
                }
            }
            LogicalKey::Named(NamedKey::ArrowRight) => {
                if self.state.stage == Stage::Practice {
                    self.state.move_cursor(1);
                } else {
                    self.state.activate_primary_keyboard_action();
                }
            }
            // Phase 3: Home/End for cursor movement
            LogicalKey::Named(NamedKey::Home) => {
                if self.state.stage == Stage::Practice {
                    self.state.move_cursor_home();
                }
            }
            LogicalKey::Named(NamedKey::End) => {
                if self.state.stage == Stage::Practice {
                    self.state.move_cursor_end();
                }
            }
            // Phase 10: s/r only work with Ctrl modifier to avoid conflicts
            LogicalKey::Character(ch)
                if event.modifiers.control && ch.eq_ignore_ascii_case("s") =>
            {
                self.state.open_stats();
            }
            LogicalKey::Character(ch)
                if event.modifiers.control && ch.eq_ignore_ascii_case("r") =>
            {
                self.state.open_review_queue();
            }
            // Phase 10: ? for shortcut help
            LogicalKey::Character(ch) if ch == "?" => {
                self.state.toggle_shortcut_help();
            }
            // Phase 9: Ctrl+Shift+A toggles authoring panel
            LogicalKey::Character(ch)
                if event.modifiers.control
                    && event.modifiers.shift
                    && ch.eq_ignore_ascii_case("a") =>
            {
                self.state.toggle_authoring_panel();
            }
            // Phase 10: Number keys 1/2/3 for hint reveal
            LogicalKey::Character(ch) if ch == "1" => {
                self.state.reveal_hint(1);
            }
            LogicalKey::Character(ch) if ch == "2" => {
                self.state.reveal_hint(2);
            }
            LogicalKey::Character(ch) if ch == "3" => {
                self.state.reveal_hint(3);
            }
            // Phase 10: Ctrl+S without Practice guard for stats
            LogicalKey::Character(ch)
                if self.state.stage != Stage::Practice && ch.eq_ignore_ascii_case("s") =>
            {
                self.state.open_stats();
            }
            LogicalKey::Character(ch)
                if self.state.stage != Stage::Practice && ch.eq_ignore_ascii_case("r") =>
            {
                self.state.open_review_queue();
            }
            // Phase 3: Backspace at cursor position
            LogicalKey::Named(NamedKey::Backspace) => {
                if self.state.stage == Stage::Practice {
                    self.state.backspace_at_cursor();
                }
            }
            // Phase 3: Delete at cursor position
            LogicalKey::Named(NamedKey::Delete) => {
                if self.state.stage == Stage::Practice {
                    self.state.delete_at_cursor();
                }
            }
            LogicalKey::Character(ch) if self.state.stage == Stage::Practice => {
                self.state.insert_char_at_cursor(ch);
            }
            _ => return,
        }
        ctx.request_paint();
    }

    fn accessibility_tree(&self, state: &WidgetState) -> AccessibilityNode {
        AccessibilityNode {
            role: AccessRole::Window,
            label: Some("Tench Study".to_string()),
            value: Some(self.state.stage.label().to_string()),
            focused: state.is_focused,
            disabled: state.is_disabled,
            children: Vec::new(),
        }
    }

    fn automation_children(&self, state: &WidgetState) -> Vec<UiAutomationNode> {
        automation::study_automation_nodes(&self.state, state.size, state.id.to_raw(), &self.i18n)
    }

    fn debug_id(&self) -> Option<&str> {
        Some("study.root")
    }
}
