# Design: research-format-roundtrip

## 한 줄 정의
spec(`plans/spec/research/research-format-roundtrip.md`)에서 정의한 라이브러리 저장/복원 동작의 UI 표면. 새 시각 요소 없음 — 기존 토스트 + 상태 텍스트로 통보.

## 시각적 레이아웃
신규 시각 요소 없음. 저장/복원 결과는 토스트(`research.toast.*`)와 헤더 상태 텍스트로 표시.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Success toast | `Label` | 토스트 스택 내 |
| Error toast | `Label` | 토스트 스택 내 |
| Header status text | `Label` | — |

모두 기존 컴포넌트 재사용. 별도 visual properties / states 명세 불필요.

## Out of scope
- 클라우드 동기화 (별도 spec).
- 충돌 해결 UI (별도 spec).
