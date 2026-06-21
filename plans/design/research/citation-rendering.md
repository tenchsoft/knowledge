# Design: citation-rendering

## 한 줄 정의
spec(`plans/spec/research/citation-rendering.md`)에서 정의한 인용 스타일 렌더링을 인스펙터 Cite 탭에서 시각적으로 표현.

## 시각적 레이아웃
기존 인스펙터 Cite 탭 구조 재사용. 새 시각 요소 없음 — 포맷 버튼 활성 상태 + 참고문헌 목록 + DOI 입력.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Format button (BibTeX) | `Button` | `research.citation.format.bibtex` |
| Format button (RIS) | `Button` | `research.citation.format.ris` |
| Format button (APA) | `Button` | `research.citation.format.apa` |
| Format button (Chicago) | `Button` | `research.citation.format.chicago` |
| Format button (MLA) | `Button` | `research.citation.format.mla` |
| Reference list item | `Label` | — |
| DOI input | `TextInput` | `research.citation.doi` |
| Fetch button | `Button` | `research.citation.fetch` |
| Import BibTeX button | `Button` | `research.citation.import_bibtex` |

모두 기존 인스펙터 디자인 사용. 별도 visual properties / states 명세 불필요 (design `research-inspector` 참조).

## Out of scope
- CSL 스타일 편집기 (별도 spec).
- 워드 프로세서 플러그인 (별도 spec).
