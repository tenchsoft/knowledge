# Design: glossary-search-field

## 한 줄 정의
용어집 검색 필드에 텍스트를 입력하면 용어집이 필터링된다. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Glossary search | `TextInput` | `study.glossary.search` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 용어집 필터링 동작 (별도 background `automatic-glossary-filter-behavior`).
- 용어집 패널 전체 (별도 design).
