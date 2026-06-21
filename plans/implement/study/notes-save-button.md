# Implement: notes-save-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 노트 패널의 Save 버튼 클릭으로 현재 입력된 노트를 저장한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::NotesSave 핸들링 | grep 'StudyHit::NotesSave' apps/study/ |
| state.rs | save_note 메서드 | grep 'fn save_note' apps/study/ |
| learn.rs | notes_save_rect 렌더링 | grep 'fn notes_save_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on notes_save_rect
- **처리**: save_note() 호출
- **출력/사이드 이펙트**: notes 벡터에 StudyNote 추가, notes_input clear


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.notes.save | button | save note | show_notes_panel && stage == Learn |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
