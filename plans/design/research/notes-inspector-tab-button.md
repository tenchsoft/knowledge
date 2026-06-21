# Design: notes-inspector-tab-button

## 한 줄 정의
인스펙터 Notes 탭 버튼을 클릭하면 Notes 탭 콘텐츠가 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Notes tab button | `Button` | `research.inspector.tab.notes` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
