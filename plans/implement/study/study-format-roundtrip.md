# Implement: study-format-roundtrip

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 세션 상태를 SessionSnapshot으로 직렬화하고 복원하여 학습 진행 상태가 보존됨을 보장한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| state.rs | auto_save_session / restore_session 메서드 | grep 'fn auto_save_session\|fn restore_session' apps/study/ |
| state/types.rs | SessionSnapshot 구조체 | grep 'struct SessionSnapshot' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 세션 스냅샷 생성
- **입력**: 현재 StudyState
- **처리**: auto_save_session() → SessionSnapshot 생성
- **출력/사이드 이펙트**: stage, unit/concept idx, problem_index, input_text, cursor_pos, streak, elapsed, session_results 포함

### 2. 세션 복원
- **입력**: SessionSnapshot
- **처리**: restore_session(snapshot) 호출
- **출력/사이드 이펙트**: 모든 세션 필드 복원, feedback=None으로 리셋

### 3. 저장소 연동 (프로덕션)
- **입력**: SessionSnapshot
- **처리**: commands를 통해 로컬 저장소에 직렬화/역직렬화
- **출력/사이드 이펙트**: 앱 재시작 시 마지막 세션 상태 복구


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
(KB 노드 — 단축키 전용, 별도 자동화 노드 없음)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
