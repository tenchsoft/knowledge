# Design: status-filter-row-control

## 한 줄 정의
좌측 라이브러리 패널에서 상태 필터 행을 클릭하면 해당 읽기 상태(unread, reading, reviewed, archived)로 논문 목록이 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Status filter row | `Button` | `research.status_filter.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
