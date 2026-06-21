# Background: automatic-inspector-tab-content-behavior

## 한 줄 정의
활성 인스펙터 탭이 변경되면 해당 탭의 콘텐츠가 자동으로 렌더링되고, 선택된 논문이 변경되면 탭 내용이 즉시 갱신된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 탭 전환 | `inspector_tab` 변경 | 즉시 |
| 논문 선택 변경 | `selected_paper` 변경 | 즉시 |
| 탭 관련 상태 변경 | notes, qa_messages 등 | 매 프레임 |

## Lifecycle & State
```
rendering_tab_A ──[tab switch]──→ rendering_tab_B ──[paper change]──→ rendering_tab_B (내용 갱신)
```

- 각 탭은 자체 상태(notes, summary, qa, visuals, write, cite)를 구독.
- 빈 상태: 논문 미선택 시 "Select a paper" 플레이스홀더.

## Concurrency
- **인스턴스 정책**: 단일 (활성 탭 1개).
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.inspector_tab`, `selected_paper`, 탭별 상태 (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 탭 콘텐츠 불일치 | 상태 diff | 다음 프레임에 자동 수정 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.inspector.tab.{notes,summary,qa,visual,write,cite}` | `Button` | `"<tab label>"` | 활성 탭 |

## UI 인터페이스
design(`plans/design/research/research-inspector.md`) §3 인스펙터 탭 및 콘텐츠 영역.

## Out of scope
- 개별 탭 동작 (각각 별도 spec).
- 인스펙터 패널 리사이징 (별도 spec).
