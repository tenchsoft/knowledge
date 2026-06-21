# Design: notes-input-field

## 한 줄 정의
노트 오버레이의 입력 필드에서 텍스트를 입력하면 `notes_input`이 변경되며 Save 또는 Enter 전까지 미저장 상태다. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Notes input | `TextInput` | `study.notes.input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 노트 저장 (별도 spec `notes-save-button`).
- 노트 패널 (별도 design `study-notes`).
