# Implement: header-shortcut-help-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더의 단축키 도움말 버튼 클릭 시 단축키 모달이 토글된다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ShortcutHelp 핸들링 | grep 'StudyHit::ShortcutHelp' apps/study/ |
| state.rs | toggle_shortcut_help 메서드 | grep 'fn toggle_shortcut_help' apps/study/ |
| curriculum.rs | hit_test 내 shortcut_btn | grep 'shortcut_btn' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on shortcut_btn rect
- **처리**: toggle_shortcut_help() 호출
- **출력/사이드 이펙트**: show_shortcut_help 토글


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.header.shortcuts | button | shortcuts | 항상 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
