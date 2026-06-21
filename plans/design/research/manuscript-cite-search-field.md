# Design: manuscript-cite-search-field

## 한 줄 정의
Write 인스펙터 탭에서 인용 검색 필드에 타이핑하면 인용 결과가 실시간으로 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Cite search input | `TextInput` | `research.manuscript.cite_search` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
