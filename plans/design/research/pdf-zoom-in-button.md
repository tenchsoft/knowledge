# Design: pdf-zoom-in-button

## 한 줄 정의
PDF 리더에서 확대 버튼을 클릭하면 `pdf_zoom`이 증가하고 확대 레이블이 업데이트되며 PDF 표면이 재렌더링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Zoom in button | `Button` | `research.pdf.zoom_in` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
