# Spec: escape-close-focus-or-overlay-shortcut

## 한 줄 정의
사용자가 Research에서 Escape Close Focus Or Overlay Shortcut을/를 조작하여 수행한다.

## 진입점
- 해당 컨트롤 활성화

## 사용자 흐름
1. From the user's perspective, this keyboard controls control is independent and must not be merged with adjacent controls. When the user presses Escape, the highest-priority open overlay or focused input closes, otherwise search clears and reader mode returns to Detail.

## 성공 조건 (Acceptance Criteria)
- [ ] Activate Escape Close Focus Or Overlay Shortcut when the required state is available; the visible result happens immediately and repaint follows.
- [ ] Activate Escape Close Focus Or Overlay Shortcut when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.
- [ ] Activate Escape Close Focus Or Overlay Shortcut while focus is in another input or panel; focus ownership and overlay priority remain deterministic.
- [ ] Repeat Escape Close Focus Or Overlay Shortcut quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 실패 / 취소 흐름
- Activate Escape Close Focus Or Overlay Shortcut when the required state is available; the visible result happens immediately and repaint follows.
- Activate Escape Close Focus Or Overlay Shortcut when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.
- Activate Escape Close Focus Or Overlay Shortcut while focus is in another input or panel; focus ownership and overlay priority remain deterministic.
- Repeat Escape Close Focus Or Overlay Shortcut quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 경계 / 예외
- Repeat Escape Close Focus Or Overlay Shortcut quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
