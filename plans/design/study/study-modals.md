# Design: study-modals

## 한 줄 정의
Stats modal, Goal modal, Result modal, Shortcut help modal, Profile wizard를 포함한 모든 모달 다이얼로그의 시각적 계약.

## 시각적 레이아웃

### Stats Modal
```
┌─ Modal (centered, 420×260 max) ────────────────────┐
│  Stats                                        [×]   │
│  Total sessions: 1                                  │
│  Pending reviews: N                                 │
│  Streak: N                                          │
│  Built-in curricula: 4                              │
│  Lessons/Visuals: N / M                             │
│  Glossary: N                                        │
│  Streak calendar: [■][■][□][□]... (14 days)         │
└─────────────────────────────────────────────────────┘
```

### Result Modal
```
┌─ Modal (centered, 420×260 max) ────────────────────┐
│  Session Result                               [×]   │
│  Accuracy: 80%                                      │
│  Solved: 5                                          │
│  Wrong: 1                                           │
└─────────────────────────────────────────────────────┘
```

### Shortcut Help Modal
```
┌─ Modal (centered, 420×260 max) ────────────────────┐
│  Shortcuts                                    [×]   │
│  Enter    Start or Submit                           │
│  Tab      Cycle Stage                               │
│  ↑/↓      Navigate Concepts                        │
│  ?        Help                                      │
│  1/2/3    Hint Levels                               │
│  Ctrl+S   Stats                                     │
│  Ctrl+R   Review                                    │
│  Space    Input (Practice) / Action                 │
└─────────────────────────────────────────────────────┘
```

### Goal Modal
```
┌─ Modal (centered, 420×260 max) ────────────────────┐
│  Goals                                        [×]   │
│  Daily Problems                                     │
│  [████████░░░░░░░░] 5 / 10 problems                 │
│  Daily Minutes                                      │
│  [██░░░░░░░░░░░░░░] 10 / 30 minutes                 │
│  Daily Accuracy                                     │
│  [████████░░░░░░░░] 60 / 80 %                       │
│  Achievements                                       │
│  ☆ First Session    ☆ Streak 10    ☆ Problems 100   │
└─────────────────────────────────────────────────────┘
```

### Profile Wizard
```
┌─ Modal (centered, 420×260 max) ────────────────────┐
│  Step 1: Identity / Step 2: Domain / Step 3: Locale │
│  [Learner ID input]                                 │
│  [Display Name input]                               │
│                          [Back] [Next/Start]        │
└─────────────────────────────────────────────────────┘
```

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Stats modal | `Dialog` | `study.modal.stats` |
| Result modal | `Dialog` | `study.modal.result` |
| Shortcut modal | `Dialog` | `study.modal.shortcuts` |
| Goal modal | `Dialog` | `study.modal.goal` |
| Profile wizard | `Dialog` | `study.profile` |
| Close button (shared) | `Button` | `study.modal.close` |
| Profile learner ID | `TextInput` | `study.profile.learner_id` |
| Profile display name | `TextInput` | `study.profile.display_name` |
| Profile domain option | `Button` | `study.profile.domain.{idx}` |
| Profile level option | `Button` | `study.profile.level.{stable_id}` |
| Profile locale option | `Button` | `study.profile.locale.{locale}` |
| Profile next | `Button` | `study.profile.next` |
| Profile back | `Button` | `study.profile.back` |

## Visual properties

| 속성 | 값 |
|------|----|
| Modal overlay | `rgba(0, 0, 0, 140)` |
| Modal background | `NEUTRAL_700` (stats/result) or `NEUTRAL_800` (wizard/shortcut/goal) |
| Modal border | `NEUTRAL_500`, 1px, border_radius=8 |
| Modal size | `min(420, viewport-48)` × `min(260, viewport-48)` |
| Title text | `NEUTRAL_100`, 18px, `FontWeight::BOLD` |
| Close button border | `NEUTRAL_500`, 1px, border_radius=6 |
| Close button text | `NEUTRAL_300`, 14px, `FontWeight::BOLD` |
| Body text | `NEUTRAL_100`, 13px |
| Streak text | `STATUS_WARNING`, 13px, `FontWeight::BOLD` |
| Accuracy text | `ACCENT_STUDY`, 16px, `FontWeight::BOLD` |
| Streak calendar cell | `STATUS_WARNING` (active) / `NEUTRAL_600` (inactive), border_radius=2, 14×14px |
| Goal progress bar bg | `NEUTRAL_700`, border_radius=4 |
| Goal progress bar fill | `ACCENT_STUDY`, border_radius=4 |
| Goal progress text | `NEUTRAL_400`, 11px |
| Achievement unlocked icon | ★ `STATUS_WARNING`, 14px |
| Achievement locked icon | ☆ `NEUTRAL_500`, 14px |
| Shortcut key text | `ACCENT_STUDY`, 12px, `FontWeight::BOLD` |
| Shortcut label text | `NEUTRAL_300`, 12px |
| Profile input bg | `NEUTRAL_700`, border_radius=6 |
| Profile input border (focused) | `ACCENT_STUDY`, 1px |
| Profile input border (unfocused) | `NEUTRAL_500`, 1px |
| Profile selected option bg | `NEUTRAL_600` + `ACCENT_STUDY` stroke |
| Profile next button | `ACCENT_STUDY` fill, `NEUTRAL_900` text |
| Profile back button | `NEUTRAL_500` stroke, `NEUTRAL_300` text |

## States

| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Close button | `NEUTRAL_500` stroke | lighten 8% | lighten 16% | 2px outline | — |
| Profile option (selected) | `NEUTRAL_600` bg + `ACCENT_STUDY` stroke | — | — | 2px outline | — |
| Profile option (unselected) | `NEUTRAL_700` bg | lighten 8% | lighten 16% | 2px outline | — |
| Profile next | `ACCENT_STUDY` fill | darken 8% | darken 16% | 2px outline | opacity 0.5 |
| Profile back | `NEUTRAL_500` stroke | lighten 8% | lighten 16% | 2px outline | opacity 0.5 |

## Animations / transitions

| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Modal open | opacity 0→1 | 150ms | ease-out |
| Modal close | opacity 1→0 | 100ms | ease-in |

## Responsive 변형
- **Desktop**: 420×260 모달 중앙 배치.
- **Mobile**: `min(420, width-48)` × `min(260, height-48)`, 최소 280×200.

## Accessibility
- Escape로 모달 닫기.
- Tab으로 모달 내 포커스 순환.
- Enter로 Profile next.
- 모달 열릴 때 포커스 트랩.

## Design tokens — 사용 / 제안
- **사용**: `NEUTRAL_700`, `NEUTRAL_800`, `NEUTRAL_600`, `NEUTRAL_500`, `NEUTRAL_400`, `NEUTRAL_300`, `NEUTRAL_100`, `NEUTRAL_900`, `ACCENT_STUDY`, `STATUS_WARNING`, `STATUS_ERROR`, `STATUS_READY`.
- **신규 제안**: `shadow_modal` — modal 그림자 (현재 미적용, 후속 구현).

## Out of scope
- 모달 내 인터랙티브 차트 (후속 spec).
- 커스텀 목표 생성 (후속 spec).
