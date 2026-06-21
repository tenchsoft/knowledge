# Design: spaced-repetition-scheduling

## 한 줄 정의
복습 세션에서 평가 버튼(Again, Hard, Good, Easy) 선택 시 SM-2 알고리즘에 따라 다음 복습 일정이 자동 계산된다. 신규 시각 요소 없음 — 기존 rating 버튼과 review card가 데이터 갱신 후 자동 렌더.

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Again button | `Button` | `study.review.rating.again` |
| Hard button | `Button` | `study.review.rating.hard` |
| Good button | `Button` | `study.review.rating.good` |
| Easy button | `Button` | `study.review.rating.easy` |

모두 기존 design(`study-review`) 사용. 간격 반복 데이터(`SpacedRepetitionEntry`)는 `StudyState::spaced_repetition_data`에 저장되고 UI에 직접 노출되지 않음 — 대기열 정렬과 다음 복습일 계산에만 사용.

## Out of scope
- FSRS 알고리즘 (별도 spec).
- 카드 생성/편집 (별도 spec).
