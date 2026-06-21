# Design: hint-level-2-button

## 한 줄 정의
튜터 패널에서 Hint 2 버튼을 클릭하면 `hint_level`이 2 이상으로 증가하고 두 번째 힌트 텍스트가 표시된다. 기존 힌트 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Hint 2 button | `Button` | `study.tutor.hint.2` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 힌트 내용 생성 (별도 spec).
- 다른 힌트 레벨 (별도 spec).
