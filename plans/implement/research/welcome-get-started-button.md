# Implement: welcome-get-started-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/research/welcome-get-started-button.md`)의 핵심 동작: Get Started 버튼 클릭 시 환영 오버레이 닫기 및 작업 공간 활성화.
- design(`plans/design/research/welcome-get-started-button.md`)의 Component breakdown / States를 코드로 옮긴다.

## 영향 받는 영역
모듈/함수 단위. **라인 번호 금지**, **현재 코드 스니펫 인용 금지**.

| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/research/src-tauri/src/ui/events/pointer.rs::on_pointer_event_impl` | Welcome 화면 클릭 핸들링 (Get Started / Import / backdrop dismiss) | `grep -n 'show_welcome'` |
| `apps/research/src-tauri/src/ui/automation/welcome.rs::research_welcome_automation_nodes` | Welcome 오버레이 자동화 노드 emit | `grep -n 'research_welcome_automation_nodes'` |
| `apps/research/src-tauri/src/ui/paint/overlays.rs` | Welcome 오버레이 페인트 | `grep -n 'paint_overlays'` |

## 필요한 변경 (의도 단위)
### 1. Welcome 버튼 클릭 처리
- **입력**: PointerEvent::Down in welcome overlay
- **처리**: Get Started / Import / backdrop 영역 hit-test
- **출력/사이드 이펙트**: show_welcome=false + 조건부 queue_import + ctx.request_paint()
- **순서/우선순위**: 다른 모든 클릭 핸들러보다 먼저 (show_welcome 시 early return)

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| `research.welcome.get_started` | `Button` | `Get Started` | show_welcome=true |

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
8. `plans/test/research/welcome-get-started-button.md`가 있으면 거기 명시된 테스트 실행하여 통과 확인.