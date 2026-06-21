# Background: automatic-session-timer-behavior

## 한 줄 정의
학습 세션이 시작되면 경과 시간을 1초 간격으로 누적하여 `StudyState::elapsed_seconds`를 갱신하고, 헤더에 HH:MM:SS 포맷으로 표시한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 세션 시작 | 앱 시작 또는 `start_practice()` | 1회 시작 |
| 타이머 tick | `session_paused == false` | 1초마다 |
| 일시정지 | `toggle_pause()` | 타이머 정지 |
| 세션 종료 | 모달 닫기 또는 개념 변경 | 타이머 정지 |

## Lifecycle & State

```
stopped ──[session start]──→ running ──[1s tick]──→ running (elapsed++)
                                │
                                ├──[pause]──→ paused ──[resume]──→ running
                                │
                                └──[session end]──→ stopped
```

- **stopped**: 타이머 비활성. `elapsed_seconds` 유지.
- **running**: 1초마다 `elapsed_seconds` 증분. 헤더에 HH:MM:SS 표시.
- **paused**: 타이머 정지. `elapsed_seconds` 변경 없음.

## Concurrency
- **인스턴스 정책**: 단일. 앱 전역 타이머 1개.
- **동시성 모델**: AnimFrame 기반 메인 스레드에서 1초 경과 확인 후 `elapsed_seconds` 증분.
- **재진입성**: 없음 (단일 타이머).
- **취소**: 세션 종료 시 타이머 정지.

## Resource budget
- CPU 거의 0 (1초에 1회 정수 증분). 메모리 추가 없음.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::session_paused` (타이머 정지 여부).
- **Write**: `StudyState::elapsed_seconds` (1초마다 증분).
- **Persistence**: `auto_save_session()` 스냅샷에 포함.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 타이머 drift | AnimFrame 간격 누적 오차 | 누적 오차 < 1초면 무시, > 1초면 보정 | 없음 |

드리프트 보정: 마지막 tick 시간을 기록하여 다음 tick에서 실제 경과 시간 계산.

## Observability
- **Log**: N/A (타이머는 로깅 불필요).
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.header.stage` | `Button` | `"learn"` / `"practice"` / `"review"` | 현재 스테이지 (타이머 컨텍스트) |

헤더 타이머 텍스트는 `study.header.stage` 영역 근처에 렌더되지만 별도 debug_id 없음. 후속 구현에서 분리 가능.

## UI 인터페이스
design(`plans/design/study/study-header.md`)에 헤더 타이머 표시 정의. 이 background는 `elapsed_seconds` 값 갱신 책임만.

## Out of scope
- 세션 타이머 알림 (별도 spec).
- 일일 학습 시간 목표와의 연동 (별도 background).
