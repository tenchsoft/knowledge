# Implement: automatic-learn-visual-surface-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Learn 화면의 비주얼 영역에 `active_visual_draw_plan()` 결과가 자동으로 렌더링되고, 드로잉 플랜이 없으면 플레이스홀더가 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/learn.rs` (비주얼 서피스) | `paint_active_visual_surface`에서 draw plan 기반 렌더 또는 플레이스홀더 | ``fn paint_active_visual_surface`` |
| `apps/study/src-tauri/src/ui/state.rs` (드로잉 플랜) | `active_visual_draw_plan`에서 현재 컨셉의 비주얼 스펙으로 플랜 생성 | ``fn active_visual_draw_plan`` |

## 필요한 변경 (의도 단위)
### 1. 비주얼 서피스 자동 렌더
- **입력**: `state.active_visual_draw_plan()`의 `Option<LearningVisualDrawPlan>` 결과
- **처리**: `Some(plan)`인 경우 `visual_surface_commands`로 변환 후 `VisualSurface::paint_in_rect`로 렌더. `None`인 경우 `NEUTRAL_700` 배경에 라벨만 표시.
- **출력/사이드 이펙트**: 현재 컨셉에 해당하는 비주얼이 Learn 화면에 자동으로 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_active_visual_surface` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
