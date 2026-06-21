# Design: header-stage-pill-button

## 한 줄 정의
헤더 스테이지 필 버튼을 클릭하면 Learn → Practice → Review 순으로 스테이지가 순환되며 surface가 변경된다. 기존 헤더 필 버튼 재사용.

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
