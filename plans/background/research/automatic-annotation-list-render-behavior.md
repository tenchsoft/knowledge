# Background: automatic-annotation-list-render-behavior

## 한 줄 정의
PDF 모드에서 `pdf_show_annotation_list`가 true일 때만 주석 행 목록이 자동으로 렌더링되며, 주석 추가/삭제 시 목록이 즉시 갱신된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 상태 변경 | `pdf_show_annotation_list` 또는 `pdf_annotations` 변경 | 매 프레임 |
| 주석 생성/삭제 | 사용자 액션 | 즉시 |

## Lifecycle & State
```
hidden ──[show=true]──→ visible ──[show=false]──→ hidden
                             │
                             └──[annotation change]──→ visible (목록 재렌더)
```

- **hidden**: PDF 모드가 아니거나 `pdf_show_annotation_list == false`.
- **visible**: 현재 페이지 주석 행이 나열됨. 빈 목록 시 "No annotations" 표시.

## Concurrency
- **인스턴스 정책**: 단일.
- **동시성 모델**: 메인 스레드 동기 렌더링.
- **재진입성**: 매 프레임 재평가. 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU/메모리: 무시 가능 (주석 수 ≤ 수백 개).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.pdf_annotations`, `pdf_show_annotation_list`, `selected_paper` (사용자 액션이 mutate).
- **Write**: 없음 (순수 렌더링).
- **Persistence**: 없음 (메모리만).
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 주석 데이터 불일치 | 상태 diff | 다음 프레임에 자동 수정 | 무알림 |

복구 정책: 매 프레임 상태 기반 렌더링.

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.annotation.{index}` | `Button` | `"<annotation text>"` | 주석 행 |

## UI 인터페이스
design(`plans/design/research/research-pdf-viewer.md`) §3 Annotation list가 사이드바에 렌더링.

## Out of scope
- 주석 생성 동작 (별도 spec `pdf-surface-annotation-placement-control`).
- 주석 편집 (별도 spec).
