# Implement: learn-review-concept-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 피드백 표시 후 Review Concept 버튼 클릭 시 Learn 스테이지로 전환하여 개념을 복습한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ReviewConcept 핸들링 | grep 'StudyHit::ReviewConcept' apps/study/ |
| practice.rs | review_concept_rect 렌더링 | grep 'fn review_concept_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on review_concept_rect
- **처리**: stage = Stage::Learn 설정
- **출력/사이드 이펙트**: Learn 스테이지로 전환, 개념 내용 표시


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.review_concept | button | review concept | stage == Practice && feedback.is_some() |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
