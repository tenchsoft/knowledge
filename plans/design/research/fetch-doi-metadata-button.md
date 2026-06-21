# Design: fetch-doi-metadata-button

## 한 줄 정의
Cite 인스펙터 탭에서 Fetch 버튼을 클릭하면 입력된 DOI/arXiv ID로 메타데이터 조회가 시작되고 "Fetching" 토스트가 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Fetch button | `Button` | `research.citation.fetch` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
