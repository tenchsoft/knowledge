# Design: doi-enter-fetch-shortcut

## 한 줄 정의
DOI 입력 필드에 포커스된 상태에서 Enter를 누르면 Fetch 버튼과 동일한 경로로 DOI/arXiv 조회가 실행된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| DOI Enter fetch (shortcut-driven) | — | — (키보드 단축키, 별도 컴포넌트 없음) |

시각적 변화는 토스트 및 인용 결과로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
