# Background: automatic-toast-stack-render-behavior

## 한 줄 정의
`toasts` 벡터에 메시지가 추가되면 하단 중앙에 최대 5개의 토스트가 자동으로 스택 렌더되고, 클릭 또는 만료 시 개별 해제된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 토스트 추가 | `add_toast()` 호출 | 이벤트 발생 시 |
| 토스트 해제 | `dismiss_toast()` 호출 (클릭) | 사용자 액션 |
| paint 사이클 | `toasts` non-empty | 매 paint |

## Lifecycle & State
```
empty ──[add_toast]──→ visible ──[dismiss]──→ empty
                          │
                          └──[max 5 reached]──→ oldest truncated
```

- **empty**: `toasts` 벡터 비어 있음. 토스트 미표시.
- **visible**: `toasts.iter().rev().take(5)`로 최대 5개를 하단에서 위로 스택.
- **truncated**: 5개 초과 시 가장 오래된 토스트가 `take(5)`에서 제외.

## Concurrency
- **인스턴스 정책**: 다중 (여러 토스트 동시 표시).
- **동시성 모델**: 동기 직렬. 메인 스레드.
- **재진입성**: `add_toast()` 호출 중복 안전 — 벡터에 push.
- **취소**: 클릭으로 개별 해제 가능.

## Resource budget
- 메모리: 토스트 메시지 문자열 — 수 KB.
- CPU: 무시 가능 (최대 5개 렌더).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.toasts` (paint 시).
- **Write**: `ResearchState.toasts` (push/pop).
- **Persistence**: 없음. 앱 재시작 시 초기화.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 없음 | — | — | — |

복구 정책: 해당 없음.

## Observability
- **Log**: `tracing::debug!("toast added: {:?}: {}", kind, message)`.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| 토스트 스택 | `Label` | message text | 종류별 색상 배경 |

## UI 인터페이스
design(`plans/design/research/research-automatic-ui.md`) §21 Toast stack이 시각 정의.

## Out of scope
- 토스트 자동 만료 타이머 (현재 미구현 — 별도 spec 필요 시 추가).
- 토스트 애니메이션 (현재 즉시 표시/숨김).
