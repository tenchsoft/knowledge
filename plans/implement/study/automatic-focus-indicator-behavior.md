# Implement: automatic-focus-indicator-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 키보드 포커스가 있는 UI 요소 주변에 `ACCENT_STUDY` 색상의 2px 둥근 테두리가 자동으로 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/mod.rs` (포커스 인디케이터) | paint 최하단에서 `focus_indicator` rect에 stroke 렌더 | ``fn paint` 내 `focus_indicator` 분기` |
| `apps/study/src-tauri/src/ui/state.rs` (포커스 rect 갱신) | 포커스 대상 변경 시 `focus_indicator` 필드에 해당 rect 저장 | ``focus_indicator` 필드` |

## 필요한 변경 (의도 단위)
### 1. 포커스 인디케이터 렌더
- **입력**: `state.focus_indicator` — `Option<Rect>`
- **처리**: `Some(focus_rect)`인 경우 `paint` 최하단에 `stroke_rounded_rect(focus_rect, ACCENT_STUDY, 2.0, 4.0)`를 그린다.
- **출력/사이드 이펙트**: 키보드 포커스가 있는 요소에 시각적 인디케이터가 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
