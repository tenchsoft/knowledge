# Design: advanced-search-toggle-button

## 한 줄 정의
헤더 검색 영역에서 고급 검색 토글 버튼을 클릭하면 고급 검색 패널이 열리거나 닫히고 화살표 방향이 전환된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Advanced search toggle | `Button` | `research.header.advanced_search` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
