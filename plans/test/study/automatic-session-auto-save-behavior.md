# Test: automatic-session-auto-save-behavior

## 검증 대상
spec(`plans/spec/study/automatic-session-auto-save-behavior.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Create a restorable SessionSnapshot from current stage, concept, problem, input, timer, and results | `session_auto_save_creates_snapshot` |
| AC2: Render it when the underlying state is empty, partial, and populated | `session_auto_save_updates_after_state_change` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/automatic_session_auto_save_ui_e2e.rs`

## Required Test Shape
- **Success**: AnimFrame 경과 후 세션 스냅샷 자동 생성 -> 함수: `session_auto_save_creates_snapshot`
- **Negative**: 상태 변화 없이 두 번째 자동 저장 시 동일 스냅샷 -> 함수: `session_auto_save_idempotent_without_changes`
- **Edge case**: 상태 변화 후 자동 저장 시 스냅샷 반영 -> 함수: `session_auto_save_updates_after_state_change`

## 사용할 자동화 노드
implement(`plans/implement/study/automatic-session-auto-save-behavior.md`)의 자동화 노드 표와 일치.

(자동 렌더링 동작 -- 별도 자동화 노드 불필요. `auto_save_session()` 호출 및 `last_saved_snapshot` 상태로 검증.)

| 검증 대상 | 검증 시점 | 기대 state |
|-----------|------------|------------|
| `state.last_saved_snapshot` | 초기 상태 | `None` |
| `state.last_saved_snapshot` | AnimFrame 30초 경과 후 | `Some(SessionSnapshot { ... })` |
| `state.last_saved_snapshot` | 상태 변화 + 다음 자동 저장 | 스냅샷 필드가 변경된 상태 반영 |

## 의존
- 선행 implement: `plans/implement/study/automatic-session-auto-save-behavior.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study automatic_session_auto_save
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. `auto_save_session` 메서드를 grep해 위치 확인.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) 사용. `dispatch_window(AnimFrame(ms))`로 시간 진행.
4. `cargo test -p tench-study automatic_session_auto_save` 통과.
5. `cargo check --workspace --locked` 통과.
