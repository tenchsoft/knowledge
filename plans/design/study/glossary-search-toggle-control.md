# Design: glossary-search-toggle-control

## 한 줄 정의
용어집 검색 토글을 클릭하면 검색 필드가 표시/숨김된다. 기존 토글 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Search toggle | `Button` | `study.glossary.search_toggle` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 용어집 검색 필드 (별도 spec `glossary-search-field`).
- 용어집 패널 전체 (별도 design).
