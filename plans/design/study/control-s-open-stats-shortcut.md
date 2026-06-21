# Design: control-s-open-stats-shortcut

## 한 줄 정의
Ctrl+S 단축키로 통계 모달을 열되 답안 텍스트는 편집하지 않는다. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Stats button | `Button` | `study.header.stats` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- Stats modal 내용 (별도 design `study-modals`).
- 답안 저장 (별도 spec).
