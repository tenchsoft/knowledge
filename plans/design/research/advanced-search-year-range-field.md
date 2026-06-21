# Design: advanced-search-year-range-field

## 한 줄 정의
고급 검색 패널에서 Year range 필드에 타이핑하면 `year_from`/`year_to`가 파싱되고 논문 목록이 연도 범위로 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Year range input | `TextInput` | `research.advanced.year` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
