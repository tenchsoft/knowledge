# Design: save-search-button

## 한 줄 정의
고급 검색 패널에서 Save Search 버튼을 클릭하면 현재 검색 조건이 저장된 검색 행으로 추가되고 "Search saved" 토스트가 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Save search button | `Button` | `research.advanced.save_search` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
