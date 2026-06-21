# Background: automatic-paper-search-highlight-behavior

## 한 줄 정의
검색어가 비어 있지 않으면 논문 목록에서 제목에 검색어가 포함된 행에 노란색 하이라이트 배경을 자동으로 적용한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 검색어 변경 | `search_query` non-empty | 즉시 (paint 시) |

## Lifecycle & State
```
no_highlight ──[search_query set]──→ highlighting ──[search_query cleared]──→ no_highlight
```

- **no_highlight**: 검색어가 비어 있거나 일치하는 결과 없음.
- **highlighting**: `paint()`에서 각 논문 제목의 `to_lowercase().contains(query_lower)` 검사 후 조건부 렌더.

## Concurrency
- **인스턴스 정책**: 단일. paint 사이클 내 동기 처리.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 해당 없음.
- **취소**: 검색어 삭제 시 즉시 하이라이트 제거.

## Resource budget
- 메모리: 추가 할당 없음.
- CPU: O(n) 문자열 포함 검사. 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.search_query`, `Paper.title`.
- **Write**: 없음 (읽기 전용, paint 시 렌더만).
- **Persistence**: 없음.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 대소문자 불일치 | — | `to_lowercase()` 비교로 자동 처리 | 무알림 |

복구 정책: 해당 없음.

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.paper.{idx}` | `Button` | paper title | 하이라이트 배경 유무로 일치 표시 |

## UI 인터페이스
design(`plans/design/research/research-sidebar.md`) §Search highlight가 yellow alpha 40 배경으로 시각화.

## Out of scope
- 본문(abstract) 검색 하이라이트 (별도 spec).
- PDF 내 검색 하이라이트 (별도 background `automatic-pdf-surface-render-behavior`).
