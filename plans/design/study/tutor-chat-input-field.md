# Design: tutor-chat-input-field

## 한 줄 정의
AI 튜터 채팅 입력 필드에서 텍스트를 입력하면 `tutor_input`이 갱신된다. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Chat input | `TextInput` | `study.tutor.input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 채팅 메시지 전송 (별도 spec `tutor-chat-send-button`).
- AI 튜터 (별도 spec).
