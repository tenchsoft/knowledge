# Design: minus-pdf-zoom-shortcut

## 한 줄 정의
PDF 모드에서 - 키를 누르면 PDF 축소가 실행된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| PDF zoom out (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 PDF 표면 축소로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
