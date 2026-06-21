# Design: glossary-term-expand-row

## 한 줄 정의
용어집 용어 행을 클릭하면 정의가 확장/축소된다. 기존 확장/축소 행 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Term row | `Button` | `study.glossary.term.{idx}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 용어집 검색 (별도 spec `glossary-search-field`).
- 용어집 편집 (별도 spec).
