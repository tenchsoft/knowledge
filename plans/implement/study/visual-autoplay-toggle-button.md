# Implement: visual-autoplay-toggle-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Learn 화면에서 자동재생 토글 버튼으로 비주얼 자동재생을 켜거나 끈다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::VisualAutoplay 핸들링 | grep 'StudyHit::VisualAutoplay' apps/study/ |
| state.rs | toggle_visual_autoplay 메서드 | grep 'fn toggle_visual_autoplay' apps/study/ |
| learn.rs | autoplay_btn 렌더링 | grep 'autoplay_btn' apps/study/src-tauri/src/ui/learn.rs |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on autoplay_btn rect
- **처리**: toggle_visual_autoplay() 호출
- **출력/사이드 이펙트**: visual_autoplay 토글, Auto: ON/OFF 라벨 및 색상 변경


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.visual.autoplay | button | autoplay visual | stage == Learn |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
