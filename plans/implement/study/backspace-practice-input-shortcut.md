# Implement: backspace-practice-input-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 Backspace 키를 누르면 커서 앞 글자가 삭제된다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Backspace 키 핸들링 분기 추가 | grep 'NamedKey::Backspace' apps/study/ |
| state.rs | backspace_at_cursor 메서드 호출 | grep 'fn backspace_at_cursor' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Backspace 키 라우팅
- **입력**: KeyboardEvent with Backspace
- **처리**: stage == Practice인지 확인 후 backspace_at_cursor() 호출
- **출력/사이드 이펙트**: input_text에서 커서 앞 문자 제거, input_cursor_pos 감소


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
