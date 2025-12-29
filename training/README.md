# training

Burn-based training/eval for TinyDet.

Contents
- `model`: TinyDet + config.
- `dataset`: DatasetConfig, RunSample, collate stub (uses data_contracts metadata).
- `util`: TrainArgs/run_train stub (NdArray backend by default; backend-wgpu optional).
- `bin/train`: Thin CLI that delegates to run_train.
- `bin/eval`: Stub eval CLI.

Notes
- Checkpoint loading is stubbed; implement in util.rs when ready.
- collate currently returns dummy tensors; hook up real image loading when adding training.
