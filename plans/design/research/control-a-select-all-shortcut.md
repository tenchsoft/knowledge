# Design: control-a-select-all-shortcut

## 한 줄 정의
Ctrl+A 단축키로 표시된 모든 논문을 선택한다. 입력 필드에 포커스가 있으면 동작하지 않는다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Select all (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 논문 목록 전체 선택 하이라이트로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
