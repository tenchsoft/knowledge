# Design: control-f-search-shortcut

## 한 줄 정의
Ctrl+F 단축키로 헤더 검색 필드에 포커스를 이동한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Search focus (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 검색 필드 포커스로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
