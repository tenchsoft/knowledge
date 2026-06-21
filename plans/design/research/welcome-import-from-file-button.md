# Design: welcome-import-from-file-button

## 한 줄 정의
환영 오버레이에서 Import from file 버튼을 클릭하면 오버레이가 닫히고 `import_status`가 queued가 되며 가져오기 진행이 시작된다.

## Component breakdown
| Component | role | debug_id |
|-----------|------|----------|
| Import from file button | `Button` | `research.welcome.import_file` |

모두 기존 컴포넌트 재사용. 별도 visual properties 불필요.

## States
기존 컴포넌트 상태(default, hover, active, disabled) 사용.
