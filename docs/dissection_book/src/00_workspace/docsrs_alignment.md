# docs.rs Alignment

How this mdBook complements auto-generated docs.rs pages and stays current with releases.

## Purpose
- mdBook focuses on architecture, flows, design rationale, and curated examples across crates.
- docs.rs provides API-level reference (types/functions) generated from each crate’s docs.
- Together: use docs.rs for exact signatures and exhaustive items; use this book for how the pieces fit, constraints, and gotchas.

## Linking strategy
- For API details, link to docs.rs pages per crate (e.g., `https://docs.rs/cortenforge-sim-core/<version>/`).
- When referencing specific items, prefer source links (`crate/module.rs:L123`) per the linking style guide to keep context tied to this repo’s version.
- Add docs.rs links in crate overview pages where appropriate (e.g., top of each crate section).

## Keeping in sync with releases
- On each release/tag:
  - Update crate versions in docs.rs links if necessary.
  - Re-run `mdbook build` and skim for stale references (features, flows, config defaults).
  - Update `integration_contracts.md` if schemas/interfaces change.
- For breaking changes:
  - Add migration notes in `migration.md` or crate-specific design reviews.
  - Note deprecations and new features in crate overview pages and examples.

## Scope boundaries
- mdBook intentionally omits auto-generated item listings; defer to docs.rs for exhaustive APIs.
- Keep mdBook code snippets runnable/minimal; point to docs.rs for full API surface and trait bounds.
