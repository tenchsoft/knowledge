# Design: annotation-list-row-control

## 한 줄 정의
PDF 주석 목록에서 주석 행을 클릭하면 해당 주석이 활성화되고 PDF 뷰포트가 해당 위치로 이동한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Annotation row | `Button` | `research.annotation.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
