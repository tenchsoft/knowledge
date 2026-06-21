# Spec: welcome-import-from-file-button

## 한 줄 정의
사용자가 Research에서 Welcome Import From File Button을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this welcome overlay control is independent and must not be merged with adjacent controls. When the user clicks the Import from file button, the welcome overlay closes, import_status becomes queued, reader mode moves to Importing, and import progress can render.

## 성공 조건 (Acceptance Criteria)
- [ ] Activate Welcome Import From File Button when the required state is available; the visible result happens immediately and repaint follows.
- [ ] Activate Welcome Import From File Button when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.
- [ ] Activate Welcome Import From File Button while focus is in another input or panel; focus ownership and overlay priority remain deterministic.
- [ ] Repeat Welcome Import From File Button quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 실패 / 취소 흐름
- Activate Welcome Import From File Button when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.

## 경계 / 예외
- Repeat Welcome Import From File Button quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
