# Background: automatic-research-visual-surface-behavior

## 한 줄 정의
Visual 인스펙터 탭에서 선택된 논문의 시각 데이터(그래프, 표, 이미지)가 자동으로 `VisualSurface`에 렌더링된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 논문 선택 변경 | `selected_paper` 변경 | 즉시 |
| 시각 데이터 변경 | 첨부 이미지/그래프 업데이트 | 즉시 |
| 탭 전환 | Visual 탭 활성화 | 즉시 |

## Lifecycle & State
```
empty ──[paper selected with visuals]──→ rendering ──[done]──→ idle
```

- **empty**: 논문 미선택 또는 시각 데이터 없음.
- **rendering**: 시각 데이터를 Canvas에 렌더링.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 이미지 크기에 비례. 수 MB까지 허용.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.selected_paper`, 시각 첨부 데이터 (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 이미지 로드 실패 | 디코딩 에러 | "Image not available" 플레이스홀더 | 무알림 |
| 시각 데이터 없음 | 빈 벡터 | 빈 상태 표시 | 무알림 |

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.visual.surface` | `Canvas` | — (시각 콘텐츠) | 시각 표면 |

## UI 인터페이스
design(`plans/design/research/research-inspector.md`) §3 Visual 탭.

## Out of scope
- 이미지 편집 (별도 spec).
- 그래프 인터랙션 (별도 spec).
