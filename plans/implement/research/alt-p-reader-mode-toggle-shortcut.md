# Implement: alt-p-reader-mode-toggle-shortcut

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/research/alt-p-reader-mode-toggle-shortcut.md`)의 핵심 동작: Alt+P 단축키로 reader_mode Detail/PDF 토글 및 중앙 패널 모드 전환.
- design(`plans/design/research/alt-p-reader-mode-toggle-shortcut.md`)의 Component breakdown / States를 코드로 옮긴다.

## 영향 받는 영역
모듈/함수 단위. **라인 번호 금지**, **현재 코드 스니펫 인용 금지**.

| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/research/src-tauri/src/ui/events/text.rs::on_text_event_impl` | 키보드 단축키 핸들링 | `grep -n 'LogicalKey|NamedKey'` |

## 필요한 변경 (의도 단위)
### 1. 키보드 단축키 처리
- **입력**: 키보드 이벤트 (해당 조합 키)
- **처리**: focus 상태 확인 후 해당 상태 메서드 호출
- **출력/사이드 이펙트**: 상태 변경 + ctx.request_paint()
- **순서/우선순위**: 다른 키보드 핸들러보다 우선

## 새 자동화 노드
자동화 노드 없음 (키보드 단축키 또는 자동 렌더 동작).

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
8. `plans/test/research/alt-p-reader-mode-toggle-shortcut.md`가 있으면 거기 명시된 테스트 실행하여 통과 확인.