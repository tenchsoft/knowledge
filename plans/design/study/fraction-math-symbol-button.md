# Design: fraction-math-symbol-button

## 한 줄 정의
수학 팔레트에서 frac( 버튼을 클릭하면 커서 위치에 frac( 기호가 삽입된다. 기존 math palette 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Fraction button | `Button` | `study.practice.math_palette.frac` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 다른 수학 기호 버튼 (별도 spec).
- 수학 팔레트 레이아웃 (별도 design).
