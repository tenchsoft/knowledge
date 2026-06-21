# Implement: authoring-panel-close-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 저작 패널 닫기 버튼 클릭 시 패널 숨김.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/tutor.rs::paint_authoring_panel` | 닫기 버튼 paint | `modal_close_rect` |
| `apps/study/src-tauri/src/ui/tutor.rs::hit_test_authoring_close` | 닫기 버튼 hit test | `fn hit_test_authoring_close` |
| `apps/study/src-tauri/src/ui/mod.rs::on_pointer_event` | `CloseModal` 처리 | `StudyHit::CloseModal` |

## 필요한 변경 (의도 단위)
### 1. 닫기 버튼 클릭 처리
- **입력**: `StudyHit::CloseModal` pointer down (authoring panel 활성 시)
- **처리**: `show_authoring_panel = false`
- **출력/사이드 이펙트**: authoring panel 숨김, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `study.modal.close` | `Button` | `"close"` | show_authoring_panel == true |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
