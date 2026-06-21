# Tench-Knowledge 레포 추출 계획

## 원칙

- 완전히 독립적인 Cargo 워크스페이스
- 필요한 공유 크레이트는 물리적으로 복사
- 다른 레포와의 동기화 없음. 각 크레이트는 독립적으로 진화

---

## 앱

| 앱 | 패키지명 | 비고 |
|----|---------|------|
| research | `tench-research` | 참고문헌 관리자 |
| study | `tench-study` | 학습 도구 |

---

## 포함할 크레이트

| 크레이트 | 패키지명 | 직접 소비 앱 | 내부 의존성 |
|----------|---------|-------------|------------|
| tench-ui | `tench-ui` | research, study | `ui-automation-core` |
| ui-automation-core | `tench-ui-automation-core` | 전체 (dev) | 없음 |
| tench-ui-test | `tench-ui-test` | 전체 (dev) | `tench-ui`, `ui-automation-core` |
| shared-types | `tench-shared-types` | (research-core, study-core 경유) | 없음 |
| storage-core | `tench-storage-core` | (research-core, study-core 경유) | 없음 |
| fs-core | `tench-fs-core` | (research-core, study-core 경유) | 없음 |
| document-core | `tench-document-core` | (research-core, study-core 경유) | 없음 |
| office-io | `tench-office-io` | (research-core 경유) | `document-core`, `fs-core`, `storage-core` |
| job-core | `tench-job-core` | research, (research-core, study-core 경유) | 없음 |
| search-core | `tench-search-core` | research, study, (research-core, study-core 경유) | 없음 |
| app-core | `tench-app-core` | research, study | 없음 |
| research-core | `tench-research-core` | research | `document-core`, `fs-core`, `job-core`, `office-io`, `search-core`, `storage-core`, `shared-types` |
| study-core | `tench-study-core` | study | `document-core`, `fs-core`, `job-core`, `search-core`, `storage-core`, `shared-types` |

---

## 크레이트 의존성 그래프

```
tench-ui ──────── ui-automation-core
tench-ui-test ─── tench-ui, ui-automation-core

research-core ─── document-core, fs-core, job-core, office-io,
                  search-core, storage-core, shared-types

study-core ────── document-core, fs-core, job-core,
                  search-core, storage-core, shared-types

office-io ─────── document-core, fs-core, storage-core

app-core (독립)
```

---

## 앱별 상세 의존성

### research (`apps/research/src-tauri`)

```
tench-app-core
tench-job-core
tench-research-core
tench-search-core
tench-ui (features = ["tauri"])
tench-ui-automation-core (dev)
tench-ui-test (dev)
```

### study (`apps/study/src-tauri`)

```
tench-app-core
tench-search-core
tench-study-core
tench-ui (features = ["tauri"])
tench-ui-automation-core (dev)
tench-ui-test (dev)
```

---

## 디렉토리 구조

```
Tench-Knowledge/
├── Cargo.toml              (워크스페이스 루트)
├── Cargo.lock
├── .gitea/
│   └── workflows/ci.yml
├── AGENTS.md
├── ARCHITECTURE.md
├── apps/
│   ├── research/
│   │   └── src-tauri/
│   └── study/
│       └── src-tauri/
├── crates/
│   ├── tench-ui/
│   ├── ui-automation-core/
│   ├── tench-ui-test/
│   ├── shared-types/
│   ├── storage-core/
│   ├── fs-core/
│   ├── document-core/
│   ├── office-io/
│   ├── job-core/
│   ├── search-core/
│   ├── app-core/
│   ├── research-core/
│   └── study-core/
├── plans/
│   ├── spec/research/
│   ├── spec/study/
│   ├── design/research/
│   ├── design/study/
│   ├── background/research/
│   ├── background/study/
│   ├── implement/research/
│   ├── implement/study/
│   ├── test/research/
│   └── test/study/
├── template/
└── tools/
    └── architecture-guard/
```

---

## 워크스페이스 설정

```toml
[workspace]
members = [
  "apps/research/src-tauri",
  "apps/study/src-tauri",
  "crates/tench-ui",
  "crates/ui-automation-core",
  "crates/tench-ui-test",
  "crates/shared-types",
  "crates/storage-core",
  "crates/fs-core",
  "crates/document-core",
  "crates/office-io",
  "crates/job-core",
  "crates/search-core",
  "crates/app-core",
  "crates/research-core",
  "crates/study-core",
  "tools/architecture-guard",
]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "UNLICENSED"
authors = ["Tench"]
```

---

## 이관 체크리스트

1. Gitea에 `Tench-Knowledge` 빈 레포 생성
2. `apps/research`, `apps/study` 복사
3. 13개 크레이트를 `crates/` 하위에 복사
4. `tools/architecture-guard` 복사, baseline을 이 레포 크레이트 13개로 재생성
5. 워크스페이스 루트 `Cargo.toml` 작성 (위 설정 기준)
6. `[workspace.dependencies]` 정리 — 이 레포에서 사용하는 외부 의존성만 남기기
7. 각 앱/크레이트의 `path` 참조 정리 — `path = "../../../crates/..."` → `path = "crates/..."` 로 통일
8. `cargo generate-lockfile` 실행
9. `.gitea/workflows/ci.yml` 작성
10. `AGENTS.md`, `ARCHITECTURE.md` 작성
11. `plans/` 하위에서 research/study 관련 문서만 복사
12. `template/` 복사
13. `cargo check --workspace --locked` 통과 확인
14. `cargo test --workspace --locked` 통과 확인
15. Gitea CI 파이프라인 녹색 확인
