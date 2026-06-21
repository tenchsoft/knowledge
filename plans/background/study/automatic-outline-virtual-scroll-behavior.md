# Background: automatic-outline-virtual-scroll-behavior

## 한 줄 정의
커리큘럼 아웃라인 패널에서 화면에 보이는 범위의 단원/개념만 렌더하고 자동화 노드에 노출하여 대규모 커리큘럼에서도 일정한 성능을 유지한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 스크롤 오프셋 변경 | `outline_scroll_offset` 변경 | 매 프레임 |
| 뷰포트 크기 변경 | `update_viewport()` 호출 | 리사이즈 시 |
| 단원 확장/축소 | `toggle_unit_expand()` | 사용자 액션 |
| 개념 선택 | `select_concept()` | 사용자 액션 |

## Lifecycle & State

```
rendering ──[compute visible range]──→ rendering (subset only)
```

- **visible range**: `visible_top = outline.y0 + 44.0` ~ `visible_bottom = review_queue_rect.y0`.
- 각 unit header의 y 좌표가 visible range 밖이면 `continue` (렌더 스킵).
- 각 concept row의 y 좌표가 visible range 밖이면 `continue` (렌더 스킵).
- 자동화 노드도 동일하게 visible range 밖 항목은 제외.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU: O(visible_items) — 화면에 보이는 항목 수에 비례. 전체 항목 수와 무관.
- 메모리: 추가 할당 없음. 기존 `units` 벡터 순회만.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::units`, `StudyState::outline_scroll_offset`, `StudyState::expanded_units`.
- **Write**: 없음 (읽기 전용 렌더링).
- **Persistence**: 없음.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 없음 | — | — | — |

순수 렌더링 로직, 실패 모드 없음.

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.unit.{idx}` | `Button` | 단원 라벨 | visible 단원 헤더 |
| `study.concept.{unit}.{concept}` | `Button` | 개념 라벨 | visible 개념 행 |

visible range 밖의 항목은 자동화 노드에 노출되지 않음.

## UI 인터페이스
design(`plans/design/study/study-curriculum.md`)에 가상 스크롤 렌더링 정의.

## Out of scope
- 스크롤바 UI (후속 spec).
- 키보드 스크롤 (현재 Arrow Up/Down으로 개념 이동 시 자동 스크롤 없음 — 후속).
