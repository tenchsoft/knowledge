# Design: alt-p-reader-mode-toggle-shortcut

## 한 줄 정의
Alt+P 단축키로 `reader_mode`를 Detail과 PDF 사이에서 토글하여 중앙 패널 모드를 전환한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Reader mode toggle (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 중앙 패널 콘텐츠 전환으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
