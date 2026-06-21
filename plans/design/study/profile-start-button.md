# Design: profile-start-button

## 한 줄 정의
프로필 설정 마법사에서 Start 버튼을 클릭하면 프로필이 저장되고 학습이 시작된다. 기존 마법사 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Start button | `Button` | `study.profile.start` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- 프로필 저장 (별도 spec).
