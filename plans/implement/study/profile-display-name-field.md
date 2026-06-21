# Implement: profile-display-name-field

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 프로필 설정 마법사 Identity 단계에서 Display Name 입력 필드에 타이핑한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ProfileSetupFocusDisplayName 핸들링 | grep 'ProfileSetupFocusDisplayName' apps/study/ |
| on_text_event (mod.rs) | 프로필 마법사 Character/Backspace 입력 | grep 'wizard_display_name' apps/study/ |
| tutor.rs | name_field 렌더링 | grep 'name_field' apps/study/src-tauri/src/ui/tutor.rs |

## 필요한 변경 (의도 단위)
### 1. 필드 포커스
- **입력**: PointerEvent::Down on name_field rect
- **처리**: wizard_active_field = DisplayName 설정
- **출력/사이드 이펙트**: 테두리 색상 ACCENT_STUDY로 변경

### 2. 문자 입력
- **입력**: Character/Backspace 키
- **처리**: wizard_display_name 문자열 조작
- **출력/사이드 이펙트**: display_name 텍스트 업데이트


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.profile.display_name | text_input | display name | show_profile_setup_modal && step == Identity |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
