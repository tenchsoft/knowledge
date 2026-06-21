# Test: new-curriculum-authoring-button

## 검증 대상
spec(`plans/spec/study/new-curriculum-authoring-button.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Use it in the normal visible state and confirm the displayed state changes immediately | `new_curriculum_authoring_button_creates_draft` |
| AC2: Use it again or at a boundary state and confirm the state does not drift | `new_curriculum_authoring_button_repeated_clicks` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/new_curriculum_authoring_button_ui_e2e.rs`

## Required Test Shape
- **Success**: New Curriculum 버튼 클릭 시 빈 커리큘럼 드래프트 생성 및 필드 초기화 -> 함수: `new_curriculum_authoring_button_creates_draft`
- **Edge case**: 연속 클릭 시 새 커리큘럼마다 깨끗한 상태로 시작 -> 함수: `new_curriculum_authoring_button_repeated_clicks`

## 사용할 자동화 노드
implement(`plans/implement/study/new-curriculum-authoring-button.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `study.authoring.new_curriculum` | 패널 열기 전 | 노드 없음 |
| `study.authoring.new_curriculum` | 패널 열기 후 | `enabled: true` |
| `study.authoring.title` | New Curriculum 클릭 후 | `""` (초기화됨) |
| `study.authoring.body` | New Curriculum 클릭 후 | `""` (초기화됨) |

## 의존
- 선행 implement: `plans/implement/study/new-curriculum-authoring-button.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study new_curriculum_authoring_button
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) 사용.
4. `cargo test -p tench-study new_curriculum_authoring_button` 통과.
5. `cargo check --workspace --locked` 통과.
