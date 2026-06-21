# Implement: home-end-practice-cursor-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Home/End 키로 Practice 모드에서 커서를 입력의 시작/끝으로 이동한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Home/End 키 처리 | grep 'NamedKey::Home\|NamedKey::End' apps/study/ |
| state.rs | move_cursor_home / move_cursor_end 메서드 | grep 'fn move_cursor_home\|fn move_cursor_end' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Home/End 키 라우팅
- **입력**: KeyboardEvent with Home/End
- **처리**: stage == Practice 확인 후 move_cursor_home() 또는 move_cursor_end() 호출
- **출력/사이드 이펙트**: input_cursor_pos = 0 또는 input_text.len()


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
