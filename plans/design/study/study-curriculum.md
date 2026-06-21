# Design: study-curriculum

## 한 줄 정의
좌측 커리큘럼 패널에 검색 입력, 북마크/노트 토글, 단원-개념 트리(가상 스크롤), 복습 대기열 버튼을 표시한다.

## 시각적 레이아웃
```
┌─ Outline (180px desktop, 30% mobile) ──────┐
│ [🔍 search...              ] [N] [★]       │
│  3 matches                                  │
│ ─────────────────────────────────────────── │
│ v Unit 1: Mathematics          2/3          │
│   ✓ Concept A                    ★          │
│   ● Concept B (active)                      │
│   ○ Concept C                               │
│ > Unit 2: Science               0/2         │
│ ─────────────────────────────────────────── │
│ [ Review Queue                          3 ] │
└─────────────────────────────────────────────┘
```

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Search input | `TextInput` | `study.curriculum.search` |
| Bookmark toggle | `Button` | `study.curriculum.bookmark` |
| Notes toggle | `Button` | `study.curriculum.notes` |
| Unit header | `Button` | `study.unit.{idx}` |
| Concept row | `Button` | `study.concept.{unit_idx}.{concept_idx}` |
| Review queue button | `Button` | `study.review.queue` |

## Visual properties

| 속성 | 값 |
|------|----|
| Panel background | `NEUTRAL_700` |
| Panel right border | `NEUTRAL_600`, 1px |
| Search input background | `NEUTRAL_800`, border_radius=4 |
| Search border (unfocused) | `NEUTRAL_600`, 1px |
| Search border (focused) | `ACCENT_STUDY`, 1px |
| Search placeholder | `NEUTRAL_500`, 12px |
| Search text | `NEUTRAL_100`, 12px |
| Search cursor | `NEUTRAL_100`, 1px wide blinking bar |
| Match count | `ACCENT_STUDY`, 10px, `FontWeight::BOLD` |
| Bookmark (off) | ☆ `NEUTRAL_400`, 14px |
| Bookmark (on) | ★ `STATUS_WARNING`, 14px |
| Notes toggle (off) | "N" `NEUTRAL_400`, 12px |
| Notes toggle (on) | "N" `ACCENT_STUDY`, 12px |
| Unit label | `NEUTRAL_100`, 14px, `FontWeight::BOLD` |
| Unit expand icon | `NEUTRAL_400`, 10px ("v"/">") |
| Unit progress | `NEUTRAL_400`, 10px ("2/3") |
| Concept label (active) | `ACCENT_STUDY`, 13px, `FontWeight::MEDIUM` |
| Concept label (inactive) | `NEUTRAL_300`, 13px, `FontWeight::NORMAL` |
| Active concept bg | `NEUTRAL_500` fill + `ACCENT_STUDY` 2px left bar |
| Concept status icon (Completed) | ✓ `NEUTRAL_400`, 11px, `FontWeight::BOLD` |
| Concept status icon (Active) | ● `ACCENT_STUDY`, 11px, `FontWeight::BOLD` |
| Concept status icon (InProgress) | ○ `NEUTRAL_300`, 11px, `FontWeight::BOLD` |
| Concept status icon (Warning) | ! `STATUS_WARNING`, 11px, `FontWeight::BOLD` |
| Bookmark indicator on concept | ★ `STATUS_WARNING`, 10px |
| Review queue bg | `NEUTRAL_800` |
| Review queue label | `STATUS_WARNING`, 12px, `FontWeight::BOLD` |
| Review queue count badge | `STATUS_WARNING` fill, `NEUTRAL_900` text, pill |

## States

| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Search input | `NEUTRAL_600` border | — | — | `ACCENT_STUDY` border | opacity 0.5 |
| Unit header | transparent | `NEUTRAL_600` bg | `NEUTRAL_500` bg | 2px outline | — |
| Concept row | transparent | `NEUTRAL_600` bg | `NEUTRAL_500` bg | 2px outline | — |
| Bookmark toggle | `NEUTRAL_400` | lighten 8% | — | 2px outline | opacity 0.5 |
| Review queue button | `NEUTRAL_800` bg | lighten 8% | lighten 16% | 2px outline | — |

## Responsive 변형
- **Desktop (≥1100px)**: 180px 고정 너비, 모든 요소 표시.
- **Tablet (700–1100px)**: 180px 고정, daily dashboard 숨김.
- **Mobile (<700px)**: 너비 = `min(width*0.30, 180)` 최소 88px, 햄버거 메뉴로 대체 가능.

## Accessibility
- 가상 스크롤: 화면 밖 항목은 자동화 노드에서 제외.
- 검색 결과 일치 수 표시.
- Arrow Up/Down으로 개념 탐색.
- Enter로 첫 검색 결과로 이동.

## Design tokens — 사용 / 제안
- **사용**: `NEUTRAL_700`, `NEUTRAL_800`, `NEUTRAL_600`, `NEUTRAL_500`, `NEUTRAL_400`, `NEUTRAL_300`, `NEUTRAL_100`, `NEUTRAL_900`, `ACCENT_STUDY`, `STATUS_WARNING`.
- **신규 제안**: 없음.

## Out of scope
- 햄버거 메뉴 내부 (mobile 전용, 별도 섹션).
- 노트 패널 (별도 design `study-notes`).
