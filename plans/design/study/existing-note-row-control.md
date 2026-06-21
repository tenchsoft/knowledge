# Design: existing-note-row-control

## 한 줄 정의
기존 노트 행을 클릭하면 해당 노트가 선택되어 읽기/편집 모드로 전환된다. 기존 노트 패널 행 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Note row | `Button` | `study.notes.row.{idx}` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 노트 편집 (별도 spec).
- 노트 삭제 (별도 spec).
