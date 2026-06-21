# Implement: again-spaced-repetition-rating-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 리뷰 화면에서 'Again' 버튼 클릭 시 SM-2 Again(quality=0) 등급 적용.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/practice.rs::paint_review_surface` | Again 버튼 paint | `SpacedRepetitionRating` |
| `apps/study/src-tauri/src/ui/practice.rs::rating_rect` | Again 버튼 rect 계산 | `fn rating_rect` |
| `apps/study/src-tauri/src/ui/curriculum.rs::hit_test` | Again 버튼 hit test | `RatingButton(rating)` |
| `apps/study/src-tauri/src/ui/state.rs::apply_spaced_repetition_rating` | SM-2 Again 로직 | `fn apply_spaced_repetition_rating` |
| `apps/study/src-tauri/src/ui/mod.rs::study_automation_nodes` | `study.review.rating.again` 노드 | `study.review.rating` |

## 필요한 변경 (의도 단위)
### 1. Again 버튼 paint 및 hit test
- **입력**: Stage == Review일 때 paint_review_surface 호출
- **처리**: `rating_rect(surface, &Again)` 위치에 버튼 paint, hit test에서 `StudyHit::RatingButton(Again)` 반환
- **출력/사이드 이펙트**: 클릭 시 `apply_spaced_repetition_rating(Again)` 호출

### 2. SM-2 Again 등급 적용
- **입력**: `SpacedRepetitionRating::Again`
- **처리**: quality=0, repetitions=0, interval_days=1, easiness_factor 갱신
- **출력/사이드 이펙트**: `pending_rating` 설정, `spaced_repetition_data` 갱신

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `study.review.rating.again` | `Button` | `"Again"` | stage == Review |

## 의존
- 선행 implement: spaced-repetition-scheduling

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
