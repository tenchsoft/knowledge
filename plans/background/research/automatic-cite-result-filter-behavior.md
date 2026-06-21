# Background: automatic-cite-result-filter-behavior

## 한 줄 정의
Write 인스펙터 탭에서 `manuscript_cite_search` 쿼리가 변경되면 인용 결과가 자동으로 필터링되어 표시된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 검색 쿼리 변경 | `manuscript_cite_search` 값 변경 | 매 프레임 |
| 논문 목록 변경 | import/export 후 | 즉시 |

## Lifecycle & State
```
idle ──[query change]──→ filtering ──[results ready]──→ idle
```

- **idle**: 검색 쿼리 없음 → 전체 인용 결과 표시.
- **filtering**: 쿼리 기반 필터링 후 결과 행 렌더링.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 필터링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능 (인용 결과 수 ≤ 수백 개).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.manuscript_cite_search`, `papers` (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 빈 결과 | 쿼리 매치 0 | "No results" 표시 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.manuscript.cite_result.{index}.insert` | `Button` | `"<cite key>"` | 필터링된 인용 결과 |

## UI 인터페이스
design(`plans/design/research/research-inspector.md`) §3 Write 탭 인용 검색 영역.

## Out of scope
- 인용 삽입 동작 (별도 spec `insert-citation-result-button`).
- 인용 포맷 (별도 spec).
