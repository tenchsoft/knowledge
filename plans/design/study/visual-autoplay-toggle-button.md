# Design: visual-autoplay-toggle-button

## 한 줄 정의
Learn 비주얼 컨트롤에서 Auto ON/OFF 버튼을 클릭하면 `visual_autoplay`가 전환되고 라벨이 갱신된다. 기존 토글 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Autoplay toggle | `Button` | `study.visual.autoplay` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 비주얼 재생/일시정지 (별도 spec `visual-play-pause-button`).
- 비주얼 surface (별도 design `study-visual`).
