# Spec: automatic-tutor-weak-points-behavior

## 한 줄 정의
Study에서 Tutor Weak Points Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. Without any direct user action, Automatic Tutor Weak Points Behavior should derive weak point labels from recent review items and show a no-weak-points state when empty.

## 성공 조건 (Acceptance Criteria)
- [ ] Render it when the underlying state is empty, partial, and populated.
- [ ] Change upstream state and verify the UI updates on the next paint.

## 실패 / 취소 흐름
- Render it when the underlying state is empty, partial, and populated.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
