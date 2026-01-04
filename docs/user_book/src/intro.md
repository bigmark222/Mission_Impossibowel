# Introduction

Welcome to the CortenForge user guide. This book focuses on the shared substrate crates (`sim_core`, `vision_core`/`vision_runtime`, `data_contracts`, `models`, `training`, `inference`, `capture_utils`, `colon_sim_tools`). The `colon_sim` app that used to serve as the worked example now lives in its own repository; app-specific commands reside there: https://github.com/via-balaena/Deep-Poo

Crates on crates.io (version `0.1.0`): `cortenforge-sim-core`, `cortenforge-vision-core`, `cortenforge-vision-runtime`, `cortenforge-data-contracts`, `cortenforge-capture-utils`, `cortenforge-models`, `cortenforge-training`, `cortenforge-inference`, `cortenforge-cli-support`, `cortenforge-burn-dataset`. Feature flags:
- Training/inference: `backend-wgpu` for GPU; defaults to NdArray.
- Inference: `tinydet`/`bigdet` features.
- Tools: `scheduler`, `tui`, `gpu_nvidia` (tools crate not published by default).
- MSRV: Rust 1.75+. `burn-core` is currently patched to a vendored 0.14.0; patch will be dropped when upstream fixes it.

Build this book:
```bash
mdbook build docs/user_book
```

What you’ll find:
- Happy-path walkthrough (capture → ETL → train → infer) using the shared crates and tooling.
- Minimal usage for capture (headless wrappers), ETL, training, inference, and tools.
- FAQ/Troubleshooting for common questions.

Prereqs:
- Rust + Cargo installed.
- GPU optional: defaults work on CPU/Metal/NDArray; WGPU paths are opt-in via features/env.
- Build docs: `mdbook build docs/user_book`
