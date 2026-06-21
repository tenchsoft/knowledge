# Spec: automatic-header-status-text-behavior

## 한 줄 정의
Research에서 Header Status Text Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. Without any direct user action, Automatic Header Status Text Behavior should show import status, reader mode, and favorites status whenever there is enough header width.

## 성공 조건 (Acceptance Criteria)
- [ ] Automatic Header Status Text Behavior updates when its source state changes without requiring a separate user command.
- [ ] Automatic Header Status Text Behavior handles empty, loading, and populated states without shifting adjacent controls unexpectedly.
- [ ] Automatic Header Status Text Behavior stays synchronized after filter, selection, import, export, or reader-mode changes.
- [ ] Automatic Header Status Text Behavior remains readable on mobile, tablet, and desktop panel widths.

## 실패 / 취소 흐름
- Automatic Header Status Text Behavior handles empty, loading, and populated states without shifting adjacent controls unexpectedly.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
