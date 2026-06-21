# Design: advanced-search-tag-field

## 한 줄 정의
고급 검색 패널에서 Tag 필드에 타이핑하면 태그 기준으로 논문 목록이 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Tag input | `TextInput` | `research.advanced.tag` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
