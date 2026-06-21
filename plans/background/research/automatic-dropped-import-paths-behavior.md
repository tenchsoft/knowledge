# Background: automatic-dropped-import-paths-behavior

## 한 줄 정의
파일이 앱 창에 드롭되면 경로를 수집하여 import 큐에 자동 등록한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 파일 드롭 이벤트 | Tauri drag-drop 이벤트 수신 | 이벤트 발생 시 |
| 앱 셸 IPC | `dropped_import_paths`에 경로 추가 | 메시지 수신 시 |

## Lifecycle & State
```
idle ──[drop event]──→ collecting ──[paths validated]──→ queued ──→ idle
                           │
                           └──[invalid path]──→ idle (toast error)
```

- **idle**: `dropped_import_paths` 비어 있음.
- **collecting**: 드롭된 경로를 `dropped_import_paths`에 추가.
- **queued**: `queue_import()` 호출로 `import_status`를 `Queued`로 변경.

## Concurrency
- **인스턴스 정책**: 다중. 여러 파일 동시 드롭 허용.
- **동시성 모델**: 메인 스레드에서 직접 처리. 외부 스레드 없음.
- **재진입성**: 드롭 이벤트 중복 수신 안전 — 경로가 벡터에 append.
- **취소**: 사용자가 직접 취소 불가. import 시작 후 취소는 별도 spec.

## Resource budget
- 메모리: 경로 문자열만 — 수 KB. 모바일/데스크톱 동일.
- CPU: 무시 가능.
- 디스크 I/O: 경로 검증만. 실제 파일 읽기는 import 프로세스에서.
- 네트워크: 없음.

## Data flow
- **Read**: `ResearchState.dropped_import_paths` (메인 스레드).
- **Write**: `ResearchState.dropped_import_paths` (push), `ResearchState.import_status` (set `Queued`).
- **Persistence**: 메모리만. 앱 재시작 시 초기화.
- **IPC**: Tauri drag-drop event → app shell → `ResearchState`.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 지원하지 않는 파일 형식 | 확장자 검사 | 해당 경로 무시 | 토스트 warning |
| 경로 접근 권한 없음 | OS 에러 | 무시 | 토스트 error |

복구 정책: 개별 경로 실패는 무시하고 나머지 계속 처리.

## Observability
- **Log**: `tracing::info!("dropped import: {} paths", count)`.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| 토스트 스택 | `Label` | `"Import started"` | import 큐 진입 통보 |

## UI 인터페이스
design(`plans/design/research/research-automatic-ui.md`) §6 Dropped import가 토스트로 통보.

## Out of scope
- 실제 파일 파싱/메타데이터 추출 (별도 background: import pipeline).
- 중복 논문 감지 (별도 spec).
