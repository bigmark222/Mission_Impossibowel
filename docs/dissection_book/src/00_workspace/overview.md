# Workspace overview

## Workspace members
- crates/cli_support (lib) — shared CLI argument helpers.
- crates/cortenforge (lib) — umbrella re-export crate with feature wiring.
- crates/burn_dataset (lib) — dataset loading/splitting for Burn.
- tools/colon_sim_tools (lib + bins) — tooling crate (overlay/prune/etl/export/cmd/single_infer; app-facing bins gated by features).
- capture_utils (lib) — recorder sinks and capture helpers.
- sim_core (lib) — Bevy runtime scaffolding, hooks, recorder types.
- vision_core (lib) — vision interfaces and overlay math.
- vision_runtime (lib) — capture/inference plugins for Bevy.
- data_contracts (lib) — schemas/validation for captures/warehouse.
- training (lib + bins) — Burn training/eval CLI.
- inference (lib) — detector factory (Burn-backed or heuristic).
- models (lib) — TinyDet/BigDet definitions.

## Workspace metadata (root Cargo.toml)
- Resolver: 2
- Patch overrides: vendored burn-core 0.14.0; local paths for all cortenforge crates (temporary until burn-core is fixed upstream).
- Publish status: most crates publishable; `colon_sim_tools` marked publish = false.

## Shared dependency themes
- Burn 0.14 (with local burn-core patch) across models/training/inference.
- serde/serde_json, anyhow/thiserror for errors.
- bevy for sim_core/vision_runtime and some tools.
- clap for CLIs; cli_support reused by tools.
- image/png, rayon for capture/tools; arrow/parquet in tools.

## Feature flags strategy
- Defaults lean: NdArray backend; GPU/WGPU behind features (`backend-wgpu`, `gpu_nvidia` in tools).
- Model variants: `tinydet`/`bigdet` in models/training/inference.
- Tools: `tui`, `scheduler`, `gpu_nvidia` gate app-specific/heavier bins.
- Umbrella crate re-exports features (e.g., sim-core, vision-core/runtime, training/inference stacks).

## Notes
- Current blocker: burn-core 0.14.0 on crates.io resolves bincode 2.0.1; workspace vendors burn-core until upstream publishes a fixed release.
- colon_sim_tools contains shared and app-specific bins; plan to split/trim and possibly fold shared helpers into existing crates.
