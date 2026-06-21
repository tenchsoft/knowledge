# Implement: profile-locale-option-row

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 프로필 설정 마법사 Locale 단계에서 로케일 옵션 행 클릭으로 언어를 선택한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ProfileLocaleSelect(idx) 핸들링 | grep 'ProfileLocaleSelect' apps/study/ |
| state.rs | wizard_locale_idx 갱신 | grep 'wizard_locale_idx' apps/study/ |
| tutor.rs | 로케일 선택 행 렌더링 | grep 'en-US.*ko-KR' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 로케일 행 클릭 핸들링
- **입력**: PointerEvent::Down on locale row rect
- **처리**: wizard_locale_idx = idx 설정
- **출력/사이드 이펙트**: 선택된 로케일 행 하이라이트, ACCENT_STUDY 테두리


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.profile.locale.{locale} | button | locale code | show_profile_setup_modal && step == Locale |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
