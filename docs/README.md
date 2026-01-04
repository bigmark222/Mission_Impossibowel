# Documentation index

Two mdBooks replace the legacy docs:
- **User guide**: `docs/user_book` (capture → ETL → train → infer, defaults, FAQs). App-specific commands now live in the app repository (`colon_sim` reference app: https://github.com/via-balaena/Deep-Poo). Crates.io versions: `0.1.0` across the shared crates; app repo is separate.
- **Contributor guide**: `docs/contributor_book` (architecture, crates, hooks, testing, migration).

Build/read:
```bash
mdbook build docs/user_book
mdbook build docs/contributor_book
```

Repository map (docs-relevant bits):
- `sim_core`, `vision_core`, `vision_runtime`, `data_contracts`, `capture_utils`: substrate crates.
- `models`, `training`, `inference`: detectors, training/eval, detector factory.
- `tools`: CLI utilities (overlay/prune/etl/cmd/scheduler/tui/single_infer) consumed by apps.
- `docs/user_book/`, `docs/contributor_book/`: mdBooks.

Quick doc pointers:
- Crate-focused workflows: see `docs/user_book/src/*.md` (app commands are maintained in the app repo).
- Architecture/crate internals/hooks/testing: `docs/contributor_book/src/*.md`.
