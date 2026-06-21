# Design: smart-collection-row-control

## 한 줄 정의
좌측 라이브러리 패널에서 스마트 컬렉션 행을 클릭하면 해당 스마트 컬렉션이 선택되고 규칙 기반 필터가 논문 목록에 적용된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Smart collection row | `Button` | `research.smart_collection.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
