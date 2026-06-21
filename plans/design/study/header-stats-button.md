# Design: header-stats-button

## 한 줄 정의
헤더 통계 버튼을 클릭하면 통계 모달이 열린다. 기존 헤더 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Stats button | `Button` | `study.header.stats` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- Stats modal 내용 (별도 design `study-modals`).
- 통계 데이터 계산 (별도 background).
