# Implement: header-goal-setting-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더의 Goals 버튼 클릭으로 목표 설정 모달을 토글한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::GoalSetting 핸들링 | grep 'StudyHit::GoalSetting' apps/study/ |
| state.rs | show_goal_modal 토글 | grep 'show_goal_modal' apps/study/ |
| curriculum.rs | goal_btn hit test | grep 'goal_btn' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on goal_btn rect
- **처리**: show_goal_modal 토글
- **출력/사이드 이펙트**: 목표 설정 모달 표시/숨김


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.header.goals | button | goals | 항상 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
