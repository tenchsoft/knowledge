# Design: header-goal-setting-button

## 한 줄 정의
헤더 목표 설정 버튼을 클릭하면 목표 모달이 열린다. 기존 헤더 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Goals button | `Button` | `study.header.goals` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 목표 모달 내용 (별도 design `study-modals`).
- 목표 달성 추적 (별도 spec).
