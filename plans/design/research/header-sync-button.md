# Design: header-sync-button

## 한 줄 정의
헤더 액션 바에서 Sync 버튼을 클릭하면 동기화가 시작된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Sync button | `Button` | `research.header.sync` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
