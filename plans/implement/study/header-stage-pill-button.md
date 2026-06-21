# Implement: header-stage-pill-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더의 스테이지 필 클릭 시 Learn→Practice→Review 순으로 스테이지가 순환한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::StageClick 핸들링 | grep 'StudyHit::StageClick' apps/study/ |
| state.rs | cycle_stage 메서드 | grep 'fn cycle_stage' apps/study/ |
| curriculum.rs | stage_rect 렌더링 | grep 'fn stage_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 필 클릭 핸들링
- **입력**: PointerEvent::Down on stage_rect
- **처리**: cycle_stage(false) 호출
- **출력/사이드 이펙트**: stage 순환, feedback=None, input_text clear


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.header.stage | button | stage label | 항상 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
