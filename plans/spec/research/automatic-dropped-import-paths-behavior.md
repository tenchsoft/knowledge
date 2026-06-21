# Spec: automatic-dropped-import-paths-behavior

## 한 줄 정의
Research에서 Dropped Import Paths Behavior이 자동으로 동작한다.

## 진입점
- 자동: 편집, 스크롤, hover, paint, 상태 변경 시 자동 발동

## 사용자 흐름
1. Without any direct user action, Automatic Dropped Import Paths Behavior should deduplicate dropped PDF/RIS/BibTeX paths and move the app into import progress state.

## 성공 조건 (Acceptance Criteria)
- [ ] Automatic Dropped Import Paths Behavior updates when its source state changes without requiring a separate user command.
- [ ] Automatic Dropped Import Paths Behavior handles empty, loading, and populated states without shifting adjacent controls unexpectedly.
- [ ] Automatic Dropped Import Paths Behavior stays synchronized after filter, selection, import, export, or reader-mode changes.
- [ ] Automatic Dropped Import Paths Behavior remains readable on mobile, tablet, and desktop panel widths.

## 실패 / 취소 흐름
- Automatic Dropped Import Paths Behavior handles empty, loading, and populated states without shifting adjacent controls unexpectedly.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
