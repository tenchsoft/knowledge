# Design: new-curriculum-authoring-button

## 한 줄 정의
저작 패널에서 New Curriculum 버튼을 클릭하면 빈 커리큘럼 초안이 생성되고 저작 필드가 초기화된다. 기존 저작 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| New curriculum button | `Button` | `study.authoring.new_curriculum` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 커리큘럼 편집 (별도 spec).
- 저작 패널 전체 (별도 design).
