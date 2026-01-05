# Dev Loop

## Watchers
- Code: `cargo watch -x fmt -x 'clippy --workspace --all-targets -D warnings' -x 'test --workspace'`
- Docs: `cargo watch -w docs/contributor_book -w docs/dissection_book -s 'mdbook build docs/contributor_book && mdbook build docs/dissection_book'`

## Doc rebuild strategy
- Use Makefile targets: `make docs` (build), `make docs-watch` (serve/rebuild), `make docs-test` (doctests).
- Mark non-runnable snippets with `ignore`/`no_run` to keep `mdbook test` green.

## Keeping mdBook in sync
- When APIs change: update crate pages (overview/API/examples) and dependency graph if edges shift.
- When features change: update `feature_flags.md` and per-crate examples/snippets.
- When releases happen: refresh version strings, burn-core patch note (drop when upstream fixed), and changelog.
- Run `mdbook test` and `mdbook build` before merging.
