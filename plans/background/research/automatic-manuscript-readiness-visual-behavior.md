# Background: automatic-manuscript-readiness-visual-behavior

## 한 줄 정의
Write 인스펙터 탭에서 원고 섹션의 인용 수와 완성도가 자동으로 계산되어 readiness dashboard에 시각적으로 표시된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 섹션/인용 변경 | manuscript sections 또는 cite 수 변경 | 매 프레임 |
| 논문 선택 변경 | `selected_paper` 변경 | 즉시 |

## Lifecycle & State
```
idle ──[section/cite change]──→ computing ──[done]──→ idle (대시보드 갱신)
```

- **computing**: 각 섹션의 인용 수를 집계하여 readiness 라인 생성.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 계산.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능 (섹션 수 ≤ 수십 개).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.manuscript_sections`, 섹션별 cite 수 (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 섹션 데이터 누락 | 빈 벡터 | 빈 대시보드 표시 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.manuscript.section.{index}` | `Button` | `"<section name> (N cite)"` | 섹션 인용 수 |

## UI 인터페이스
design(`plans/design/research/research-inspector.md`) §3 Write 탭 readiness dashboard.

## Out of scope
- 섹션 추가/삭제 동작 (별도 spec).
- 인용 삽입 (별도 spec).
