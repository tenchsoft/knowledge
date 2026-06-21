# Design: study-learn-area

## 한 줄 정의
중앙 학습 영역에 활성 개념의 제목, 요약, 정의 카드, 예제 카드, 퀵 체크, 비주얼 표면, Start Practice 버튼을 표시한다.

## 시각적 레이아웃
```
┌─ Surface (center, variable width) ───────────────────────────────┐
│                                                                    │
│  Concept Title                                                     │
│  Summary text                                                      │
│                                                                    │
│  ┌─ Definition card ──────────────────────────────────────────┐   │
│  │ ▎ definition label                                         │   │
│  │ ▎ N visuals available                                      │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  ┌─ Example card ─────────────────────────────────────────────┐   │
│  │  example label                                              │   │
│  │  concept example_prefix unit example_suffix                 │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  ┌─ Quick check ──────────────────────────────────────────────┐   │
│  │  quick_check label                                          │   │
│  │  quick_question text                                        │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                    │
│  ┌─ Visual surface ───────────────────────────────────────────┐   │
│  │  (LearningVisual rendered via VisualSurface)                │   │
│  └────────────────────────────────────────────────────────────┘   │
│  [▶/‖] [═══════════════●═══════════════] [Auto: OFF]              │
│                                                                    │
│  [Start Practice]  [Review Concept]                                │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Start Practice button | `Button` | `study.learn.start_practice` |
| Visual play/pause | `Button` | `study.visual.play_pause` |
| Visual autoplay | `Button` | `study.visual.autoplay` |
| Review Concept button | `Button` | (outline button, no explicit debug_id) |

## Visual properties

| 속성 | 값 |
|------|----|
| Surface background | `NEUTRAL_800` |
| Concept title | `NEUTRAL_100`, 18px, `FontWeight::BOLD` |
| Summary text | `NEUTRAL_100`, 16px |
| Definition card bg | `NEUTRAL_700`, border_radius=4 |
| Definition left accent bar | `ACCENT_STUDY`, 3px |
| Definition label | `ACCENT_STUDY`, 11px, `FontWeight::BOLD` |
| Visuals count text | `NEUTRAL_100`, 13px |
| Example card bg | `NEUTRAL_700`, border_radius=6 |
| Example label | `NEUTRAL_300`, 11px, `FontWeight::BOLD` |
| Example text | `NEUTRAL_100`, 13px |
| Quick check card bg | `NEUTRAL_700`, border_radius=8 |
| Quick check border | `NEUTRAL_600`, 1px |
| Quick check label | `STATUS_RUNNING`, 11px, `FontWeight::BOLD` |
| Quick check question | `NEUTRAL_100`, 13px, `FontWeight::BOLD` |
| Start Practice button | `ACCENT_STUDY` fill, border_radius=6, `NEUTRAL_800` text, 13px, `FontWeight::BOLD` |
| Review Concept button | `NEUTRAL_500` stroke, border_radius=6, `NEUTRAL_100` text |

## States

| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Start Practice | `ACCENT_STUDY` fill | darken 8% | darken 16% | 2px outline `ACCENT_STUDY` | opacity 0.5 |
| Review Concept | `NEUTRAL_500` stroke | lighten 8% | lighten 16% | 2px outline | opacity 0.5 |

## Responsive 변형
- **Desktop**: 전체 레이아웃 표시.
- **Tablet**: 비주얼 표면 높이 축소 가능.
- **Mobile**: 카드 세로 스크롤, 비주얼 표면 최소화.

## Accessibility
- Start Practice 버튼에 i18n 라벨 `t!("study.learn.start_practice")`.
- Visual surface에 alt text 제공 (`plan.accessibility.alt_text`).
- Space/Enter로 Start Practice 실행.

## Design tokens — 사용 / 제안
- **사용**: `NEUTRAL_800`, `NEUTRAL_700`, `NEUTRAL_600`, `NEUTRAL_500`, `NEUTRAL_100`, `ACCENT_STUDY`, `STATUS_RUNNING`.
- **신규 제안**: 없음.

## Out of scope
- 비주얼 타임라인 컨트롤 (별도 design `study-visual`).
- 노트 패널 오버레이 (별도 design `study-notes`).
