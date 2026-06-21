# Design: bookmark-concept-toggle-button

## 한 줄 정의
북마크 토글 버튼을 클릭하면 활성 개념의 북마크 상태가 전환된다. 기존 커리큘럼 북마크 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Bookmark toggle | `Button` | `study.curriculum.bookmark` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 북마크 필터링 (별도 spec).
- 북마크 표시 (별도 background `automatic-bookmark-indicator-behavior`).
