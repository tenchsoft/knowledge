# Implement: practice-character-input-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 임의의 문자 키 입력 시 커서 위치에 문자를 삽입한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Practice 모드 일반 문자 입력 분기 | grep 'LogicalKey::Character.*stage == Stage::Practice' apps/study/ |
| state.rs | insert_char_at_cursor 메서드 | grep 'fn insert_char_at_cursor' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 문자 입력 라우팅
- **입력**: KeyboardEvent with Character (stage == Practice, 다른 단축키 불매칭 시)
- **처리**: insert_char_at_cursor(ch) 호출
- **출력/사이드 이펙트**: input_text에 ch 삽입, 커서 이동


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
