# Test: saved-search-row-control

## 검증 대상
spec(`plans/spec/research/saved-search-row-control.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Activate when required state is available; visible result happens immediately | `saved_search_row_restores_query_on_click` |
| AC2: Activate when required state is missing; disabled/empty/no-op without corrupting data | `saved_search_row_noop_when_empty` |
| AC3: Activate while focus is in another input; focus ownership deterministic | `saved_search_row_steals_focus_from_search` |
| AC4: Repeat quickly; state changes idempotent | `saved_search_row_rapid_clicks_idempotent` |

## 테스트 파일 위치
`apps/research/src-tauri/tests/saved_search_row_control_ui_e2e.rs`

## Required Test Shape
- **Success**: 저장된 검색 행 클릭 시 쿼리 복원 및 매칭 논문 표시 -> 함수: `saved_search_row_restores_query_on_click`
- **Negative**: 저장된 검색 없을 때 행 미노출 -> 함수: `saved_search_row_noop_when_empty`
- **Edge case**: 연속 클릭 시 멱등성 -> 함수: `saved_search_row_rapid_clicks_idempotent`

## 사용할 자동화 노드
implement(`plans/implement/research/saved-search-row-control.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `research.saved_search.0` | 저장된 검색 있을 때 | `enabled: true`, label: 검색 이름 |
| `research.saved_search.0` | 클릭 후 | 검색 쿼리가 search field에 반영 |
| `research.header.search` | 저장된 검색 클릭 후 | `value`: 저장된 쿼리 텍스트 |

## 의존
- 선행 implement: `plans/implement/research/saved-search-row-control.md`
- 픽스처: 저장된 검색 데이터 필요 (state 초기화 시 `saved_searches`에 항목 추가)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-research saved_search_row
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) + C(컨텐츠 반영) 사용.
4. `cargo test -p tench-research saved_search_row` 통과.
5. `cargo check --workspace --locked` 통과.
