# Implement: profile-back-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 프로필 설정 마법사에서 Back 버튼 클릭으로 이전 단계로 돌아간다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ProfileSetupBack 핸들링 | grep 'StudyHit::ProfileSetupBack' apps/study/ |
| state.rs | go_back_profile_step 메서드 | grep 'fn go_back_profile_step' apps/study/ |
| tutor.rs | paint_profile_setup_wizard back_btn 렌더링 | grep 'back_btn' apps/study/src-tauri/src/ui/tutor.rs |

## 필요한 변경 (의도 단위)
### 1. 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on back_btn rect
- **처리**: go_back_profile_step() 호출
- **출력/사이드 이펙트**: DomainLevel→Identity, Locale→DomainLevel, Done→Locale


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.profile.back | button | back | show_profile_setup_modal && step != Identity |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
