# Implement: next-problem-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 피드백 표시 후 Next 버튼 클릭 시 다음 문제로 이동하거나 세션 결과 모달을 표시한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::NextProblem 핸들링 | grep 'StudyHit::NextProblem' apps/study/ |
| state.rs | next_problem 메서드 | grep 'fn next_problem' apps/study/ |
| practice.rs | next_rect 렌더링 | grep 'fn next_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on next_rect
- **처리**: next_problem() 호출
- **출력/사이드 이펙트**: problem_index 증가, feedback=None, input_text clear, hint_level=0; 마지막 문제 시 show_result_modal=true


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.next | button | next problem | stage == Practice && feedback.is_some() |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
