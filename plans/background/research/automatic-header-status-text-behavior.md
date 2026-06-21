# Background: automatic-header-status-text-behavior

## 한 줄 정의
넓은 화면(≥1040px)에서 import 상태, reader 모드, 즐겨찾기 필터 상태가 헤더 우측에 자동으로 표시된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| paint 사이클 | `size.width >= 1040` | 매 paint |
| import 상태 변경 | `queue_import()` | 사용자 액션 |
| reader 모드 변경 | `toggle_reader_mode()` | 사용자 액션 |
| 즐겨찾기 토글 | `toggle_favorites_only()` | 사용자 액션 |

## Lifecycle & State
```
hidden ──[width >= 1040]──→ visible ──[width < 1040]──→ hidden
```

- **hidden**: 뷰포트 <1040px. 상태 텍스트 미표시.
- **visible**: `"{import_status} | {reader_mode} | Favorites {on|off}"` 형식으로 paint.

## Concurrency
- **인스턴스 정책**: 단일. paint 내 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 해당 없음.
- **취소**: 해당 없음.

## Resource budget
- 메모리: 추가 할당 없음.
- CPU: 무시 가능 (문자열 포맷팅).
- 모바일/데스크톱 동일 (모바일에서는 hidden).

## Data flow
- **Read**: `ResearchState.import_status`, `reader_mode`, `favorites_only`.
- **Write**: 없음 (읽기 전용).
- **Persistence**: 없음.
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
| 헤더 상태 텍스트 | `Label` | `"{status} \| {mode} \| Favorites {on/off}"` | 통합 상태 표시 |

## UI 인터페이스
design(`plans/design/research/research-header.md`)가 헤더 우측 상태 텍스트 위치 정의.

## Out of scope
- 상태 텍스트 클릭 인터랙션 (현재 없음).
- 알림 배지 (별도 spec).
