# Implement: profile-wizard-tab-focus-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 프로필 설정 마법사 Identity 단계에서 Tab 키로 LearnerId/DisplayName 필드 포커스를 전환한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | 프로필 마법사 내 Tab 키 처리 | grep 'show_profile_setup_modal' apps/study/ |
| state.rs | wizard_active_field 전환 | grep 'wizard_active_field' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Tab 키 라우팅 (프로필 마법사)
- **입력**: KeyboardEvent with Tab (show_profile_setup_modal == true)
- **처리**: wizard_active_field를 LearnerId↔DisplayName 전환
- **출력/사이드 이펙트**: 활성 입력 필드 변경, 테두리 색상 업데이트


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
