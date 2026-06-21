# Implement: automatic-review-card-render-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Review 모드에서 현재 리뷰 아이템(`current_review()`)의 문제 텍스트, 오답, 정답, 해설, cause_tag, related_concept가 자동으로 카드 형태로 렌더링된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/practice.rs` (리뷰 카드) | `current_review()` 결과를 카드에 렌더 | ``fn paint_review_surface`` |

## 필요한 변경 (의도 단위)
### 1. 리뷰 카드 자동 렌더
- **입력**: `state.current_review()` — `Option<&ReviewItem>`
- **처리**: `Some(item)`인 경우 카드(`NEUTRAL_600` 배경, `NEUTRAL_500` 테두리)에 problem_text(`NEUTRAL_100`), wrong_answer(`STATUS_ERROR`), correct_answer(`STATUS_READY`), solution(`NEUTRAL_100`), cause_tag/related_concept(`NEUTRAL_300`)를 순서대로 그린다.
- **출력/사이드 이펙트**: 현재 리뷰 아이템의 모든 정보가 카드 형태로 자동 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_review_surface` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
