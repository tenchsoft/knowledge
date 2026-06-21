# Design: welcome-get-started-button

## 한 줄 정의
환영 오버레이에서 Get Started 버튼을 클릭하면 오버레이가 닫히고 `show_welcome`이 false가 되며 라이브러리 작업 공간이 활성화된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Get Started button | `Button` | `research.welcome.get_started` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
