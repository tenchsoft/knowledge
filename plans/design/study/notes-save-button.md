# Design: notes-save-button

## 한 줄 정의
노트 오버레이에서 Save 버튼을 클릭하면 활성 개념에 노트가 생성되고 `notes_input`이 초기화되며 노트 목록이 갱신된다. 기존 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Save button | `Button` | `study.notes.save` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 노트 입력 (별도 spec `notes-input-field`).
- 노트 패널 (별도 design `study-notes`).
