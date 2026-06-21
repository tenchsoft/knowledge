# Implement: good-spaced-repetition-rating-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Good 버튼 클릭으로 적당함 평가를 적용하고 표준 간격으로 설정한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::RatingButton(Good) 핸들링 | grep 'StudyHit::RatingButton' apps/study/ |
| state.rs | apply_spaced_repetition_rating 메서드 | grep 'fn apply_spaced_repetition_rating' apps/study/ |
| practice.rs | rating_rect 렌더링 | grep 'fn rating_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 평가 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on rating_rect(Good)
- **처리**: apply_spaced_repetition_rating(Good) 호출
- **출력/사이드 이펙트**: pending_rating = Some(Good), SM-2 알고리즘으로 간격/EF 갱신


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.review.rating.good | button | good | stage == Review |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
