# Test: save-draft-authoring-button

## 검증 대상
spec(`plans/spec/study/save-draft-authoring-button.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Use it in the normal visible state and confirm the displayed state changes immediately | `save_draft_authoring_button_saves_without_closing` |
| AC2: Use it again or at a boundary state and confirm the state does not drift | `save_draft_authoring_button_saves_empty_draft` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/save_draft_authoring_button_ui_e2e.rs`

## Required Test Shape
- **Success**: Save Draft 클릭 시 드래프트 저장, 패널 유지 -> 함수: `save_draft_authoring_button_saves_without_closing`
- **Negative**: 빈 필드 상태에서 Save Draft 클릭 시 에러 없이 처리 -> 함수: `save_draft_authoring_button_saves_empty_draft`
- **Edge case**: 연속 Save Draft 클릭 시 멱등성 유지 -> 함수: `save_draft_authoring_button_idempotent`

## 사용할 자동화 노드
implement(`plans/implement/study/save-draft-authoring-button.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `study.authoring.save_draft` | 패널 열기 전 | 노드 없음 |
| `study.authoring.save_draft` | 패널 열기 후 | `enabled: true` |
| `study.authoring.title` | Save Draft 후 | 이전 입력값 유지 |

## 의존
- 선행 implement: `plans/implement/study/save-draft-authoring-button.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study save_draft_authoring_button
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) + B(출현/소멸) 사용.
4. `cargo test -p tench-study save_draft_authoring_button` 통과.
5. `cargo check --workspace --locked` 통과.
