# Background: automatic-paper-detail-refresh-behavior

## 한 줄 정의
선택된 논문이 변경되면 상세 패널과 인스펙터의 논문 정보가 자동으로 갱신된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 논문 선택 변경 | `selected_paper` 변경 | 즉시 |
| 논문 메타데이터 변경 | import/fetch 후 | 즉시 |

## Lifecycle & State
```
showing_paper_A ──[selection change]──→ refreshing ──[done]──→ showing_paper_B
```

- **refreshing**: 새 논문의 메타데이터, 노트, Q&A, 주석을 읽어 패널 갱신.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.selected_paper`, `papers` 벡터 (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 논문 미선택 | `selected_paper == None` | 빈 상세 패널 | 무알림 |
| 삭제된 논문 참조 | 인덱스 초과 | 선택 해제 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.paper.{index}` | `Button` | `"<paper title>"` | 선택된 논문 하이라이트 |

## UI 인터페이스
design(`plans/design/research/research-paper-list.md`) §3 논문 선택 하이라이트.

## Out of scope
- 논문 선택 동작 (별도 spec `paper-row-selection-control`).
- 논문 메타데이터 편집 (별도 spec).
