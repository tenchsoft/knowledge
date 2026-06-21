# Spec: status-filter-row-control

## 한 줄 정의
사용자가 Research에서 Status Filter Row Control을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this left library panel control is independent and must not be merged with adjacent controls. When the user clicks a status filter row, the status label filters the paper list and selected paper remains visible.

## 성공 조건 (Acceptance Criteria)
- [ ] Activate Status Filter Row Control when the required state is available; the visible result happens immediately and repaint follows.
- [ ] Activate Status Filter Row Control when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.
- [ ] Activate Status Filter Row Control while focus is in another input or panel; focus ownership and overlay priority remain deterministic.
- [ ] Repeat Status Filter Row Control quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 실패 / 취소 흐름
- Activate Status Filter Row Control when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.

## 경계 / 예외
- Repeat Status Filter Row Control quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
