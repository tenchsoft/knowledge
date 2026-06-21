# Design: text-input-routing-control

## 한 줄 정의
일반 텍스트 입력 시 타이핑된 문자가 포커스된 검색, Q&A, PDF 검색, DOI, 또는 인용 검색 필드에만 전달된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Text input routing (shortcut-driven) | — | — (라우팅 로직, 별도 컴포넌트 없음) |

시각적 변화는 해당 입력 필드 텍스트 변경으로 나타남. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
