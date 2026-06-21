# Implement: automatic-bookmark-indicator-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 북마크된 컨셉에 골든 별(`★`) 아이콘이 아웃라인 행에 자동 표시되고, 검색 바 옆 토글 버튼의 상태가 현재 컨셉의 북마크 여부를 반영한다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/curriculum.rs` (북마크 표시) | 컨셉 행 우측에 별 아이콘 렌더 | ``fn paint_outline` 내 `is_bookmarked` 분기` |
| `apps/study/src-tauri/src/ui/curriculum.rs` (토글 버튼) | 검색 바 옆 북마크 버튼 상태 렌더 | ``fn paint_outline` 내 `bookmark_rect` 영역` |

## 필요한 변경 (의도 단위)
### 1. 컨셉 행에 북마크 아이콘 표시
- **입력**: `bookmarked_concept_ids` 집합에 컨셉 id 포함 여부
- **처리**: 포함된 경우 컨셉 행 우측에 `★` 문자를 `STATUS_WARNING` 색상으로 그린다.
- **출력/사이드 이펙트**: 북마크된 컨셉이 아웃라인에서 시각적으로 표시된다.
### 2. 북마크 토글 버튼 상태 반영
- **입력**: 현재 활성 컨셉의 id가 `bookmarked_concept_ids`에 포함되었는지
- **처리**: 포함되면 `★`(채운 별, `STATUS_WARNING`), 아니면 `☆`(빈 별, `NEUTRAL_400`)로 렌더한다.
- **출력/사이드 이펙트**: 토글 버튼이 현재 컨셉의 북마크 상태를 시각적으로 나타낸다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, `paint_outline` 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
