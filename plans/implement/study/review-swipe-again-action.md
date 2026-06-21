# Implement: review-swipe-again-action

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 리뷰 모드에서 오른쪽 스와이프(첫 번째 매핑)로 Again 평가를 적용한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | PointerEvent::Up 스와이프 감지 | grep 'swipe_start' apps/study/ |
| state.rs | apply_spaced_repetition_rating 메서드 | grep 'fn apply_spaced_repetition_rating' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 스와이프 제스처 감지
- **입력**: PointerEvent::Up (distance >= 50px, speed >= 0.3px/ms, stage == Review)
- **처리**: 수평 스와이프 방향에 따라 TouchReviewAction 매핑, SpacedRepetitionRating 변환 후 apply_spaced_repetition_rating() 호출
- **출력/사이드 이펙트**: pending_rating 설정, SM-2 알고리즘으로 간격/EF 갱신


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
(KB 노드 — 단축키 전용, 별도 자동화 노드 없음)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
