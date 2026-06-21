# Implement: automatic-accessibility-label-coverage-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Study 앱의 모든 주요 UI 영역(Header, Curriculum, LearnSurface, PracticeSurface, ReviewSurface, TutorPanel, StatsModal)에 접근성 라벨이 자동으로 부여되고, i18n 키 누락이 UX audit에 반영된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/state/builders.rs` (기본 라벨 생성) | 7개 영역에 대한 `StudyAccessibilityLabel` 항목 생성 | ``fn default_accessibility_labels`` |
| `apps/study/src-tauri/src/ui/state.rs` (UX audit 갱신) | `accessibility_labels` 필드 길이를 `StudyUxAudit.accessibility_label_count`에 반영 | ``fn refresh_ux_audit`` |
| `apps/study/src-tauri/src/ui/mod.rs` (접근성 트리) | `accessibility_tree`에서 라벨 문자열 반환 | ``fn accessibility_tree`` |

## 필요한 변경 (의도 단위)
### 1. 접근성 라벨 기본 목록 구성
- **입력**: `StudyAccessibilityTarget` 열거형의 각 변형(Header, Curriculum, LearnSurface, PracticeSurface, ReviewSurface, TutorPanel, StatsModal)
- **처리**: 각 변형에 대해 i18n 키(`study.a11y.<target>`)를 매핑한 `StudyAccessibilityLabel` 벡터를 생성한다.
- **출력/사이드 이펙트**: StudyState의 `accessibility_labels` 필드가 초기화 시 7개 이상의 항목을 포함한다.
### 2. UX audit에 라벨 커버리지 반영
- **입력**: `accessibility_labels` 벡터의 길이와 i18n catalog의 coverage report
- **처리**: `refresh_ux_audit`에서 `accessibility_label_count`를 라벨 길이로 설정하고, 누락된 i18n 키를 `missing_i18n_keys`에 추가한다.
- **출력/사이드 이펙트**: `StudyUxAudit.accessibility_label_count`가 항상 실제 라벨 수와 일치한다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `accessibility_tree` 및 `refresh_ux_audit`에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
