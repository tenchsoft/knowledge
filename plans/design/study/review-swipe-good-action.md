# Design: review-swipe-good-action

## 한 줄 정의
모바일에서 우측 스와이프로 Good 평가를 적용. 기존 스와이프 제스처 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Good button (fallback) | `Button` | `study.review.rating.good` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 다른 스와이프 액션 (별도 spec).
- 스와이프 임계값 (별도 design `study-review`).
