# Design: research-sidebar

## 한 줄 정의
왼쪽 패널에 컬렉션 트리, 태그 칩, 상태 필터, 논문 목록을 수직으로 나열.

## 시각적 레이아웃
```
┌─ Left Panel (240px) ──────────┐
│ COLLECTIONS                    │
│  ▾ All Papers (12)             │
│  ▸ Machine Learning (5)        │
│  ▸ NLP (3)                     │
│  Smart                         │
│  ⚙ Recently Added (4)          │
│                                │
│ TAGS                           │
│  [attention]  [transformer]    │
│  [nlp]                         │
│                                │
│ Saved Searches                 │
│  🔍 My Query [attention]       │
│                                │
│ STATUS                         │
│   All                          │
│   Read                         │
│   Reading                      │
│   To Read                      │
│                                │
│ PAPERS              [Title ↑]  │
│  ☑ Attention Is All You Need   │
│  * BERT: Pre-training...       │
│  GPT-3: Language Models...     │
└────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Left panel surface | `Surface` | — (region `research_regions().left`) |
| Collections heading | `Label` | — (`t!("research.collection.heading")`) |
| Collection row | `Button` | `research.collection.{index}` |
| Smart collection row | `Button` | — (indexed) |
| Tags heading | `Label` | — (`t!("research.tag.heading")`) |
| Tag chip | `Button` | `research.tag.{index}` |
| Saved searches heading | `Label` | — |
| Saved search row | `Button` | — (indexed) |
| Status heading | `Label` | — (`t!("research.status.heading")`) |
| Status filter row | `Button` | `research.status.{index}` |
| Papers heading | `Label` | — (`t!("research.paper.heading")`) |
| Sort indicator button | `Button` | `research.paper.sort` |
| Paper row | `Button` | `research.paper.{index}` |

## Visual properties
| 속성 | 값 |
|------|----|
| Panel background | `theme.surface` |
| Panel width | 240px (constant `LEFT_W`), mobile: `(width*0.30).clamp(96, 240)` |
| Section heading typography | `theme.font_size_small`, weight `BOLD`, color `theme.secondary` |
| Collection row selected bg | `theme.primary` |
| Collection row selected fg | `theme.on_primary` |
| Collection row default bg | `theme.surface` |
| Collection row default fg | `theme.on_surface` |
| Row height (collection/status) | 20px |
| Tag chip bg | `theme.primary` |
| Tag chip fg | `theme.on_primary` |
| Tag chip border radius | `theme.border_radius` |
| Tag row height | 24px |
| Paper row selected bg | `theme.primary` |
| Paper row multi-select bg | `theme.primary` alpha 40 |
| Paper row default bg | `theme.surface` |
| Paper row height | 22px |
| Search highlight bg | yellow alpha 40 (`rgba8(255,235,59,40)`) |
| Separator line | `theme.border`, 1px |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Collection row | bg `theme.surface` | — | — | — | — |
| Collection row (selected) | bg `theme.primary` | — | — | — | — |
| Tag chip | bg `theme.primary` | — | — | — | — |
| Paper row (selected) | bg `theme.primary` | — | — | — | — |
| Paper row (multi-selected) | bg primary alpha 40 | — | — | — | — |
| Sort button | bg `theme.surface`, border `theme.border` | — | — | — | — |

## Responsive 변형
- **Desktop (≥560px)**: 240px 고정 폭.
- **Mobile (<560px)**: `(width*0.30).clamp(96, 240)` — 최소 96px 보장.

## Accessibility
- 화살표 키(↑/↓)로 논문 선택 이동.
- 우클릭으로 다중 선택 토글.
- 컬렉션 확장/축소: 클릭 x < spacing+10 영역.

## Design tokens — 사용 / 제안
- **사용**: `theme.surface`, `theme.primary`, `theme.on_primary`, `theme.on_surface`,
  `theme.secondary`, `theme.border`, `theme.font_size`, `theme.font_size_small`,
  `theme.spacing`, `theme.spacing_large`, `theme.border_radius`.

## Out of scope
- 논문 상세 (별도 design `research-paper-list` center area).
- PDF 뷰어 (별도 design `research-pdf-viewer`).
