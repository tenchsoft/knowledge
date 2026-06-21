# Implement: profile-wizard-enter-key-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 프로필 설정 마법사에서 Enter 키를 누르면 Next/Start 버튼과 동일하게 다음 단계로 진행한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | 프로필 마법사 내 Enter 키 처리 | grep 'show_profile_setup_modal' apps/study/ |
| state.rs | advance_profile_step 메서드 | grep 'fn advance_profile_step' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Enter 키 라우팅 (프로필 마법사)
- **입력**: KeyboardEvent with Enter (show_profile_setup_modal == true)
- **처리**: advance_profile_step() 호출
- **출력/사이드 이펙트**: Identity→DomainLevel→Locale→Done 순서 진행, Done에서 complete_profile_setup 실행


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
