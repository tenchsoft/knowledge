# Design: study-review

## 한 줄 정의
복습 모드 중앙 영역에 복습 카드(문제, 오답, 정답, 해설, 원인 태그)와 간격 반복 평가 버튼(Again, Hard, Good, Easy)을 표시한다.

## 시각적 레이아웃
```
┌─ Surface (center) ──────────────────────────────────────────────┐
│                                                                   │
│  Review 1 / 3                                                     │
│                                                                   │
│  ┌─ Review card ──────────────────────────────────────────────┐   │
│  │  Problem text                                              │   │
│  │  Wrong: user_answer                                        │   │
│  │  Correct: correct_answer                                   │   │
│  │  Solution explanation                                      │   │
│  │  cause: tag / related: concept                             │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                   │
│  [Again]  [Hard]  [Good]  [Easy]                                  │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

## Component breakdown

| Component | role | debug_id |
|-----------|------|----------|
| Review queue button (curriculum) | `Button` | `study.review.queue` |
| Again rating | `Button` | `study.review.rating.again` |
| Hard rating | `Button` | `study.review.rating.hard` |
| Good rating | `Button` | `study.review.rating.good` |
| Easy rating | `Button` | `study.review.rating.easy` |

## Visual properties

| 속성 | 값 |
|------|----|
| Surface background | `NEUTRAL_800` |
| Title text | `NEUTRAL_100`, 18px, `FontWeight::BOLD` |
| Review card bg | `NEUTRAL_600`, border_radius=8 |
| Review card border | `NEUTRAL_500`, 1px |
| Problem text | `NEUTRAL_100`, 15px, `FontWeight::BOLD` |
| Wrong answer label | `STATUS_ERROR`, 13px |
| Correct answer label | `STATUS_READY`, 13px |
| Solution text | `NEUTRAL_100`, 12px |
| Cause/related text | `NEUTRAL_300`, 12px |
| Rating button width | 64px, gap 8px |
| Rating button bg | `NEUTRAL_600` fill, border_radius=4 |
| Rating button border | `NEUTRAL_500`, 1px |
| Rating button text | `NEUTRAL_100`, 10px |

## States

| Component | Default | Hover | Active/Pressed | Focus | Disabled |
|-----------|---------|-------|----------------|-------|----------|
| Again button | `NEUTRAL_600` bg | `STATUS_ERROR` lighten | pressed | 2px outline | opacity 0.5 |
| Hard button | `NEUTRAL_600` bg | `STATUS_WARNING` lighten | pressed | 2px outline | opacity 0.5 |
| Good button | `NEUTRAL_600` bg | `STATUS_READY` lighten | pressed | 2px outline | opacity 0.5 |
| Easy button | `NEUTRAL_600` bg | `ACCENT_STUDY` lighten | pressed | 2px outline | opacity 0.5 |

## Responsive 변형
- **Desktop**: 4개 버튼 가로 배치.
- **Mobile**: 버튼 크기 44px 최소 터치 타겟, 스와이프 제스처 대체.

## Accessibility
- 각 버튼에 i18n 라벨.
- 모바일에서 좌/우 스와이프로 Again/Good 평가.
- Swipe threshold: 50px 거리, 0.3 px/ms 속도.

## Design tokens — 사용 / 제안
- **사용**: `NEUTRAL_800`, `NEUTRAL_600`, `NEUTRAL_500`, `NEUTRAL_300`, `NEUTRAL_100`, `ACCENT_STUDY`, `STATUS_ERROR`, `STATUS_READY`, `STATUS_WARNING`.
- **신규 제안**: 없음.

## Out of scope
- 간격 반복 알고리즘 자체 (별도 spec `spaced-repetition-scheduling`).
- 복습 카드 편집.
