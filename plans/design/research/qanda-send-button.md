# Design: qanda-send-button

## 한 줄 정의
Q&A 인스펙터 탭에서 Send 버튼을 클릭하면 `qa_input`의 내용이 Q&A 메시지로 전송된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Q&A send button | `Button` | `research.qa.send` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
