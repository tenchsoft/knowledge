# Implement: glossary-term-expand-row

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 용어집 항목 클릭 시 정의를 펼치거나 접는다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::GlossaryExpand(idx) 핸들링 | grep 'StudyHit::GlossaryExpand' apps/study/ |
| state.rs | toggle_glossary_expand 메서드 | grep 'fn toggle_glossary_expand' apps/study/ |
| tutor.rs | 용어집 항목 렌더링 | grep 'expanded_glossary_idx' apps/study/src-tauri/src/ui/tutor.rs |

## 필요한 변경 (의도 단위)
### 1. 용어 항목 클릭 핸들링
- **입력**: PointerEvent::Down on glossary term rect
- **처리**: toggle_glossary_expand(idx) 호출
- **출력/사이드 이펙트**: expanded_glossary_idx 토글, +/- 표시 변경, 전체 정의 표시


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.glossary.term.{idx} | button | glossary term | tutor.width() >= 160 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
