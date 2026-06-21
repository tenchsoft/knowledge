# Background: automatic-search-match-count-behavior

## 한 줄 정의
커리큘럼 검색 입력이 변경될 때마다 즉시 전체 단원/개념에서 일치 항목을 검색하고, 일치 수를 검색창 옆에 표시하며, Enter 시 첫 결과로 이동한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 검색어 변경 | `search_query` 문자열 수정 (키 입력) | 매 키 입력 |
| Enter 키 | `search_focused == true`, `stage != Practice` | 사용자 액션 |

## Lifecycle & State

```
idle ──[keystroke]──→ searching ──[compute matches]──→ results_shown
                                                    │
                                                    └──[Enter]──→ navigated
```

- **idle**: `search_query` 비어있음. 일치 수 표시 없음.
- **searching**: `search_matches()` 호출. 즉시 완료 (동기).
- **results_shown**: 일치 수가 검색창 옆에 `ACCENT_STUDY`로 표시.
- **navigated**: `select_concept()`로 첫 일치 항목으로 이동.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 안전. 검색이 멱등.
- **취소**: Escape로 검색 포커스 해제.

## Resource budget
- CPU: O(units × concepts) 선형 검색. 최대 ~100개 항목 가정 시 < 1ms.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::units` (모든 unit/concept label).
- **Write**: 없음 (읽기 전용 검색).
- **Persistence**: 없음.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 일치 항목 없음 | `matches.is_empty()` | 일치 수 "0" 표시, Enter 무시 | 없음 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.curriculum.search` | `TextInput` | 검색어 | 검색 입력 |

일치 수 텍스트는 검색창 옆에 렌더되지만 별도 debug_id 없음.

## UI 인터페이스
design(`plans/design/study/study-curriculum.md`)에 검색창과 일치 수 표시 정의.

## Out of scope
- 검색 결과 하이라이트 (후속 spec).
- 정규식 검색 (후속 spec).
