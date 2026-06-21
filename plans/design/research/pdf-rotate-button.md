# Design: pdf-rotate-button

## 한 줄 정의
PDF 리더에서 Rotate 버튼을 클릭하면 `pdf_rotation`이 90도 증가하고 PDF 표면이 새 방향으로 재렌더링된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Rotate button | `Button` | `research.pdf.rotate` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
