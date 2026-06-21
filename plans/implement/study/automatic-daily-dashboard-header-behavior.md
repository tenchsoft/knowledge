# Implement: automatic-daily-dashboard-header-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 데스크톱 뷰포트(≥700px)에서 헤더에 due review 수, new lesson 수, accuracy 퍼센트가 자동으로 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (헤더 대시보드) | 헤더 폭 ≥700px 조건부 대시보드 미니 위젯 렌더 | ``fn paint_shell` 내 `dash_x` 분기` |
| `apps/study/src-tauri/src/ui/state.rs` (대시보드 갱신) | dashboard 필드를 현재 상태에서 재계산 | ``fn refresh_daily_dashboard`` |

## 필요한 변경 (의도 단위)
### 1. 대시보드 미니 위젯 렌더
- **입력**: `state.dashboard`의 `due_review_count`, `new_lesson_count`, `accuracy_percent`
- **처리**: 헤더 폭이 700px 이상일 때 `dash_x` 위치에 세 개의 텍스트 항목을 렌더한다. due review는 `STATUS_WARNING`, 나머지는 `NEUTRAL_400` 색상.
- **출력/사이드 이펙트**: 헤더에 일일 대시보드가 자동으로 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_shell` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
