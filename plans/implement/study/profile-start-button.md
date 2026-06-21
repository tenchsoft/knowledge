# Implement: profile-start-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 프로필 설정 마법사 Done 단계에서 Start 버튼 클릭으로 설정을 완료하고 앱을 시작한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ProfileSetupNext 핸들링 (Done 단계) | grep 'ProfileSetupNext' apps/study/ |
| state.rs | advance_profile_step → complete_profile_setup | grep 'fn complete_profile_setup' apps/study/ |
| tutor.rs | next_btn 렌더링 (Done 시 Start 라벨) | grep 'study.profile.start' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. Start 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on next_btn rect (step == Done)
- **처리**: advance_profile_step() → complete_profile_setup() 호출
- **출력/사이드 이펙트**: 프로필 설정 완료, show_profile_setup_modal=false, select_domain_level_locale 적용


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.profile.next | button | start | show_profile_setup_modal && step == Done |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
