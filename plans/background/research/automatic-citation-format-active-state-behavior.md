# Background: automatic-citation-format-active-state-behavior

## 한 줄 정의
인용 포맷 버튼 중 활성 포맷이 `theme.primary` 배경으로 자동 강조되고, 비활성 포맷은 `theme.surface` + border로 표시된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 포맷 변경 | `set_citation_export_format()` 호출 | 즉시 |
| Ctrl+Shift+C | `cycle_citation_format()` | 사용자 액션 |
| 인용 탭 진입 | `active_inspector_tab == 5` | 탭 전환 시 |

## Lifecycle & State
```
idle ──[format change]──→ updating ──[paint]──→ idle
```

- **idle**: 현재 `citation_export_format`과 버튼 시각이 일치.
- **updating**: `citation_export_format` 변경 후 다음 paint에서 버튼 상태 재평가.

## Concurrency
- **인스턴스 정책**: 단일. 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 해당 없음.
- **취소**: 해당 없음.

## Resource budget
- 메모리: 추가 할당 없음.
- CPU: 무시 가능 (5개 버튼 비교).
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `ResearchState.citation_export_format`.
- **Write**: `ResearchState.citation_export_format` (포맷 변경 시).
- **Persistence**: 메모리만.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 없음 | — | — | — |

복구 정책: 해당 없음.

## Observability
- **Log**: N/A.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.citation.format.bibtex` | `Button` | `"BibTeX"` | 활성 시 primary bg |
| `research.citation.format.ris` | `Button` | `"RIS"` | 활성 시 primary bg |
| `research.citation.format.apa` | `Button` | `"APA"` | 활성 시 primary bg |
| `research.citation.format.chicago` | `Button` | `"Chicago"` | 활성 시 primary bg |
| `research.citation.format.mla` | `Button` | `"MLA"` | 활성 시 primary bg |

## UI 인터페이스
design(`plans/design/research/citation-rendering.md`)가 포맷 버튼의 시각 정의.

## Out of scope
- 인용 텍스트 생성 (별도 spec).
- 클립보드 복사 (별도 spec).
