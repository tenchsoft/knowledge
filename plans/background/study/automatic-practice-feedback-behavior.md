# Background: automatic-practice-feedback-behavior

## 한 줄 정의
사용자가 답안을 제출하면 즉시 정답/오답 판정을 수행하고, 피드백 카드(정답 여부, 해설, 원인 태그)를 렌더하며, 오답 시 복습 대기열에 자동 추가한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 답안 제출 | `submit_answer()` 호출, `input_text` 비어있지 않음, `feedback == None` | 매 제출 |

## Lifecycle & State

```
answering ──[submit]──→ grading ──[correct]──→ feedback_correct
                            │
                            └──[wrong]──→ feedback_wrong ──[next/retry]──→ answering
```

- **answering**: 입력 대기. `feedback == None`.
- **grading**: `tench_study_core::grade_answer()` 호출. 즉시 완료.
- **feedback_correct**: `feedback = Some(true)`. STATUS_READY border 카드 표시.
- **feedback_wrong**: `feedback = Some(false)`. STATUS_ERROR border 카드 + 복습 대기열 추가.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드에서 동기 실행.
- **동시성 모델**: 동기 직렬.
- **재진입성**: `feedback.is_some()`이면 `submit_answer()`가 무시됨.
- **취소**: 불가. 제출 후 판정 취소 불가.

## Resource budget
- CPU: `grade_answer()` 호출, O(1). 메모리: `ReviewItem` 1개 추가 (오답 시).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `Problem::answer_key`, `StudyState::input_text`.
- **Write**: `StudyState::feedback`, `StudyState::session_results`, `StudyState::review_queue` (오답 시).
- **Persistence**: `auto_save_session()` 스냅샷에 포함.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 현재 문제 없음 | `current_problem() == None` | `submit_answer()` 조기 반환 | 없음 |
| 빈 입력 | `input_text.trim().is_empty()` | `submit_answer()` 조기 반환 | 없음 |

## Observability
- **Log**: `tracing::debug!("practice feedback correct={} problem={}", correct, problem_index)`.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.practice.submit` | `Button` | "Submit" | 제출 버튼 |
| `study.practice.retry` | `Button` | "retry" | 오답 시 재시도 버튼 |
| `study.practice.next` | `Button` | "next problem" | 다음 문제 버튼 |
| `study.practice.review_concept` | `Button` | "review concept" | 개념 복습 버튼 |

피드백 카드 자체는 별도 debug_id 없이 surface 내에 렌더.

## UI 인터페이스
design(`plans/design/study/study-learn-area.md`)와 design(`plans/design/study/study-automatic-ui.md`) practice feedback 섹션에 시각 정의.

## Out of scope
- 부분 점수 (별도 spec).
- 코드 실행형 문제 (별도 spec).
