# Feature Flags

## Workspace-wide themes
- Default to NdArray: training/inference/models enable `backend-ndarray` by default; GPU/WGPU is opt-in via `backend-wgpu`.
- Model variants: `tinydet`/`bigdet` across models/training/inference.
- Tools: `tui`, `scheduler`, `gpu_nvidia` gate app-specific/heavy bins in colon_sim_tools.
- Umbrella crate (`cortenforge`): re-exports features to enable stacks (sim-core, vision-core/runtime, training/inference).
- Burn-core patch: vendored `burn-core 0.14.0` until upstream fixes bincode; drop override once fixed.

## Per-crate highlights
- `models`: features `tinydet` (default), `bigdet`.
- `training`: `backend-ndarray` (default), `backend-wgpu`, `tinydet` (default), `bigdet`.
- `inference`: `backend-ndarray` (default), `backend-wgpu`, `tinydet` (default), `bigdet`.
- `cortenforge` (umbrella): features map to member crates (sim-core, vision-core/runtime, models, training, inference, capture-utils, cli-support, burn-dataset); `burn-runtime`/`burn-wgpu` stacks wire burn deps.
- `colon_sim_tools`: `tui`, `scheduler`, `gpu_nvidia`; defaults are lean (no extra features).
- `cli_support`: optional `bevy`/`bevy-resource` for resource integration.
- `burn_dataset`: `burn-runtime` wires burn + rayon/memmap2/crossbeam; `burn-ndarray`/`burn-wgpu` optional.

## Hygiene guidance
- Keep defaults light (NdArray, no heavy GPU deps) to keep CI fast.
- Gate app-specific or heavy tooling behind explicit features; avoid enabling by default.
- When adding new features, document what they gate and ensure clippy/tests run with and without them as appropriate.
- Drop the burn-core patch override once upstream fixes publish; remove the `[patch.crates-io]` entry then.
