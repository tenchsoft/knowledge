# Design: question-mark-shortcut-help-key

## 한 줄 정의
? 키를 누르면 전역 단축키가 활성 상태일 때 단축키 도움말이 토글된다. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Shortcuts button | `Button` | `study.header.shortcuts` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 단축키 도움말 모달 (별도 design `study-modals`).
- 단축키 바인딩 (별도 spec).
