# Background: automatic-paper-list-filtering-behavior

## 한 줄 정의
검색어, 컬렉션 선택, 즐겨찾기 필터, 정렬 모드가 변경되면 논문 목록이 자동으로 필터링·정렬된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 검색어 변경 | `search_query` mutation | 즉시 |
| 컬렉션 선택 변경 | `select_collection()` | 즉시 |
| 즐겨찾기 토글 | `toggle_favorites_only()` | 즉시 |
| 정렬 모드 변경 | `cycle_sort_mode()` | 즉시 |

## Lifecycle & State
```
stable ──[filter change]──→ recomputing ──[visible_papers() called]──→ stable
```

- **stable**: 현재 필터 상태와 표시된 논문이 일치.
- **recomputing**: `visible_papers()`가 새 조건으로 인덱스 재계산. 동기, 즉시 완료.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드에서 동기 처리.
- **동시성 모델**: 동기 직렬. `paint()` 호출 시마다 `visible_papers()` 재평가.
- **재진입성**: 해당 없음 — 상태 변경 후 다음 paint에서 자동 반영.
- **취소**: 해당 없음.

## Resource budget
- 메모리: 필터링 결과 인덱스 벡터 — O(n) where n = papers count.
- CPU: 선형 스캔 O(n). 10,000 논문 기준 <1ms.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.papers`, `search_query`, `selected_collection`, `favorites_only`, `sort_mode`.
- **Write**: `ResearchState.selected_paper` (필터 후 인덱스 재조정).
- **Persistence**: 메모리만.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 필터 결과 0건 | 빈 벡터 | 빈 목록 표시 | 무알림 (빈 상태 UI) |

복구 정책: 해당 없음 — 항상 성공.

## Observability
- **Log**: N/A (너무 빈번).
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.paper.{idx}` | `Button` | paper title | 필터링된 논문 행 |
| 빈 목록 | — | — | 결과 없음 |

## UI 인터페이스
design(`plans/design/research/research-sidebar.md`) §Paper list가 필터링된 결과를 렌더.

## Out of scope
- 검색 하이라이트 (별도 background `automatic-paper-search-highlight-behavior`).
- 서버 검색 (별도 spec).
