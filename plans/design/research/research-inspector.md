# Design: research-inspector

## 한 줄 정의
오른쪽 패널에 Notes, Summary, Q&A, Visuals, Write, Cite 6개 탭으로 논문 인스펙터를 제공.

## 시각적 레이아웃
```
┌─ Right Panel (280px) ─────────────────────────────────────┐
│ [Notes] [Summary] [Q&A] [Visual] [Write] [Cite]          │
│ ────────────────────────────────────────────────────────── │
│ (tab content area)                                        │
│                                                           │
│ ┌─ Tab: Notes ──────────────────────────────────────────┐ │
│ │ Notes heading                                         │ │
│ │ Paper notes text...                                   │ │
│ └───────────────────────────────────────────────────────┘ │
│                                                           │
│ ┌─ Tab: Q&A ────────────────────────────────────────────┐ │
│ │ system: Paper context loaded...                       │ │
│ │ assistant: The selected paper focuses on...           │ │
│ │ [Ask about this paper...] [Send]                      │ │
│ │ [Summarize] [Key Points] [Limitations]                │ │
│ └───────────────────────────────────────────────────────┘ │
│                                                           │
│ ┌─ Tab: Write ──────────────────────────────────────────┐ │
│ │ Outline                                               │ │
│ │ readiness dashboard lines                             │ │
│ │ ▸ Introduction (2 cite)                               │ │
│ │ ▪ Methodology                                         │ │
│ │ [Add Section]                                         │ │
│ │ [Cite search input]                                   │ │
│ │ > result 1 [Insert]                                   │ │
│ └───────────────────────────────────────────────────────┘ │
│                                                           │
│ ┌─ Tab: Cite ───────────────────────────────────────────┐ │
│ │ [BibTeX] [RIS] [APA] [Chicago] [MLA]                 │ │
│ │ reference list                                        │ │
│ │ DOI / arXiv ID [________] [Fetch]                     │ │
│ │ [Import BibTeX]                                       │ │
│ └───────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Inspector tab | `tab` | `research.inspector.tab.{0-5}` |
| Notes heading | `Label` | — (`t!("research.inspector.notes")`) |
| Notes text | `Label` | — (paper notes) |
| Summary heading | `Label` | — (`t!("research.inspector.summary")`) |
| Summary text | `Label` | — (`t!("research.inspector.summary_text")`) |
| Q&A heading | `Label` | — (`t!("research.inspector.qa")`) |
| Q&A message bubble (user) | `Label` | — (role `theme.primary`) |
| Q&A message bubble (assistant) | `Label` | — (role green `#34D399`) |
| Q&A input | `TextInput` | `research.qa.input` |
| Q&A send button | `Button` | `research.qa.send` |
| Q&A quick: Summarize | `Button` | `research.qa.quick.summarize` |
| Q&A quick: Key Points | `Button` | `research.qa.quick.key_points` |
| Q&A quick: Limitations | `Button` | `research.qa.quick.limitations` |
| Visuals heading | `Label` | — (`t!("research.inspector.visuals")`) |
| Visual surface | `Canvas` | — (`VisualSurface`) |
| Write outline heading | `Label` | — (`t!("research.manuscript.outline")`) |
| Write summary lines | `Label` | — (readiness dashboard) |
| Write section row | `Button` | `research.manuscript.section.{index}` |
| Write add section | `Button` | `research.manuscript.add_section` |
| Write cite search | `TextInput` | `research.manuscript.cite_search` |
| Write cite result insert | `Button` | `research.manuscript.cite_result.{index}.insert` |
| Cite heading | `Label` | — (`t!("research.inspector.citations")`) |
| Cite format button | `Button` | `research.citation.format.{bibtex,ris,apa,chicago,mla}` |
| Cite DOI input | `TextInput` | `research.citation.doi` |
| Cite fetch button | `Button` | `research.citation.fetch` |
| Cite import BibTeX | `Button` | `research.citation.import_bibtex` |

## Visual properties
| 속성 | 값 |
|------|----|
| Panel background | `theme.surface` |
| Panel width | 280px (constant `RIGHT_W`), hidden if <980px |
| Active tab bg | `theme.primary` |
| Active tab fg | `theme.on_primary` |
| Inactive tab fg | `theme.secondary` |
| Tab width | 36px each, gap `theme.spacing` |
| Tab border radius | `theme.border_radius` |
| Section heading | `theme.on_background`, `theme.font_size`, `BOLD` |
| Body text | `theme.on_surface`, `theme.font_size_small` |
| Q&A user role color | `theme.primary` |
| Q&A assistant role color | `#34D399` (green) |
| Q&A input bg | `theme.background` |
| Q&A input border (focused) | `theme.primary`, 2px |
| Quick action button bg | `theme.surface`, border `theme.border` |
| Quick action button fg | `theme.on_surface` |
| Manuscript active section bg | `theme.primary` alpha 0x1F |
| Manuscript active section fg | `theme.primary` |
| Format button active bg | `theme.primary` |
| Format button inactive bg | `theme.surface`, border `theme.border` |
| DOI input bg | `theme.background` |
| Fetch button bg | `theme.primary` |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Inspector tab (active) | bg `theme.primary` | — | — | — | — |
| Inspector tab (inactive) | fg `theme.secondary` | — | — | — | — |
| Q&A input | border `theme.border` | — | — | border `theme.primary` 2px | — |
| Quick action | bg `theme.surface` | — | — | — | — |
| Format button (active) | bg `theme.primary` | — | — | — | — |
| Format button (inactive) | bg `theme.surface` | — | — | — | — |
| Section row (active) | bg primary alpha | — | — | — | — |

## Responsive 변형
- **Desktop (≥980px)**: 280px 인스펙터 표시.
- **Tablet/Mobile (<980px)**: 인스펙터 숨김 (`right_w = 0`).

## Accessibility
- 탭 전환: 클릭 또는 키보드.
- Q&A: Enter로 메시지 전송.
- Write: 섹션 클릭으로 활성 섹션 전환.
- Cite: DOI 입력 후 Enter로 fetch.

## Design tokens — 사용 / 제안
- **사용**: `theme.surface`, `theme.primary`, `theme.on_primary`, `theme.on_surface`,
  `theme.on_background`, `theme.secondary`, `theme.background`, `theme.border`,
  `theme.font_size`, `theme.font_size_small`, `theme.spacing`, `theme.spacing_large`,
  `theme.border_radius`, `theme.input_height`.
- **신규 제안**: `theme.role_assistant` — Q&A assistant 말풍선 색상 (`#34D399`).

## Out of scope
- PDF 뷰어 (별도 design `research-pdf-viewer`).
- 토스트 (별도 design `research-automatic-ui`).
