# Implement: skip-problem-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 Skip 버튼 클릭 시 현재 문제를 스킵하고 오답으로 리뷰 큐에 추가한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::SkipProblem 핸들링 | grep 'StudyHit::SkipProblem' apps/study/ |
| state.rs | skip_problem 메서드 | grep 'fn skip_problem' apps/study/ |
| practice.rs | skip_rect 렌더링 | grep 'fn skip_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on skip_rect
- **처리**: skip_problem() 호출
- **출력/사이드 이펙트**: review_queue에 (skipped) 오답 추가, next_problem() 호출


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.skip | button | skip | stage == Practice |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
