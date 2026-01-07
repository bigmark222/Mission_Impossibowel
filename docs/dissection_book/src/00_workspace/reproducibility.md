# Reproducibility

## Lockfile policy
- Use the workspace `Cargo.lock` for local builds/tests; enforce with `--locked` before publish.
- Note: published crates resolve without a lockfile; burn-core 0.14.1 fixes the prior bincode break.

## MSRV
- Target Rust 1.75+ across crates (umbrella uses 2024 edition).
- Keep MSRV aligned in docs/metadata; bump intentionally.

## CI expectations
- fmt: `cargo fmt -- --check`
- clippy: `cargo clippy --workspace --all-targets --all-features -D warnings`
- tests: `cargo test --workspace --locked`
- optional: `cargo deny check`, `cargo hakari generate && cargo hakari manage-deps`
- docs: `mdbook build docs/contributor_book && mdbook build docs/dissection_book`; `mdbook test` for doctests.

## Deterministic builds
- Use `--locked` to pin deps; avoid adding `path` patches except for local dev needs.
- Document feature sets used in builds (NdArray default; GPU/WGPU opt-in).
- Avoid network fetches in tests; keep fixtures small and included.
