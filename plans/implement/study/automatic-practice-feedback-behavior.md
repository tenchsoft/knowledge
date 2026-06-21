# Implement: automatic-practice-feedback-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 정답 제출 후 `feedback`이 `Some(correct)`로 설정되면, 정답/오답에 따른 색상 피드백 카드와 해설, cause_tag가 자동으로 렌더링된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/practice.rs` (피드백 카드) | `state.feedback`가 `Some`인 경우 피드백 카드 렌더 | ``fn paint_practice_surface` 내 `if let Some(correct) = state.feedback` 분기` |
| `apps/study/src-tauri/src/ui/state.rs` (피드백 설정) | `submit_answer`에서 정답/오답 판정 후 `feedback` 설정 | ``fn submit_answer`` |

## 필요한 변경 (의도 단위)
### 1. 피드백 카드 자동 렌더
- **입력**: `state.feedback` — `Option<bool>`, `state.current_problem()`의 solution 및 cause_tag
- **처리**: `feedback`이 `Some(true)`면 `STATUS_READY` 테두리에 정답 메시지, `Some(false)`면 `STATUS_ERROR` 테두리에 오답 메시지를 표시. solution 텍스트와 cause_tag도 함께 렌더.
- **출력/사이드 이펙트**: 정답/오답에 따른 시각적 피드백이 자동으로 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_practice_surface` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
