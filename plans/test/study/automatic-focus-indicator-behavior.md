# Test: automatic-focus-indicator-behavior

## 검증 대상
spec(`plans/spec/study/automatic-focus-indicator-behavior.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Render it when the underlying state is empty, partial, and populated | `focus_indicator_appears_on_focused_element` |
| AC2: Change upstream state and verify the UI updates on the next paint | `focus_indicator_moves_on_focus_change` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/automatic_focus_indicator_ui_e2e.rs`

## Required Test Shape
- **Success**: 포커스 변경 시 focus_indicator rect 갱신 -> 함수: `focus_indicator_appears_on_focused_element`
- **Negative**: 포커스 None일 때 focus_indicator 없음 -> 함수: `focus_indicator_absent_when_no_focus`
- **Edge case**: 연속 포커스 변경 시 인디케이터 추적 -> 함수: `focus_indicator_moves_on_focus_change`

## 사용할 자동화 노드
implement(`plans/implement/study/automatic-focus-indicator-behavior.md`)의 자동화 노드 표와 일치.

(자동 렌더링 동작 -- 별도 자동화 노드 불필요. `focus_indicator` 필드 상태로 검증.)

| 검증 대상 | 검증 시점 | 기대 state |
|-----------|------------|------------|
| `state.focus_indicator` | 포커스 None | `None` |
| `state.focus_indicator` | SearchBox 포커스 후 | `Some(Rect { ... })` |
| `state.focus_indicator` | 포커스 해제 후 | `None` |

## 의존
- 선행 implement: `plans/implement/study/automatic-focus-indicator-behavior.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study automatic_focus_indicator
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. `focus_indicator` 필드를 grep해 노출 확인.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 B(출현/소멸) 사용.
4. `cargo test -p tench-study automatic_focus_indicator` 통과.
5. `cargo check --workspace --locked` 통과.
