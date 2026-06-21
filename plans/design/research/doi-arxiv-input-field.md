# Design: doi-arxiv-input-field

## 한 줄 정의
Cite 인스펙터 탭에서 DOI / arXiv ID 필드에 타이핑하면 `doi_input`이 업데이트되고 Fetch 또는 Enter 전까지 대기한다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| DOI / arXiv input | `TextInput` | `research.citation.doi` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
