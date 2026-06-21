# Background: automatic-glossary-filter-behavior

## 한 줄 정의
터터 패널의 용어집 검색어가 변경될 때마다 대소문자 무시로 term/definition에서 일치하는 용어만 자동 필터링하여 표시한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 검색어 변경 | `glossary_search_query` 수정 | 매 키 입력 |
| 용어집 토글 | `GlossarySearchToggle` 클릭 | 사용자 액션 |

## Lifecycle & State

```
all_terms ──[query non-empty]──→ filtered ──[query cleared]──→ all_terms
```

- **all_terms**: `active_glossary_terms()` 전체 표시 (최대 3개).
- **filtered**: `glossary_search_query.to_lowercase()`가 term 또는 definition에 포함된 항목만 표시.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 안전. 필터링이 멱등.
- **취소**: Escape로 검색 포커스 해제.

## Resource budget
- CPU: O(3) 최대 3개 용어 필터링. < 0.1ms.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::active_glossary_terms()`, `StudyState::glossary_search_query`.
- **Write**: 없음 (읽기 전용 필터링).
- **Persistence**: 없음.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 일치 항목 없음 | 필터 후 빈 목록 | 빈 목록 표시 | 없음 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.glossary.search` | `TextInput` | 검색어 | 용어집 검색 입력 |
| `study.glossary.term.{idx}` | `Button` | 용어 텍스트 | 필터링된 용어 행 |

## UI 인터페이스
design(`plans/design/study/study-automatic-ui.md`) glossary filter 섹션에 정의.

## Out of scope
- 용어집 전체 화면 (후속 spec).
- 용어집 편집 (후속 spec).
