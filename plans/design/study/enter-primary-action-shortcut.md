# Design: enter-primary-action-shortcut

## 한 줄 정의
Enter 키로 현재 스테이지의 주요 액션을 실행: Practice 시작, 답안 제출, 피드백 진행, 모달 입력 제출. 신규 시각 요소 없음.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Start practice button | `Button` | `study.learn.start_practice` |
| Submit button | `Button` | `study.practice.submit` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 각 스테이지별 주요 액션 (별도 spec).
- 모달 입력 (별도 spec).
