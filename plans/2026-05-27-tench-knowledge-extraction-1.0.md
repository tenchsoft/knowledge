# Tench-Knowledge 레포 추출 계획

## Objective

`~/tench/Tench-One` 모노레포에서 `research` 및 `study` 앱과 13개 의존 크레이트를 추출하여, 완전히 독립적인 Cargo 워크스페이스 `Tench-Knowledge`를 구성한다. 다른 레포와의 동기화 없이 각 크레이트는 독립적으로 진화한다.

---

## 사전 분석: 발견된 이슈

소스 프로젝트 분석 중 발견된 문제점들:

1. **`chrono` 누락**: `apps/study/src-tauri/Cargo.toml:20`이 `chrono = "0.4"`를 인라인으로 사용 중. `[workspace.dependencies]`에 추가 필요.
2. **tench-ui 경로 참조 불일치**: research 앱은 직접 path(`path = "../../../crates/tench-ui"`) 사용, study 앱은 `workspace = true` 사용. 추출 후 통일 필요.
3. **`tools/workspace-guard` 누락**: 원본 계획에 없으나 CI에서 `cargo run -p tench-workspace-guard` 실행. 포함 필요.
4. **`tench-ci-core` 미포함**: CI에서 forbidden command scanner로 사용하나 13개 크레이트에 없음. CI에서 해당 스텝 제거 필요.
5. **architecture-guard baseline 정리**: 347개 항목 중 research/study/13개 크레이트 관련 항목만 필터링 필요.
6. **plans/ 하위 디렉토리 규모**: research에 103개, study에 118개 implement plan 존재. 전체 복사 필요.

---

## Implementation Plan

### Phase 1: 대상 디렉토리 및 파일 준비

- [ ] **1.1** `~/tench/Tench-Knowledge/` 디렉토리 생성 (또는 기존 디렉토리 확인)
- [ ] **1.2** 기본 디렉토리 구조 생성: `apps/`, `crates/`, `tools/`, `plans/`, `template/`, `.gitea/workflows/`

### Phase 2: 앱 복사

- [ ] **2.1** `apps/research/` 전체를 `~/tench/Tench-Knowledge/apps/research/`로 복사
  - `apps/research/src-tauri/` 포함 (Cargo.toml, build.rs, tauri.conf.json, capabilities/, frontend/, src/, tests/)
- [ ] **2.2** `apps/study/` 전체를 `~/tench/Tench-Knowledge/apps/study/`로 복사
  - `apps/study/src-tauri/` 포함 (Cargo.toml, build.rs, tauri.conf.json, capabilities/, frontend/, src/, tests/)

### Phase 3: 13개 크레이트 복사

다음 크레이트를 `~/tench/Tench-Knowledge/crates/` 하위에 복사:

- [ ] **3.1** `crates/tench-ui/` → `crates/tench-ui/`
- [ ] **3.2** `crates/ui-automation-core/` → `crates/ui-automation-core/`
- [ ] **3.3** `crates/tench-ui-test/` → `crates/tench-ui-test/`
- [ ] **3.4** `crates/shared-types/` → `crates/shared-types/`
- [ ] **3.5** `crates/storage-core/` → `crates/storage-core/`
- [ ] **3.6** `crates/fs-core/` → `crates/fs-core/`
- [ ] **3.7** `crates/document-core/` → `crates/document-core/`
- [ ] **3.8** `crates/office-io/` → `crates/office-io/`
- [ ] **3.9** `crates/job-core/` → `crates/job-core/`
- [ ] **3.10** `crates/search-core/` → `crates/search-core/`
- [ ] **3.11** `crates/app-core/` → `crates/app-core/`
- [ ] **3.12** `crates/research-core/` → `crates/research-core/`
- [ ] **3.13** `crates/study-core/` → `crates/study-core/`

### Phase 4: 도구 복사 및 수정

- [ ] **4.1** `tools/architecture-guard/` 복사 → `~/tench/Tench-Knowledge/tools/architecture-guard/`
- [ ] **4.2** `tools/workspace-guard/` 복사 → `~/tench/Tench-Knowledge/tools/workspace-guard/`
  - 원본 계획에 누락되어 있었으나 CI에서 필요함
- [ ] **4.3** `tools/architecture-guard/line_budget_baseline.txt`에서 research/study/13개 크레이트 관련 항목만 필터링하여 재생성
  - 제거 대상: apps/docs/, apps/sheets/, apps/slides/, apps/code/, apps/composer/, apps/player/, apps/view/, apps/kodocs/, apps/universe/, apps/story/, apps/engine/, apps/one/, apps/pixel-design/ 및 관련 크레이트 항목
- [ ] **4.4** `tools/workspace-guard/`의 하드코딩된 패키지명 매핑이 15개 멤버(research, study, 13개 크레이트)에 맞게 동작하는지 확인

### Phase 5: 워크스페이스 루트 Cargo.toml 작성

- [ ] **5.1** `[workspace]` members에 15개 항목 + tools 2개 = 17개 멤버 등록
  ```
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
    "tools/workspace-guard",
  ]
  resolver = "3"
  ```
- [ ] **5.2** `[workspace.package]` 설정: version `0.1.0`, edition `2021`, license `UNLICENSED`, authors `["Tench"]`
- [ ] **5.3** `[workspace.dependencies]`에 필요한 외부 의존성만 포함:
  - **필수 외부**: `serde`(1, derive), `serde_json`(1), `rusqlite`(0.39.0, bundled), `regex`(1), `unicode-segmentation`(1), `zip`(8.6, deflate), `getrandom`(0.3), `aes-gcm`(0.10), `sha2`(0.10), `image`(0.25, features), `pollster`(0.4), `tauri`(2), `tauri-build`(2), `chrono`(0.4)
  - **tench-ui 전용 외부** (vello, parley, accesskit, kurbo, peniko, smallvec, log 등): tench-ui의 Cargo.toml에 인라인 선언되어 있으므로 workspace.dependencies에 추가 불필요. 단, 일관성을 위해 workspace로 끌어올릴지 결정 필요
  - **내부 path 의존성**: 13개 크레이트 모두 `path = "crates/<name>"` 형식으로 등록
- [ ] **5.4** Tench-One의 workspace.dependencies에서 불필요한 항목 제거:
  - 제거: `axum`, `tower-http`, `tokio`(full), `base64`, `kamadak-exif`, `sevenz-rust2`, `dirs`, `dirs-dev`, `roxmltree`, `tauri-plugin-dialog`, `unrar-ng`, `ureq`, `toml`, `arboard` 등

### Phase 6: 앱/크레이트 Cargo.toml 경로 참조 정리

- [ ] **6.1** `apps/research/src-tauri/Cargo.toml` 수정:
  - `tench-ui` 경로를 `path = "../../../crates/tench-ui"`에서 `workspace = true`로 변경 (features = ["tauri"] 유지)
  - 모든 `workspace = true` 참조가 새 워크스페이스에서 해석 가능한지 확인
- [ ] **6.2** `apps/study/src-tauri/Cargo.toml` 수정:
  - `chrono = "0.4"` 인라인을 `chrono = { workspace = true }`로 변경 (workspace.dependencies에 추가 후)
- [ ] **6.3** 13개 크레이트의 `Cargo.toml`에서 `path` 참조가 모두 `workspace = true`로 정상 해석되는지 확인
  - 각 크레이트 내부 의존성이 새 워크스페이스 members에 존재하는지 검증

### Phase 7: Cargo.lock 생성

- [ ] **7.1** `cargo generate-lockfile` 실행하여 새 Cargo.lock 생성
- [ ] **7.2** lock 파일 내용 검토 — 예상치 못한 의존성(제거한 크레이트 관련)이 없는지 확인

### Phase 8: plans/ 및 template/ 복사

- [ ] **8.1** `plans/spec/research/` 전체 복사
- [ ] **8.2** `plans/spec/study/` 전체 복사
- [ ] **8.3** `plans/design/research/` 전체 복사
- [ ] **8.4** `plans/design/study/` 전체 복사
- [ ] **8.5** `plans/background/research/` 전체 복사
- [ ] **8.6** `plans/background/study/` 전체 복사
- [ ] **8.7** `plans/implement/research/` 전체 복사 (약 103개 파일)
- [ ] **8.8** `plans/implement/study/` 전체 복사 (약 118개 파일)
- [ ] **8.9** `plans/test/research/` 전체 복사
- [ ] **8.10** `plans/test/study/` 전체 복사
- [ ] **8.11** research/study 관련 최상위 plans 파일 복사:
  - `plans/2026-05-06-research-ui-user-perspective-review.md`
  - `plans/2026-05-08-research-implement-plan-audit-v1.md`
  - `plans/2026-05-06-study-ui-user-perspective-review.md`
  - `plans/2026-05-08-study-implement-audit-1.md`
  - `plans/ui/research_report.txt`
  - `plans/ui/study_report.txt`
- [ ] **8.12** `template/` 전체 복사 (5개 템플릿 파일)

### Phase 9: CI 설정

- [ ] **9.1** `.gitea/workflows/ci.yml` 작성 — Tench-One의 CI를 기반으로 수정:
  - `static-quality` 잡에서 `tench-ci-core` forbidden scanner 스텝 **제거** (해당 크레이트 미포함)
  - `architecture-guard` 실행 유지 (baseline은 Phase 4에서 재생성)
  - `workspace-guard` 실행 유지
  - `cargo fmt`, `cargo clippy`, `cargo check` 유지
  - `unit-tests`, `integration-tests` 유지
  - `security-regression`, `product-e2e`, `ui-e2e`, `ui-automation`, `app-smoke`, `release-validation` 유지
  - 캐시 설정 동일 적용 (`~/.cargo/registry` + `~/.cargo/git`, `target/` 제외)
  - `RUSTFLAGS: "-Dwarnings"` 유지
  - 트리거: push to main, PR to main, workflow_dispatch

### Phase 10: 문서 파일 작성

- [ ] **10.1** `AGENTS.md` 작성 — Tench-One의 AGENTS.md를 기반으로 Tench-Knowledge에 맞게 수정:
  - 제품 목록을 research, study 2개로 한정
  - 크레이트 목록을 13개로 한정
  - 원격 저장소 주소를 `Tench-Knowledge`용으로 변경
  - 나머지 원칙(로컬 퍼스트, 100% Rust, Tauri 2, tench-ui 등)은 동일
- [ ] **10.2** `ARCHITECTURE.md` 작성 — 의존성 그래프와 크레이트 소유권 매트릭스를 research/study 중심으로 재작성

### Phase 11: 빌드 및 테스트 검증

- [ ] **11.1** `cargo check --workspace --locked` 실행 — 컴파일 에러 없는지 확인
- [ ] **11.2** `cargo test --workspace --locked` 실행 — 테스트 통과 확인
- [ ] **11.3** `cargo fmt --all -- --check` 실행 — 포맷팅 확인
- [ ] **11.4** `cargo clippy --workspace --locked --all-targets -- -D warnings` 실행 — 린트 통과 확인
- [ ] **11.5** `cargo run --locked -p tench-architecture-guard -- --strict` 실행 — 아키텍처 가드 통과 확인
- [ ] **11.6** `cargo run --locked -p tench-workspace-guard` 실행 — 워크스페이스 가드 통과 확인

### Phase 12: Gitea 레포 생성 및 푸시

- [ ] **12.1** Gitea에 `Tench-Knowledge` 빈 레포 생성
- [ ] **12.2** `git init` 후 전체 파일 커밋
- [ ] **12.3** 원격 저장소 추가 및 push
- [ ] **12.4** Gitea CI 파이프라인 녹색 확인

---

## Verification Criteria

- [ ] `cargo check --workspace --locked` 성공 (에러 0)
- [ ] `cargo test --workspace --locked` 성공 (모든 테스트 통과)
- [ ] `cargo fmt --all -- --check` 성공
- [ ] `cargo clippy --workspace --locked --all-targets -- -D warnings` 성공
- [ ] `tench-architecture-guard --strict` 성공 (baseline과 일치)
- [ ] `tench-workspace-guard` 성공
- [ ] Gitea CI 파이프라인 모든 잡 녹색
- [ ] Tench-One에 대한 어떤 path 참조도 존재하지 않음 (독립성 확인)
- [ ] workspace.dependencies에 불필요한 외부 의존성이 없음

---

## Potential Risks and Mitigations

1. **숨겨진 의존성 누락**
   - 13개 크레이트가 Tench-One의 다른 크레이트(engine-core, media-core 등)를 간접 참조할 가능성
   - 완화: `cargo check --workspace --locked`로 미사용 의존성 및 누락 의존성 즉시 발견

2. **tench-ui의 복잡한 외부 의존성**
   - vello, parley, accesskit, kurbo, peniko 등 GPU 렌더링 관련 크레이트가 특정 플랫폼에서 빌드 이슈를 일으킬 가능성
   - 완화: Tench-One에서 이미 빌드된 적 있는 버전을 그대로 사용하므로 위험 낮음

3. **architecture-guard baseline 불일치**
   - 복사 후 소스 파일의 라인 수가 baseline과 다를 경우 `--strict` 모드에서 실패
   - 완화: Phase 4에서 baseline을 실제 파일 기준으로 재생성

4. **workspace-guard 하드코딩 매핑**
   - `workspace-guard/main.rs`에 하드코딩된 패키지명 매핑이 15개 멤버에 맞지 않을 수 있음
   - 완화: Phase 4에서 매핑 검증 및 필요시 수정

5. **plans/ 내부의 다른 제품 참조**
   - plans 문서 내부에서 docs, sheets 등 다른 제품을 참조할 수 있음
   - 완화: plans는 참고용 문서이므로 빌드/테스트에 영향 없음. 필요시 점진적 정리

6. **study 앱의 chrono 인라인 의존성**
   - 원본에서 workspace.dependencies에 없어서 `workspace = true` 전환 시 누락 가능
   - 완화: Phase 5에서 명시적으로 workspace.dependencies에 `chrono = "0.4"` 추가

---

## Alternative Approaches

1. **git filter-branch / git subtree split**: Git 히스토리를 보존하면서 추출. 단, 모노레포 구조상 경로가 분산되어 있어 적용이 복잡하고, 13개 크레이트 + 2개 앱의 교차 의존성을 정확히 분리하기 어려움. 현재 접근(물리적 복사)이 더 확실함.

2. **Cargo workspace inheritance 최소화**: 각 크레이트의 Cargo.toml에서 `workspace = true` 대신 직접 버전을 명시. 독립성은 극대화되지만 유지보수 부담이 증가함. 현재 계획대로 workspace.dependencies를 사용하는 것이 효율적.

3. **tench-ci-core 포함**: forbidden command scanner를 위해 `tench-ci-core`도 포함. 보안 검사의 일관성은 유지되지만, 불필요한 크레이트까지 포함하게 됨. CI 스텝 제거가 더 깔끔함.
