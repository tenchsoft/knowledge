# Spec: concept-row-selection-control

## 한 줄 정의
사용자가 Study에서 Concept Row Selection Control을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this outline panel control has its own target. When the user clicks a concept row, the active unit/concept changes, practice progress resets, and the detail surface refreshes.

## 성공 조건 (Acceptance Criteria)
- [ ] Use it in the normal visible state and confirm the displayed state changes immediately.
- [ ] Use it again or at a boundary state and confirm the state does not drift.

## 실패 / 취소 흐름
- 컨트롤이 비활성화 상태면 클릭해도 반응 없다.
- 다른 모달이 활성 중이면 입력이 무시된다.

## 경계 / 예외
- Use it again or at a boundary state and confirm the state does not drift.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
