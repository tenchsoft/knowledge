# Design: review-concept-feedback-button

## 한 줄 정의
Practice 피드백에서 Review Concept 버튼을 클릭하면 해당 개념의 복습 정보가 표시된다. 기존 Practice 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Review concept button | `Button` | `study.practice.review_concept` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 복습 경로 (별도 design `study-review`).
- 피드백 표시 (별도 background `automatic-practice-feedback-behavior`).
