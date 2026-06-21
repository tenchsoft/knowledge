# Design: header-search-field

## 한 줄 정의
헤더 검색 필드에 타이핑하면 `search_query`가 업데이트되고 논문 목록이 실시간으로 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Search input | `TextInput` | `research.header.search` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
