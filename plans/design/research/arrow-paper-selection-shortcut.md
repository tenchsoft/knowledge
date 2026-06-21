# Design: arrow-paper-selection-shortcut

## 한 줄 정의
상/하 화살표 키로 논문 목록에서 선택된 논문을 이동한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Arrow selection (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 논문 목록 선택 하이라이트로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
