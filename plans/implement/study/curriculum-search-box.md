# Implement: curriculum-search-box

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 아웃라인 상단의 검색 박스에 타이핑하여 컨셉을 검색하고 Enter로 첫 결과로 이동한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::SearchBox 핸들링 | grep 'StudyHit::SearchBox' apps/study/ |
| on_text_event (mod.rs) | search_focused 시 키보드 입력 처리 | grep 'search_focused' apps/study/ |
| state.rs | search_matches 메서드 | grep 'fn search_matches' apps/study/ |
| curriculum.rs | search_rect 렌더링 | grep 'fn search_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 검색 박스 포커스
- **입력**: PointerEvent::Down on search_rect
- **처리**: search_focused=true, focus_target=SearchBox
- **출력/사이드 이펙트**: 테두리 색상 ACCENT_STUDY로 변경

### 2. 검색 쿼리 입력
- **입력**: Character/Backspace/Escape/Enter 키
- **처리**: search_query 문자열 조작, search_matches()로 결과 표시
- **출력/사이드 이펙트**: 검색 결과 수 표시, Enter로 첫 결과 select_concept


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.curriculum.search | text_input | search | 항상 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
