# Design: toast-dismiss-control

## 한 줄 정의
토스트를 클릭하면 해당 토스트가 토스트 스택에서 제거되고 나머지 토스트가 예측 가능하게 이동한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Toast item | `Label` | `research.toast.{index}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
