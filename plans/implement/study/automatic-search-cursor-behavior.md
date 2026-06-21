# Implement: automatic-search-cursor-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 검색 입력이 포커스된 상태에서 검색 상자에 깜빡이는 커서(1px 세로 막대)가 자동으로 표시된다. 빈 쿼리면 좌측 끝에, 텍스트가 있으면 텍스트 끝에 위치한다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (검색 커서) | `search_focused` 조건부로 커서 렌더 | ``fn paint_outline` 내 `search_focused` 분기` |

## 필요한 변경 (의도 단위)
### 1. 검색 커서 자동 렌더
- **입력**: `state.search_focused` 불리언, `state.search_query` 문자열
- **처리**: 포커스된 경우: 쿼리가 비어있으면 `search.x0 + 8.0` 위치에, 텍스트가 있으면 `search.x0 + 8.0 + text_width` 위치에 1px 너비의 `NEUTRAL_100` 막대를 `fill_rect`로 그린다.
- **출력/사이드 이펙트**: 검색 상자에 커서가 자동으로 표시된다.

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
