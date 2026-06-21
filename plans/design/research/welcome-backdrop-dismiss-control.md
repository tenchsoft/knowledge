# Design: welcome-backdrop-dismiss-control

## 한 줄 정의
환영 카드 외부를 클릭하면 환영 오버레이가 가져오기나 선택 변경 없이 닫힌다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Welcome backdrop | `Surface` | `research.welcome.backdrop` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
