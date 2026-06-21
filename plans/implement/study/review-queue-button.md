# Implement: review-queue-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 아웃라인 하단의 Review Queue 버튼 클릭 시 Review 스테이지로 전환한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ReviewQueue 핸들링 | grep 'StudyHit::ReviewQueue' apps/study/ |
| state.rs | open_review_queue 메서드 | grep 'fn open_review_queue' apps/study/ |
| curriculum.rs | review_queue_rect 렌더링 | grep 'fn review_queue_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on review_queue_rect
- **처리**: open_review_queue() 호출
- **출력/사이드 이펙트**: stage=Review, review_index=1, feedback=None


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.review.queue | button | review queue | 항상 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
