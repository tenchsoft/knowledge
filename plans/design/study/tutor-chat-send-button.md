# Design: tutor-chat-send-button

## 한 줄 정의
AI 튜터 채팅에서 Send 버튼을 클릭하면 메시지가 전송된다. 기존 채팅 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Send button | `Button` | `study.tutor.send` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 채팅 메시지 렌더 (별도 background `automatic-tutor-chat-message-render-behavior`).
- AI 튜터 (별도 spec).
