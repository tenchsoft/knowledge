# Design: profile-next-button

## 한 줄 정의
프로필 설정 마법사에서 Next 버튼을 클릭하면 Identity → Domain/Level → Locale 순으로 단계가 진행된다. 기존 마법사 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Next button | `Button` | `study.profile.next` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- Back 버튼 (별도 spec `profile-back-button`).
