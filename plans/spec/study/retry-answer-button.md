# Spec: retry-answer-button

## 한 줄 정의
사용자가 Study에서 Retry Answer Button을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this Practice feedback control has its own target. When the user clicks Retry, feedback clears and the same problem can be answered again.

## 성공 조건 (Acceptance Criteria)
- [ ] Use it in the normal visible state and confirm the displayed state changes immediately.
- [ ] Use it again or at a boundary state and confirm the state does not drift.

## 실패 / 취소 흐름
- 조건이 충족되지 않으면 동작이 발동하지 않는다.
- 다른 모달/오버레이가 활성 중이면 무시된다.

## 경계 / 예외
- Use it again or at a boundary state and confirm the state does not drift.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
