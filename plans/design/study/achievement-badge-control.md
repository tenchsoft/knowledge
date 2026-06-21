# Design: achievement-badge-control

## 한 줄 정의
목표 모달에서 성취 뱃지를 클릭하면 뱃지 상세 정보가 선택된다. 신규 시각 요소 없음 — 기존 목표 모달 내 뱃지 컴포넌트 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Achievement badge | `Button` | `study.modal.goal.badge.{idx}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 뱃지 잠금 해제 진행도 (별도 spec).
- 뱃지 알림 (별도 spec).
