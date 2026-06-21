# Design: authoring-body-field

## 한 줄 정의
저작 패널의 본문 입력 필드에서 텍스트를 입력하면 `authoring_body`가 갱신된다. 여러 줄 입력 가능. 기존 TextInput 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Body field | `TextInput` | `study.authoring.body` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 리치 텍스트 편집 (별도 spec).
- 미리보기 (별도 spec).
