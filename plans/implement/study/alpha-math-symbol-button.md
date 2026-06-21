# Implement: alpha-math-symbol-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 수학 팔레트에서 'alpha' 버튼 클릭 시 입력 커서에 'alpha' 삽입.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/practice.rs::paint_practice_surface` | 수학 팔레트 paint | `math_symbols` |
| `apps/study/src-tauri/src/ui/curriculum.rs::hit_test` | MathSymbol(4) hit test (alpha=idx 4) | `MathSymbol(idx)` |
| `apps/study/src-tauri/src/ui/mod.rs::on_pointer_event` | 심볼 삽입 | `symbols.get(idx)` |
| `apps/study/src-tauri/src/ui/state.rs::insert_math_symbol` | 커서에 심볼 삽입 | `fn insert_math_symbol` |

## 필요한 변경 (의도 단위)
### 1. Alpha 버튼 paint 및 hit test
- **입력**: `show_math_palette == true && stage == Practice`
- **처리**: symbols 배열에서 idx=4("alpha") 위치에 버튼 paint, 클릭 시 `StudyHit::MathSymbol(4)` 반환
- **출력/사이드 이펙트**: `insert_math_symbol("alpha")` 호출

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `study.practice.math_symbol.alpha` | `Button` | `"alpha"` | show_math_palette && stage == Practice |

## 의존
- 선행 implement: math-palette-toggle-button

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
