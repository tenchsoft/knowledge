# Design: page-up-previous-pdf-page-shortcut

## 한 줄 정의
PageUp 키로 PDF 리더에서 이전 페이지로 이동한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Page up (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 PDF 페이지 전환으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
