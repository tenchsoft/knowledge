# Design: question-mark-shortcut-help-toggle

## 한 줄 정의
? 키를 누르면 텍스트 입력에 포커스가 없을 때 단축키 도움말 오버레이가 열리거나 닫힌다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Shortcut help overlay | `Dialog` | `research.shortcuts.help` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
