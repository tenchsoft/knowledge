# Design: hamburger-learn-menu-row

## 한 줄 정의
모바일 햄버거 메뉴에서 Learn 항목을 선택하면 Learn 스테이지로 전환되고 메뉴가 닫힌다. 기존 메뉴 항목 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Learn menu row | `Button` | `study.menu.learn` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 햄버거 메뉴 전체 (별도 design).
- 다른 스테이지 메뉴 항목 (별도 spec).
