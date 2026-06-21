# Design: research-paper-list

## 한 줄 정의
센터 패널에 선택한 논문의 상세 정보(제목, 저자, 태그, 상태 배지, 초록, 참고문헌)를 표시. 선택된 논문이 없으면 빈 상태 메시지.

## 시각적 레이아웃
```
┌─ Center Panel ───────────────────────────────────────────────────────────┐
│ (no paper selected)                                                      │
│  Empty Title                                                             │
│  Empty body description                                                  │
│                                                                          │
│ ── or ──                                                                │
│                                                                          │
│  Paper Title (large, bold)                                               │
│  Authors (Year) | Venue | Pages pg | filename.pdf                        │
│  [tag1] [tag2] [tag3]                                                    │
│  [Status Badge]                                                          │
│                                                                          │
│  Abstract                                                                │
│  Wrapped abstract text lines...                                          │
│                                                                          │
│  (PDF mode: search bar + PDF surface + nav controls)                     │
│  (Detail mode: Visual analysis area)                                     │
│                                                                          │
│  References                                                              │
│  - ref line 1                                                            │
│  - ref line 2                                                            │
│                                                                          │
│  (PDF mode: Annotations list when toggled)                               │
└──────────────────────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Empty title | `Label` | — (`t!("research.empty.title")`) |
| Empty body | `Label` | — (`t!("research.empty.body")`) |
| Paper title | `Label` | — (selected paper title) |
| Authors/year/venue line | `Label` | — |
| Tag chip | `Label` | — |
| Status badge | `Label` | — (color per `status_color()`) |
| Abstract heading | `Label` | — (`t!("research.section.abstract")`) |
| Abstract text | `Label` | — (wrapped) |
| Visual heading | `Label` | — (`t!("research.section.visual")`) |
| References heading | `Label` | — (`t!("research.section.references")`) |
| Reference line | `Label` | — |

## Visual properties
| 속성 | 값 |
|------|----|
| Center background | `theme.background` (default) |
| Empty title | `theme.on_background`, `theme.font_size_large`, `BOLD` |
| Empty body | `theme.secondary`, `theme.font_size` |
| Paper title | `theme.on_background`, `theme.font_size_large`, `BOLD` |
| Authors line | `theme.secondary`, `theme.font_size` |
| Tag chip bg | `theme.primary`, border radius `theme.border_radius` |
| Tag chip fg | `theme.on_primary`, `theme.font_size_small`, `MEDIUM` |
| Status badge bg | `status_color(status)` — Reviewed: green, Reading: amber, other: blue |
| Status badge fg | `theme.on_primary` |
| Abstract heading | `theme.on_background`, `theme.font_size`, `BOLD` |
| Abstract text | `theme.on_surface`, `theme.font_size` |
| Visual area bg | `theme.background`, border `theme.border` |
| Reference text | `theme.on_surface`, `theme.font_size_small` |
| Padding | `theme.spacing_large` |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Tag chip | bg `theme.primary` | — | — | — | — |
| Status badge | bg `status_color()` | — | — | — | — |

## Responsive 변형
- **Desktop (≥980px)**: 센터 + 라이트 인스펙터 표시.
- **Tablet (560–980px)**: 센터만, 인스펙터 숨김 (`right_w = 0`).
- **Mobile (<560px)**: 좁은 좌측 + 센터만.

## Accessibility
- 빈 상태: 의미 있는 안내 텍스트.
- 태그 칩과 상태 배지: 색상만으로 구분 가능 (WCAG AA 텍스트 대비).

## Design tokens — 사용 / 제안
- **사용**: `theme.background`, `theme.surface`, `theme.on_background`, `theme.on_surface`,
  `theme.on_primary`, `theme.primary`, `theme.secondary`, `theme.border`,
  `theme.font_size`, `theme.font_size_large`, `theme.font_size_small`,
  `theme.spacing`, `theme.spacing_large`, `theme.border_radius`.
- **신규 제안**: `theme.status_reviewed` / `theme.status_reading` / `theme.status_default` —
  상태 배지 색상 통합 관리. 현재 `paper_list::status_color()` 하드코딩.

## Out of scope
- PDF 뷰어 (별도 design `research-pdf-viewer`).
- 인스펙터 패널 (별도 design `research-inspector`).
