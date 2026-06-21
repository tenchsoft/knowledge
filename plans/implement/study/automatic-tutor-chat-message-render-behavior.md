# Implement: automatic-tutor-chat-message-render-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 튜터 패널 하단에 최근 3개의 채팅 메시지가 자동으로 렌더링된다. 사용자 메시지는 `ACCENT_STUDY`, 어시스턴트 메시지는 `NEUTRAL_300` 색상으로 구분된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/tutor.rs` (채팅 메시지) | 최근 3개 메시지를 입력 필드 위에 역순으로 렌더 | ``fn paint_tutor_panel` 내 `tutor_chat_messages` 루프` |

## 필요한 변경 (의도 단위)
### 1. 채팅 메시지 자동 렌더
- **입력**: `state.tutor_chat_messages` 벡터의 마지막 3개 항목
- **처리**: 메시지를 역순(`iter().rev().take(3)`)으로 순회하며 `msg_y`를 위로 이동시키며 렌더. `TutorChatRole::User`는 `ACCENT_STUDY`, `TutorChatRole::Assistant`는 `NEUTRAL_300` 색상.
- **출력/사이드 이펙트**: 최근 채팅 메시지가 튜터 패널에 자동으로 표시된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_tutor_panel` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
