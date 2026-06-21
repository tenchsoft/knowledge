# Implement: automatic-notes-panel-overlay-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: Learn 모드에서 `show_notes_panel`이 true일 때 서피스 우측에 노트 패널이 오버레이로 자동 표시되고, 현재 컨셉에 해당하는 노트만 필터링되어 나열된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/mod.rs` (노트 패널 분기) | `show_notes_panel && stage == Learn` 조건부로 `paint_notes_panel` 호출 | ``fn paint` 내 notes panel 분기` |
| `apps/study/src-tauri/src/ui/learn.rs` (노트 패널 렌더) | 패널 배경, 입력 필드, 저장 버튼, 노트 목록 렌더 | ``fn paint_notes_panel`` |

## 필요한 변경 (의도 단위)
### 1. 노트 패널 오버레이 자동 렌더
- **입력**: `state.show_notes_panel` 불리언과 `state.stage == Stage::Learn`
- **처리**: 둘 다 true인 경우 서피스 우측에 패널(최대 280px, 서피스 폭의 40%)을 오버레이로 그린다. 반투명 `NEUTRAL_800` 배경에 좌측 경계선.
- **출력/사이드 이펙트**: Learn 모드에서 노트 패널이 서피스 위에 오버레이로 표시된다.
### 2. 컨셉 필터 노트 목록
- **입력**: `state.notes` 벡터와 `state.active_concept().id`
- **처리**: 노트 목록에서 `concept_id == active_concept().id`인 항목만 필터링하여 패널에 나열한다.
- **출력/사이드 이펙트**: 현재 컨셉과 관련된 노트만 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_notes_panel` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
