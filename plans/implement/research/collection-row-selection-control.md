# Implement: collection-row-selection-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/research/collection-row-selection-control.md`)의 핵심 동작: 컬렉션 행 본체 클릭 시 해당 컬렉션 선택 및 논문 목록 필터링.
- design(`plans/design/research/collection-row-selection-control.md`)의 Component breakdown / States를 코드로 옮긴다.

## 영향 받는 영역
모듈/함수 단위. **라인 번호 금지**, **현재 코드 스니펫 인용 금지**.

| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/research/src-tauri/src/ui/events/pointer.rs::on_pointer_event_impl` | 컬렉션 행 클릭/확장 핸들링 | `grep -n 'select_collection|toggle_collection_expanded'` |
| `apps/research/src-tauri/src/ui/automation/mod.rs::research_automation_nodes` | 컬렉션 행 자동화 노드 emit | `grep -n 'research.collection'` |
| `apps/research/src-tauri/src/ui/collection_tree.rs` | 컬렉션 hit-test 및 Y 좌표 계산 | `grep -n 'hit_test_collection_row'` |

## 필요한 변경 (의도 단위)
### 1. 클릭 이벤트 처리
- **입력**: PointerEvent::Down in target area
- **처리**: hit-test로 대상 식별 → 상태 메서드 호출
- **출력/사이드 이펙트**: 상태 변경 + ctx.request_paint()
- **순서/우선순위**: 영역별 우선순위에 따라

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `research.collection.{index}` | `Button` | `collection name` | 컬렉션 섹션 |

## 의존
- 선행 implement: 없음 (독립 기능)

## 작업 절차
1. spec / design / background 중 존재하는 것을 모두 먼저 읽음.
2. "영향 받는 영역" 표의 각 항목에 대해 **현재 코드를 먼저 읽고** grep으로 위치 확정.
3. 위치가 표와 다르면 (리네임/이동 발생) 표를 갱신 (PR에 포함).
4. "필요한 변경"의 의도대로 코드 변경.
5. 새 자동화 노드는 design + background 표를 합집합으로 추가.
6. `cargo check --workspace --locked` 통과 확인.
7. 새 노드가 `harness.automation_report()`에 노출되는지 확인.
8. `plans/test/research/collection-row-selection-control.md`가 있으면 거기 명시된 테스트 실행하여 통과 확인.