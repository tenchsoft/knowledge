# Background: automatic-qanda-message-bubble-behavior

## 한 줄 정의
Q&A 메시지가 추가되면 인스펙터 Q&A 탭에 말풍선이 자동으로 렌더링되며, 사용자/어시스턴트 역할에 따라 색상이 구분된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 메시지 추가 | `qa_messages` 벡터 변경 | 즉시 |
| 탭 전환 | Q&A 탭 활성화 | 즉시 |

## Lifecycle & State
```
idle ──[message added]──→ rendering ──[done]──→ idle (스크롤 최하단)
```

- **rendering**: 새 말풍선 추가 후 스크롤을 최하단으로 이동.
- 사용자 말풍선: `theme.primary` 배경.
- 어시스턴트 말풍선: `#34D399` 배경.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 메시지 수에 비례. 수백 개 메시지까지 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.qa_messages` (사용자 액션 및 Engine 응답이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: Engine IPC로 Q&A 응답 수신.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| Engine 응답 없음 | 타임아웃 | 에러 말풍선 표시 | "No response" |
| 빈 메시지 | 빈 문자열 | 무시 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.qa.bubble.{index}` | `Label` | `"<message text>"` | Q&A 말풍선 |

## UI 인터페이스
design(`plans/design/research/research-inspector.md`) §3 Q&A 탭 말풍선.

## Out of scope
- Q&A 메시지 전송 (별도 spec `qanda-send-button`).
- Engine 추론 로직 (별도 spec).
