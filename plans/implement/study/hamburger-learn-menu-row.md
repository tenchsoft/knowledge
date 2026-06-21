# Implement: hamburger-learn-menu-row

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 햄버거 메뉴에서 Learn 행 클릭 시 해당 스테이지로 전환한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| curriculum.rs | paint_hamburger_menu 렌더링 | grep 'paint_hamburger_menu' apps/study/ |
| state.rs | stage 필드 설정 | grep 'pub stage:' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Learn 메뉴 행 클릭
- **입력**: PointerEvent::Down on learn menu row
- **처리**: state.stage = Stage::Learn
- **출력/사이드 이펙트**: Learn 스테이지 전환, 활성 행 하이라이트


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.hamburger.learn | button | learn | show_hamburger_menu |

## 의존
- 선행 implement: mobile-hamburger-menu-button

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
