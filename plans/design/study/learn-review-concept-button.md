# Design: learn-review-concept-button

## 한 줄 정의
Learn surface에서 Review Concept 버튼을 클릭하면 활성 개념의 복습 경로가 열리거나 명확한 피드백과 함께 Learn에 유지된다. 기존 Learn 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Review concept button | `Button` | `study.learn.review_concept` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 복습 경로 전체 (별도 design `study-review`).
- Learn surface (별도 design `study-learn-area`).
