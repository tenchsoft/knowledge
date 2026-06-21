# Background: automatic-welcome-overlay-behavior

## 한 줄 정의
`show_welcome`이 true이면 라이브러리 작업 공간 위에 환영 오버레이가 자동으로 렌더링되어 작업 공간 접근을 차단한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 앱 시작 | `show_welcome == true` | 1회 |
| 상태 변경 | `show_welcome` 토글 | 즉시 |

## Lifecycle & State
```
hidden ──[show_welcome=true]──→ visible ──[show_welcome=false]──→ hidden
```

- **hidden**: 오버레이 미렌더링, 작업 공간 접근 가능.
- **visible**: 반투명 배경 위 환영 카드 렌더링. Get Started / Import from file 버튼 제공.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.show_welcome` (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: `show_welcome`은 앱 설정에 저장.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 상태 불일치 | `show_welcome` diff | 다음 프레임에 자동 수정 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.welcome.backdrop` | `Surface` | — | 환영 오버레이 배경 |
| `research.welcome.get_started` | `Button` | `"<Get Started>"` | 시작 버튼 |
| `research.welcome.import_file` | `Button` | `"<Import from file>"` | 가져오기 버튼 |

## UI 인터페이스
design(`plans/design/research/research-automatic-ui.md`) §3 Welcome overlay.

## Out of scope
- Get Started 동작 (별도 spec `welcome-get-started-button`).
- Import from file 동작 (별도 spec `welcome-import-from-file-button`).
