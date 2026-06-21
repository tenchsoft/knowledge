# Design: math-palette-toggle-button

## 한 줄 정의
수학 팔레트 토글 버튼을 클릭하면 팔레트가 표시/숨김된다. 기존 토글 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Math palette toggle | `Button` | `study.practice.math_palette.toggle` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 수학 팔레트 렌더 (별도 background `automatic-math-palette-render-behavior`).
- 개별 기호 버튼 (별도 spec).
