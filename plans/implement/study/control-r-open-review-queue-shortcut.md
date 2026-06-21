# Implement: control-r-open-review-queue-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Ctrl+R을 누르면 리뷰 큐가 열리고 Review 스테이지로 전환된다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Ctrl+R 단축키 처리 | grep 'control.*&quot;r&quot;' apps/study/ |
| state.rs | open_review_queue 메서드 | grep 'fn open_review_queue' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Ctrl+R 라우팅
- **입력**: KeyboardEvent with Ctrl+R
- **처리**: open_review_queue() 호출 — stage = Review, review_index = 1
- **출력/사이드 이펙트**: Review 스테이지 전환 및 리뷰 큐 열기


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
