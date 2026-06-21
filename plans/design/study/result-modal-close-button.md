# Design: result-modal-close-button

## 한 줄 정의
세션 결과 모달 닫기 버튼을 클릭하면 모달이 닫힌다. 기존 모달 닫기 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Close button | `Button` | `study.modal.result.close` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 결과 모달 내용 (별도 design `study-modals`).
- 세션 결과 계산 (별도 background).
