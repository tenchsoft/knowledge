# Design: control-r-open-review-queue-shortcut

## 한 줄 정의
Ctrl+R 단축키로 복습 대기열을 열고 Review 스테이지로 전환. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Review queue button | `Button` | `study.review.queue` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 다른 단축키 (별도 spec).
- 스테이지 전환 애니메이션 (별도 spec).
