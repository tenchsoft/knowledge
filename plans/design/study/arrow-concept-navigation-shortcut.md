# Design: arrow-concept-navigation-shortcut

## 한 줄 정의
ArrowUp/ArrowDown 키로 커리큘럼 개념 간 이동. Practice 모드에서는 답안 커서 이동. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Concept row (target) | `Button` | `study.concept.{unit}.{concept}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- Practice 모드 답안 커서 (별도 spec).
- Page Up/Down (별도 spec).
