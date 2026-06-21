# Design: research-pdf-viewer

## 한 줄 정의
센터 패널에서 PDF 페이지를 렌더하고 검색, 탐색, 확대/축소, 회전, 주석 도구를 제공.

## 시각적 레이아웃
```
┌─ Center Panel (PDF mode) ────────────────────────────────────────────────┐
│  ...paper detail above...                                                │
│                                                                          │
│  [Search in PDF...           ] 3/5                                       │
│  ┌─ PDF Surface (180px) ──────────────────────────────────────────────┐  │
│  │  Rendered page with annotation overlays + search highlights        │  │
│  └────────────────────────────────────────────────────────────────────┘  │
│  Page 1/12  [<] [>]  100%  [-] [+]  [Rot]  [H][U][S][N]  [Ann]        │
│                                                                          │
│  Annotations (when toggled)                                              │
│  HL p1: Attention mechanism is...                                        │
│  Note p2: Important methodology                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| PDF search input | `TextInput` | `research.pdf.search` |
| PDF search counter | `Label` | — (format `{active+1}/{total}`) |
| PDF surface | `Canvas` | `research.pdf.surface` |
| Previous page button | `Button` | `research.pdf.prev` |
| Next page button | `Button` | `research.pdf.next` |
| Zoom out button | `Button` | `research.pdf.zoom_out` |
| Zoom in button | `Button` | `research.pdf.zoom_in` |
| Rotate button | `Button` | `research.pdf.rotate` |
| Highlight tool | `Button` | `research.pdf.tool.highlight` |
| Underline tool | `Button` | `research.pdf.tool.underline` |
| Strikeout tool | `Button` | `research.pdf.tool.strikeout` |
| Sticky note tool | `Button` | `research.pdf.tool.sticky_note` |
| Annotation list toggle | `Button` | `research.pdf.annotation_list_toggle` |
| Annotation row | `Label` | — (kind + page + text preview) |

## Visual properties
| 속성 | 값 |
|------|----|
| PDF search bar bg | `theme.background` |
| PDF search bar border | `theme.border`, 1px |
| PDF search bar border radius | `theme.border_radius` |
| PDF search placeholder color | `theme.disabled` |
| PDF search counter color | `theme.secondary` |
| PDF surface height | 180px |
| Nav button bg (prev/next) | `theme.primary` |
| Nav button fg | `theme.on_primary` |
| Nav button size | 28×20px |
| Zoom label | `theme.on_background`, `theme.font_size_small` |
| Zoom +/- button bg | `theme.surface`, border `theme.border` |
| Zoom +/- button fg | `theme.on_surface` |
| Rotate button bg | `theme.surface`, border `theme.border` |
| Tool button active bg | `theme.primary` |
| Tool button inactive bg | `theme.surface`, border `theme.border` |
| Tool button size | 24×20px |
| Annotation list toggle active bg | `theme.primary` |
| Annotation list toggle inactive bg | `theme.surface`, border `theme.border` |
| Annotation row selected bg | `theme.primary` |
| Annotation row default bg | `theme.surface` |

Annotation overlay colors:
| Kind | Color |
|------|-------|
| Highlight | yellow alpha 90 (`rgba8(255,235,59,90)`) |
| Underline | blue 200 (`rgba8(33,150,243,200)`) |
| Strikeout | red 200 (`rgba8(244,67,54,200)`) |
| Sticky note | amber 220 (`rgba8(255,193,7,220)`) |
| Search result | yellow (`#FFEB3B`), active highlighted |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| PDF search input | border `theme.border` | — | — | border `theme.primary` 2px | — |
| Nav button | bg `theme.primary` | — | — | — | — |
| Tool button (active) | bg `theme.primary` | — | — | — | — |
| Tool button (inactive) | bg `theme.surface` | — | — | — | — |
| Annotation toggle (on) | bg `theme.primary` | — | — | — | — |
| Annotation toggle (off) | bg `theme.surface` | — | — | — | — |

## Animations / transitions
| Trigger | Property | Duration | Easing |
|---------|----------|----------|--------|
| Page change | PDF surface repaint | immediate | — |
| Annotation add | Toast notification | — | — |

## Responsive 변형
- PDF 모드는 데스크톱 전용. 모바일에서는 Detail 모드로 대체.
- 센터 패널 너비에 따라 surface가 가로로 축소.

## Accessibility
- PageUp/PageDown: 페이지 이동.
- +/-: 확대/축소.
- Enter: PDF 검색에서 다음 결과로 이동.
- 클릭: 활성 주석 도구일 때 주석 배치.

## Design tokens — 사용 / 제안
- **사용**: `theme.background`, `theme.surface`, `theme.primary`, `theme.on_primary`,
  `theme.on_surface`, `theme.on_background`, `theme.border`, `theme.secondary`,
  `theme.disabled`, `theme.font_size_small`, `theme.spacing`, `theme.spacing_large`,
  `theme.border_radius`.
- **신규 제안**: `theme.annotation_highlight` / `theme.annotation_underline` /
  `theme.annotation_strikeout` / `theme.annotation_sticky` — 주석 색상 토큰화.

## Out of scope
- PDF 텍스트 선택 (별도 spec).
- 주석 편집 모달 (별도 spec).
