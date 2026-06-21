# Design: research-automatic-ui

## 한 줄 정의
사용자 직접 조작 없이 상태 변화에 따라 자동으로 렌더되는 모든 UI 요소의 시각적 계약.

## 자동 UI 요소 목록

### 1. Advanced search panel
검색 입력 우측 토글 클릭 시 헤더 아래에 확장 패널이 나타남.
- 배경: `theme.surface`, border `theme.border`.
- 필드: Title, Author, Venue, Tag (text input), Year range.
- "Save Search" 버튼: bg `theme.primary`.
- debug_ids: `research.advanced.title`, `research.advanced.author`, `research.advanced.venue`, `research.advanced.tag`.

### 2. Annotation list
PDF 모드에서 토글 시 센터 하단에 주석 목록이 나타남.
- 빈 상태: "No annotations yet." — `theme.disabled`.
- 행: kind label(HL/UL/ST/Note) + page + text preview(40자 제한).
- 선택된 행: bg `theme.primary`.

### 3. Backup/restore progress
`ResearchBackupRestoreUi` 상태에 따라 진행 표시. 현재 UI 표면 없음 — 토스트로 통보.

### 4. Citation format active state
인용 탭에서 활성 포맷 버튼이 `theme.primary` 배경으로 강조. 비활성은 `theme.surface` + `theme.border`.

### 5. Cite result filter
Write 탭에서 `manuscript_cite_search` 입력에 따라 결과가 자동 필터링. 각 결과에 "Insert" 버튼.

### 6. Dropped import
`dropped_import_paths`에 파일 경로가 있으면 자동으로 import 큐에 추가. 토스트로 통보.

### 7. Header status text
넓은 화면(≥1040px)에서 import status + reader mode + favorites 상태가 `theme.secondary` 텍스트로 표시.

### 8. Inspector tab content
`active_inspector_tab` 인덱스에 따라 해당 탭 내용만 렌더. 다른 탭은 페인트 생략.

### 9. Manuscript readiness
Write 탭 상단에 `manuscript_summary_lines` 대시보드 표시. 섹션당 인용 수, 활성 섹션 강조.

### 10. Paper detail refresh
`selected_paper` 변경 시 센터 패널 전체 재페인트. 빈 상태 ↔ 상세 전환.

### 11. Paper list filtering
`search_query`, `selected_collection`, `favorites_only`, `sort_mode`에 따라 `visible_papers()` 결과가 자동 갱신.

### 12. Paper search highlight
검색어와 일치하는 논문 제목에 yellow alpha 40 배경 하이라이트.

### 13. Paper selection highlight
선택된 논문 행: bg `theme.primary`. 다중 선택: bg primary alpha 40 + ☑ 체크마크.

### 14. PDF search counter
PDF 검색 결과가 있으면 `{active+1}/{total}` 형식으로 카운터 표시. `theme.secondary`.

### 15. PDF surface render
`build_pdf_surface_for_paper()`가 페이지 이미지 + 주석 오버레이 + 검색 결과 오버레이를 `PdfSurface`로 렌더.

### 16. Q&A message bubble
role에 따라 색상 구분: user=`theme.primary`, assistant=`#34D399`, other=`theme.secondary`.
메시지 텍스트는 40자 줄바꿈.

### 17. Visual surface
`visual_draw_plan`이 있으면 `VisualSurface`로 그래프/타임라인/매트릭스 렌더.

### 18. Responsive layout
`research_regions(size)`가 width에 따라 자동으로 4개 영역(header/left/center/right) 분할.
- <560px: 좁은 left.
- 560–980px: left + center, right 숨김.
- ≥980px: left + center + right.

### 19. Sort indicator
논문 목록 헤더 우측에 현재 정렬 모드 라벨이 버튼으로 표시. 클릭 시 순환.

### 20. Status badge color
`paper_list::status_color()`가 ReadingStatus별 색상 반환: Reviewed=green, Reading=amber, other=blue.

### 21. Toast stack
하단 중앙에 최대 5개 토스트가 수직으로 스택. 종류별 색상:
- Success: green alpha 224
- Error: red alpha 224
- Warning: amber alpha 224
- Info: blue alpha 224
클릭으로 개별 해제.

### 22. Welcome overlay
`show_welcome=true` 시 전체 화면 반투명 오버레이 + 중앙 카드(420px).
- 카드 bg: `theme.background`, border `theme.border`, radius 12px.
- "Get Started" 버튼: bg `theme.primary`.
- "Import" 버튼: border `theme.primary`.
- debug_ids: `research.welcome`, `research.welcome.get_started`, `research.welcome.import`.

## Out of scope
- 각 요소의 상세한 시각 속성은 개별 design 문서 참조.
- 백그라운드 동작은 `plans/background/research/` 참조.
