# Design: profile-back-button

## 한 줄 정의
프로필 설정 마법사에서 Back 버튼을 클릭하면 이전 단계로 돌아가며 입력값이 유지된다. 기존 마법사 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Back button | `Button` | `study.profile.back` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- Next/Start 버튼 (별도 spec).
