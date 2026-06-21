# Design: pdf-next-page-button

## 한 줄 정의
PDF 리더에서 Next Page 버튼을 클릭하면 다음 페이지로 이동한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Next page button | `Button` | `research.pdf.next_page` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
