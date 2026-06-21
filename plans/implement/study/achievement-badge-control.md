# Implement: achievement-badge-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 목표 모달 내 업적 뱃지를 탭/클릭하면 상세 정보를 토글한다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/tutor.rs::paint_goal_modal` | 뱃지 행에 클릭 가능한 영역 추가 | `achievement.unlocked` |
| `apps/study/src-tauri/src/ui/state/types.rs::StudyHit` | `AchievementBadge(idx)` 변형 | `AchievementBadge` |
| `apps/study/src-tauri/src/ui/mod.rs::on_pointer_event` | `AchievementBadge` hit 처리 | `StudyHit::AchievementBadge` |
| `apps/study/src-tauri/src/ui/mod.rs::study_automation_nodes` | 뱃지 자동화 노드 | `study.achievement` |

## 필요한 변경 (의도 단위)
### 1. 뱃지 클릭 시 상세 토글
- **입력**: `StudyHit::AchievementBadge(idx)` pointer down
- **처리**: 선택된 업적 인덱스를 저장, 상세 설명 표시 토글
- **출력/사이드 이펙트**: `state.expanded_achievement_idx` 갱신, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `study.achievement.{idx}` | `Button` | unlocked 상태 | show_goal_modal == true |

## 의존
- 선행 implement: goal-modal-close-button

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
