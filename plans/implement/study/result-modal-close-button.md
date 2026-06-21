# Implement: result-modal-close-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 세션 결과 모달의 Close 버튼 클릭으로 모달을 닫는다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::CloseModal 핸들링 | grep 'StudyHit::CloseModal' apps/study/ |
| state.rs | close_modals 메서드 | grep 'fn close_modals' apps/study/ |
| tutor.rs | modal_close_rect 렌더링 | grep 'fn modal_close_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 항들링
- **입력**: PointerEvent::Down on modal_close_rect
- **처리**: close_modals() 호출
- **출력/사이드 이펙트**: show_result_modal=false, 마지막 문제 시 stage=Learn


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.modal.close | button | close | show_result_modal |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
