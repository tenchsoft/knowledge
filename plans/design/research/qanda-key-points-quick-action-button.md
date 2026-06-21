# Design: qanda-key-points-quick-action-button

## 한 줄 정의
Q&A 인스펙터 탭에서 Key Points 빠른 액션 버튼을 클릭하면 "key points" 프롬프트가 전송된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Key Points quick action | `Button` | `research.qa.quick.key_points` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
