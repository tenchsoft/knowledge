# Repository Architecture

This repo is organized around shared foundations first, product shells second.

## Layers

```text
apps/*              Thin product app shells (research, study)
crates/*-core       Shared Rust domain and platform contracts
crates/tench-ui     Self-built retained-mode UI framework
tools/*             Repo automation and CI entrypoints
```

## Crate Dependency Graph

```text
tench-ui ──────── ui-automation-core
tench-ui-test ─── tench-ui, ui-automation-core

research-core ─── document-core, fs-core, job-core, office-io,
                  search-core, storage-core, shared-types

study-core ────── document-core, fs-core, job-core,
                  search-core, storage-core, shared-types

office-io ─────── document-core, fs-core, storage-core

app-core (독립)
```

## Shared Feature Ownership

| Shared area | Rust crate | Reused by |
| --- | --- | --- |
| App modules/routes | `app-core` | research, study |
| Local files/permissions | `fs-core` | research, study |
| Local storage policy | `storage-core` | research, study |
| Documents/notes/annotations/office content | `document-core` | research, study |
| Search/indexing | `search-core` | research, study |
| Background jobs | `job-core` | research, study |
| Office document I/O | `office-io` | research |
| Research domain | `research-core` | research |
| Study domain | `study-core` | study |
| Shared types | `shared-types` | research, study |
| UI framework | `tench-ui` | research, study |
| UI test harness | `tench-ui-test` | research, study (dev) |
| UI automation protocol | `ui-automation-core` | research, study (dev) |

## Product Shell Rule

Product apps should only own product-specific composition and domain glue. If a
feature appears in multiple plan directories, it starts in a shared crate.

## Plan Mapping

| Plans | App slot | Primary shared crates |
| --- | --- | --- |
| `research` | `apps/research` | `research-core`, `document-core`, `search-core`, `job-core`, `office-io`, `storage-core`, `fs-core`, `shared-types` |
| `study` | `apps/study` | `study-core`, `document-core`, `search-core`, `job-core`, `storage-core`, `fs-core`, `shared-types` |
