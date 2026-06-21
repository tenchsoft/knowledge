# Test: smart-collection-row-control

## 검증 대상
spec(`plans/spec/research/smart-collection-row-control.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Activate when required state is available; visible result happens immediately | `smart_collection_row_filters_papers_on_click` |
| AC2: Activate when required state is missing; disabled/empty/no-op without corrupting data | `smart_collection_row_noop_when_empty` |
| AC3: Activate while focus is in another input; focus ownership deterministic | `smart_collection_row_click_from_search_focus` |
| AC4: Repeat quickly; state changes idempotent | `smart_collection_row_rapid_clicks_idempotent` |

## 테스트 파일 위치
`apps/research/src-tauri/tests/smart_collection_row_control_ui_e2e.rs`

## Required Test Shape
- **Success**: 스마트 컬렉션 행 클릭 시 규칙 기반 논문 필터링 -> 함수: `smart_collection_row_filters_papers_on_click`
- **Negative**: 스마트 컬렉션 없을 때 행 미노출 -> 함수: `smart_collection_row_noop_when_empty`
- **Edge case**: 연속 클릭 시 멱등성 -> 함수: `smart_collection_row_rapid_clicks_idempotent`

## 사용할 자동화 노드
implement(`plans/implement/research/smart-collection-row-control.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `research.smart_collection.0` | 스마트 컬렉션 있을 때 | `enabled: true`, label: 컬렉션 이름 |
| `research.smart_collection.0` | 클릭 후 | search_query가 규칙 기반 쿼리로 설정 |
| `research.header.search` | 스마트 컬렉션 클릭 후 | `value`: 규칙 기반 쿼리 텍스트 |

## 의존
- 선행 implement: `plans/implement/research/smart-collection-row-control.md`
- 픽스처: 스마트 컬렉션 데이터 필요 (state 초기화 시 `smart_collections`에 항목 추가)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-research smart_collection_row
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) + C(컨텐츠 반영) 사용.
4. `cargo test -p tench-research smart_collection_row` 통과.
5. `cargo check --workspace --locked` 통과.
