# Implement: new-concept-authoring-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 저작 패널에서 New Concept 버튼 클릭으로 새 컨셉 작성을 시작한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::AuthoringNewConcept 핸들링 | grep 'StudyHit::AuthoringNewConcept' apps/study/ |
| state.rs | create_new_concept 메서드 | grep 'fn create_new_concept' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on authoring panel new concept area
- **처리**: create_new_concept() 호출
- **출력/사이드 이펙트**: authoring_title/body clear


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.authoring.new_concept | button | new concept | show_authoring_panel |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
