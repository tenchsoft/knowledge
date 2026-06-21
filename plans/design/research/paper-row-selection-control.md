# Design: paper-row-selection-control

## 한 줄 정의
논문 행을 클릭하면 해당 논문이 선택되고 이전 다중 선택이 해제되며 상세/인스펙터 패널이 갱신된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Paper row | `Button` | `research.paper.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
