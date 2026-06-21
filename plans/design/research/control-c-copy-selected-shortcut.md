# Design: control-c-copy-selected-shortcut

## 한 줄 정의
Ctrl+C 단축키로 선택된 논문의 인용 텍스트를 현재 포맷으로 클립보드에 복사한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Copy selected (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 토스트 알림으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
