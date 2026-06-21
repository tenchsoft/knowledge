# Spec: enter-primary-action-shortcut

## 한 줄 정의
사용자가 Study에서 Enter Primary Action Shortcut을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, pressing Enter should run the primary action for the current stage: start practice, submit answer, advance feedback, or submit focused modal input.

## 성공 조건 (Acceptance Criteria)
- [ ] Press Enter in the intended stage and verify the action runs.
- [ ] Press Enter while a text input owns focus and verify text-edit precedence is respected.

## 실패 / 취소 흐름
- 조건이 충족되지 않으면 동작이 발동하지 않는다.
- 다른 모달/오버레이가 활성 중이면 무시된다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
