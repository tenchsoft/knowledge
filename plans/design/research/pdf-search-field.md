# Design: pdf-search-field

## 한 줄 정의
PDF 리더 검색 바에서 검색 필드에 타이핑하면 PDF 내 텍스트 검색이 실행된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| PDF search input | `TextInput` | `research.pdf.search` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
