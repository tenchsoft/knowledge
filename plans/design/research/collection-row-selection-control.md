# Design: collection-row-selection-control

## 한 줄 정의
좌측 라이브러리 패널에서 컬렉션 행 본체를 클릭하면 해당 컬렉션이 선택되고 논문 목록이 해당 컬렉션으로 필터링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Collection row | `Button` | `research.collection.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
