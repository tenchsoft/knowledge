# Design: authoring-panel-close-button

## 한 줄 정의
저작 패널 닫기 버튼을 클릭하면 패널이 닫힌다. 기존 모달/패널 닫기 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Close button | `Button` | `study.authoring.close` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 저장 확인 다이얼로그 (별도 spec).
- 저작 패널 전체 (별도 design).
