# Design: add-manuscript-section-button

## 한 줄 정의
Write 인스펙터 탭에서 Add Section 버튼을 클릭하면 새 원고 섹션이 추가되고 활성화된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Add Section button | `Button` | `research.manuscript.add_section` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
