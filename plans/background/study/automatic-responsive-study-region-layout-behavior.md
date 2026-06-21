# Background: automatic-responsive-study-region-layout-behavior

## 한 줄 정의
뷰포트 크기 변경 시 즉시 4개 영역(header, outline, surface, tutor)의 위치와 크기를 재계산하여 모바일/태블릿/데스크톱 레이아웃에 맞게 자동 조정한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 뷰포트 리사이즈 | `layout()` 호출 | 매 리사이즈 |
| 초기 렌더 | `StudyApp::new()` | 1회 |

## Lifecycle & State

```
desktop ──[width < 1100]──→ tablet ──[width < 700]──→ mobile
   ↑                                                        │
   └──────────────────[width ≥ 1100]────────────────────────┘
```

- **Desktop (≥1100px)**: outline=180px, tutor=260px, surface=나머지. 모든 패널 표시.
- **Tablet (700–1100px)**: outline=180px, tutor=0px (숨김), surface=나머지.
- **Mobile (<700px)**: outline=`min(width*0.30, 180)` 최소 88px, tutor=0px, hamburger menu 활성화.

`StudyViewportClass`와 `TouchReviewState`도 함께 갱신:
- Mobile: `touch_review.enabled = true`, `min_hit_size_px = 48`.
- Desktop/Tablet: `touch_review.enabled = false`, `min_hit_size_px = 44`.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU: O(1) 사칙연산. < 0.01ms.
- 메모리: 추가 없음.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `Size` (뷰포트 크기).
- **Write**: `StudyState::viewport_class`, `StudyState::touch_review`.
- **Persistence**: 없음.
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 없음 | — | — | — |

순수 계산 로직, 실패 모드 없음.

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `study.root` | `Window` | stage label | 뷰포트 클래스는 자동화 노드로 간접 관찰 |

`regions()` 함수의 테스트(`responsive_regions_do_not_overlap_across_core_viewports`)가 390×844, 768×1024, 1440×900에서 영역 비중첩을 검증.

## UI 인터페이스
design(`plans/design/study/study-header.md`, `study-curriculum.md`)에 반응형 변형 정의.

## Out of scope
- 레이아웃 애니메이션 (후속 spec).
- 분할 화면/멀티윈도우 (후속 spec).
