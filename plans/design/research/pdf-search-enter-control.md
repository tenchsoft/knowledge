# Design: pdf-search-enter-control

## 한 줄 정의
PDF 검색 필드에 포커스된 상태에서 Enter를 누르면 활성 검색 결과가 다음 일치 항목으로 이동하고 뷰포트가 해당 위치를 표시한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| PDF search Enter (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 PDF 검색 결과 하이라이트 이동으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
