# Background: automatic-search-cursor-behavior

## 한 줄 정의
커리큘럼 검색 입력이 포커스되면 커서를 빈 칸 또는 텍스트 끝 위치에 자동으로 표시한다.

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
- **Read**: `StudyState::search_query`, 검색 입력 포커스 상태.
- **Write**: 검색 입력의 커서 속성.
- **Persistence**: `auto_save_session()` 스냅샷에 포함.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| state 불일치 | `StudyState` 필드 None | 기본값 렌더 | 없음 |

## Observability
- **Log**: `tracing::debug!("automatic-search-cursor-behavior updated")`.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.curriculum.search` | `TextInput` | search_query | 검색 입력 (커서 위치) |

## UI 인터페이스
design(`plans/design/study/study-curriculum.md`)에 검색 입력 시각 정의.

## Out of scope
- 검색 결과 필터링 (별도 background `automatic-glossary-filter-behavior`).
- 검색 일치 수 표시 (별도 background `automatic-search-match-count-behavior`).
