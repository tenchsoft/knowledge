# Design: practice-character-input-control

## 한 줄 정의
Practice 모드의 문자 입력 컨트롤에서 개별 문자를 입력하면 해당 셀이 채워진다. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Character input | `TextInput` | `study.practice.char_input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 답안 제출 (별도 spec `submit-answer-button`).
- Practice surface (별도 design `study-learn-area`).
