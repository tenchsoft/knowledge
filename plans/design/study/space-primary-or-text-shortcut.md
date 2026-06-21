# Design: space-primary-or-text-shortcut

## 한 줄 정의
Space 키로 현재 컨텍스트에 따라 주요 액션을 실행하거나 텍스트에 공백을 입력. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Answer input | `TextInput` | `study.practice.answer_input` |
| Submit button | `Button` | `study.practice.submit` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- Enter 키 (별도 spec `enter-primary-action-shortcut`).
- 답안 입력 (별도 spec `practice-answer-input-field`).
