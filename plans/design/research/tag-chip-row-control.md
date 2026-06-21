# Design: tag-chip-row-control

## 한 줄 정의
좌측 라이브러리 패널에서 태그 칩을 클릭하면 태그 텍스트가 `search_query`에 복사되고 해당 태그의 논문만 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Tag chip | `Button` | `research.tag.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
