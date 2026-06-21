# Spec: tag-chip-row-control

## 한 줄 정의
사용자가 Research에서 Tag Chip Row Control을/를 클릭하여 수행한다.

## 진입점
- 클릭: 해당 버튼/컨트롤 클릭

## 사용자 흐름
1. From the user's perspective, this left library panel control is independent and must not be merged with adjacent controls. When the user clicks a tag chip, the tag text is copied into search_query and the paper list filters to matching tagged papers.

## 성공 조건 (Acceptance Criteria)
- [ ] Activate Tag Chip Row Control when the required state is available; the visible result happens immediately and repaint follows.
- [ ] Activate Tag Chip Row Control when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.
- [ ] Activate Tag Chip Row Control while focus is in another input or panel; focus ownership and overlay priority remain deterministic.
- [ ] Repeat Tag Chip Row Control quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 실패 / 취소 흐름
- Activate Tag Chip Row Control when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.

## 경계 / 예외
- Repeat Tag Chip Row Control quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
