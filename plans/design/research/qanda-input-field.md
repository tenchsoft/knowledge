# Design: qanda-input-field

## 한 줄 정의
Q&A 인스펙터 탭에서 입력 필드를 클릭하면 포커스가 이동하고 타이핑한 텍스트가 `qa_input`을 편집한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Q&A input | `TextInput` | `research.qa.input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
