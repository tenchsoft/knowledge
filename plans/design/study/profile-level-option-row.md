# Design: profile-level-option-row

## 한 줄 정의
프로필 설정 마법사의 학습 수준 옵션 행. 기존 옵션 행 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Level option | `Button` | `study.profile.level.{idx}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- 다른 프로필 필드 (별도 spec).
