# Implement: header-high-contrast-toggle-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더의 HC 버튼 클릭으로 하이 컨트라스트 모드를 토글한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::HighContrastToggle 핸들링 | grep 'StudyHit::HighContrastToggle' apps/study/ |
| state.rs | toggle_high_contrast 메서드 | grep 'fn toggle_high_contrast' apps/study/ |
| curriculum.rs | hc_btn 렌더링 | grep 'hc_btn' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on hc_btn rect
- **처리**: toggle_high_contrast() 호출
- **출력/사이드 이펙트**: high_contrast_mode 토글, HC/hc 라벨 및 색상 변경


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.header.high_contrast | button | high contrast | 항상 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
