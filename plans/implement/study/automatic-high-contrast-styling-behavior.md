# Implement: automatic-high-contrast-styling-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더의 HC 토글 버튼 클릭 시 `high_contrast_mode`가 전환되고, 버튼 라벨과 색상이 모드 상태를 반영하여 자동으로 갱신된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (HC 버튼 렌더) | HC 버튼의 라벨/색상이 `high_contrast_mode` 상태에 따라 전환 | ``fn paint_shell` 내 `hc_btn` 분기` |
| `apps/study/src-tauri/src/ui/state.rs` (모드 토글) | `toggle_high_contrast`로 `high_contrast_mode` 전환 | ``fn toggle_high_contrast`` |

## 필요한 변경 (의도 단위)
### 1. HC 버튼 상태 반영 렌더
- **입력**: `state.high_contrast_mode` 불리언
- **처리**: 활성 시 버튼 라벨을 `"HC"`(대문자)로, 색상을 `ACCENT_STUDY`로 렌더. 비활성 시 `"hc"`(소문자), `NEUTRAL_400`으로 렌더.
- **출력/사이드 이펙트**: HC 버튼이 현재 모드 상태를 시각적으로 나타낸다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_shell` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
