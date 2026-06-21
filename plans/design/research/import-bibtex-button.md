# Design: import-bibtex-button

## 한 줄 정의
Cite 인스펙터 탭에서 Import BibTeX 버튼을 클릭하면 BibTeX 파일 가져오기가 시작된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Import BibTeX button | `Button` | `research.citation.import_bibtex` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
