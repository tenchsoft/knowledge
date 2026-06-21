# Implement: enter-primary-action-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Enter 키가 현재 스테이지에 맞는 주요 액션을 실행한다 (Learn→start_practice, Practice→submit/next, Review→learn 전환)

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Enter 키 처리 분기 | grep 'NamedKey::Enter' apps/study/ |
| state.rs | activate_primary_keyboard_action 메서드 | grep 'fn activate_primary_keyboard_action' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Enter 키 라우팅
- **입력**: KeyboardEvent with Enter
- **처리**: activate_primary_keyboard_action() 호출 — stage별 분기
- **출력/사이드 이펙트**: Learn→start_practice, Practice→submit_answer 또는 next_problem, Review→stage=Learn


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
(KB 노드 — 단축키 전용, 별도 자동화 노드 없음)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
