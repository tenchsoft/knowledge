# Design: concept-row-selection-control

## 한 줄 정의
커리큘럼 개념 행을 클릭하면 활성 단원/개념이 변경되고 Practice 진행률이 초기화되며 상세 surface가 갱신된다. 기존 커리큘럼 행 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Concept row | `Button` | `study.concept.{unit}.{concept}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 가상 스크롤 (별도 background `automatic-outline-virtual-scroll-behavior`).
- 단원 확장/축소 (별도 spec `unit-expand-collapse-row`).
