# Design: curriculum-search-box

## 한 줄 정의
커리큘럼 검색 상자에 텍스트를 입력하면 `search_query`가 갱신되고 커서가 이동하며 일치 수가 업데이트된다. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Search input | `TextInput` | `study.curriculum.search` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 검색 결과 필터링 (별도 background).
- 검색 일치 수 (별도 background `automatic-search-match-count-behavior`).
