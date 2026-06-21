# Design: control-i-import-shortcut

## 한 줄 정의
Ctrl+I 단축키로 가져오기를 시작하고 "Import started" 토스트를 표시한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Import shortcut (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 토스트 알림으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
