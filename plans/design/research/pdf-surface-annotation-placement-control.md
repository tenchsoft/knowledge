# Design: pdf-surface-annotation-placement-control

## 한 줄 정의
PDF 페이지에서 주석 도구가 활성화된 상태로 클릭하면 현재 페이지의 클릭 위치에 새 주석이 생성되고 성공 토스트가 표시된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| PDF surface (click target) | `Canvas` | `research.pdf.surface` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
