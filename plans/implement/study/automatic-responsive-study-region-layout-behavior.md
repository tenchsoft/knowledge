# Implement: automatic-responsive-study-region-layout-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 뷰포트 크기에 따라 아웃라인 폭, 튜터 패널 표시 여부, 서피스 영역이 자동으로 조정된다. 모바일(<700px)에서는 아웃라인이 축소되고 튜터가 숨겨지며, 터치 리뷰가 활성화된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (리전 계산) | 뷰포트 크기에 따라 outline_w, tutor_w, surface 영역을 동적 계산 | ``fn regions`` |
| `apps/study/src-tauri/src/ui/state.rs` (뷰포트 클래스) | `update_viewport`에서 Mobile/Tablet/Desktop 분류 및 touch_review 활성화 | ``fn update_viewport`` |

## 필요한 변경 (의도 단위)
### 1. 반응형 리전 자동 계산
- **입력**: `size: Size` (뷰포트 크기)
- **처리**: 폭 <520px이면 아웃라인 폭을 30%(최소 88px, 최대 180px)로 축소. 폭 <900px이면 튜터 폭을 0으로 설정. 서피스는 나머지 공간을 차지하되 최소 220px 보장.
- **출력/사이드 이펙트**: 뷰포트 크기에 맞는 리전이 자동으로 계산되어 겹침이 없다.
### 2. 뷰포트 클래스에 따른 상태 전환
- **입력**: `size.width` 값
- **처리**: `update_viewport`에서 Mobile(<700px)이면 `touch_review.enabled = true`, `min_hit_size_px = 48`. Desktop(≥1100px)이면 `touch_review.enabled = false`.
- **출력/사이드 이펙트**: 모바일에서 터치 리뷰가 자동으로 활성화된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `regions` 및 `update_viewport`에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
