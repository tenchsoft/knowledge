# Implement: glossary-search-field

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 용어집 검색 필드에 타이핑하여 용어를 필터링한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_text_event (mod.rs) | glossary_search_focused 시 키보드 입력 처리 | grep 'glossary_search_focused' apps/study/ |
| state.rs | glossary_search_query 문자열 | grep 'glossary_search_query' apps/study/ |
| tutor.rs | 검색 결과 필터링 | grep 'glossary_search_query' apps/study/src-tauri/src/ui/tutor.rs |

## 필요한 변경 (의도 단위)
### 1. 검색 쿼리 입력
- **입력**: Character/Backspace/Escape 키 (glossary_search_focused == true)
- **처리**: glossary_search_query 문자열 조작
- **출력/사이드 이펙트**: 검색어에 맞는 용어만 표시 필터링


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.glossary.search | text_input | glossary search | tutor.width() >= 160 && glossary_search_focused |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
