# Design: profile-wizard-escape-key-control

## 한 줄 정의
프로필 마법사가 열린 상태에서 Escape 키를 누르면 이전 단계로 돌아가거나 첫 단계에서 유지된다. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Back button | `Button` | `study.profile.back` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- Enter 키 (별도 spec `profile-wizard-enter-key-control`).
