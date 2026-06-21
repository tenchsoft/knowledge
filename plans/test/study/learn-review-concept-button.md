# Test: learn-review-concept-button

## 검증 대상
spec(`plans/spec/study/learn-review-concept-button.md`)의 acceptance criteria -> 테스트 함수 매핑.

| Acceptance Criteria | 시나리오 (테스트 함수명) |
|---------------------|---------------------------|
| AC1: Use it in the normal visible state and confirm the displayed state changes immediately | `learn_review_concept_switches_to_learn_stage` |
| AC2: Use it again or at a boundary state and confirm the state does not drift | `learn_review_concept_button_disabled_without_feedback` |

## 테스트 파일 위치
`apps/study/src-tauri/tests/learn_review_concept_button_ui_e2e.rs`

## Required Test Shape
- **Success**: Review Concept 버튼 클릭 시 Learn 스테이지로 전환 -> 함수: `learn_review_concept_switches_to_learn_stage`
- **Negative**: 피드백 없을 때 버튼 disabled -> 함수: `learn_review_concept_button_disabled_without_feedback`

## 사용할 자동화 노드
implement(`plans/implement/study/learn-review-concept-button.md`)의 자동화 노드 표와 일치.

| debug_id | 검증 시점 | 기대 value/state |
|----------|------------|-------------------|
| `study.practice.review_concept` | 피드백 없는 Practice 스테이지 | `enabled: false` |
| `study.practice.review_concept` | 피드백 있는 Practice 스테이지 | `enabled: true` |
| `study.stage` | Review Concept 클릭 후 | `"Learn"` |

## 의존
- 선행 implement: `plans/implement/study/learn-review-concept-button.md`
- 픽스처: 불필요
- 다이얼로그 주입: 불필요

## Verification
```bash
cargo test -p tench-study learn_review_concept_button
cargo check --workspace --locked
```

## 작업 절차 (실행 에이전트가 매번 따른다)
1. spec과 implement를 먼저 읽음.
2. 자동화 노드 셀렉터를 현재 코드에 grep해 노출 확인. 없으면 implement로 회귀.
3. 각 시나리오 함수 작성 -- 행위 검증 패턴 A(Value 변이) + disabled 검증 사용.
4. `cargo test -p tench-study learn_review_concept_button` 통과.
5. `cargo check --workspace --locked` 통과.
