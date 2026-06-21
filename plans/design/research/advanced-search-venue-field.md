# Design: advanced-search-venue-field

## 한 줄 정의
고급 검색 패널에서 Venue 필드에 타이핑하면 `advanced_search.venue_query`가 업데이트되고 논문 목록이 학회/저널 기준으로 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Venue input | `TextInput` | `research.advanced.venue` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
