# Design: qanda-limitations-quick-action-button

## 한 줄 정의
Q&A 인스펙터 탭에서 Limitations 빠른 액션 버튼을 클릭하면 "limitations" 프롬프트가 전송된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Limitations quick action | `Button` | `research.qa.quick.limitations` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
