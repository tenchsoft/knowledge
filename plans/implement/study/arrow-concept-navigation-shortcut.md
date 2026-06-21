# Implement: arrow-concept-navigation-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: ArrowUp/ArrowDown 키로 개념 간 탐색, Practice 모드에서는 커서 이동.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/mod.rs::on_text_event` | ArrowUp/ArrowDown 분기 | `NamedKey::ArrowUp` |
| `apps/study/src-tauri/src/ui/state.rs::move_concept` | 개념 이동 | `fn move_concept` |
| `apps/study/src-tauri/src/ui/state.rs::move_cursor` | 커서 이동 | `fn move_cursor` |

## 필요한 변경 (의도 단위)
### 1. ArrowUp/ArrowDown 키 라우팅
- **입력**: ArrowUp/ArrowDown 키 이벤트
- **처리**: Practice 모드면 `move_cursor(-1/1)`, 아니면 `move_concept(-1/1)`
- **출력/사이드 이펙트**: active_concept_idx 또는 input_cursor_pos 갱신, repaint

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(키보드 단축키 — 자동화 노드 불필요)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
