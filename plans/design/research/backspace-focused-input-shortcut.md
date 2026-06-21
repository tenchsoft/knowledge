# Design: backspace-focused-input-shortcut

## 한 줄 정의
포커스된 입력 필드에서 Backspace를 누르면 이전 문자가 삭제되고 관련 필터가 업데이트된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Backspace in input (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 해당 입력 필드 텍스트 변경으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
