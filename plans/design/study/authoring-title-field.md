# Design: authoring-title-field

## 한 줄 정의
저작 패널의 제목 입력 필드에서 텍스트를 입력하면 `authoring_title`이 갱신되고 placeholder가 사라진다. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Title field | `TextInput` | `study.authoring.title` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 제목 유효성 검사 (별도 spec).
- 자동 완성 (별도 spec).
