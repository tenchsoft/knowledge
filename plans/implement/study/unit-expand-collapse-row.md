# Implement: unit-expand-collapse-row

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 아웃라인에서 유닛 헤더 클릭 시 해당 유닛의 컨셉 목록을 펼치거나 접는다

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| on_pointer_event (mod.rs) | StudyHit::ToggleUnit 핸들링 | grep 'StudyHit::ToggleUnit' apps/study/ |
| state.rs | toggle_unit_expand 메서드 | grep 'fn toggle_unit_expand' apps/study/ |
| curriculum.rs | unit_header_y 렌더링 | grep 'fn unit_header_y' apps/study/ |

## 필요한 변경 (의도 단위)
### 1. 유닛 헤더 클릭 핸들링
- **입력**: PointerEvent::Down on unit header rect
- **처리**: toggle_unit_expand(unit_idx) 호출
- **출력/사이드 이펙트**: expanded_units[unit_idx] 토글, v/> 아이콘 변경


## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|
| study.unit.{unit_idx} | button | unit label | 항상 (가시 범위 내) |

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
