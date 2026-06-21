# Design: start-practice-button

## 한 줄 정의
Learn surface에서 Start Practice 버튼을 클릭하면 Practice 스테이지로 전환되고 세션이 시작된다. 기존 Learn 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Start practice button | `Button` | `study.learn.start_practice` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- Practice surface (별도 design `study-learn-area`).
- 세션 타이머 (별도 background `automatic-session-timer-behavior`).
