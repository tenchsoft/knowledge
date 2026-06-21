# Design: study-notes

## 한 줄 정의
학습 모드에서 노트 패널 오버레이가 우측에 표시되며, 노트 입력, 저장 버튼, 기존 노트 목록을 보여준다.

## 시각적 레이아웃
```
┌─ Notes Panel (280px or 40% of surface, right-aligned overlay) ──┐
│ Notes                                                            │
│ ┌────────────────────────────────────────────────────────────┐   │
│ │ Write a note...                                            │   │
│ └────────────────────────────────────────────────────────────┘   │
│                                              [Save]               │
│ ┌────────────────────────────────────────────────────────────┐   │
│ │ Note text...                                               │   │
│ │ 42s                                                        │   │
│ └────────────────────────────────────────────────────────────┘   │
│ ┌────────────────────────────────────────────────────────────┐   │
│ │ Another note...                                            │   │
│ │ 120s                                                       │   │
│ └────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
```

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Notes panel | `Panel` | `study.notes.panel` |
| Notes input | `TextInput` | `study.notes.input` |
| Save button | `Button` | `study.notes.save` |
| Note row | `Button` | `study.notes.row.{idx}` |

## Visual properties

| 속성 | 값 |
|------|----|
| Panel background | `rgba(0x1A, 0x1A, 0x1A, 240)` |
| Panel left border | `NEUTRAL_600`, 1px |
| Panel title | `NEUTRAL_100`, 14px, `FontWeight::BOLD` |
| Input bg | `NEUTRAL_700`, border_radius=6 |
| Input border (unfocused) | `NEUTRAL_500`, 1px |
| Input border (focused) | `ACCENT_STUDY`, 1px |
| Input placeholder | `NEUTRAL_500`, 12px |
| Input text | `NEUTRAL_100`, 12px |
| Save button | `ACCENT_STUDY` fill, border_radius=6, `NEUTRAL_800` text, 11px, `FontWeight::BOLD` |
| Note row bg | `NEUTRAL_700`, border_radius=4 |
| Note text | `NEUTRAL_100`, 11px |
| Note timestamp | `NEUTRAL_500`, 9px |

## States

| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Notes input | `NEUTRAL_500` border | — | — | `ACCENT_STUDY` border | opacity 0.5 |
| Save button | `ACCENT_STUDY` fill | darken 8% | darken 16% | 2px outline | opacity 0.5 |
| Note row | `NEUTRAL_700` bg | lighten 8% | lighten 16% | 2px outline | — |

## Responsive 변형
- **Desktop**: 280px 고정 너비 오버레이.
- **Mobile**: `min(280, surface_width * 0.4)` 너비.

## Accessibility
- Enter로 노트 저장.
- Escape로 입력 포커스 해제.
- 활성 개념에 해당하는 노트만 표시.

## Design tokens — 사용 / 제안
- **사용**: `NEUTRAL_700`, `NEUTRAL_600`, `NEUTRAL_500`, `NEUTRAL_100`, `NEUTRAL_800`, `ACCENT_STUDY`.
- **신규 제안**: 없음.

## Out of scope
- 노트 편집/삭제 (후속 spec).
- 노트 검색 (후속 spec).
