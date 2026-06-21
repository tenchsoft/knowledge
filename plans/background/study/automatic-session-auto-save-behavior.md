# Background: automatic-session-auto-save-behavior

## 한 줄 정의
학습/복습 세션 진행 중 사용자 액션(답안 제출, 개념 이동, 스테이지 전환) 후 5초간 idle 상태면 세션 스냅샷을 자동 저장한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 세션 상태 변경 후 idle | `StudyState` dirty (input_text, problem_index, stage, streak 등 변경) | 마지막 변경 후 5초 |
| 앱 종료 직전 | 모든 dirty 세션 | 1회 |
| 강제 저장 (Ctrl+S) | 즉시 | 사용자 액션 |

## Lifecycle & State

```
saved ──[state change]──→ dirty ──[5s idle]──→ saving ──[ok]──→ saved
                                            │
                                            └──[error]──→ dirty (retry on next idle)
```

- **saved**: `SessionSnapshot`이 디스크에 기록된 상태. UI 변화 없음.
- **dirty**: 마지막 저장 이후 상태 변경 발생. 타이머 대기 중.
- **saving**: `auto_save_session()` 호출하여 `SessionSnapshot` 직렬화 중.

## Concurrency
- **인스턴스 정책**: 세션당 단일. 같은 세션의 두 번째 saving은 첫 번째 완료 후 재평가.
- **동시성 모델**: 메인 스레드 timer (AnimFrame 기반) + std::thread로 디스크 write 분리.
- **재진입성**: 상태 변경이 saving 중에 또 오면 saving 완료 후 다시 dirty → 5초 카운트 리셋.
- **취소**: 앱 종료 시 진행 중 saving은 끝까지 기다림 (사용자 데이터 보존).

## Resource budget
- 디스크 I/O만, CPU 거의 0. 메모리 추가 할당 없음 (StudyState가 이미 데이터 보유).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState` 전체 상태 (`auto_save_session()`이 `SessionSnapshot` 생성).
- **Write**: `tench_storage_core`를 통한 로컬 파일 저장 (경로: 앱 데이터 디렉토리).
- **Persistence**: 디스크 경로에 JSON/binary 직렬화.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 디스크 가득 | I/O Err | dirty로 복귀, 다음 5s idle에 재시도 | 토스트 에러 |
| 권한 없음 | I/O Err | 동일 | 토스트 에러 |
| 직렬화 실패 | serialize Err | dirty 유지, retry | 토스트 (드물고 심각) |

자동 재시도 무한. backoff 없음 — 단순 idle 재트리거.

## Observability
- **Log**: `tracing::info!("autosave session unit={} concept={}", unit_idx, concept_idx)` on success. `tracing::error!` on failure.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.save_status` | `Label` | `"saved"` / `"saving"` / `"dirty"` | 저장 상태 (미구현, 후속) |

## UI 인터페이스
design(`plans/design/study/study-automatic-ui.md`)에 명시적 UI 노드 없음. 자동 저장은 백그라운드 전용. 향후 헤더에 상태 표시 추가 가능.

## Out of scope
- 클라우드 동기화 (별도 spec).
- 충돌 해결 (별도 spec).
- Save As 흐름 (사용자 액션, 별도 spec).
