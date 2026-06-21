# Spec: control-r-open-review-queue-shortcut

## 한 줄 정의
사용자가 Study에서 Control R Open Review Queue Shortcut을/를 단축키로 수행한다.

## 진입점
- 단축키: (해당 단축키)

## 사용자 흐름
1. From the user's perspective, pressing Control+R should open the review queue and switch to Review stage.

## 성공 조건 (Acceptance Criteria)
- [ ] Press Control+R in the intended stage and verify the action runs.
- [ ] Press Control+R while a text input owns focus and verify text-edit precedence is respected.

## 실패 / 취소 흐름
- 모달/다이얼로그가 활성 중이면 단축키가 무시된다.
- 입력 필드에 포커스가 있으면 단축키가 입력으로 처리된다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
