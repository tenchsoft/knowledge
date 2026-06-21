# Design: mobile-hamburger-menu-button

## 한 줄 정의
모바일에서 햄버거 메뉴 버튼을 클릭하면 메뉴가 열린다. 기존 햄버거 메뉴 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Hamburger button | `Button` | `study.menu.hamburger` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 햄버거 메뉴 내부 항목 (별도 spec).
- 데스크톱 네비게이션 (별도 design).
