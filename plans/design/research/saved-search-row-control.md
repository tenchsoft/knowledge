# Design: saved-search-row-control

## 한 줄 정의
좌측 라이브러리 패널에서 저장된 검색 행을 클릭하면 저장된 쿼리와 고급 검색 조건이 복원되고 일치하는 논문이 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Saved search row | `Button` | `research.saved_search.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
