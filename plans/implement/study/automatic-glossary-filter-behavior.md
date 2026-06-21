# Implement: automatic-glossary-filter-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 튜터 패널의 용어집 검색 입력에 텍스트가 있을 때, 용어(term) 또는 정의(definition)에 해당 문자열이 포함된 항목만 자동으로 필터링되어 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/tutor.rs` (용어집 필터) | 용어집 순회 중 `glossary_search_query`와 매칭되지 않은 항목 skip | ``fn paint_tutor_panel` 내 glossary 루프의 `glossary_search_query` 분기` |

## 필요한 변경 (의도 단위)
### 1. 용어집 검색 필터링
- **입력**: `state.glossary_search_query` 문자열과 각 용어집 항목의 `term`/`definition`
- **처리**: 쿼리가 비어있지 않은 경우, 각 항목의 `term`과 `definition`을 소문자로 변환하여 쿼리가 포함되어 있는지 검사한다. 포함되지 않으면 `continue`로 건너뛴다.
- **출력/사이드 이펙트**: 검색 쿼리와 일치하는 용어집 항목만 튜터 패널에 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_tutor_panel` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
