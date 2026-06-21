# Design: skip-problem-button

## 한 줄 정의
Practice에서 Skip 버튼을 클릭하면 현재 문제를 건너뛰고 다음 문제로 이동한다. 기존 Practice 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Skip button | `Button` | `study.practice.skip` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 문제 건너뛰기 로직 (별도 spec).
- Practice surface (별도 design `study-learn-area`).
