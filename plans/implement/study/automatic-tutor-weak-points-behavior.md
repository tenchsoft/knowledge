# Implement: automatic-tutor-weak-points-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 튜터 패널에 현재 약점(cause_tag) 목록이 자동으로 표시된다. `review_queue`의 앞부분에서 최대 5개의 고유 cause_tag를 추출하여 렌더한다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/tutor.rs` (약점 표시) | `weak_points()` 결과를 튜터 패널에 렌더 | ``fn paint_tutor_panel` 내 `weak_points` 분기` |
| `apps/study/src-tauri/src/ui/state.rs` (약점 계산) | `weak_points`에서 review_queue의 cause_tag 추출 | ``fn weak_points`` |

## 필요한 변경 (의도 단위)
### 1. 약점 목록 자동 렌더
- **입력**: `state.weak_points()` — `Vec<String>` (최대 5개 cause_tag)
- **처리**: 목록이 비어있으면 `study.tutor.no_weak_points` i18n 키 텍스트를 `NEUTRAL_400`으로 표시. 항목이 있으면 각 cause_tag를 `STATUS_WARNING` 색상으로 22px 간격으로 나열.
- **출력/사이드 이펙트**: 현재 약점이 튜터 패널에 자동으로 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_tutor_panel` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
