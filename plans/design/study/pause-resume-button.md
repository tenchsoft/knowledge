# Design: pause-resume-button

## 한 줄 정의
Practice에서 일시정지/재개 버튼을 클릭하면 세션 타이머가 일시정지/재개된다. 기존 Practice 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Pause/Resume button | `Button` | `study.practice.pause` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 세션 타이머 (별도 background `automatic-session-timer-behavior`).
- Practice surface (별도 design `study-learn-area`).
