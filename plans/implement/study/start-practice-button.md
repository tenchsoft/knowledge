# Implement: start-practice-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Learn 화면에서 Start Practice 버튼 클릭 시 Practice 스테이지로 전환하고 첫 문제를 표시한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::StartPractice 핸들링 | grep 'StudyHit::StartPractice' apps/study/ |
| state.rs | start_practice 메서드 | grep 'fn start_practice' apps/study/ |
| learn.rs | start_practice_rect 렌더링 | grep 'fn start_practice_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on start_practice_rect
- **처리**: start_practice() 호출
- **출력/사이드 이펙트**: stage=Practice, problem_index=1, feedback=None, input_text clear, session_paused=false


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.learn.start_practice | button | start practice | stage == Learn |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
