# Crate Index

| Crate | Version | Path | Type | Edition | Notes |
| ----- | ------- | ---- | ---- | ------- | ----- |
| cortenforge-cli-support | 0.1.1 | crates/cli_support | lib | 2021 | Shared CLI args/helpers; optional Bevy feature |
| cortenforge | 0.1.1 | crates/cortenforge | lib | 2024 | Umbrella re-export; feature wiring |
| cortenforge-burn-dataset | 0.1.1 | crates/burn_dataset | lib | 2021 | Burn dataset loading/splitting |
| colon_sim_tools | 0.1.1 | tools | lib + bins | 2021 | Tooling crate; bins include overlay/prune/etl/export/cmd/single_infer; app-facing bins gated by features |
| cortenforge-capture-utils | 0.1.1 | capture_utils | lib | 2021 | Recorder sinks and capture helpers |
| cortenforge-sim-core | 0.1.1 | sim_core | lib | 2021 | Bevy runtime scaffolding, hooks, recorder types |
| cortenforge-vision-core | 0.1.1 | vision_core | lib | 2021 | Vision interfaces, overlay math |
| cortenforge-vision-runtime | 0.1.1 | vision_runtime | lib | 2021 | Capture/inference plugins for Bevy |
| cortenforge-data-contracts | 0.1.1 | data_contracts | lib | 2021 | Schemas/validation for captures/warehouse |
| cortenforge-training | 0.1.1 | training | lib + bins | 2021 | Burn training/eval CLI (train/eval bins) |
| cortenforge-inference | 0.1.1 | inference | lib | 2021 | Detector factory (Burn-backed/heuristic) |
| cortenforge-models | 0.1.1 | models | lib | 2021 | TinyDet/BigDet definitions |

*Default features:* NdArray backends enabled by default on training/inference/models; GPU/WGPU behind `backend-wgpu`. Tools features gate TUI/scheduler/GPU. Umbrella crate re-exports features; burn-core is temporarily patched to vendored 0.14.0.
