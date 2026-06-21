# Test: authoring-title-field

## 검증 대상
spec(`plans/spec/study/authoring-title-field.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Type text, Backspace, and Escape while the field is focused | `authoring_title_field_types_and_deletes` |
| AC2: Move focus away and confirm later keyboard input no longer edits this field | `authoring_title_field_ignores_keys_after_defocus` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/authoring_title_field_ui_e2e.rs`

## Required Test Shape
- **Success**: 타이틀 필드에 텍스트 입력 시 `authoring_title` 갱신 -> 함수: `authoring_title_field_types_and_deletes`
- **Negative**: 포커스 해제 후 키보드 입력이 타이틀 필드에 영향 없음 -> 함수: `authoring_title_field_ignores_keys_after_defocus`
- **Edge case**: Backspace로 빈 문자열까지 삭제 후 추가 입력 -> 함수: `authoring_title_field_backspace_to_empty_then_type`

## 사용할 자동화 노드
implement(`plans/implement/study/authoring-title-field.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `study.authoring.title` | 패널 열기 전 | 노드 없음 |
| `study.authoring.title` | 패널 열기 후 | `""` (빈 문자열) |
| `study.authoring.title` | "Hello" 타이핑 후 | `"Hello"` |
| `study.authoring.title` | Backspace 후 | `"Hell"` |
| `study.authoring.title` | 포커스 해제 후 타이핑 | `"Hell"` (변화 없음) |

## 의존
- 선행 implement: `plans/implement/study/authoring-title-field.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study authoring_title_field
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) + C(컨텐츠 반영) 사용.
4. `cargo test -p tench-study authoring_title_field` 통과.
5. `cargo check --workspace --locked` 통과.
