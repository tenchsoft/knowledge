# Implement: square-root-math-symbol-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: sqrt( 기호를 답안 커서 위치에 삽입한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::MathSymbol(1) 핸들링 | grep 'StudyHit::MathSymbol' apps/study/ |
| state.rs | insert_math_symbol 메서드 | grep 'fn insert_math_symbol' apps/study/ |
| practice.rs | 수학 기호 버튼 렌더링 | grep 'math_symbols' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 수학 기호 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on math symbol idx=1
- **처리**: insert_math_symbol("sqrt(") 호출
- **출력/사이드 이펙트**: input_text에 "sqrt(" 삽입, input_cursor_pos += len("sqrt(")


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.math_symbol.sqrt | button | sqrt | stage == Practice && show_math_palette |

## 의존
- 선행 implement: math-palette-toggle-button

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
