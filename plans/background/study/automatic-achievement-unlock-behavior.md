# Background: automatic-achievement-unlock-behavior

## 한 줄 정의
학습 세션 진행 중 성취 조건(첫 세션 완료, 연속 10회 정답, 누적 100문제)이 충족되면 자동으로 성취를 해금하고 Goal modal에 반영한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 답안 제출 | `submit_answer()` 호출 시 | 매 제출 |
| 세션 완료 | `next_problem()`에서 마지막 문제 도달 시 | 1회 |
| Goal modal 열기 | `show_goal_modal = true` | 모달 열릴 때 |

## Lifecycle & State

```
locked ──[condition met]──→ unlocked
```

- **locked**: `achievement.unlocked == false`, `progress < 1.0`.
- **unlocked**: `achievement.unlocked == true`, `progress == 1.0`.

`StudyState::check_achievements()`가 호출될 때마다 모든 성취 조건을 재평가.

## Concurrency
- **인스턴스 정책**: 단일. `StudyState` 메인 스레드에서 동기 실행.
- **동시성 모델**: 동기 직렬. 외부 스레드 없음.
- **재진입성**: 안전. 조건 재평가가 멱등(idempotent).
- **취소**: 불가. 한 번 해금된 성취는 복구되지 않음.

## Resource budget
- CPU: 3개 성취 조건 평가, O(1). 메모리 추가 없음.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::streak`, `StudyState::session_results.len()`.
- **Write**: `StudyState::achievements` (각 achievement의 `unlocked`, `progress` 필드).
- **Persistence**: `auto_save_session()` 스냅샷에 포함.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 없음 | — | — | — |

순수 메모리 연산, 실패 모드 없음.

## Observability
- **Log**: `tracing::info!("achievement unlocked: {}", achievement.id)` on unlock.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| Goal modal achievements | `Label` | ★/☆ + label | 해금 상태 시각 표시 |

Goal modal(`study.modal.goal`) 내 성취 섹션에서 자동 렌더.

## UI 인터페이스
design(`plans/design/study/study-modals.md`) Goal modal 섹션에 성취 시각 정의. 이 background는 `achievements` 벡터 값 갱신 책임만.

## Out of scope
- 커스텀 성취 생성 (후속 spec).
- 성취 알림 토스트 (후속 spec).
