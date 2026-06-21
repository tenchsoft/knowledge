# Implement: glossary-search-toggle-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 튜터 패널의 용어집 검색 필드 클릭으로 검색 모드를 토글한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::GlossarySearchToggle 핸들링 | grep 'StudyHit::GlossarySearchToggle' apps/study/ |
| state.rs | glossary_search_focused 토글 | grep 'glossary_search_focused' apps/study/ |
| curriculum.rs | glossary_search hit test | grep 'glossary_search' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 검색 토글 핸들링
- **입력**: PointerEvent::Down on glossary_search rect
- **처리**: glossary_search_focused 토글
- **출력/사이드 이펙트**: 검색 입력 필드 활성화/비활성화


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.glossary.search | text_input | glossary search | tutor.width() >= 160 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
