# Background: automatic-pdf-surface-render-behavior

## 한 줄 정의
선택된 논문이 PDF 모드이면 `build_pdf_surface_for_paper()`가 페이지 이미지, 주석 오버레이, 검색 결과 오버레이를 조합하여 PDF surface를 자동 렌더한다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| paint 사이클 | `reader_mode == Pdf` && selected paper 존재 | 매 paint |
| 페이지 변경 | `pdf_prev_page()` / `pdf_next_page()` | 사용자 액션 |
| 확대/축소 | `pdf_zoom_in()` / `pdf_zoom_out()` | 사용자 액션 |
| 주석 추가 | `add_pdf_annotation()` | 사용자 액션 |
| 검색 결과 변경 | `advance_pdf_search()` | 사용자 액션 |

## Lifecycle & State
```
idle ──[paint in Pdf mode]──→ rendering ──[surface built]──→ idle
```

- **idle**: PDF surface 렌더 대기.
- **rendering**: `build_pdf_surface_for_paper()` 호출 → `PdfSurface` 생성 → `paint_in_rect()`.

## Concurrency
- **인스턴스 정책**: 단일. paint 사이클 내 동기 처리.
- **동시성 모델**: 동기 직렬. 메인 스레드에서 수행.
- **재진입성**: paint 중 다시 paint 트리거 불가.
- **취소**: 해당 없음.

## Resource budget
| 자원 | 데스크톱 한계 | 모바일 한계 |
|------|----------------|--------------|
| 메모리 | 페이지 이미지 RGBA 버퍼 (예: 612×792×4 ≈ 1.9MB) | 동일 |
| CPU | paint 사이클 내 완료 | 동일 |
| 디스크 I/O | 없음 (메모리 버퍼 사용) | 없음 |

## Data flow
- **Read**: `ResearchState.pdf_current_page`, `pdf_zoom`, `pdf_rotation`, `pdf_annotations`, `pdf_search_results`, `pdf_search_active_index`, `pdf_page_image_data`, `Paper.pages`.
- **Write**: 없음 (읽기 전용 렌더).
- **Persistence**: 없음.
- **IPC**: app shell이 Tauri command로 `pdf_page_image_data`를 설정.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 페이지 이미지 없음 | `pdf_page_image_data == None` | 빈 페이지 렌더 (PdfSurfacePage without image) | 무알림 |
| 손상된 이미지 데이터 | 빈 pixels_rgba | `None`으로 폴백 | 무알림 |

복구 정책: 이미지 없이도 surface는 정상 렌더.

## Observability
- **Log**: N/A (paint 빈도로 로깅 부적절).
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| `research.pdf.surface` | `Canvas` | "PDF surface" | 렌더된 PDF 페이지 |

## UI 인터페이스
design(`plans/design/research/research-pdf-viewer.md`)가 PDF surface의 시각적 레이아웃 정의.

## Out of scope
- PDF 텍스트 추출 (별도 spec).
- PDF 파일 로딩 (app shell 책임).
