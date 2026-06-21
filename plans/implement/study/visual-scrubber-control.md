# Implement: visual-scrubber-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Learn 화면의 타임라인 스크러버로 비주얼 재생 위치를 드래그하여 조절한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::VisualScrubber 핸들링 | grep 'StudyHit::VisualScrubber' apps/study/ |
| state.rs | set_visual_timeline 메서드 | grep 'fn set_visual_timeline' apps/study/ |
| learn.rs | scrubber 렌더링 | grep 'scrubber' apps/study/src-tauri/src/ui/learn.rs |

## 필요한 변경 (의도 단위)
### 1. 스크러버 드래그 핸들링
- **입력**: PointerEvent::Down/Move on scrubber rect
- **처리**: set_visual_timeline(position) 호출
- **출력/사이드 이펙트**: visual_timeline_position 업데이트, 채워진 바 너비 변경


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.visual.scrubber | slider | timeline | stage == Learn |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
