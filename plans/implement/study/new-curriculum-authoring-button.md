# Implement: new-curriculum-authoring-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 저작 패널에서 New Curriculum 버튼 클릭으로 새 커리큘럼 작성을 시작한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::AuthoringNewCurriculum 핸들링 | grep 'StudyHit::AuthoringNewCurriculum' apps/study/ |
| state.rs | create_new_curriculum 메서드 | grep 'fn create_new_curriculum' apps/study/ |
| tutor.rs | new_curriculum_btn 렌더링 | grep 'new_curriculum_btn' apps/study/src-tauri/src/ui/tutor.rs |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on new_curriculum_btn rect
- **처리**: create_new_curriculum() 호출
- **출력/사이드 이펙트**: authoring_title/body clear, show_authoring_panel=true


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.authoring.new_curriculum | button | new curriculum | show_authoring_panel |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
