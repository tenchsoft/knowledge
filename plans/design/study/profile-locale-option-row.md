# Design: profile-locale-option-row

## 한 줄 정의
프로필 설정 마법사의 언어/지역 옵션 행. 기존 옵션 행 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Locale option | `Button` | `study.profile.locale.{idx}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 프로필 마법사 전체 (별도 design).
- 다른 프로필 필드 (별도 spec).
