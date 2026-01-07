# Troubleshooting

Common pitfalls and fixes when building/publishing with the substrate.

## Resolved issues
- See [Resolved issues](resolved_issues.md) for historical fixes that are no longer active.

## Burn-core / bincode publish failure
- Status: resolved in `burn-core 0.14.1`.
- If you see this error: update to `burn-core 0.14.1` (or newer), refresh the lockfile, and retry publish.

## Cargo.lock masking issues
- Libraries don’t ship lockfiles; `cargo publish` re-resolves deps. If a build works only with `Cargo.lock`, reproduce without it to mirror publish behavior.

## GPU/WGPU issues
- Symptom: WGPU init failures or GPU-only deps failing CI.
- Fix: gate GPU paths behind features (`backend-wgpu`, `gpu_nvidia`), default to NdArray; skip GPU tests on non-GPU runners. Provide a manual repro command with the feature flags.

## CLI/tool failures
- Verify schemas: ensure outputs still match `data_contracts`.
- Run minimal smoke: `overlay_labels`, `prune_empty`, `warehouse_etl` on small fixtures.
- Keep feature flags minimal in tests; only enable `tui`/`scheduler`/`gpu_nvidia` when needed.

## Recorder issues
- Missing meta/world state: ensure app inserts `RecorderMetaProvider` and updates `RecorderWorldState`.
- Custom sinks: verify they implement `RecorderSink` and preserve schema compatibility for ETL/training.

## Documentation build
- If docs fail: `mdbook build docs/contributor_book`; install `mdbook-mermaid` (`cargo install mdbook-mermaid`) for Mermaid diagrams.

## Debugging core paths (quick pointers)
- Recorder failures: log meta/world state values; validate against `data_contracts`; ensure sinks are registered.
- Runtime hangs: check mode gating (`SimRunMode`/`ModeSet`), system ordering, and that plugins are added.
- Dataset issues: run `warehouse_etl` on a tiny capture; validate manifests with `data_contracts`; inspect schema versions.
