# Design: header-export-button

## 한 줄 정의
헤더 액션 바에서 Export 버튼을 클릭하면 선택된 참고문헌이 내보내기 큐에 추가되고 토스트/상태가 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Export button | `Button` | `research.header.export` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
