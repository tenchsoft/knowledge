# Implement: practice-answer-input-field

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 답안 입력 필드에 타이핑하여 답을 작성한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Practice 모드 Character 키 입력 처리 | grep 'stage == Stage::Practice' apps/study/ |
| state.rs | insert_char_at_cursor 메서드 | grep 'fn insert_char_at_cursor' apps/study/ |
| practice.rs | input 텍스트 렌더링 | grep 'input_text' apps/study/src-tauri/src/ui/practice.rs |

## 필요한 변경 (의도 단위)
### 1. 문자 입력 처리
- **입력**: KeyboardEvent with Character (stage == Practice)
- **처리**: insert_char_at_cursor(ch) 호출
- **출력/사이드 이펙트**: input_text에 문자 삽입, input_cursor_pos 증가


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.answer | text_input | answer | stage == Practice |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
