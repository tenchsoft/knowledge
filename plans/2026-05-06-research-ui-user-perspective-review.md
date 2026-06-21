# Tench Research — UI 사용자 관점 검토

- **작성일**: 2026-05-06
- **검토 대상**: `apps/research/src-tauri/src/ui/` (약 5.2k 줄, `mod.rs`가 3.4k 줄로 비대)
- **검토 관점**: 학술 논문을 관리하려는 사용자가 첫 5~10분간 시도하는 동작 기준
- **벤치마크**: Mendeley (스펙상)

## 첫인상

3-패널 레이아웃(Collections + Tags · Paper list / Detail + PDF reader / Inspector). Welcome 모달, 다국어(i18n catalog), 키보드 단축키, 토스트, PDF 주석(highlight/underline/strikeout/sticky note), Q&A 채팅, Manuscript 아웃라인 등 *기능 범위는 굉장히 넓다*. 그러나 핵심 워크플로(논문을 가져와서, 읽고, 인용/내보내기) 거의 전부가 *placeholder/TODO*다.

## 강점

- **다국어**: `t("research...")` 키 기반 i18n catalog. status·reader_mode 라벨도 `localized_research_status`/`localized_research_reader_mode`로 번역
- **세련된 키보드 모델**: `?` 단축키 도움말, `Esc` 단계적 후퇴, `Alt+P` 리더 모드 토글, `PageUp/Down` 페이지 이동, `+/-` PDF 줌, 화살표로 논문 선택
- **PDF 주석 모델**: Highlight/Underline/Strikeout/StickyNote/TextSelect 도구 + 좌표를 PDF 페이지(612×792)로 정규화 + 페이지별 필터링 + 검색 결과 오버레이
- **Inspector 탭 6종**(Notes/Summary/Q&A/Visual/Manuscript/Cite)이 모두 그려짐 — 빈 상태도 처리
- **고급 검색 패널**(Title/Author/Venue/Tag/Year range) 별도 토글
- **Welcome 카드** 신규 사용자 안내 — Get Started / Import Library 두 갈래
- **다중 선택**(checkbox + 우클릭 토글), 정렬 모드 토글 버튼
- **Q&A Quick Actions**(Summarize / Key Points / Limitations) 미리 등록된 프롬프트

---

## A. 결정적 결함 — *대부분 핵심 기능이 placeholder* (P0)

### A-1. Import / Export / Sync 모두 가짜 버튼 (P0)

상단 헤더의 3개 버튼은 시각적으로 가장 눈에 띄는 액션인데:

| 버튼 | 실제 동작 (`state.rs`) |
|---|---|
| **Import** | `queue_import`: `ReaderMode::Importing` 으로 전환하고 progress event 1건 push. 파일 다이얼로그 안 열림, 실제 파싱 없음 (`state.rs:564~`) |
| **Export** | `export_action`: progress 기록 + 토스트 `"Export queued"`. 실제 출력 없음 (`state.rs:987~`) |
| **Sync** | `sync_action`: 토스트 `"Sync started"`만 (`state.rs:997~`) |

Welcome 카드의 "Import Library" 버튼도 동일한 `queue_import` 호출 → 사용자가 처음부터 막힘.

### A-2. DOI/BibTeX/RIS 메타데이터 fetch가 모두 TODO (P0)

`state.rs:1090~1102`:
```
pub fn fetch_doi_metadata(&mut self) {
    ...
    // TODO: connect to Tauri command for DOI/arXiv metadata fetch
    self.add_toast(...)
}
pub fn import_bibtex(&mut self) {
    // TODO: connect to Tauri command for BibTeX import
    ...
}
```

Mendeley 호환을 표방하면서 *논문을 라이브러리로 들여올 진입점이 단 하나도 동작하지 않음*. Inspector의 Cite 탭에서 DOI 입력해도 토스트만 뜸.

### A-3. PDF 페이지 이미지가 외부 의존 (P1)

`helpers.rs:21~33` `build_pdf_surface_for_paper`는 `pdf_state.pdf_page_image_data`(앱 셸이 채워주는 RGBA 픽셀)을 그대로 `PdfSurface`로 넘긴다. 실제로 데이터를 채워주는 백엔드 명령이 정상 작동해야 PDF 보임. 코드만 봐서는 *언제, 누가* 그 이미지를 채워주는지 명확하지 않음. 채워지지 않으면 PDF 모드가 빈 회색 박스.

### A-4. 상태 필터 클릭이 검색어로 들어감 (P1)

`mod.rs:1929~1933`. 상태 필터 행 클릭 시 라벨 그대로 `set_search_query(label)`. 라벨이 `"Read"`/`"Reading"`인데 검색어로 들어가면 **substring 매칭** → "Read"를 누르면 "Reading" 논문도 매치됨 (`paper.title.to_lowercase().contains(&query_lower)`로 보이는데 실제로는 어디서 필터링하는지 확인 필요). 어쨌든 *상태 필터가 검색에 흘러들어가는 설계 자체*가 의도와 어긋남 — 진짜 상태 필터 모드가 따로 있어야 함.

### A-5. 클릭 시 Y 위치를 paint 코드 상수에서 재계산 (P1)

`mod.rs:2042~2050,2114~2123` 등에서 PDF 영역 hit-test가
```
let search_y = header_h + spacing_large + 28.0 + 24.0 + 24.0 + 24.0 + 20.0
             + 18.0 * abstract_lines.len() as f64 + spacing_large;
let fig_y = search_y + 28.0;
```
처럼 paint 코드와 *별개*로 상수 더하기. paint 코드의 어떤 라인 높이가 한 픽셀이라도 바뀌면 hit-test가 조용히 어긋남. 특히 `abstract_lines.len()`로 가변 — 초록이 길면 PDF 클릭 위치가 미묘하게 어긋남.

---

## B. 주요 UX 이슈

### B-1. PDF 주석 도구 라벨 `H / U / S / N` (P1)

`mod.rs:932`. Highlight/Underline/Strikeout/StickyNote를 1글자로. 툴팁이 보이는지 확인했는데 hover 툴팁 코드가 없어 **이 의미를 알 길이 없음**. 더 나아가 `Rot`(rotate), `Ann`(annotation list) 같은 3글자 약어도 같은 문제. 아이콘 도입 또는 hover 툴팁 추가 필요.

### B-2. Inspector 탭이 36px 너비에 3~4글자 라벨 (P1)

`mod.rs:1126~1136`. 6 탭이 `tw = 36.0`로 좁고 라벨은 i18n catalog의 `..._short` 키. 한국어/일본어 등에서는 잘릴 가능성. Notes/Summary/Q&A/Visual/Write/Cite 정도면 적어도 50px는 필요.

### B-3. 검색 결과 하이라이트가 텍스트 너비를 `len() * 6.5`로 추정 (P2)

`mod.rs:572~578`. 영문에는 그럭저럭, 한글/CJK 사용 시 하이라이트가 너무 좁거나 너무 김. 최근 한국어 i18n도 추가됐는데 라이브러리에 한국어 논문이 들어오면 즉시 시각 깨짐.

### B-4. 토스트 메시지 너비도 `len() * 7.0` 추정 (P2)

`mod.rs:1882`. 같은 문제. 한글 토스트는 잘림.

### B-5. 정렬 버튼이 텍스트 라벨에 `len() * 6.0 + 12` 너비 (P2)

`mod.rs:520~`. CJK 라벨 시 너비 부족.

### B-6. 사이드바 태그가 `tw = 100.0` 고정 (P2)

`mod.rs:662`. 긴 태그 이름은 잘림. 자동 너비 또는 wrapping 필요.

### B-7. 클릭 한 번으로 포커스 손실 (P2)

`mod.rs:2020`. 헤더 검색박스/특정 영역 외 클릭 시 `set_focus(FocusTarget::None)` 무조건 실행. 사용자가 검색 박스 옆 빈 공간을 살짝 클릭하면 입력 중인 검색어 입력이 끊김. 빈 영역은 포커스 유지가 자연스러움.

### B-8. Escape가 검색어를 비워버림 (P2)

`mod.rs:2526`. 검색 후 Detail 모드에서 Esc 누르면 `set_search_query("")` → 검색어가 사라지고 ReaderMode를 Detail로 강제. 검색을 마치고 결과를 둘러보다가 Esc로 *팝업 닫기* 의도로 누르면 검색 컨텍스트 자체가 날아감. 좀 과격함.

---

## C. 작은 일관성·세부 이슈

- **즐겨찾기 표시는 라벨 앞 `"* "`** (`mod.rs:568`). 별 아이콘(★) 또는 동그란 채움 표시가 표준
- **확장 토글**은 `▾`/`▸` (`mod.rs:361`) — 정상
- **Smart 컬렉션** 라벨이 `"⚙ name (count)"` — 톱니가 *설정* 아이콘 → "smart"라기보다 "설정"으로 오해
- **Saved searches**가 `"🔍 name [query]"` — 이모지 사용. 다른 사이드바 항목과 시각 일관성 어긋남
- **Welcome 모달의 본문**이 `cx - 160.0` 절대 좌표(`mod.rs:1786,1795`) — 번역이 길면 카드 밖으로 나감
- **"Save Search" 버튼**(`mod.rs:310`)이 input 옆에 있지만 hit-test 코드를 확인하지 않은 한 클릭 동작 불명. 적어도 paint만으론 클릭 핸들러 못 찾음
- **PDF 검색 입력 필드** 활성화 후 cursor 시각화 없음. focus 됐는지 여부도 outline 색만으로 표시
- **PDF 주석을 추가**하면 `rect: rel_x * 612.0 ± 40.0, rel_y * 792.0 ± 8.0` 즉 *항상 80×16* 박스로 고정 (`mod.rs:2065~2069`) — 사용자가 드래그로 영역 지정하지 못함, 단지 클릭 위치에 고정 박스 하나
- **Q&A 채팅의 Send 버튼은 36px 너비** — 클릭 영역이 좁다
- **헤더 영역 status 텍스트** `import_status | reader_mode | favorites: on/off` 가 `size.width >= 1040.0` 조건이라 작은 창에서는 사라짐 — 사용자에게 가장 중요한 정보가 응답형 분기에 의해 숨겨짐

---

## D. 코드 품질 / 유지보수 (참고)

- `mod.rs` 3,369줄로 비대. Welcome / Inspector / PDF / Hit-test / Keyboard 분리 필요 (`inspector.rs`는 12줄짜리 stub만 있음)
- `paint` 함수에서 위치 누적 `cy += 24.0`이 수십 곳 — 한 곳을 바꾸면 hit-test도 따라 바꿔야 함
- `helpers::build_pdf_surface_for_paper` 같은 변환은 깔끔. 이런 패턴을 더 늘리면 좋음

---

## 우선순위 권장

| Priority | 항목 | 사용자 임팩트 |
|---|---|---|
| **P0** | Import/Export/Sync 실제 구현 또는 disabled 표시 (A-1) | 핵심 워크플로 진입 자체 |
| **P0** | DOI/BibTeX/RIS fetch 백엔드 연결 (A-2) | 라이브러리 채울 방법 |
| **P0** | 상태 필터를 검색어와 분리 (A-4) | "Read" 누르면 "Reading"도 잡힘 |
| **P1** | PDF 주석 도구 H/U/S/N → 아이콘 또는 호버 툴팁 (B-1) | 인지 비용 |
| **P1** | Hit-test 좌표를 paint와 단일 소스로 (A-5) | 레이아웃 변경 시 회귀 |
| **P1** | PDF 페이지 이미지 채우기 보장(없을 때 빈 페이지/안내 표시) (A-3) | PDF 모드 진입 시 빈 화면 |
| **P1** | Inspector 탭 너비 확대 + 라벨 길이 분기 (B-2) | 다국어 깨짐 |
| **P2** | 검색 하이라이트·토스트·정렬 라벨의 텍스트 너비 추정 정확화 (B-3,4,5,6) | 한국어 사용자 시각 깨짐 |
| **P2** | Esc/포커스 손실 정책 완화 (B-7,B-8) | 검색 흐름 끊김 |
| **P2** | PDF 주석 영역 드래그로 지정 가능하게 (C) | Highlight 본래 의미 |
| **P3** | 즐겨찾기 `"* "` → ★ 아이콘 통일, Smart 톱니 → 다른 아이콘 | 시각 일관성 |
| **P3** | `mod.rs` 분리 (Welcome / Inspector / PDF) | 유지보수성 |

---

## 한 줄 요약

스펙은 *기본 셸*이라 했지만 실제 코드는 PDF 주석·Q&A·Manuscript 아웃라인·Visual·Citation Cite·다국어·키보드 단축키까지 *시각적으로* 굉장히 풍성하다. 그러나 라이브러리 채우는 첫 단계(Import / DOI / BibTeX) 거의 전부가 TODO/토스트라 사용자는 *5분 안에 막다른 길*에 부딪힌다. 화려한 inspector 탭들보다 먼저 P0의 "들이고 내보내기" 파이프라인부터 메워야 의미가 있다.
