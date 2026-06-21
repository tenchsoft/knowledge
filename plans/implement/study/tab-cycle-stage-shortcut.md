# Implement: tab-cycle-stage-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Tab 키로 Learn→Practice→Review 스테이지를 순환하고, Shift+Tab으로 역방향 순환한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | Tab 키 처리 | grep 'NamedKey::Tab' apps/study/ |
| state.rs | cycle_stage 메서드 | grep 'fn cycle_stage' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Tab 키 라우팅
- **입력**: KeyboardEvent with Tab + modifiers.shift
- **처리**: cycle_stage(reverse) 호출
- **출력/사이드 이펙트**: stage가 Learn→Practice→Review 순환, 역방향 시 Review→Practice→Learn


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
