# Design: escape-close-focus-or-overlay-shortcut

## 한 줄 정의
Escape 키로 최우선 순위의 열린 오버레이 또는 포커스된 입력을 닫고, 없으면 검색을 초기화하고 reader mode를 Detail로 복귀한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Escape dismiss (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 오버레이 닫힘 / 포커스 해제 / 검색 초기화로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
