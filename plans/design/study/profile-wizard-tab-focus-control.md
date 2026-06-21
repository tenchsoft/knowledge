# Design: profile-wizard-tab-focus-control

## 한 줄 정의
Identity 단계에서 Tab 키를 누르면 Learner ID와 Display Name 필드 간 포커스가 이동된다. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Learner ID field | `TextInput` | `study.profile.learner_id` |
| Display name field | `TextInput` | `study.profile.display_name` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- Shift+Tab 역방향 (별도 spec).
