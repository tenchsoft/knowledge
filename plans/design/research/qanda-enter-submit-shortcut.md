# Design: qanda-enter-submit-shortcut

## 한 줄 정의
Q&A 입력 필드에 포커스된 상태에서 Enter를 누르면 Q&A Send 버튼과 동일한 경로로 메시지가 전송된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Q&A Enter submit (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 Q&A 말풍선 추가로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
