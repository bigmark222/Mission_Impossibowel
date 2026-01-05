# Maintenance Routine (Weekly)

Keep the dissection book aligned with the codebase and releases.

## Weekly checklist
- Build status: run `mdbook build docs/dissection_book` to ensure no warnings/errors.
- Dependency graph & crate index: re-run `tools/extract_for_book.sh` and update `crate_index.md` / `dependency_graph.md` if membership changes.
- Feature flags: review `feature_flags.md` for new/removed flags or defaults.
- Flows & contracts: skim `canonical_flows.md` and `integration_contracts.md` after major refactors (capture/train/inference).
- Crate chapters: spot-check any crates that changed that week; ensure public API/traits/error/perf/ownership pages still match code.
- Open questions: update statuses, add new unknowns as they appear.
- Changelog: add entries for notable changes (new crates, breaking changes, refactors).
- Links: verify `linking_style` usage and docs.rs links if versions bumped.

## Release prep (when tagging/publishing)
- Confirm `[patch]`/vendor notes are current; update `overview.md` and crate pages.
- Update versions/links in `docsrs_alignment.md` if docs.rs URLs change.
- Run `mdbook build` and fix warnings before publish.
- Deploy: ensure GitHub Actions `mdbook` workflow runs clean for both contributor and dissection books.

## Quick commands
- `tools/extract_for_book.sh` — crate list, module tree, pub items.
- `mdbook build docs/dissection_book` — build check.
- `tools/book_tasks.sh build` — shortcut to build.
