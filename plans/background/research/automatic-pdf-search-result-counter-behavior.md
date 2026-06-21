# Background: automatic-pdf-search-result-counter-behavior

## 한 줄 정의
PDF 검색 결과가 있으면 현재 활성 결과 인덱스와 총 결과 수를 `{active+1}/{total}` 형식으로 자동 표시한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| PDF 검색 결과 변경 | `pdf_search_results` non-empty | 즉시 (paint 시) |
| 활성 인덱스 변경 | `advance_pdf_search()` | 사용자 액션 |

## Lifecycle & State
```
hidden ──[results found]──→ visible ──[results cleared]──→ hidden
                                │
                                └──[advance]──→ visible (index update)
```

- **hidden**: `pdf_search_results` 비어 있음. 카운터 미표시.
- **visible**: 카운터가 `{active+1}/{total}` 형식으로 paint. `active_index == None`이면 `0/{total}`.

## Concurrency
- **인스턴스 정책**: 단일. paint 사이클 내 동기 처리.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 해당 없음.
- **취소**: Escape로 `clear_pdf_search()` 호출 시 hidden으로 복귀.

## Resource budget
- 메모리: 추가 할당 없음 (기존 벡터 길이 읽기).
- CPU: 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.pdf_search_results.len()`, `pdf_search_active_index`.
- **Write**: 없음 (읽기 전용).
- **Persistence**: 없음.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 결과 0건 | 빈 벡터 | 카운터 숨김 | 무알림 |

복구 정책: 해당 없음.

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| PDF search bar 내 | `Label` | `"{active+1}/{total}"` | 검색 결과 카운터 |

## UI 인터페이스
design(`plans/design/research/research-pdf-viewer.md`)가 PDF search bar 내 카운터 위치 정의.

## Out of scope
- 검색 결과 하이라이트 렌더 (별도 background `automatic-pdf-surface-render-behavior`).
- PDF 텍스트 검색 알고리즘 (별도 spec).
