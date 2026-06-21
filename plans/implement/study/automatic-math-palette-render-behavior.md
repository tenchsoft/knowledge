# Implement: automatic-math-palette-render-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 `show_math_palette`가 true일 때 수학 기호 팔레트(^, sqrt, frac, pi, alpha, beta, inf, sum)가 자동으로 렌더링된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/practice.rs` (수학 팔레트) | `show_math_palette` 조건부로 4×2 그리드 버튼 렌더 | ``fn paint_practice_surface` 내 `show_math_palette` 분기` |

## 필요한 변경 (의도 단위)
### 1. 수학 팔레트 자동 렌더
- **입력**: `state.show_math_palette` 불리언
- **처리**: true인 경우 8개 기호 배열을 4열 2행 그리드로 배치하여 각 버튼에 `NEUTRAL_600` 배경, `NEUTRAL_500` 테두리, `NEUTRAL_100` 텍스트로 렌더.
- **출력/사이드 이펙트**: Practice 화면에 수학 기호 선택 팔레트가 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_practice_surface` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
