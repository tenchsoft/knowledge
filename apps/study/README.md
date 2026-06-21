# Tench Study

Product app slot for learning plans, curricula, documents, practice, and AI tutor
workflows.

Primary plan source: `~/docs/plans/study`.

Expected shared foundations:

- `packages/app-shell`
- `packages/engine-client`
- `crates/document-core`
- `crates/search-core`
- `crates/job-core`
- `crates/storage-core`

Study-specific code should remain here only when it is tied to learning domain
behavior. Document viewing, notes, search, background jobs, and Engine access
belong in shared layers.
