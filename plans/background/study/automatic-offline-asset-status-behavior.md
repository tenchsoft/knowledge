# Background: automatic-offline-asset-status-behavior

## 한 줄 정의
비주얼 에셋의 로컬 캐시 상태를 모니터링하여 `OfflineAssetState::cache_ready`를 갱신하고, Daily dashboard에 `offline_ready` 상태를 반영한다.

## Trigger / Schedule

| Trigger | 조건 | 빈도 |
|---------|------|------|
| 앱 시작 | `StudyState::default()` | 1회 |
| 에셋 상태 갱신 | `refresh_offline_asset_state()` | 호출 시 |
| Daily dashboard 갱신 | `refresh_daily_dashboard()` | 세션 이벤트 시 |

## Lifecycle & State

```
checking ──[scan scene_refs]──→ ready ──[asset missing]──→ degraded
                                                      │
                                                      └──[asset restored]──→ ready
```

- **checking**: `offline_asset_state()`가 `visual_specs`의 `scene_ref`를 스캔.
- **ready**: 모든 필수 `scene_ref`가 비어있지 않음. `cache_ready = true`.
- **degraded**: 일부 `scene_ref`가 비어있음. `cache_ready = false`. `missing_scene_refs`에 기록.

## Concurrency
- **인스턴스 정책**: 단일. 메인 스레드 동기.
- **동시성 모델**: 동기 직렬.
- **재진입성**: 안전.
- **취소**: 해당 없음.

## Resource budget
- CPU: O(visual_specs) 스캔. 최대 ~100개 가정 시 < 1ms.
- 메모리: `required_scene_refs`, `missing_scene_refs` 벡터. 수십 항목.
- 모바일/데스크톱 동일.

## Data flow
- **Read**: `StudyState::visual_specs` (각 `LearningVisualSpec::renderer::scene_ref`).
- **Write**: `StudyState::offline_assets` (`OfflineAssetState`).
- **Persistence**: 없음 (런타임 상태만).
- **IPC**: 없음.

## Failure & Recovery

| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 누락 에셋 | `scene_ref` 빈 문자열 | `missing_scene_refs`에 기록, `cache_ready = false` | Daily dashboard에 반영 |

## Observability
- **Log**: `tracing::warn!("offline asset missing: {:?}", missing_scene_refs)` if not empty.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| Daily dashboard | `Label` | due/new/acc 텍스트 | `offline_ready`는 dashboard 필드로 간접 반영 |

`offline_ready`는 현재 명시적 UI 노드 없이 dashboard 데이터로만 존재.

## UI 인터페이스
design(`plans/design/study/study-automatic-ui.md`) offline asset status 섹션에 정의.

## Out of scope
- 에셋 다운로드/프리페치 (별도 spec).
- 에셋 버전 관리 (별도 spec).
