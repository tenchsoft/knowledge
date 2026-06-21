# Design: home-end-practice-cursor-shortcut

## 한 줄 정의
Practice 모드에서 Home/End 키로 답안 커서를 입력 텍스트의 시작/끝으로 이동. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Answer input | `TextInput` | `study.practice.answer_input` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 다른 커서 이동 단축키 (별도 spec).
- 답안 입력 필드 전체 (별도 spec `practice-answer-input-field`).
