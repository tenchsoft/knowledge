# Background: automatic-backup-restore-progress-behavior

## 한 줄 정의
백업/복원 작업의 진행 상태가 `ResearchBackupRestoreUi`에 기록되고, 완료/실패 시 토스트로 사용자에게 통보된다.

## Trigger / Schedule
| Trigger | 조건 | 빈도 |
|---------|------|------|
| 백업 시작 | 사용자 액션 (메뉴/단축키) | 사용자 요청 |
| 복원 시작 | 사용자 액션 | 사용자 요청 |
| 작업 완료 | 백그라운드 스레드 완료 | 1회 |
| 작업 실패 | 백그라운드 스레드 에러 | 1회 |

## Lifecycle & State
```
idle ──[backup/restore start]──→ running ──[ok]──→ complete ──→ idle
                                     │
                                     └──[error]──→ failed ──→ idle
```

- **idle**: `backup_restore.last_backup_path` / `last_restore_label` 없음.
- **running**: `progress_history`에 `ResearchProgressEvent` 추가, status `Running`.
- **complete**: status `Complete`, `last_backup_path` / `last_restore_label` 업데이트.
- **failed**: status `Failed`, 토스트 에러 통보.

## Concurrency
- **인스턴스 정책**: 세션당 단일. 동시 백업/복원 불가.
- **동시성 모델**: std::thread로 백업 I/O 분리, 완료 시 메인 스레드에 상태 전달.
- **재진입성**: 실행 중 추가 요청 무시.
- **취소: 앱 종료 시 진행 중 작업은 완료 대기.

## Resource budget
| 자원 | 데스크톱 한계 | 모바일 한계 |
|------|----------------|--------------|
| 메모리 | 진행 이벤트 벡터 — 수 KB | 동일 |
| CPU | I/O 대기 — 유휴 | 동일 |
| 디스크 I/O | 라이브러리 파일 크기만큼 | 동일 |
| 네트워크 | 없음 | 없음 |

## Data flow
- **Read**: `ResearchState.papers`, `collections`, `tags`, `pdf_annotations` (백업 시).
- **Write**: `ResearchState.backup_restore`, `progress_history`.
- **Persistence**: 백업 파일은 디스크에 기록. 복원은 디스크에서 읽기.
- **IPC**: 없음.

## Failure & Recovery
| 실패 모드 | 감지 | 처리 | 사용자 통보 |
|-----------|------|------|--------------|
| 디스크 가득 | I/O Err | failed 상태, 이전 백업 유지 | 토스트 error |
| 권한 없음 | I/O Err | 동일 | 토스트 error |
| 손상된 백업 파일 | 파싱 Err | 복원 중단 | 토스트 error |
| 디스크 공간 부족 (복원) | I/O Err | 복원 중단 | 토스트 error |

자동 재시도 없음. 사용자가 수동으로 재시도.

## Observability
- **Log**: `tracing::info!("backup complete: {}", path)`, `tracing::error!("backup failed: {}", err)`.
- **Metric**: N/A.
- **사용자 가시 상태**:

| debug_id | role | value | 의미 |
|----------|------|-------|------|
| 토스트 스택 | `Label` | `"Backup complete"` / `"Backup failed: ..."` | 결과 통보 |

## UI 인터페이스
design(`plans/design/research/research-automatic-ui.md`) §3 Backup/restore progress가 토스트로 통보.

## Out of scope
- 클라우드 백업 (별도 spec).
- 증분 백업 (별도 spec).
