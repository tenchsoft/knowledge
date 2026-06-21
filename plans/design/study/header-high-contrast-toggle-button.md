# Design: header-high-contrast-toggle-button

## 한 줄 정의
헤더 고대비 토글 버튼을 클릭하면 고대비 모드가 전환된다. 기존 헤더 토글 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| HC toggle | `Button` | `study.header.high_contrast` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 고대비 스타일링 (별도 background `automatic-high-contrast-styling-behavior`).
- 색맹 모드 (별도 spec).
