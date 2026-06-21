# Background: automatic-active-concept-highlight-behavior

## 한 줄 정의
현재 활성 개념(active concept)이 변경되면 커리큘럼 패널의 해당 행에 하이라이트와 액센트 바를 자동으로 표시한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 상태 변경 | 관련 state 변경 시 | paint 시마다 |

## Lifecycle & State

```
idle ──[state change]──→ updating ──[ok]──→ idle
                              │
                              └──[error]──→ idle (기본값 유지)
```

- **idle**: 대기. UI가 현재 state를 반영한 상태.
- **updating**: state 변경 감지 후 UI 갱신. 동기 처리로 즉시 완료.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드에서 동기 실행.
- **동시성 모델**: 동기 직렬. paint cycle 내에서 처리.
- **재진입성**: 안전. 동일 state 변경이 연속 발생해도 최종 상태만 반영.
- **취소**: 불필요. 동기 처리.

## Resource budget
- CPU 거의 0 (단순 state read + UI 갱신). 메모리 추가 없음.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::active_unit_idx`, `StudyState::active_concept_idx`.
- **Write**: 커리큘럼 개념 행의 하이라이트 스타일 속성.
- **Persistence**: `auto_save_session()` 스냅샷에 포함.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| state 불일치 | `StudyState` 필드 None | 기본값 렌더 | 없음 |

## Observability
- **Log**: `tracing::debug!("automatic-active-concept-highlight-behavior updated")`.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.concept.{unit}.{concept}` | `Button` | concept label | 활성 개념 행 (하이라이트 여부) |

## UI 인터페이스
design(`plans/design/study/study-curriculum.md`)에 활성 개념 행 시각 정의 (ACCENT_STUDY left bar).

## Out of scope
- 개념 선택 동작 자체 (별도 spec `concept-row-selection-control`).
- 가상 스크롤 (별도 background `automatic-outline-virtual-scroll-behavior`).
