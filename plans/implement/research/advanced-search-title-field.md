# Implement: advanced-search-title-field

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/research/advanced-search-title-field.md`)의 핵심 동작: 고급 검색 패널 Title 필드 타이핑 시 title_query 업데이트 및 논문 목록 제목 필터링.
- design(`plans/design/research/advanced-search-title-field.md`)의 Component breakdown / States를 코드로 옮긴다.

## 영향 받는 영역
모듈/함수 단위. **라인 번호 금지**, **현재 코드 스니펫 인용 금지**.

| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/research/src-tauri/src/ui/events/pointer.rs::on_pointer_event_impl` | 고급 검색 토글/필드 클릭 핸들링 | `grep -n 'toggle_advanced_search'` |
| `apps/research/src-tauri/src/ui/automation/mod.rs::research_automation_nodes` | 고급 검색 필드 자동화 노드 emit | `grep -n 'research.advanced'` |
| `apps/research/src-tauri/src/ui/state.rs::update_advanced_search` | 고급 검색 상태 업데이트 | `grep -n 'fn update_advanced_search'` |

## 필요한 변경 (의도 단위)
### 1. 클릭 이벤트 처리
- **입력**: PointerEvent::Down in target area
- **처리**: hit-test로 대상 식별 → 상태 메서드 호출
- **출력/사이드 이펙트**: 상태 변경 + ctx.request_paint()
- **순서/우선순위**: 영역별 우선순위에 따라

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `research.advanced.title` | `TextInput` | `Title` | show_advanced_search=true |

## 의존
- 선행 implement: `advanced-search-toggle-button`

## 작업 절차
1. spec / design / background 중 존재하는 것을 모두 먼저 읽음.
2. "영향 받는 영역" 표의 각 항목에 대해 **현재 코드를 먼저 읽고** grep으로 위치 확정.
3. 위치가 표와 다르면 (리네임/이동 발생) 표를 갱신 (PR에 포함).
4. "필요한 변경"의 의도대로 코드 변경.
5. 새 자동화 노드는 design + background 표를 합집합으로 추가.
6. `cargo check --workspace --locked` 통과 확인.
7. 새 노드가 `harness.automation_report()`에 노출되는지 확인.
8. `plans/test/research/advanced-search-title-field.md`가 있으면 거기 명시된 테스트 실행하여 통과 확인.