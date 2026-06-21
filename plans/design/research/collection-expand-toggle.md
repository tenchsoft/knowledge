# Design: collection-expand-toggle

## 한 줄 정의
좌측 라이브러리 패널에서 컬렉션 확장 아이콘을 클릭하면 컬렉션 행이 펼쳐지거나 접히고 선택 상태는 유지된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Collection expand icon | `Button` | `research.collection.{index}.expand` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
