# Design: insert-citation-result-button

## 한 줄 정의
Write 인스펙터 탭에서 Insert 버튼을 클릭하면 선택된 인용 키가 활성 섹션에 삽입되고 인용 수가 증가한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Insert citation result button | `Button` | `research.manuscript.cite_result.{index}.insert` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
