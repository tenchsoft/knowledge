# Background: automatic-advanced-search-panel-render-behavior

## 한 줄 정의
고급 검색 패널이 소스 상태 변화 시 자동으로 렌더링되며, 열림/닫힘 상태와 필드 값을 `advanced_search` 상태와 동기화하여 유지한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 상태 변경 | `advanced_search.open` 또는 필드 값 변경 | 매 프레임 |
| 패널 너비 변화 | `research_regions()` 재계산 시 | 리사이즈 시 |

## Lifecycle & State
```
hidden ──[toggle open]──→ visible ──[toggle close]──→ hidden
                              │
                              └──[field change]──→ visible (필드 재렌더)
```

- **hidden**: 패널 미렌더링, 공간 점유 없음.
- **visible**: 필드 행이 `advanced_search` 값으로 렌더링. 빈/로딩/채워짐 상태 처리.

## Concurrency
- **인스턴스 정책**: 단일. 패널은 0 또는 1개.
- **동시성 모델**: 메인 스레드에서 paint 시 동기 렌더링.
- **재진입성**: 상태 변경 시마다 매 프레임 재평가. 중복 없음.
- **취소**: 해당 없음 (렌더링 전용).

## Resource budget
- CPU/메모리: 무시 가능 (UI 렌더링 비용만).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.advanced_search` (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 필드 값 불일치 | 상태 diff | 다음 프레임에 자동 수정 | 무알림 |

복구 정책: 매 프레임 상태 기반 렌더링으로 자동 복구.

## Observability
- **Log**: N/A (렌더링 전용).
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.header.advanced_search` | `Button` | `"▶"` / `"▼"` | 패널 열림/닫힘 상태 |
| `research.advanced.author` | `TextInput` | `"<query>"` | 저자 필드 |
| `research.advanced.title` | `TextInput` | `"<query>"` | 제목 필드 |

## UI 인터페이스
design(`plans/design/research/research-header.md`) §3 Advanced search toggle이 패널 열림/닫힘 제어.

## Out of scope
- 고급 검색 필터링 로직 (별도 background `automatic-paper-list-filtering-behavior`).
- Save search 동작 (별도 spec).
