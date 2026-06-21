# Design: practice-answer-input-field

## 한 줄 정의
Practice 모드의 답안 입력 필드에서 텍스트를 입력하면 `input_text`가 갱신된다. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Answer input | `TextInput` | `study.practice.answer_input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 답안 제출 (별도 spec `submit-answer-button`).
- Practice surface (별도 design `study-learn-area`).
