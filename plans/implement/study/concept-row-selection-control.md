# Implement: concept-row-selection-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 아웃라인에서 컨셉 행 클릭 시 해당 컨셉이 선택되고 Learn 스테이지로 전환한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::Concept 핸들링 | grep 'StudyHit::Concept' apps/study/ |
| state.rs | select_concept 메서드 | grep 'fn select_concept' apps/study/ |
| curriculum.rs | concept_rect 렌더링 | grep 'fn concept_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 컨셉 행 클릭 핸들링
- **입력**: PointerEvent::Down on concept_rect
- **처리**: select_concept(unit, concept) 호출
- **출력/사이드 이펙트**: active_unit_idx/concept_idx 갱신, stage=Learn, problem_index=1, feedback=None, input_text clear


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.concept.{unit}.{concept} | button | concept label | 항상 (가시 범위 내) |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
