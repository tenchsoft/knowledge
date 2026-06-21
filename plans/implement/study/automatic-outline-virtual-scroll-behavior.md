# Implement: automatic-outline-virtual-scroll-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 아웃라인의 유닛/컨셉 목록이 `outline_scroll_offset`에 따라 가상 스크롤되어, 화면에 보이는 범위 밖의 항목은 렌더되지 않는다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (가상 스크롤) | 순회 중 y 좌표가 visible_top/visible_bottom 범위 밖이면 skip/break | ``fn paint_outline` 내 `visible_top`/`visible_bottom` 분기` |

## 필요한 변경 (의도 단위)
### 1. 가상 스크롤 범위 계산 및 적용
- **입력**: `state.outline_scroll_offset`과 `regions.outline`의 y 범위
- **처리**: `visible_top`을 `regions.outline.y0 + 44.0`, `visible_bottom`을 `review_queue_rect(regions.outline).y0`로 설정. 유닛 헤더가 `visible_top` 위면 `continue`, `visible_bottom` 아래면 `break`. 컨셉 행도 동일하게 필터링.
- **출력/사이드 이펙트**: 보이는 영역의 항목만 렌더되어 성능이 최적화된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_outline` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
