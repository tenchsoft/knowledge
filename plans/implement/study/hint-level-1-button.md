# Implement: hint-level-1-button

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 튜터 패널에서 Hint 1 버튼 클릭 시 첫 번째 힌트가 공개된다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::RevealHint(1) 핸들링 | grep 'StudyHit::RevealHint' apps/study/ |
| state.rs | reveal_hint 메서드 | grep 'fn reveal_hint' apps/study/ |
| tutor.rs | hint_rect(tutor, 1) 렌더링 | grep 'fn hint_rect' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 힌트 버튼 클릭 핸들링
- **입력**: PointerEvent::Down on hint_rect(tutor, 1)
- **처리**: reveal_hint(1) 호출
- **출력/사이드 이펙트**: hint_level = max(hint_level, 1), 힌트 텍스트 표시


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.tutor.hint.1 | button | hint 1 | tutor.width() >= 160 |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
