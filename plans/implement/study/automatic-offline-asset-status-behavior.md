# Implement: automatic-offline-asset-status-behavior

> 작성 시점과 실행 시점 사이 코드 변경 가능성. 위치는 항상 grep으로 재확인 후 변경.

## 목표
- spec: 오프라인 에셋 캐시 상태(`cache_ready`, `required_scene_refs`, `missing_scene_refs`)가 대시보드와 UX audit에 자동으로 반영된다.

## 영향 받는 영역
| 영역 | 무엇이 바뀌나 | 찾기 전략 |
|------|----------------|-----------|
| `apps/study/src-tauri/src/ui/state.rs` (오프라인 상태 갱신) | `refresh_offline_asset_state`에서 에셋 상태 재계산 | ``fn refresh_offline_asset_state`` |
| `apps/study/src-tauri/src/ui/state.rs` (대시보드 반영) | `refresh_daily_dashboard`에서 `offline_ready`를 `cache_ready`로 설정 | ``fn refresh_daily_dashboard`` |

## 필요한 변경 (의도 단위)
### 1. 오프라인 에셋 상태 재계산
- **입력**: `visual_specs` 벡터
- **처리**: `offline_asset_state` 빌더 함수로 `required_scene_refs`, `missing_scene_refs`, `cache_ready`를 재계산하여 `offline_assets` 필드에 저장한다.
- **출력/사이드 이펙트**: 오프라인 에셋 상태가 최신 상태로 유지된다.
### 2. 대시보드에 오프라인 상태 반영
- **입력**: `offline_assets.cache_ready` 불리언
- **처리**: `refresh_daily_dashboard`에서 `dashboard.offline_ready`를 `cache_ready` 값으로 설정한다.
- **출력/사이드 이펙트**: 일일 대시보드에 오프라인 준비 상태가 반영된다.

## 새 자동화 노드
| debug_id | role | value | 노출 조건 |
|----------|------|-------|----------|

(자동 렌더링 동작 — 별도 자동화 노드 불필요, state 메서드 내에서 처리)

## 의존
- 선행 implement: 없음

## 작업 절차
1. spec/design 읽기
2. grep으로 위치 확정
3. 의도대로 코드 변경
4. cargo check 통과 확인
