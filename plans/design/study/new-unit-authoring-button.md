# Design: new-unit-authoring-button

## 한 줄 정의
저작 워크플로우에서 New Unit 버튼을 클릭하면 활성 커리큘럼에 단원 초안이 추가되고 편집 가능하다. 기존 저작 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| New unit button | `Button` | `study.authoring.new_unit` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 단원 편집 (별도 spec).
- 저작 패널 전체 (별도 design).
