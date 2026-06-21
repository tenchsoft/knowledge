# Background: automatic-status-badge-color-behavior

## 한 줄 정의
논문의 읽기 상태(unread, reading, reviewed, archived)에 따라 상태 배지 색상이 자동으로 선택되어 시각적으로 구분된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 논문 상태 변경 | 읽기 상태 업데이트 | 즉시 |
| 논문 목록 렌더 | 필터/스크롤 시 | 매 프레임 |

## Lifecycle & State
```
badge_color_A ──[status change]──→ badge_color_B
```

- **unread**: `theme.secondary` (회색).
- **reading**: `theme.primary` (파란색).
- **reviewed**: `#34D399` (녹색).
- **archived**: `theme.disabled` (흐린 회색).

## Concurrency
- **인스턴스 정책**: 다중 (논문 행마다 1개).
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.papers.{index}.reading_status` (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 알 수 없는 상태 | 매치 실패 | 기본 색상 (`theme.secondary`) | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.paper.{index}.status` | `Label` | `"<status>"` | 읽기 상태 배지 |

## UI 인터페이스
design(`plans/design/research/research-paper-list.md`) §3 논문 행 상태 배지.

## Out of scope
- 상태 변경 동작 (별도 spec).
- 상태 필터 (별도 spec `status-filter-row-control`).
