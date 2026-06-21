# Design: next-problem-button

## 한 줄 정의
Practice 피드백에서 Next 버튼을 클릭하면 `problem_index`가 증가하고, 마지막 문제 이후에는 세션 결과 모달이 열린다. 기존 Practice 버튼 재사용.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Next button | `Button` | `study.practice.next` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.

## Out of scope
- 세션 결과 모달 (별도 design `study-modals`).
- 문제 생성 (별도 spec).
