# Implement: automatic-session-timer-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 헤더 폭 ≥560px일 때 경과 시간이 `HH:MM:SS` 형식으로 자동 표시된다. `elapsed_seconds` 필드를 기반으로 계산된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (타이머 렌더) | 헤더 폭 ≥560px 조건부로 `HH:MM:SS` 포맷 텍스트 렌더 | ``fn paint_shell` 내 `elapsed_seconds` 타이머 분기` |

## 필요한 변경 (의도 단위)
### 1. 세션 타이머 자동 렌더
- **입력**: `state.elapsed_seconds` — i32 (초 단위)
- **처리**: `elapsed_seconds / 3600`, `(elapsed_seconds % 3600) / 60`, `elapsed_seconds % 60`으로 시/분/초를 계산하여 `{HH:02}:{MM:02}:{SS:02}` 형식으로 포맷. `NEUTRAL_400` 색상으로 렌더.
- **출력/사이드 이펙트**: 헤더에 경과 시간이 자동으로 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_shell` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
