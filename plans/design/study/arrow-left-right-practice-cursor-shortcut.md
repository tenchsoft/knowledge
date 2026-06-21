# Design: arrow-left-right-practice-cursor-shortcut

## 한 줄 정의
Practice 모드에서 ArrowLeft/ArrowRight 키로 답안 입력 커서를 좌/우로 이동. 신규 시각 요소 없음 — 기존 TextInput 커서 동작.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Answer input | `TextInput` | `study.practice.answer_input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- Home/End 키 (별도 spec `home-end-practice-cursor-shortcut`).
- 답안 입력 필드 전체 (별도 spec `practice-answer-input-field`).
