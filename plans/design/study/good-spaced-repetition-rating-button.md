# Design: good-spaced-repetition-rating-button

## 한 줄 정의
복습 카드에서 Good 버튼을 클릭하면 현재 복습 항목에 Good 평가가 적용된다. 기존 review rating 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Good button | `Button` | `study.review.rating.good` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- SM-2 알고리즘 (별도 spec `spaced-repetition-scheduling`).
- 다른 평가 버튼 (별도 spec).
