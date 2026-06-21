# Design: pdf-underline-tool-button

## 한 줄 정의
PDF 주석 툴바에서 밑줄 버튼을 클릭하면 밑줄 도구가 활성화되고 다시 클릭하면 None으로 복귀한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Underline tool button | `Button` | `research.pdf.tool.underline` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
