# Background: automatic-notes-panel-overlay-behavior

## 한 줄 정의
노트 토글이 활성화되면 커리큘럼 패널 위에 노트 목록과 입력 필드를 오버레이로 자동 표시한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 상태 변경 | 관련 state 변경 시 | paint 시마다 |

## Lifecycle & State

```
idle ──[state change]──→ updating ──[ok]──→ idle
                              │
                              └──[error]──→ idle (기본값 유지)
```

- **idle**: 대기. UI가 현재 state를 반영한 상태.
- **updating**: state 변경 감지 후 UI 갱신. 동기 처리로 즉시 완료.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드에서 동기 실행.
- **동시성 모델**: 동기 직렬. paint cycle 내에서 처리.
- **재진입성**: 안전. 동일 state 변경이 연속 발생해도 최종 상태만 반영.
- **취소**: 불필요. 동기 처리.

## Resource budget
- CPU 거의 0 (단순 state read + UI 갱신). 메모리 추가 없음.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::notes_panel_open` (bool), `StudyState::notes`.
- **Write**: 노트 패널 오버레이의 표시 속성.
- **Persistence**: `auto_save_session()` 스냅샷에 포함.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| state 불일치 | `StudyState` 필드 None | 기본값 렌더 | 없음 |

## Observability
- **Log**: `tracing::debug!("automatic-notes-panel-overlay-behavior updated")`.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.notes.panel` | `Container` | visible / hidden | 노트 패널 표시 여부 |
| `study.notes.input` | `TextInput` | notes_input | 노트 입력 필드 |
| `study.notes.save` | `Button` | "Save" | 저장 버튼 |

## UI 인터페이스
design(`plans/design/study/study-notes.md`)에 노트 패널 시각 정의.

## Out of scope
- 노트 저장 동작 (별도 spec `notes-save-button`).
- 노트 편집 (별도 spec).
