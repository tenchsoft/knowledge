# Implement: automatic-dropped-import-paths-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec(`plans/spec/research/automatic-dropped-import-paths-behavior.md`)의 핵심 동작: 드롭된 PDF/RIS/BibTeX 경로 중복 제거 및 가져오기 진행 상태 자동 전환.

## 영향 받는 영역
모듈/함수 단위. **라인 번호 금지**, **현재 코드 스니펫 인용 금지**.

| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/research/src-tauri/src/ui/paint/mod.rs::paint_impl` | 자동 렌더 페인트 오케스트레이션 | `grep -n 'paint_impl'` |
| `apps/research/src-tauri/src/ui/state.rs` | 상태 기반 자동 렌더 트리거 | `grep -n 'visible_paper_indices|research_regions'` |

## 필요한 변경 (의도 단위)
### 1. 자동 렌더 트리거
- **입력**: 상태 변경 (paint cycle)
- **처리**: 해당 상태 필드 읽기 및 조건부 렌더
- **출력/사이드 이펙트**: 패널/오버레이/위젯 repaint
- **순서/우선순위**: paint_impl 내에서 다른 페인트와 함께

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
8. `plans/test/research/automatic-dropped-import-paths-behavior.md`가 있으면 거기 명시된 테스트 실행하여 통과 확인.