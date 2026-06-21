# Background: automatic-responsive-region-layout-behavior

## 한 줄 정의
윈도우 크기가 변경되면 header, left, center, right 영역의 크기가 자동으로 재계산되어 패널이 겹치지 않도록 한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 윈도우 리사이즈 | `width`/`height` 변경 | 매 프레임 |
| 패널 토글 | 사이드바/인스펙터 표시 전환 | 즉시 |

## Lifecycle & State
```
layout_A ──[resize]──→ recomputing ──[done]──→ layout_B
```

- **recomputing**: `research_regions()`가 `width`/`height`에서 header, left, center, right 영역을 재계산.
- 데스크톱(≥980px): left 220px + center 유동 + right 280px.
- 태블릿/모바일(<980px): left/right 숨김, center 전체 너비.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 계산.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능 (단순 산술).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: 윈도우 `width`/`height`, `advanced_search.open` (리사이즈 이벤트가 mutate).
- **Write**: `research_regions()` 반환값.
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 최소 크기 미달 | width < 360 | 최소값 클램프 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.root` | `Surface` | — (전체 레이아웃) | 루트 컨테이너 |

## UI 인터페이스
design(`plans/design/research/research-header.md`) §3 Responsive 변형.

## Out of scope
- 패널 리사이즈 드래그 (별도 spec).
- 전체화면 모드 (별도 spec).
