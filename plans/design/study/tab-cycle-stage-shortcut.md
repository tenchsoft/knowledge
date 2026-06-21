# Design: tab-cycle-stage-shortcut

## 한 줄 정의
Tab 키로 스테이지를 순방향 전환, Shift+Tab으로 역방향 전환. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Stage pill | `Button` | `study.header.stage` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 스테이지 전환 애니메이션 (별도 spec).
- 각 스테이지 surface (별도 design).
