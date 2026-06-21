# Implement: automatic-achievement-unlock-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 목표 달성 시(first-session, streak-10, problems-100) 업적이 자동으로 잠금 해제되고, 골 모달에 별 아이콘과 진행률이 반영된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/state.rs` (업적 체크) | `check_achievements`에서 streak/session_len 기준으로 unlocked/progress 갱신 | ``fn check_achievements`` |
| `apps/study/src-tauri/src/ui/tutor.rs` (골 모달 렌더) | 업적 섹션에 잠금/해제 아이콘과 라벨 표시 | ``fn paint_goal_modal`` |

## 필요한 변경 (의도 단위)
### 1. 업적 조건 평가
- **입력**: session 결과(`session_results`), streak 수치, 총 문제 수
- **처리**: `check_achievements`에서 각 업적 id(`first-session`, `streak-10`, `problems-100`)에 대해 조건을 평가하고 `unlocked`/`progress`를 갱신한다.
- **출력/사이드 이펙트**: `achievements` 벡터의 해당 항목이 갱신된다.
### 2. 골 모달에 업적 표시
- **입력**: `achievements` 벡터의 `unlocked` 및 `progress` 값
- **처리**: `paint_goal_modal`에서 업적 섹션에 별 아이콘(잠금/해제)과 i18n 라벨을 렌더한다.
- **출력/사이드 이펙트**: 잠금 해제된 업적은 `STATUS_WARNING` 별 아이콘, 미해제는 `NEUTRAL_500` 빈 별 아이콘으로 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_goal_modal` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
