# Design: retry-answer-button

## 한 줄 정의
Practice 피드백에서 Retry 버튼을 클릭하면 같은 문제에 대해 답안을 다시 입력할 수 있다. 기존 Practice 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Retry button | `Button` | `study.practice.retry` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 답안 제출 (별도 spec `submit-answer-button`).
- Practice surface (별도 design `study-learn-area`).
