# Design: manuscript-section-row-control

## 한 줄 정의
Write 인스펙터 탭에서 원고 섹션 행을 클릭하면 해당 섹션이 활성화되고 미리보기/인용 컨트롤이 아래에 렌더링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Manuscript section row | `Button` | `research.manuscript.section.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
