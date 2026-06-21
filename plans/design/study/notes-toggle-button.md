# Design: notes-toggle-button

## 한 줄 정의
노트 토글 버튼을 클릭하면 노트 패널 오버레이가 표시/숨김된다. 기존 커리큘럼 토글 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Notes toggle | `Button` | `study.curriculum.notes` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 노트 패널 오버레이 (별도 background `automatic-notes-panel-overlay-behavior`).
- 노트 패널 (별도 design `study-notes`).
