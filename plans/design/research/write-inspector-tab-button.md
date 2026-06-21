# Design: write-inspector-tab-button

## 한 줄 정의
인스펙터 Write 탭 버튼을 클릭하면 Write 탭 콘텐츠가 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Write tab button | `Button` | `research.inspector.tab.write` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
