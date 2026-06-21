# Implement: automatic-visual-timeline-fill-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Learn 화면의 비주얼 타임라인 스크러버가 `visual_timeline_position`(0.0~1.0)에 비례하여 `ACCENT_STUDY` 색상으로 자동 채워진다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/learn.rs` (타임라인 렌더) | scrubber 영역을 `NEUTRAL_600` 배경으로 그리고, `filled_w`만큼 `ACCENT_STUDY`로 채움 | ``fn paint_learn_surface` 내 scrubber 분기` |

## 필요한 변경 (의도 단위)
### 1. 타임라인 채우기 자동 렌더
- **입력**: `state.visual_timeline_position` — f64 (0.0~1.0)
- **처리**: scrubber의 전체 폭에 `visual_timeline_position`을 곱하여 `filled_w`를 계산. `filled_w > 0`인 경우 scrubber 좌측부터 `filled_w`까지 `ACCENT_STUDY` 색상으로 `fill_rounded_rect`.
- **출력/사이드 이펙트**: 타임라인이 현재 위치에 따라 자동으로 채워진다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_learn_surface` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
