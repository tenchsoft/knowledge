# Background: automatic-paper-selection-highlight-behavior

## 한 줄 정의
선택된 논문이 변경되면 논문 목록에서 해당 행의 하이라이트가 자동으로 업데이트되고 다중 선택 상태가 시각적으로 반영된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 선택 변경 | `selected_paper` 또는 `selected_papers` 변경 | 매 프레임 |
| 필터/정렬 변경 | 논문 목록 재정렬 | 즉시 |

## Lifecycle & State
```
highlighting_A ──[selection change]──→ highlighting_B
```

- 선택된 논문 행에 `theme.primary` 배경, 미선택 행에 기본 배경.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.selected_paper`, `selected_papers` (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 선택 인덱스 무효 | 범위 초과 | 하이라이트 제거 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.paper.{index}` | `Button` | `"<paper title>"` | 선택 하이라이트 |

## UI 인터페이스
design(`plans/design/research/research-paper-list.md`) §3 논문 행 하이라이트.

## Out of scope
- 논문 선택 동작 (별도 spec `paper-row-selection-control`).
- 다중 선택 토글 (별도 spec `paper-row-multi-select-toggle`).
