# Test: annotation-list-row-control

## 검증 대상
spec(`plans/spec/research/annotation-list-row-control.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Activate when required state is available; visible result happens immediately | `annotation_list_row_selects_on_click` |
| AC2: Activate when required state is missing; disabled/empty/no-op without corrupting data | `annotation_list_row_noop_when_no_annotations` |
| AC3: Activate while focus is in another input; focus ownership deterministic | `annotation_list_row_click_from_pdf_search` |
| AC4: Repeat quickly; state changes idempotent | `annotation_list_row_rapid_clicks_idempotent` |

## 테스트 파일 위치
`apps/research/src-tauri/tests/annotation_list_row_control_ui_e2e.rs`

## Required Test Shape
- **Success**: 어노테이션 행 클릭 시 해당 어노테이션 선택 -> 함수: `annotation_list_row_selects_on_click`
- **Negative**: 어노테이션 없을 때 빈 목록 메시지 -> 함수: `annotation_list_row_noop_when_no_annotations`
- **Edge case**: 연속 클릭 시 멱등성 -> 함수: `annotation_list_row_rapid_clicks_idempotent`

## 사용할 자동화 노드
implement(`plans/implement/research/annotation-list-row-control.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `research.pdf.annotation_list_toggle` | PDF 모드 진입 후 | `enabled: true` |
| `research.pdf.annotation.0` | 어노테이션 목록 토글 후 | `enabled: true` |
| `research.pdf.annotation.0` | 클릭 후 | 해당 어노테이션 selected 상태 |
| `research.pdf.annotation.1` | 첫 번째 클릭 후 | selected 아님 |

## 의존
- 선행 implement: `plans/implement/research/annotation-list-row-control.md`
- 픽스처: PDF 어노테이션 데이터 필요 (state에 `pdf_annotations` 항목 추가)
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-research annotation_list_row
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) + B(출현/소멸) 사용.
4. `cargo test -p tench-research annotation_list_row` 통과.
5. `cargo check --workspace --locked` 통과.
