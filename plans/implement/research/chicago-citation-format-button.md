# Implement: chicago-citation-format-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/research/chicago-citation-format-button.md`)의 핵심 동작: Cite 인스펙터 탭 Chicago 포맷 버튼 클릭 시 인용 스타일 Chicago 전환.
- design(`plans/design/research/chicago-citation-format-button.md`)의 Component breakdown / States를 코드로 옮긴다.

## 영향 받는 영역
모듈/함수 단위. **라인 번호 금지**, **현재 코드 스니펫 인용 금지**.

| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/research/src-tauri/src/ui/events/pointer.rs::on_pointer_event_impl` | 인용 포맷 버튼 클릭 핸들링 | `grep -n 'set_citation_export_format'` |
| `apps/research/src-tauri/src/ui/automation/inspector.rs::push_inspector_nodes` | 인용 포맷 버튼 자동화 노드 emit | `grep -n 'research.citation.format'` |
| `apps/research/src-tauri/src/ui/state.rs::set_citation_export_format` | 인용 포맷 상태 변경 | `grep -n 'fn set_citation_export_format'` |

## 필요한 변경 (의도 단위)
### 1. 인용 포맷 버튼 클릭 처리
- **입력**: PointerEvent::Down on citation format button
- **처리**: 버튼 rect hit-test → set_citation_export_format 호출
- **출력/사이드 이펙트**: citation_export_format 변경 + ctx.request_paint()
- **순서/우선순위**: 인용 포맷 버튼 순서대로 처리

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `research.citation.format.chicago` | `Button` | `Chicago` | Cite 탭 활성 시 |

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
8. `plans/test/research/chicago-citation-format-button.md`가 있으면 거기 명시된 테스트 실행하여 통과 확인.