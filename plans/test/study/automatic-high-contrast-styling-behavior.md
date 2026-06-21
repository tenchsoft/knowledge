# Test: automatic-high-contrast-styling-behavior

## 검증 대상
spec(`plans/spec/study/automatic-high-contrast-styling-behavior.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Render it when the underlying state is empty, partial, and populated | `high_contrast_applies_styling_when_toggled_on` |
| AC2: Change upstream state and verify the UI updates on the next paint | `high_contrast_removes_styling_when_toggled_off` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/automatic_high_contrast_styling_ui_e2e.rs`

## Required Test Shape
- **Success**: HC 토글 ON 시 고대비 색상 적용 -> 함수: `high_contrast_applies_styling_when_toggled_on`
- **Negative**: HC 토글 OFF 시 일반 색상 복원 -> 함수: `high_contrast_removes_styling_when_toggled_off`
- **Edge case**: 토글 반복 시 색상 일관성 -> 함수: `high_contrast_toggle_roundtrip`

## 사용할 자동화 노드
implement(`plans/implement/study/automatic-high-contrast-styling-behavior.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `study.header.high_contrast` | 초기 상태 | `value: "off"` |
| `study.header.high_contrast` | 토글 후 | `value: "on"` |
| `study.header.high_contrast` | 다시 토글 후 | `value: "off"` |

## 의존
- 선행 implement: `plans/implement/study/automatic-high-contrast-styling-behavior.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study automatic_high_contrast
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) + D(라운드트립) 사용.
4. `cargo test -p tench-study automatic_high_contrast` 통과.
5. `cargo check --workspace --locked` 통과.
