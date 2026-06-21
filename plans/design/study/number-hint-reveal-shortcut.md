# Design: number-hint-reveal-shortcut

## 한 줄 정의
숫자 키(1, 2, 3)로 해당 레벨의 힌트를 즉시 공개. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Hint 1 button | `Button` | `study.tutor.hint.1` |
| Hint 2 button | `Button` | `study.tutor.hint.2` |
| Hint 3 button | `Button` | `study.tutor.hint.3` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 힌트 내용 생성 (별도 spec).
- 힌트 버튼 클릭 (별도 spec `hint-level-*-button`).
