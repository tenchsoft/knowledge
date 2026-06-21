# Design: shortcut-help-modal-close-button

## 한 줄 정의
단축키 도움말 모달 닫기 버튼을 클릭하면 모달이 닫힌다. 기존 모달 닫기 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Close button | `Button` | `study.modal.shortcut_help.close` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 단축키 도움말 모달 내용 (별도 design `study-modals`).
- 단축키 바인딩 (별도 spec).
