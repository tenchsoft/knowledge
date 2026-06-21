# Study Implement Plan Audit — Full Results

## Summary

| Status | Count |
|--------|-------|
| IMPLEMENTED | 106 |
| INCOMPLETE | 12 |
| MISSING | 0 |

## INCOMPLETE (12 plans)

### 1. achievement-badge-control
**What's incomplete:** Badges are painted in the goal modal but are not interactive.
- `StudyHit::AchievementBadge(idx)` is defined but **never produced by any hit_test** (`curriculum.rs:515-779`)
- Widget handler is a **no-op**: `widget.rs:267` — `Some(StudyHit::AchievementBadge(_)) => {}`
- `expanded_achievement_idx` field **does not exist** in `StudyState` — only mentioned in the plan itself
- No expanded detail view rendered in `tutor/goals.rs:86-130`
- No `study.achievement.{idx}` automation nodes registered (`automation.rs:577-594` only registers dialog + close)

### 2. visual-scrubber-control
**What's incomplete:** Scrubber bar is painted but has zero interactivity.
- Painted at `learn.rs:143-158` but rect is a local variable, never exposed
- **No hit_test** for scrubber in `curriculum.rs:717-737` (only play/autoplay tested)
- Widget handler is a **no-op**: `widget.rs:268-269` — `Some(StudyHit::VisualScrubber(_)) => {}`
- `set_visual_timeline()` exists at `state/visual.rs:12-13` but is **never called** from anywhere
- No `PointerEvent::Move` drag handling for scrubber (`widget.rs:288-292` only handles swipe)
- No `study.visual.scrubber` automation node registered

### 3. mobile-hamburger-menu-button
**What's incomplete:** Button hit region exists but is invisible; no automation node.
- Hit region at `curriculum.rs:653-658` — `Rect::new(4.0, 4.0, 40.0, 36.0)` — works for clicks
- **No visual rendering** of hamburger icon in `paint_shell` (`curriculum.rs:39-225`) — invisible touch target
- Toggle works: `widget.rs:258` → `state/accessibility.rs:12-14`
- **No `study.hamburger.menu` automation node** in `automation.rs`

### 4. hamburger-learn-menu-row
**What's incomplete:** Row is painted but not clickable.
- Painted at `curriculum.rs:869-894` with active highlighting
- **No `StudyHit` variant** for hamburger menu row selection (only `HamburgerMenu` toggle exists)
- **No hit_test** for menu rows — rect computed in paint but never used for hit detection
- **No click handler** to switch stage from hamburger menu
- **No `study.hamburger.learn` automation node**

### 5. hamburger-practice-menu-row
**What's incomplete:** Same as hamburger-learn-menu-row.
- Painted at `curriculum.rs:869-894`
- No `StudyHit` variant, no hit_test, no click handler, no automation node

### 6. hamburger-review-menu-row
**What's incomplete:** Same as hamburger-learn-menu-row.
- Painted at `curriculum.rs:869-894`
- No `StudyHit` variant, no hit_test, no click handler, no automation node

### 7. practice-answer-input-field
**What's incomplete:** Input is painted and keyboard works, but no click-to-focus, no automation node, number key conflict.
- Painted at `practice.rs:61-89` — input rect is a local variable
- **No `StudyHit` variant** for answer input focus (no `AnswerInputFocus` or `PracticeInputFocus`)
- **No click-to-focus** — clicking the input field produces no hit
- **No `study.practice.answer` automation node** in `automation.rs`
- **Number key conflict**: keys `1`/`2`/`3` trigger hint reveal (`widget.rs:589-596`) before the Practice character handler (`widget.rs:621`), making it impossible to type those digits in the answer field
- Keyboard input routes unconditionally by stage (no `StudyFocusTarget::PracticeInput`)

### 8. review-swipe-good-action
**What's incomplete:** Swipe gesture for Good rating is unreachable.
- `TouchReviewAction::Good` exists at `types.rs:190`
- Default `swipe_actions[2] = Good` at `defaults.rs:65`
- But swipe detection at `widget.rs:306-322` only maps 2 directions: right→`[0]`(Again), left→`[1]`(Hard)
- **`swipe_actions[2]` is never read** by any code path
- Desktop button path works (RatingButton(Good) at `curriculum.rs:635`)

### 9. review-swipe-easy-action
**What's incomplete:** Same issue as review-swipe-good-action.
- `swipe_actions[3] = Easy` is never reached by swipe gesture
- Desktop button path works

### 10. automatic-modal-priority-behavior
**What's incomplete:** Priority ordering works via early returns, but mutual exclusion is not enforced.
- Paint order is correct: profile wizard > authoring > workspace > modals (`widget.rs:17-89`)
- Keyboard/pointer routing is correct with early returns
- **But**: `open_stats()` (`learning.rs:281-283`) does NOT clear other modal flags
- `toggle_shortcut_help()` and `toggle_hamburger_menu()` only toggle their own flag
- `close_modals()` (`learning.rs:289-297`) does NOT clear `show_hamburger_menu`, `show_authoring_panel`, or `show_profile_setup_modal`
- Multiple modal flags can be `true` simultaneously — no `Option<ModalType>` or similar

### 11. study-format-roundtrip
**What's incomplete:** Snapshot creation and restore work, but persistence is missing and no roundtrip test exists.
- `auto_save_session()` at `session.rs:4-16` creates a `SessionSnapshot` with 9 fields ✓
- `restore_session()` at `session.rs:18-29` restores all fields + resets feedback ✓
- **Snapshot is discarded**: `widget.rs:26` — `let _snapshot = self.state.auto_save_session();`
- **No persistence** to storage — comment says "In production, snapshot would be persisted"
- **No roundtrip test** — no test verifies `save → restore` field equality
- **No test plan doc** at `plans/test/study/study-format-roundtrip.md`

### 12. automatic-session-auto-save-behavior
**What's incomplete:** Timer and snapshot creation work, but the snapshot is never persisted.
- Timer increments `elapsed_seconds` every tick during Practice/Review (`widget.rs:22`)
- Every 30 seconds, creates a `SessionSnapshot` (`widget.rs:24-27`)
- **Snapshot is immediately dropped** — `let _snapshot = ...`
- No Tauri command invocation to persist the snapshot to disk

---

## IMPLEMENTED (106 plans)

### Buttons (42 implemented)

| Plan | Evidence |
|------|----------|
| next-problem-button | `practice.rs:225-241`, `curriculum.rs:609`, `automation.rs:332` |
| start-practice-button | `learn.rs:181-191`, `curriculum.rs:600`, `automation.rs:173` |
| submit-answer-button | `practice.rs:91-101`, `curriculum.rs:606`, `automation.rs:250` |
| retry-answer-button | `practice.rs:225-241`, `curriculum.rs:612`, `automation.rs:316` |
| skip-problem-button | `practice.rs:129-140`, `curriculum.rs:620`, `automation.rs:258` |
| pause-resume-button | `practice.rs:142-158`, `curriculum.rs:624`, `automation.rs:266` |
| review-queue-button | `curriculum.rs:477-513`, `curriculum.rs:529`, `automation.rs:114` |
| bookmark-concept-toggle-button | `curriculum.rs:309-334`, `curriculum.rs:556`, `automation.rs:88` |
| notes-toggle-button | `curriculum.rs:337-355`, `curriculum.rs:567`, `automation.rs:101` |
| notes-save-button | `learn.rs:461-476`, `automation.rs:224` |
| header-stats-button | `curriculum.rs:171-180`, `curriculum.rs:526`, `automation.rs:25` |
| header-stage-pill-button | `curriculum.rs:128-143`, `curriculum.rs:534`, `automation.rs:33` |
| header-shortcut-help-button | `curriculum.rs:747`, `automation.rs:41` |
| header-high-contrast-toggle-button | `curriculum.rs:107-126`, `curriculum.rs:545`, `automation.rs:54` |
| header-goal-setting-button | `curriculum.rs:669`, `automation.rs:67` |
| learn-review-concept-button | `learn.rs:193-204`, `curriculum.rs:603`, `automation.rs:325` |
| review-concept-feedback-button | `practice.rs:225-241`, `automation.rs:325` |
| again-spaced-repetition-rating-button | `practice.rs:245-326`, `curriculum.rs:635`, `automation.rs:349` |
| hard-spaced-repetition-rating-button | `practice.rs:245-326`, `curriculum.rs:635`, `automation.rs:349` |
| good-spaced-repetition-rating-button | `practice.rs:245-326`, `curriculum.rs:635`, `automation.rs:349` |
| easy-spaced-repetition-rating-button | `practice.rs:245-326`, `curriculum.rs:635`, `automation.rs:349` |
| hint-level-1-button | `tutor/panel.rs:83-106`, `curriculum.rs:641`, `automation.rs:363` |
| hint-level-2-button | `tutor/panel.rs:83-106`, `curriculum.rs:641`, `automation.rs:363` |
| hint-level-3-button | `tutor/panel.rs:83-106`, `curriculum.rs:641`, `automation.rs:363` |
| math-palette-toggle-button | `practice.rs:160-180`, `curriculum.rs:758`, `automation.rs:274` |
| power-math-symbol-button | `practice.rs:104-127`, `automation.rs:284` |
| square-root-math-symbol-button | `practice.rs:104-127`, `automation.rs:285` |
| fraction-math-symbol-button | `practice.rs:104-127`, `automation.rs:286` |
| pi-math-symbol-button | `practice.rs:104-127`, `automation.rs:287` |
| alpha-math-symbol-button | `practice.rs:104-127`, `automation.rs:288` |
| beta-math-symbol-button | `practice.rs:104-127`, `automation.rs:289` |
| infinity-math-symbol-button | `practice.rs:104-127`, `automation.rs:290` |
| sum-math-symbol-button | `practice.rs:104-127`, `automation.rs:291` |
| visual-play-pause-button | `learn.rs:127-141`, `curriculum.rs:726`, `automation.rs:181` |
| visual-autoplay-toggle-button | `learn.rs:161-179`, `curriculum.rs:735`, `automation.rs:194` |
| tutor-chat-send-button | `tutor/panel.rs:284-318`, `curriculum.rs:690`, `automation.rs:385` |
| authoring-panel-close-button | `tutor/authoring.rs:37-47`, `automation.rs:439` |
| new-unit-authoring-button | `tutor/authoring.rs:121-138`, `automation.rs:473` |
| new-concept-authoring-button | `tutor/authoring.rs:140-157`, `automation.rs:486` |
| new-curriculum-authoring-button | `tutor/authoring.rs:159-175`, `automation.rs:499` |
| save-draft-authoring-button | `tutor/authoring.rs:177-192`, `automation.rs:512` |
| profile-back-button | `tutor/profile.rs:307`, `automation.rs:720` |
| profile-next-button | `tutor/profile.rs:295`, `automation.rs:706` |
| profile-start-button | `tutor/profile.rs:295`, `automation.rs:706` (same as profile-next) |
| stats-modal-close-button | `automation.rs:537` |
| result-modal-close-button | `automation.rs:555` |
| shortcut-help-modal-close-button | `automation.rs:573` |
| goal-modal-close-button | `automation.rs:591` |

### Fields (6 implemented)

| Plan | Evidence |
|------|----------|
| authoring-title-field | `tutor/authoring.rs:49-83`, `automation.rs:447`, keyboard at `widget.rs:381-426` |
| authoring-body-field | `tutor/authoring.rs:85-119`, `automation.rs:460`, keyboard at `widget.rs:381-426` |
| notes-input-field | `learn.rs:425-458`, `automation.rs:216`, keyboard at `widget.rs:455-475` |
| tutor-chat-input-field | `tutor/panel.rs:284-318`, `automation.rs:372`, keyboard at `widget.rs:478-496` |
| profile-learner-id-field | `tutor/profile.rs:320`, `automation.rs:623`, keyboard at `widget.rs:340-378` |
| profile-display-name-field | `tutor/profile.rs:329`, `automation.rs:635`, keyboard at `widget.rs:340-378` |
| curriculum-search-box | `curriculum.rs:244-291`, `automation.rs:80`, keyboard at `widget.rs:429-452` |
| glossary-search-field | `tutor/panel.rs:142-262`, `automation.rs:398`, keyboard at `widget.rs:499-513` |

### Shortcuts (15 implemented)

| Plan | Evidence |
|------|----------|
| enter-primary-action-shortcut | `widget.rs:520-522` |
| tab-cycle-stage-shortcut | `widget.rs:531` |
| space-primary-or-text-shortcut | `widget.rs:523-529` |
| backspace-practice-input-shortcut | `widget.rs:610` |
| delete-practice-input-shortcut | `widget.rs:616` |
| home-end-practice-cursor-shortcut | `widget.rs:563-571` |
| arrow-left-right-practice-cursor-shortcut | `widget.rs:547-560` |
| arrow-concept-navigation-shortcut | `widget.rs:532-544` |
| control-s-open-stats-shortcut | `widget.rs:574-577` |
| control-r-open-review-queue-shortcut | `widget.rs:579-582` |
| question-mark-shortcut-help-key | `widget.rs:585-587` |
| number-hint-reveal-shortcut | `widget.rs:589-596` |
| profile-wizard-escape-key-control | `widget.rs:343` |
| profile-wizard-enter-key-control | `widget.rs:342` |
| profile-wizard-tab-focus-control | `widget.rs:344` |
| practice-character-input-control | `widget.rs:621-623` |

### Behaviors (24 implemented)

| Plan | Evidence |
|------|----------|
| automatic-daily-dashboard-header-behavior | `curriculum.rs:183-224` |
| automatic-active-concept-highlight-behavior | `curriculum.rs:421-431` |
| automatic-search-cursor-behavior | `curriculum.rs:262-289` |
| automatic-search-match-count-behavior | `curriculum.rs:294-306` |
| automatic-bookmark-indicator-behavior | `curriculum.rs:309-334, 449-460` |
| automatic-outline-virtual-scroll-behavior | `curriculum.rs:358-416` |
| automatic-concept-progress-display-behavior | `curriculum.rs:391-407` |
| automatic-math-palette-render-behavior | `practice.rs:104-127` |
| automatic-practice-feedback-behavior | `practice.rs:182-242` |
| automatic-review-card-render-behavior | `practice.rs:270-325` |
| automatic-visual-timeline-fill-behavior | `learn.rs:143-158` |
| automatic-learn-visual-surface-behavior | `learn.rs:221-254` |
| automatic-header-breadcrumb-behavior | `curriculum.rs:67-104` |
| automatic-high-contrast-styling-behavior | `curriculum.rs:107-126` |
| automatic-session-timer-behavior | `curriculum.rs:155-170` |
| automatic-tutor-weak-points-behavior | `tutor/panel.rs:108-140` |
| automatic-tutor-chat-message-render-behavior | `tutor/panel.rs:320-337` |
| automatic-glossary-filter-behavior | `tutor/panel.rs:191-197` |
| automatic-notes-panel-overlay-behavior | `learn.rs:394-509` |
| automatic-offline-asset-status-behavior | `dashboard.rs:72-75`, `builders.rs:169-189` |
| automatic-accessibility-label-coverage-behavior | `builders.rs:231-262`, `dashboard.rs:77-100` |
| automatic-achievement-unlock-behavior | `progress.rs:24-44` |
| automatic-responsive-study-region-layout-behavior | `curriculum.rs:22-37`, `dashboard.rs:6-16` |
| automatic-focus-indicator-behavior | `widget.rs:80-88` (rendered via focus_target; focus_indicator field is dead code but behavior works) |

### Controls (5 implemented)

| Plan | Evidence |
|------|----------|
| concept-row-selection-control | `curriculum.rs:594`, `automation.rs:160` |
| existing-note-row-control | `learn.rs:483-508`, `automation.rs:238` |
| glossary-search-toggle-control | `curriculum.rs:713`, `automation.rs:398` |
| glossary-term-expand-row | `tutor/panel.rs:142-262`, `automation.rs:413` |
| unit-expand-collapse-row | `curriculum.rs:363-411`, `automation.rs:136` |

### Rows (3 implemented)

| Plan | Evidence |
|------|----------|
| profile-domain-option-row | `tutor/profile.rs:340`, `automation.rs:652` |
| profile-level-option-row | `tutor/profile.rs:349`, `automation.rs:668` |
| profile-locale-option-row | `tutor/profile.rs:359`, `automation.rs:685` |

### Actions (1 implemented)

| Plan | Evidence |
|------|----------|
| review-swipe-again-action | `widget.rs:294-324` (swipe right → swipe_actions[0] = Again) |

### Algorithm (1 implemented)

| Plan | Evidence |
|------|----------|
| spaced-repetition-scheduling | `progress.rs:46-93` (SM-2 algorithm) |

---

## Additional Issues Found

1. **Test assertion bug**: `state/tests.rs:162` asserts `swipe_actions.len() == 3` but default has 4 entries
2. **Dead code**: `StudyHit::SwipeAction` match arm at `widget.rs:203-211` is unreachable
3. **Dead code**: `focus_indicator: Option<Rect>` field and `update_focus_indicator()` method are never used
4. **Dead code**: `StudyHit::AchievementBadge` and `StudyHit::VisualScrubber` are no-op placeholders
5. **Number key conflict**: Keys 1/2/3 trigger hint reveal during Practice mode, preventing digit entry in answer field
