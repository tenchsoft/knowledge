# Implement: math-palette-toggle-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Practice 모드에서 fx 버튼 클릭으로 수학 기호 팔레트를 토글한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::MathPaletteToggle 핸들링 | grep 'StudyHit::MathPaletteToggle' apps/study/ |
| state.rs | toggle_math_palette 메서드 | grep 'fn toggle_math_palette' apps/study/ |
| practice.rs | math_btn 렌더링 | grep 'math_btn' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on math_btn rect
- **처리**: toggle_math_palette() 호출
- **출력/사이드 이펙트**: show_math_palette 토글, fx 라벨 색상 변경


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.practice.math_palette | button | math palette | stage == Practice |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
