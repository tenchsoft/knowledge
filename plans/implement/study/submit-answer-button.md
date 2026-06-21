# Implement: submit-answer-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 Submit 버튼 클릭 시 답안을 제출하고 채점 결과를 표시한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::SubmitAnswer 핸들링 | grep 'StudyHit::SubmitAnswer' apps/study/ |
| state.rs | submit_answer 메서드 | grep 'fn submit_answer' apps/study/ |
| practice.rs | submit_rect 렌더링 | grep 'fn submit_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on submit_rect
- **처리**: submit_answer() 호출
- **출력/사이드 이펙트**: tench_study_core::grade_answer로 채점, feedback=Some(correct), session_results push, 오답 시 review_queue push


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.submit | button | submit | stage == Practice && feedback.is_none() |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
