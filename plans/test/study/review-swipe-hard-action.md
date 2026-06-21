# Test: review-swipe-hard-action

## 검증 대상
spec(`plans/spec/study/hard-spaced-repetition-rating-button.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Hard 스와이프 액션 시 SpacedRepetitionRating::Hard 적용 | `review_swipe_hard_applies_hard_rating` |
| AC2: Hard 버튼이 다른 스와이프 액션과 독립 동작 | `review_swipe_hard_independent_from_others` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/review_swipe_hard_action_ui_e2e.rs`

## Required Test Shape
- **Success**: Hard 스와이프 액션 선택 시 `SpacedRepetitionRating::Hard` 적용 -> 함수: `review_swipe_hard_applies_hard_rating`
- **Negative**: 다른 액션(Again/Good/Easy) 클릭 시 Hard 미적용 -> 함수: `review_swipe_hard_independent_from_others`
- **Edge case**: 연속 Hard 선택 시 스케줄링 간격 일관성 -> 함수: `review_swipe_hard_repeated_selection`

## 사용할 자동화 노드
implement(`plans/implement/study/hard-spaced-repetition-rating-button.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `study.review.swipe.hard` | 리뷰 스테이지 진입 전 | 노드 없음 |
| `study.review.swipe.hard` | 리뷰 스테이지 진입 후 | `enabled: true` |
| `study.review.rating.hard` | Hard 클릭 후 | `value: "selected"` |

## 의존
- 선행 implement: `plans/implement/study/hard-spaced-repetition-rating-button.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study review_swipe_hard
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) 사용.
4. `cargo test -p tench-study review_swipe_hard` 통과.
5. `cargo check --workspace --locked` 통과.
