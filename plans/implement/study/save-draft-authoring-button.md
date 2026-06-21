# Implement: save-draft-authoring-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 저작 패널에서 Save Draft 버튼 클릭으로 작성 중인 내용을 임시 저장한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::AuthoringSaveDraft 핸들링 | grep 'StudyHit::AuthoringSaveDraft' apps/study/ |
| state.rs | save_authoring_draft 메서드 | grep 'fn save_authoring_draft' apps/study/ |
| tutor.rs | save_btn 렌더링 | grep 'save_btn' apps/study/src-tauri/src/ui/tutor.rs |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on save_btn rect
- **처리**: save_authoring_draft() 호출
- **출력/사이드 이펙트**: show_authoring_panel=false (프로덕션에서는 저장소에 저장)


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.authoring.save_draft | button | save draft | show_authoring_panel |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
