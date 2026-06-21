# Design: review-queue-button

## 한 줄 정의
커리큘럼 패널의 복습 대기열 버튼을 클릭하면 Review 스테이지로 전환되고 대기열이 열린다. 기존 커리큘럼 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Review queue button | `Button` | `study.review.queue` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 복습 대기열 (별도 design `study-review`).
- 스테이지 전환 (별도 spec `header-stage-pill-button`).
