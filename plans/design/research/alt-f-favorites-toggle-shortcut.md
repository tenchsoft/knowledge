# Design: alt-f-favorites-toggle-shortcut

## 한 줄 정의
Alt+F 단축키로 즐겨찾기 필터를 토글하여 논문 목록을 즐겨찾기 논문만 표시하거나 전체 표시로 전환한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Favorites toggle (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 헤더 상태 텍스트와 논문 목록 필터링으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
