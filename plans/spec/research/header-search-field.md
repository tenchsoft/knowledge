# Spec: header-search-field

## 한 줄 정의
사용자가 Research에서 Header Search Field을/를 입력하여 수행한다.

## 진입점
- 입력: 해당 필드에 포커스 후 타이핑

## 사용자 흐름
1. From the user's perspective, this header search area control is independent and must not be merged with adjacent controls. When the user types in the search field, search_query updates, visible papers filter immediately, and selection moves to the first visible result if needed.

## 성공 조건 (Acceptance Criteria)
- [ ] From the user's perspective, this header search area control is independent and must not be merged with adjacent controls. When the user types in the search field, search_query updates, visible papers filter immediately, and selection moves to the first visible result if needed.
- [ ] Activate Header Search Field when the required state is available; the visible result happens immediately and repaint follows.
- [ ] Activate Header Search Field when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.
- [ ] Activate Header Search Field while focus is in another input or panel; focus ownership and overlay priority remain deterministic.
- [ ] Repeat Header Search Field quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 실패 / 취소 흐름
- Activate Header Search Field when required state is missing; the app shows a disabled, empty, or clear no-op state without corrupting library data.

## 경계 / 예외
- Repeat Header Search Field quickly; state changes remain idempotent and no duplicate unintended operation is queued.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
