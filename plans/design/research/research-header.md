# Design: research-header

## 한 줄 정의
spec(`plans/spec/research/`)에서 정의한 검색, 가져오기, 내보내기, 동기화 동작을 헤더 바에서 시각적으로 표현.

## 시각적 레이아웃
```
┌─ Header (full-width, 48px) ──────────────────────────────────────────────────┐
│ [Title]  [ Search input .............. ▶]  [Import] [Export] [Sync]  status  │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Header surface | `Surface` | — (region `research_regions().header`) |
| App title | `Label` | — (`t!("research.app.title")`) |
| Search input | `TextInput` | `research.header.search` |
| Advanced search toggle | `Button` | `research.header.advanced_search` |
| Import button | `Button` | `research.header.import` |
| Export button | `Button` | `research.header.export` |
| Sync button | `Button` | `research.header.sync` |
| Status text | `Label` | — (import status / reader mode / favorites) |

## Visual properties
| 속성 | 값 |
|------|----|
| Header background | `theme.surface` |
| Header height | 48px (constant `HEADER_H`) |
| Title typography | `theme.font_size_large`, weight `BOLD` |
| Search input background | `theme.background` |
| Search input border (default) | `theme.border`, 1px |
| Search input border (focused) | `theme.primary`, 2px |
| Search input border radius | `theme.border_radius` |
| Search input height | `theme.input_height` |
| Search placeholder color | `theme.disabled` |
| Search query color | `theme.on_background` |
| Action button background | `theme.primary` |
| Action button text | `theme.on_primary` |
| Action button height | `theme.button_height` |
| Action button width | 80px |
| Action button border radius | `theme.border_radius` |
| Separator line | `theme.border`, 1px |
| Status text color | `theme.secondary` |
| Status text size | `theme.font_size_small` |

## States
| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Search input | border `theme.border` | — | — | border `theme.primary` 2px | — |
| Action button | bg `theme.primary` | — | — | — | — |
| Advanced toggle | `▶` icon | — | — | — | — |
| Advanced toggle (open) | `▼` icon | — | — | — | — |

## Responsive 변형
- **Desktop (≥700px)**: 검색 입력 + 액션 버튼 표시. 검색 너비 `(width-540).clamp(180, 280)`.
- **Mobile (<700px)**: 검색 및 버튼 숨김. 타이틀만 표시.
- **Wide (≥1040px)**: import status / reader mode / favorites 상태 텍스트 추가 표시.

## Accessibility
- 검색 입력: 포커스 시 `theme.primary` 2px outline.
- 버튼: 키보드 탭 순서로 접근 가능.
- Ctrl+F: 검색 포커스 단축키.
- Ctrl+I: 가져오기 단축키.

## Design tokens — 사용 / 제안
- **사용**: `theme.surface`, `theme.background`, `theme.primary`, `theme.on_primary`,
  `theme.on_background`, `theme.border`, `theme.disabled`, `theme.secondary`,
  `theme.font_size`, `theme.font_size_large`, `theme.font_size_small`,
  `theme.spacing`, `theme.border_radius`, `theme.input_height`, `theme.button_height`.

## Out of scope
- 고급 검색 패널 (별도 design `research-automatic-ui`).
- 토스트 알림 (별도 design `research-automatic-ui`).
