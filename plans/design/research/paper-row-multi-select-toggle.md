# Design: paper-row-multi-select-toggle

## 한 줄 정의
논문 행을 Ctrl+클릭하면 다중 선택이 토글되고 선택 상태가 업데이트된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Paper row (multi-select) | `Button` | `research.paper.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
