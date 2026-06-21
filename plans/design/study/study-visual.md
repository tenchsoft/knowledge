# Design: study-visual

## 한 줄 정의
학습 모드 비주얼 표면 아래에 재생/일시정지 버튼, 타임라인 스크러버, 자동재생 토글을 표시한다.

## 시각적 레이아웃
```
[▶] [═══════════════════●════════════] [Auto: OFF]
```

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Play/Pause button | `Button` | `study.visual.play_pause` |
| Autoplay toggle | `Button` | `study.visual.autoplay` |

## Visual properties

| 속성 | 값 |
|------|----|
| Play button border | `NEUTRAL_500`, 1px, border_radius=4 |
| Play icon (paused) | ">" `NEUTRAL_100`, 10px, `FontWeight::BOLD` |
| Pause icon (playing) | "‖" `ACCENT_STUDY`, 10px, `FontWeight::BOLD` |
| Scrubber track | `NEUTRAL_600` fill, border_radius=2 |
| Scrubber filled | `ACCENT_STUDY` fill, border_radius=2 |
| Autoplay border | `NEUTRAL_500`, 1px, border_radius=4 |
| Autoplay text (off) | "Auto: OFF" `NEUTRAL_400`, 9px |
| Autoplay text (on) | "Auto: ON" `ACCENT_STUDY`, 9px |

## States

| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Play/Pause | `NEUTRAL_500` stroke | lighten 8% | lighten 16% | 2px outline `ACCENT_STUDY` | opacity 0.5 |
| Autoplay | `NEUTRAL_500` stroke | lighten 8% | lighten 16% | 2px outline | opacity 0.5 |

## Responsive 변형
- **Desktop**: 전체 타임라인 표시.
- **Mobile**: 스크러버 너비 축소, 버튼 크기 동일.

## Accessibility
- Play/Pause 상태가 자동화 노드 label에 반영.
- Autoplay 상태가 자동화 노드 label에 반영.

## Design tokens — 사용 / 제안
- **사용**: `NEUTRAL_600`, `NEUTRAL_500`, `NEUTRAL_400`, `NEUTRAL_100`, `ACCENT_STUDY`.
- **신규 제안**: 없음.

## Out of scope
- 비주얼 표면 자체 (VisualSurface 컴포넌트가 처리).
- 스크러버 드래그 인터랙션 (현재 click-only).
