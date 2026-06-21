# Implement: retry-answer-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 오답 후 Retry 버튼 클릭 시 입력을 초기화하고 같은 문제를 다시 풀 수 있다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::RetryAnswer 핸들링 | grep 'StudyHit::RetryAnswer' apps/study/ |
| state.rs | retry_answer 메서드 | grep 'fn retry_answer' apps/study/ |
| practice.rs | retry_rect 렌더링 | grep 'fn retry_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on retry_rect
- **처리**: retry_answer() 호출
- **출력/사이드 이펙트**: feedback=None, input_text clear, input_cursor_pos=0, pending_rating=None


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.retry | button | retry | stage == Practice && feedback.is_some() |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
