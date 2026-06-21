# Design: sort-mode-button

## 한 줄 정의
논문 목록에서 정렬 모드 버튼을 클릭하면 정렬 기준이 순환된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Sort mode button | `Button` | `research.sort_mode` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
