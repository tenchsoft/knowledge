# Design: control-shift-c-citation-format-shortcut

## 한 줄 정의
Ctrl+Shift+C 단축키로 `citation_format`을 순환하고 인용 상태를 업데이트한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Citation format cycle (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 Cite 탭 포맷 버튼 활성 상태 전환으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
