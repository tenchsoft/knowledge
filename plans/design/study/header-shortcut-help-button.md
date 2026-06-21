# Design: header-shortcut-help-button

## 한 줄 정의
헤더 단축키 도움말 버튼을 클릭하면 단축키 도움말 모달이 열린다. 기존 헤더 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Shortcuts button | `Button` | `study.header.shortcuts` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 단축키 도움말 모달 내용 (별도 design `study-modals`).
- 단축키 바인딩 (별도 spec).
