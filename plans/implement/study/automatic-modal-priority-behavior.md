# Implement: automatic-modal-priority-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 여러 모달이 동시에 열릴 수 없도록, 새 모달이 열리면 기존 모달이 자동으로 닫히고 paint 순서가 항상 정해진 우선순위대로 실행된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/mod.rs` (paint 순서) | profile setup → authoring → stage surfaces → modals → shortcut help → goal → hamburger → focus indicator 순서 보장 | ``fn paint` 내 분기 순서` |
| `apps/study/src-tauri/src/ui/state.rs` (모달 열기) | 모달 열기 메서드에서 다른 모달 플래그를 false로 설정 | ``fn open_stats`, `fn toggle_shortcut_help`, `fn toggle_hamburger_menu` 등` |

## 필요한 변경 (의도 단위)
### 1. paint 순서 보장
- **입력**: `show_profile_setup_modal`, `show_authoring_panel`, `stage`, 각 모달 플래그
- **처리**: `paint`에서 profile setup이 true면 early return. 그 외에는 shell → outline → notes → authoring → stage surfaces → tutor → modals → shortcut help → goal → hamburger → focus indicator 순으로 실행. 각 모달은 독립적인 if 분기로 처리.
- **출력/사이드 이펙트**: 모달 간 z-order가 항상 일정하게 유지된다.
### 2. 모달 상호 배제
- **입력**: 모달 열기 트리거(버튼 클릭, 키보드 단축키)
- **처리**: 모달 열기 시 다른 모달 플래그를 명시적으로 false로 설정한다. 예: `open_stats`에서 `show_result_modal = false`, `show_shortcut_help = false`, `show_goal_modal = false`.
- **출력/사이드 이펙트**: 두 개 이상의 모달이 동시에 열리지 않는다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, paint 함수 내 분기 순서로 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
