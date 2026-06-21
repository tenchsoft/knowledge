# Design: new-concept-authoring-button

## 한 줄 정의
저작 워크플로우에서 New Concept 버튼을 클릭하면 활성 단원에 개념 초안이 추가되고 선택된다. 기존 저작 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| New concept button | `Button` | `study.authoring.new_concept` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 개념 편집 (별도 spec).
- 저작 패널 전체 (별도 design).
