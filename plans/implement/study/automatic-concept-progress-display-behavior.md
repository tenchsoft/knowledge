# Implement: automatic-concept-progress-display-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 아웃라인의 각 유닛 헤더에 완료된 컨셉 수/전체 컨셉 수(예: `2/5`)가 자동 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (유닛 헤더) | 유닛 라벨 옆에 진행률 텍스트 렌더 | ``fn paint_outline` 내 `completed`/`total` 계산 분기` |

## 필요한 변경 (의도 단위)
### 1. 유닛 진행률 텍스트 렌더
- **입력**: 유닛의 `concepts` 벡터에서 `ConceptStatus::Completed`인 항목 수와 전체 수
- **처리**: `completed/total` 형식의 문자열을 유닛 헤더 우측에 `NEUTRAL_400` 색상으로 그린다.
- **출력/사이드 이펙트**: 각 유닛의 진행 상황이 아웃라인에 표시된다.

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
