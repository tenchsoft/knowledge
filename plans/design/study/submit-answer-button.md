# Design: submit-answer-button

## 한 줄 정의
Practice에서 Submit 버튼을 클릭하면 답안이 채점되고 피드백이 표시되며 `session_results`가 갱신되고 오답은 복습 대기열에 추가된다. 기존 Practice 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Submit button | `Button` | `study.practice.submit` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 답안 채점 (별도 background `automatic-practice-feedback-behavior`).
- Practice surface (별도 design `study-learn-area`).
