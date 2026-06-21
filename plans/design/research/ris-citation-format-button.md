# Design: ris-citation-format-button

## 한 줄 정의
Cite 인스펙터 탭에서 RIS 포맷 버튼을 클릭하면 인용 스타일이 RIS로 전환된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| RIS format button | `Button` | `research.citation.format.ris` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
