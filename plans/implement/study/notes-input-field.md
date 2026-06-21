# Implement: notes-input-field

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 노트 패널의 텍스트 입력 필드에서 타이핑으로 노트 내용을 작성한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::NotesInput 핸들링 | grep 'StudyHit::NotesInput' apps/study/ |
| on_text_event (mod.rs) | NotesInput 포커스 시 키보드 입력 처리 | grep 'StudyFocusTarget::NotesInput' apps/study/ |
| learn.rs | notes_input_rect 렌더링 | grep 'fn notes_input_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 입력 필드 포커스
- **입력**: PointerEvent::Down on notes_input_rect
- **처리**: focus_target = NotesInput 설정
- **출력/사이드 이펙트**: 테두리 색상 ACCENT_STUDY로 변경

### 2. 키보드 입력 처리
- **입력**: Character/Backspace/Escape/Enter 키
- **처리**: notes_input 문자열 조작
- **출력/사이드 이펙트**: 문자 추가/삭제, Escape로 포커스 해제, Enter로 save_note()


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.notes.input | text_input | note input | show_notes_panel && stage == Learn |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
