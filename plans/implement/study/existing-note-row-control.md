# Implement: existing-note-row-control

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 노트 패널에서 저장된 노트 행을 표시하고 자동화 노드로 노출한다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| learn.rs | 노트 행 렌더링 루프 | grep 'notes\.iter' apps/study/ |
| mod.rs | automation_children 내 노트 행 노드 | grep 'study.notes.row' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 노트 행 렌더링
- **입력**: notes 벡터에서 active_concept().id 필터링
- **처리**: 각 노트를 note_row_rect에 렌더링
- **출력/사이드 이펙트**: 텍스트와 타임스탬프 표시, 자동화 노드 노출


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.notes.row.{idx} | button | note text | show_notes_panel && stage == Learn |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
