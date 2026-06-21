# Design: save-draft-authoring-button

## 한 줄 정의
저작 패널에서 Save Draft 버튼을 클릭하면 현재 저작 내용이 임시 저장된다. 기존 저작 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Save draft button | `Button` | `study.authoring.save_draft` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 저작 패널 전체 (별도 design).
- 출판/최종 저장 (별도 spec).
