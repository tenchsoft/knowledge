# Design: study-automatic-ui

## 한 줄 정의
사용자 직접 조작 없이 자동으로 렌더링/갱신되는 UI 요소들의 시각적 계약. 개별 automatic behavior spec들의 UI 표현을 통합 정의.

## 자동 UI 요소 목록

### 1. Accessibility labels
- 7개 대상(Header, Curriculum, LearnSurface, PracticeSurface, ReviewSurface, TutorPanel, StatsModal)에 i18n 라벨 자동 부여.
- `StudyAccessibilityLabel` 구조체가 `label_key` 보유, `StudyState::accessibility_labels`에서 관리.

### 2. Achievement unlock
- Goal modal 내 성취 섹션에서 자동 표시.
- 잠금: ☆ `NEUTRAL_500`, 14px. 해금: ★ `STATUS_WARNING`, 14px + `NEUTRAL_100` label.
- `StudyState::check_achievements()`가 세션 결과 기반으로 자동 갱신.

### 3. Active concept highlight
- 커리큘럼 패널에서 활성 개념에 `NEUTRAL_500` 배경 + `ACCENT_STUDY` 2px 좌측 바 자동 표시.

### 4. Bookmark indicator
- 북마크된 개념에 ★ `STATUS_WARNING`, 10px 아이콘이 커리큘럼 행 우측에 자동 표시.
- `StudyState::bookmarked_concept_ids` BTreeSet 기반.

### 5. Concept progress
- 각 단원 헤더에 `completed/total` 텍스트 자동 표시 (`NEUTRAL_400`, 10px).
- `ConceptStatus::Completed` 카운트 기반.

### 6. Daily dashboard
- 헤더에 due/new/accuracy 미니 위젯 자동 표시 (≥700px).
- `StudyState::refresh_daily_dashboard()`가 세션 이벤트 시 자동 갱신.

### 7. Focus indicator
- `StudyState::focus_indicator: Option<Rect>` 설정 시 `ACCENT_STUDY` 2px 둥근 사각형 자동 렌더.

### 8. Glossary filter
- `glossary_search_query` 비어있지 않으면 glossary term이 필터링되어 자동으로 일치 항목만 표시.
- 대소문자 무시, term+definition 모두 검색.

### 9. Header breadcrumb
- viewport ≥560px 시 `subject / unit / concept` 자동 표시.
- 개념 선택 변경 시 즉시 갱신.

### 10. High contrast
- `high_contrast_mode` 토글 시 HC 버튼이 "hc"→"HC", `NEUTRAL_400`→`ACCENT_STUDY`로 자동 전환.

### 11. Learn visual surface
- `active_visual_draw_plan()` 결과에 따라 VisualSurface가 자동 렌더.
- hint_level, timeline_position 파라미터 기반.

### 12. Math palette
- `show_math_palette` true 시 8개 기호 버튼 그리드 자동 렌더 (4×2).
- `NEUTRAL_600` fill, `NEUTRAL_500` stroke, `NEUTRAL_100` text.

### 13. Modal priority
- Profile wizard > Authoring panel > Notes panel > Result/Stats/Shortcut/Goal modals 순서.
- 동시에 하나만 표시 (가장 위에 있는 것).

### 14. Notes overlay
- `show_notes_panel && stage == Learn` 시 surface 우측에 오버레이 자동 표시.

### 15. Offline asset status
- `OfflineAssetState::cache_ready`가 daily dashboard에 `offline_ready`로 반영.

### 16. Outline virtual scroll
- `outline_scroll_offset` 기반으로 visible range만 렌더.
- 화면 밖 항목은 자동화 노드에서 제외.

### 17. Practice feedback
- `feedback: Some(bool)` 시 정답/오답 피드백 카드 자동 렌더.
- 정답: `STATUS_READY` border. 오답: `STATUS_ERROR` border.
- cause_tag 자동 표시.

### 18. Responsive layout
- `StudyViewportClass` (Mobile/Tablet/Desktop)에 따라 regions 자동 계산.
- Mobile: tutor panel 숨김, outline 축소, hamburger menu 표시.

### 19. Review card render
- `review_queue[review_index-1]` 데이터로 카드 자동 렌더.
- wrong_answer, correct_answer, solution, cause_tag 표시.

### 20. Search cursor/match count
- `search_focused` 시 커서 바 자동 렌더.
- `search_query` 비어있지 않으면 일치 수 `ACCENT_STUDY`로 자동 표시.

### 21. Session auto-save
- `auto_save_session()`이 `SessionSnapshot` 생성. UI 표면 없음 (순수 백그라운드).

### 22. Session timer
- `elapsed_seconds` 기반으로 HH:MM:SS 포맷 자동 표시 (`NEUTRAL_400`, 11px).

### 23. Tutor chat
- 최근 3개 메시지 자동 렌더. User: `ACCENT_STUDY`, Assistant: `NEUTRAL_300`.

### 24. Tutor weak points
- `weak_points()` 결과(최대 5개 cause_tag) 자동 표시. `STATUS_WARNING`, 12px.

### 25. Visual timeline fill
- `visual_timeline_position`에 비례하여 scrubber filled 영역 자동 렌더. `ACCENT_STUDY` fill.

## Design tokens — 사용
모든 자동 UI 요소는 기존 토큰만 사용: `NEUTRAL_*`, `ACCENT_STUDY`, `STATUS_*`.

## Out of scope
- 각 자동 UI의 백그라운드 동작은 별도 background 문서에 정의.
