# Implement: automatic-header-breadcrumb-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더 폭이 ≥560px일 때 `subject / unit / concept` 순서의 브레드크럼이 자동으로 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (브레드크럼) | 헤더 폭 ≥560px 조건부로 subject, `/`, unit, `/`, concept 텍스트 렌더 | ``fn paint_shell` 내 `regions.header.width() >= 560.0` 분기` |

## 필요한 변경 (의도 단위)
### 1. 브레드크럼 자동 렌더
- **입력**: `state.active_subject`, `state.active_unit().label`, `state.active_concept().label`
- **처리**: 헤더 폭 ≥560px일 때 고정 x 위치에 순서대로 subject(`NEUTRAL_300`), `/` 구분자(`NEUTRAL_500`), unit(`NEUTRAL_300`), `/`, concept(`NEUTRAL_100`)을 그린다.
- **출력/사이드 이펙트**: 현재 탐색 경로가 헤더에 브레드크럼으로 표시된다.

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
