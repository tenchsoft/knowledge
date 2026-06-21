# Spec: alt-f-favorites-toggle-shortcut

## 한 줄 정의
사용자가 Research에서 Alt F Favorites Toggle Shortcut을/를 단축키로 수행한다.

## 진입점
- 단축키: Alt+F

## 사용자 흐름
1. From the user's perspective, this keyboard controls control is independent and must not be merged with adjacent controls. When the user presses Alt+F, favorites_only toggles and the paper list filters to favorite references when enabled.

## 성공 조건 (Acceptance Criteria)
- [ ] Activate Alt F Favorites Toggle Shortcut when the required state is available; the visible result happens immediately and repaint follows.
- [ ] Activate Alt F Favorites Toggle Shortcut when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.
- [ ] Activate Alt F Favorites Toggle Shortcut while focus is in another input or panel; focus ownership and overlay priority remain deterministic.
- [ ] Repeat Alt F Favorites Toggle Shortcut quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 실패 / 취소 흐름
- Activate Alt F Favorites Toggle Shortcut when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.

## 경계 / 예외
- Repeat Alt F Favorites Toggle Shortcut quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
