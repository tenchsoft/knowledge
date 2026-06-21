# Design: unit-expand-collapse-row

## 한 줄 정의
커리큘럼 단원 헤더 행을 클릭하면 해당 단원이 확장/축소되고 하위 개념 행이 표시/숨김된다. 기존 커리큘럼 행 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Unit header | `Button` | `study.unit.{idx}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 개념 행 선택 (별도 spec `concept-row-selection-control`).
- 가상 스크롤 (별도 background `automatic-outline-virtual-scroll-behavior`).
