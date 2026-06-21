# Design: visual-scrubber-control

## 한 줄 정의
Learn 비주얼 타임라인 scrubber를 조작하면 재생 위치가 변경된다. 기존 Slider 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Scrubber | `Slider` | `study.visual.scrubber` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 타임라인 채움 (별도 background `automatic-visual-timeline-fill-behavior`).
- 비주얼 surface (별도 design `study-visual`).
