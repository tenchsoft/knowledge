<div align="center">

# Tench Knowledge

**Research & study tools, built in 100% Rust.**

Research · Study — from academic paper management to spaced-repetition tutoring, all running locally.

[![Language: Rust](https://img.shields.io/badge/Language-Rust-dea584.svg)](https://www.rust-lang.org/)
[![Framework: Tauri 2](https://img.shields.io/badge/Framework-Tauri_2-FFC140.svg)](https://v2.tauri.app/)
[![License: UNLICENSED](https://img.shields.io/badge/License-UNLICENSED-red.svg)](#license)
[![Status: Preview](https://img.shields.io/badge/Status-Preview-orange.svg)](#roadmap)
[![Pricing: $1/mo](https://img.shields.io/badge/Pricing-%241%2Fmo-1ca096.svg)](https://tenchsoft.com/pricing)

</div>

---

## Overview

Tench Knowledge bundles two learning tools — a reference manager for researchers and a study companion for students. Both apps run locally and use Tench Engine for AI tutoring, summarization, and spaced-repetition scheduling. No accounts, no cloud sync, no telemetry.

## Products

| | Product | Benchmarked against | Description |
|---|---|---|---|
| 📚 | **Research** | Mendeley, Zotero | Reference manager — DOI fetch, PDF annotation, manuscript assembly, BibTeX/RIS/APA/Chicago/MLA citations |
| 🎓 | **Study** | Anki | Spaced-repetition tutor — SM-2 algorithm, AI tutor, multi-subject decks, math palette, profile wizard |

## Features

### Research

- DOI / arXiv metadata fetch
- PDF annotation (highlight, underline, strikeout, sticky note, freehand)
- Manuscript assembly with section management
- Citation formats: **APA**, **MLA**, **Chicago**, **BibTeX**, **RIS**
- Inspector tabs: Cite, Notes, Q&A, Summary, Visual, Write
- Smart collections, saved searches
- Backup/restore
- Q&A on papers (key points, limitations, summarize)

### Study

- Spaced repetition via **SM-2** algorithm
- AI tutor that adapts to weak points
- Multi-subject decks & tags
- Practice mode with instant feedback
- Math palette (Greek letters, fractions, powers, sums, π, ∞)
- Profile wizard (locale, level, domain)
- Visual timeline & achievement badges
- High-contrast mode for accessibility
- Import Anki decks (`.apkg`, `.csv`)

## Architecture

```
apps/<product>/src-tauri/        Product shells (Tauri 2)
crates/document-core/            Shared document model
crates/office-io/                Format readers/writers
crates/research-core/            Reference management (DOI, citations, manuscripts)
crates/study-core/               Spaced repetition, deck model
crates/job-core/                 Background jobs
crates/search-core/              Full-text indexing
crates/app-core/                 Shared modules/routes
crates/storage-core/             Local persistence + AES-GCM encryption
crates/fs-core/                  File-system access policy
crates/engine-core/              Tench Engine client
crates/tench-ui/                 Self-built widget framework
crates/tench-ui-test/            Headless E2E harness
tools/architecture-guard/        Repo structure enforcement
tools/workspace-guard/           Workspace integrity check
```

## Build

```bash
cargo check --workspace --locked
cargo build --workspace --locked
cargo test --workspace --locked
cargo run --locked -p research    # or: study
```

## Roadmap

- [x] Research — DOI fetch, PDF annotation, citations, Q&A
- [x] Study — SM-2 spaced repetition, AI tutor, math palette
- [ ] Research — web-clipper browser extension
- [ ] Study — collaborative decks (local network)
- [ ] Mobile companion (review-only)

## Pricing

- **$1 / month per device** — every Tench app.

→ https://tenchsoft.com/pricing

## License

UNLICENSED — source available for review, binary distribution requires a subscription.

## Sister Projects

- **[Tench Office](https://github.com/tenchsoft/office)** — Docs / Sheets / Slides / Kodocs
- **[Tench Media](https://github.com/tenchsoft/media)** — View / Pixel Design / Player / Composer
- **[Tench Authoring](https://github.com/tenchsoft/authoring)** — Story / Universe
- **[Tench Code](https://github.com/tenchsoft/code)** — AI-augmented code editor
- **[tenchsoft.com](https://tenchsoft.com)** — account, license, downloads
