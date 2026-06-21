# Implement: space-primary-or-text-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Space 키는 Practice 모드에서는 공백 문자를 입력하고, 다른 모드에서는 주요 액션을 실행한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Space 키 처리 분기 | grep 'NamedKey::Space' apps/study/ |
| state.rs | insert_char_at_cursor / activate_primary_keyboard_action | grep 'fn insert_char_at_cursor\|fn activate_primary_keyboard_action' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Space 키 라우팅
- **입력**: KeyboardEvent with Space
- **처리**: stage == Practice이면 insert_char_at_cursor(" "), 아니면 activate_primary_keyboard_action()
- **출력/사이드 이펙트**: Practice에서는 공백 삽입, Learn/Review에서는 주요 액션 실행


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
