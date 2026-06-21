# Design: pdf-annotation-list-toggle-button

## 한 줄 정의
PDF 주석 툴바에서 Ann 버튼을 클릭하면 `pdf_show_annotation_list`가 전환되고 주석 사이드바가 열리거나 닫힌다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Annotation list toggle | `Button` | `research.pdf.annotation_list_toggle` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
