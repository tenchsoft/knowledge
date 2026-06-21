# Design: advanced-search-author-field

## 한 줄 정의
고급 검색 패널에서 Author 필드에 타이핑하면 `advanced_search.author_query`가 업데이트되고 논문 목록이 저자 기준으로 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Author input | `TextInput` | `research.advanced.author` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
