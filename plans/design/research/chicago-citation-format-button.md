# Design: chicago-citation-format-button

## 한 줄 정의
Cite 인스펙터 탭에서 Chicago 포맷 버튼을 클릭하면 인용 스타일이 Chicago로 전환된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Chicago format button | `Button` | `research.citation.format.chicago` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
