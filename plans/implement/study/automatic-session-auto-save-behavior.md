# Implement: automatic-session-auto-save-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 현재 세션 상태(stage, active indices, input, streak, elapsed, results)가 `auto_save_session` 호출 시 `SessionSnapshot`으로 자동 직렬화된다. `restore_session`으로 복원 가능하다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/state.rs` (세션 저장) | `auto_save_session`에서 현재 상태를 `SessionSnapshot`으로 생성 | ``fn auto_save_session`` |
| `apps/study/src-tauri/src/ui/state.rs` (세션 복원) | `restore_session`에서 스냅샷을 상태에 복원 | ``fn restore_session`` |

## 필요한 변경 (의도 단위)
### 1. 세션 스냅샷 자동 생성
- **입력**: `StudyState`의 현재 필드 값들
- **처리**: `auto_save_session`에서 `stage`, `active_unit_idx`, `active_concept_idx`, `problem_index`, `input_text`, `input_cursor_pos`, `streak`, `elapsed_seconds`, `session_results`를 `SessionSnapshot` 구조체로 패킹한다.
- **출력/사이드 이펙트**: 현재 세션 상태를 나타내는 `SessionSnapshot`이 반환된다.
### 2. 세션 복원
- **입력**: `SessionSnapshot` 구조체
- **처리**: `restore_session`에서 스냅샷의 각 필드를 `StudyState`에 복사하고 `feedback`을 `None`으로 초기화한다.
- **출력/사이드 이펙트**: 이전 세션 상태가 복원된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, state 메서드 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
