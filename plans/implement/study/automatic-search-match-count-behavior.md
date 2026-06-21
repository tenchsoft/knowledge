# Implement: automatic-search-match-count-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 검색 쿼리가 비어있지 않을 때, 아웃라인 상단에 일치하는 컨셉 수가 `ACCENT_STUDY` 색상으로 자동 표시된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (매치 카운트) | 검색 쿼리가 있을 때 `search_matches` 결과 수를 표시 | ``fn paint_outline` 내 `search_query.is_empty()` 체크 분기` |
| `apps/study/src-tauri/src/ui/state.rs` (검색 매치) | `search_matches`에서 유닛/컨셉 라벨과 쿼리 비교 | ``fn search_matches`` |

## 필요한 변경 (의도 단위)
### 1. 매치 카운트 자동 표시
- **입력**: `state.search_query` 문자열과 `state.search_matches()` 결과
- **처리**: 쿼리가 비어있지 않은 경우 `search_matches`를 호출하여 결과 수를 계산하고, 아웃라인 상단 우측에 `ACCENT_STUDY` 색상으로 숫자를 렌더한다.
- **출력/사이드 이펙트**: 검색 결과 수가 아웃라인에 자동으로 표시된다.

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
