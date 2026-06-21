# Implement: mobile-hamburger-menu-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 모바일 뷰포트에서 햄버거 메뉴 버튼 클릭으로 사이드 메뉴를 연다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::HamburgerMenu 핸들링 | grep 'StudyHit::HamburgerMenu' apps/study/ |
| state.rs | toggle_hamburger_menu 메서드 | grep 'fn toggle_hamburger_menu' apps/study/ |
| curriculum.rs | hamburger hit test | grep 'hamburger' apps/study/src-tauri/src/ui/curriculum.rs |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on hamburger rect (viewport_class == Mobile)
- **처리**: toggle_hamburger_menu() 호출
- **출력/사이드 이펙트**: show_hamburger_menu 토글, 오버레이 메뉴 표시


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.hamburger.menu | button | menu | viewport_class == Mobile |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
