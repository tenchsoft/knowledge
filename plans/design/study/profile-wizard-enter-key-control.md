# Design: profile-wizard-enter-key-control

## 한 줄 정의
프로필 마법사가 열린 상태에서 Enter 키를 누르면 Next 또는 Start 버튼과 동일한 경로로 마법사가 진행된다. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Next button | `Button` | `study.profile.next` |
| Start button | `Button` | `study.profile.start` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- Tab/Escape 키 (별도 spec).
