# Implement: authoring-body-field

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 저작 패널의 본문 입력 필드. 텍스트 입력 시 `authoring_body` 상태 갱신.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/tutor.rs::paint_authoring_panel` | body 필드 paint | `authoring_body` |
| `apps/study/src-tauri/src/ui/state/types.rs::StudyState` | `authoring_body` 필드 | `pub authoring_body` |

## 필요한 변경 (의도 단위)
### 1. Body 필드 paint
- **입력**: `show_authoring_panel == true`
- **처리**: `authoring_body`가 비어있으면 placeholder 표시, 아니면 내용 표시
- **출력/사이드 이펙트**: paint만

### 2. 키보드 입력 라우팅
- **입력**: 저작 패널 활성 + body 필드 포커스 시 키보드 이벤트
- **처리**: 문자 입력 → `authoring_body` append, Backspace → pop
- **출력/사이드 이펙트**: authoring_body 갱신, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `study.authoring.body` | `TextInput` | `authoring_body` 내용 | show_authoring_panel == true |

## 의존
- 선행 implement: authoring-panel-close-button

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
