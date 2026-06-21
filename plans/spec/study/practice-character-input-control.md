# Spec: practice-character-input-control

## 한 줄 정의
사용자가 Study에서 Practice Character Input Control을/를 입력하여 수행한다.

## 진입점
- 입력: 해당 필드에 포커스 후 타이핑

## 사용자 흐름
1. 사용자가 Practice Character Input Control 컨트롤을 확인한다.
2. 사용자가 해당 컨트롤을 활성화한다.
3. 화면에 결과가 즉시 반영된다.

## 성공 조건 (Acceptance Criteria)
- [ ] Press ordinary character keys in Practice in the intended stage and verify the action runs.
- [ ] Press ordinary character keys in Practice while a text input owns focus and verify text-edit precedence is respected.

## 실패 / 취소 흐름
- 필드가 비활성화되면 입력이 무시된다.
- 다른 모달이 활성 중이면 입력이 해당 모달로 라우팅된다.

## 경계 / 예외
- 같은 동작을 연속으로 수행해도 상태가 일관성 있게 유지된다.
- 빈 입력/미선택 상태에서 동작 시 에러 없이 처리된다.

## 범위 외
- 관련된 다른 기능은 별도 spec으로 분리.
