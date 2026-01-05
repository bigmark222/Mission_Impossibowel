# Linking Style

Standardize source links to keep references consistent and stable.

## Format
- Use repo-relative paths with line anchors: ``crate/path/to/file.rs:L123``.
- Examples:
  - `sim_core/src/hooks.rs:L10`
  - `vision_runtime/src/lib.rs:L75`
  - `docs/contributor_book/src/architecture.md:L42` (for cross-book refs)
- Avoid range anchors; link to the first relevant line.

## When to link
- Point to source when explaining specific functions/types or design decisions.
- Prefer linking to code over docs.rs when tying to this repo’s version.
- For public API reference, optionally include a docs.rs link alongside the source link.

## Maintenance
- When code moves, update links in the same PR as the code change.
- Keep links in crate pages and flow docs current after refactors.
- If line numbers are likely to drift, consider linking to the symbol in docs.rs as a secondary reference.
