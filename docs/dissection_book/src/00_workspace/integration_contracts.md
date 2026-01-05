# Integration Contracts

Key assumptions and contracts between crates. Keep these in sync as APIs evolve.

## Types and data shapes
- `vision_core::interfaces::{Frame, DetectionResult, Label, FrameRecord}`: Shared across `vision_runtime`, `inference`, `capture_utils`, and recorders. Changes here propagate everywhere.
- `data_contracts::capture::{CaptureMetadata, PolypLabel}`: Recorder outputs and dataset loaders assume this schema. Validators and ETL rely on fields being stable.
- `burn_dataset` tensors: `BatchIter`/`collate` expect consistent image dimensions per batch and `max_boxes` alignment with `models`/`training`.
- `models` output: `TinyDet`/`BigDet` scores align with `max_boxes`; inference and training must agree on this.

## Feature expectations
- Backends: default NdArray; enable `backend-wgpu` in `models`, `training`, `inference` to use GPU.
- Models: `bigdet` feature switches inference model type; downstream code should not assume TinyDet dimensions when `bigdet` is on.
- `vision_runtime` assumes detectors are `Send + Sync`; `InferenceFactory` supplies such detectors.
- `colon_sim_tools` bins: features `tui`, `scheduler`, `gpu_nvidia` gate heavy deps; default footprint is minimal.

## Runtime assumptions
- `vision_runtime::CapturePlugin` runs only in modes `Datagen`/`Inference` (from `sim_core::SimRunMode`); app must set the mode.
- `vision_runtime` inference loop assumes a single detector; async tasks swap detector instances without locks (uses ownership passing).
- Recorders and datasets assume captures are stored under `run_dir` with `images/`, `labels/`, optional `overlays/`.
- `burn_dataset::BatchIter` expects consistent image sizes unless `target_size` is set to force resizing.
- Validation thresholds/env vars (`BURN_DATASET_*`, `WAREHOUSE_*`) affect permissive behavior; CI/publish should set these explicitly.
- `colon_sim_tools` is not published; app-specific bins should live in app repos. Shared helpers may migrate into other crates.

## Error/compatibility expectations
- Inference may fall back to heuristic detector if checkpoints fail to load; consumers should surface this state (e.g., overlay fallback message).
- Dataset loaders return `BurnDatasetError`; training/ETL should decide whether to skip (permissive) or fail-fast.
- Changes to schemas (`data_contracts`) or interfaces (`vision_core`) require coordinated updates across recorders, loaders, and runtime.
