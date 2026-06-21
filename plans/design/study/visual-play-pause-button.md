# Design: visual-play-pause-button

## 한 줄 정의
Learn 비주얼 컨트롤에서 재생/일시정지 버튼을 클릭하면 `visual_playing`이 전환되고 아이콘이 변경된다. 기존 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Play/Pause button | `Button` | `study.visual.play_pause` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 비주얼 자동재생 (별도 spec `visual-autoplay-toggle-button`).
- 비주얼 surface (별도 design `study-visual`).
