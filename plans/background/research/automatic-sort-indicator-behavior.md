# Background: automatic-sort-indicator-behavior

## 한 줄 정의
현재 정렬 모드 라벨이 논문 목록 헤더 우측에 버튼으로 자동 표시되고, 클릭 시 정렬 모드가 순환한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 정렬 모드 변경 | `cycle_sort_mode()` | 사용자 액션 |
| paint 사이클 | 좌측 패널 렌더 시 | 매 paint |

## Lifecycle & State
```
stable ──[cycle_sort_mode]──→ updating ──[paint]──→ stable
```

- **stable**: `sort_mode` 라벨과 버튼 텍스트 일치.
- **updating**: `sort_mode` 변경 후 다음 paint에서 라벨 갱신.

사용 가능한 정렬 모드 (순환 순서):
TitleAsc → TitleDesc → YearAsc → YearDesc → AuthorAsc → AuthorDesc → DateAddedAsc → DateAddedDesc

## Concurrency
- **인스턴스 정책**: 단일. 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 해당 없음.
- **취소**: 해당 없음.

## Resource budget
- 메모리: 추가 할당 없음.
- CPU: 무시 가능 (라벨 문자열 생성).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.sort_mode`.
- **Write**: `ResearchState.sort_mode` (cycle 시).
- **Persistence**: 메모리만.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 없음 | — | — | — |

복구 정책: 해당 없음.

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.paper.sort` | `Button` | sort mode label | 현재 정렬 기준 |

## UI 인터페이스
design(`plans/design/research/research-sidebar.md`) §Sort indicator button이 시각 정의.

## Out of scope
- 커스텀 정렬 기준 (별도 spec).
- 드래그앤드롭 순서 변경 (별도 spec).
