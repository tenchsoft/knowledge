# Implement: automatic-active-concept-highlight-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 현재 선택된 컨셉이 아웃라인에서 활성 배경(`NEUTRAL_500`)과 좌측 액센트 바(`ACCENT_STUDY`)로 자동 하이라이트된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (아웃라인 렌더) | 활성 컨셉 행에 배경색과 액센트 바 표시 | ``fn paint_outline` 내 `active` 분기` |

## 필요한 변경 (의도 단위)
### 1. 활성 컨셉 하이라이트 렌더
- **입력**: `active_unit_idx`, `active_concept_idx`와 순회 중 `(unit_idx, concept_idx)`의 일치 여부
- **처리**: 일치하는 컨셉 행에 `NEUTRAL_500` 배경 `fill_rect`와 좌측 2px `ACCENT_STUDY` 바를 그린다. 라벨 색상도 `ACCENT_STUDY`로 전환한다.
- **출력/사이드 이펙트**: 아웃라인에서 활성 컨셉이 시각적으로 구분된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_outline` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
