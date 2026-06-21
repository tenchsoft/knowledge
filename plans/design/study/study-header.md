# Design: study-header

## 한 줄 정의
Study 화면 상단 헤더 바에 과목/단원/개념 breadcrumb, 스테이지 필, 통계 버튼, 스트릭/타이머, 단축키/고대비/목표 버튼, 일일 대시보드 미니 위젯을 표시한다.

## 시각적 레이아웃
```
┌─ Header (full-width, 40px) ──────────────────────────────────────────────────────┐
│ [과목] / [단원] / [개념]    [?] [hc]  [Learn▼]  [streak 0] [00:00:00] [Goals] [Stats] │
│                              (mobile: subject only, stage pill repositioned)      │
│ (≥700px header)  daily: due:N  new:N  acc:N%                                      │
└───────────────────────────────────────────────────────────────────────────────────┘
```

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Stats button | `Button` | `study.header.stats` |
| Stage pill | `Button` | `study.header.stage` |
| Shortcuts button | `Button` | `study.header.shortcuts` |
| High contrast toggle | `Button` | `study.header.high_contrast` |
| Goals button | `Button` | `study.header.goals` |

## Visual properties

| 속성 | 값 |
|------|----|
| Header background | `NEUTRAL_800` |
| Header bottom border | `NEUTRAL_600`, 1px |
| Breadcrumb text (subject/unit) | `NEUTRAL_300`, 12px, `FontWeight::BOLD` |
| Breadcrumb separator "/" | `NEUTRAL_500`, 12px, `FontWeight::NORMAL` |
| Breadcrumb text (concept) | `NEUTRAL_100`, 12px, `FontWeight::BOLD` |
| Stage pill background | `NEUTRAL_700`, pill (border_radius=999) |
| Stage pill text | `Stage::color()`, 11px, `FontWeight::BOLD` |
| Stats button border | `NEUTRAL_500`, 1px, border_radius=6 |
| Stats button text | `NEUTRAL_100`, 12px |
| Shortcuts button | outlined rect, `NEUTRAL_500` stroke |
| HC toggle (off) | outlined, text "hc", `NEUTRAL_400` |
| HC toggle (on) | outlined, text "HC", `ACCENT_STUDY` |
| Streak text | `STATUS_WARNING`, 12px, `FontWeight::BOLD` |
| Timer text | `NEUTRAL_400`, 11px, `FontWeight::NORMAL` |
| Goals button | outlined rect |
| Daily dashboard (due) | `STATUS_WARNING`, 10px, `FontWeight::BOLD` |
| Daily dashboard (new/acc) | `NEUTRAL_400`, 10px |

## States

| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Stats button | `NEUTRAL_500` stroke | lighten 8% | lighten 16% | 2px outline `ACCENT_STUDY` | opacity 0.5 |
| Stage pill | `NEUTRAL_700` fill | lighten 8% | lighten 16% | 2px outline `ACCENT_STUDY` | — |
| HC toggle (off) | `NEUTRAL_400` text | lighten 8% | — | 2px outline | opacity 0.5 |
| HC toggle (on) | `ACCENT_STUDY` text | darken 8% | — | 2px outline | opacity 0.5 |

## Responsive 변형
- **Desktop (≥1100px)**: 전체 breadcrumb, daily dashboard, timer, 모든 버튼 표시.
- **Tablet (700–1100px)**: Breadcrumb 숨김, daily dashboard 숨김, stage pill/stats 유지.
- **Mobile (<700px)**: Subject만 표시, 햄버거 메뉴로 대체, stage pill 위치 조정.

## Accessibility
- 모든 버튼에 focus indicator 명시.
- Breadcrumb 텍스트 WCAG AA 대비.
- Tab 순서: shortcuts → HC → stage → goals → stats.

## Design tokens — 사용 / 제안
- **사용**: `NEUTRAL_800`, `NEUTRAL_700`, `NEUTRAL_600`, `NEUTRAL_500`, `NEUTRAL_400`, `NEUTRAL_300`, `NEUTRAL_100`, `ACCENT_STUDY`, `STATUS_WARNING`, `Stage::color()`.
- **신규 제안**: 없음 (기존 색상 토큰으로 충분).

## Out of scope
- 햄버거 메뉴 내부 (별도 design `study-curriculum` hamburger 섹션에서 처리).
- Stats modal 내부 (별도 design `study-modals`).
