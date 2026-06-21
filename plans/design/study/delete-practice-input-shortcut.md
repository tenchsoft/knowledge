# Design: delete-practice-input-shortcut

## 한 줄 정의
Practice 모드에서 Delete 키로 커서 뒤의 답안 텍스트를 삭제. 신규 시각 요소 없음 — 기존 TextInput 동작.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Answer input | `TextInput` | `study.practice.answer_input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- Backspace 키 (별도 spec `backspace-practice-input-shortcut`).
- 답안 입력 필드 전체 (별도 spec `practice-answer-input-field`).
