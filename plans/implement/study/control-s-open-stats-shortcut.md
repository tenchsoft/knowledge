# Implement: control-s-open-stats-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Ctrl+S를 누르면 통계 모달이 열린다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Ctrl+S 단축키 처리 | grep 'control.*&quot;s&quot;' apps/study/ |
| state.rs | open_stats 메서드 | grep 'fn open_stats' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Ctrl+S 라우팅
- **입력**: KeyboardEvent with Ctrl+S
- **처리**: open_stats() 호출 — show_stats_modal = true
- **출력/사이드 이펙트**: 통계 모달 표시


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
