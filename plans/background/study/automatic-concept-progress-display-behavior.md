# Background: automatic-concept-progress-display-behavior

## 한 줄 정의
각 단원 헤더에 `ConceptStatus::Completed`인 개념 수를 전체 개념 수로 나눈 진행률을 자동 계산하여 "completed/total" 형식으로 표시한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 아웃라인 렌더 | `paint_outline()` 호출 | 매 프레임 |
| 개념 상태 변경 | `apply_batch_concept_status()` 또는 `select_concept()` | 사용자 액션 |

## Lifecycle & State

```
stale ──[paint_outline]──→ computed ──[render]──→ displayed
```

- **stale**: 상태 변경 후 아직 렌더되지 않음.
- **computed**: `paint_outline()`에서 completed/total 카운트 계산.
- **displayed**: "2/3" 형식으로 단원 헤더 우측에 렌더.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU: O(concepts per unit) 카운트. 단원당 < 10개 가정 시 < 0.1ms.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `Unit::concepts` (각 `Concept::status`).
- **Write**: 없음 (읽기 전용 표시).
- **Persistence**: 없음.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 빈 단원 | `total == 0` | 진행률 표시 생략 (`if total > 0`) | 없음 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.unit.{idx}` | `Button` | 단원 라벨 + 진행률 | 단원 헤더에 진행률 포함 |

진행률 텍스트는 단원 헤더 내에 렌더되지만 별도 debug_id 없음.

## UI 인터페이스
design(`plans/design/study/study-curriculum.md`)에 concept progress 표시 정의.

## Out of scope
- 진행률 퍼센트 바 (후속 spec).
- 개념별 상세 진행률 (후속 spec).
