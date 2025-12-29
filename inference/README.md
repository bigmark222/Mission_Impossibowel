# inference crate

- Purpose: provide the Burn-backed detector factory and inference plugin used by Bevy apps (sim_view/inference_view).
- Backend: defaults to `backend-ndarray`; enable `--features backend-wgpu` for WGPU. Needs `burn` features enabled in the root build if you want GPU.
- Model: loads `TinyDet` from the shared `models` crate via `BinFileRecorder` (full precision). Pass a weights path to the factory to load a checkpoint; otherwise it falls back to a heuristic detector.
- Use: `run_app` (root crate) inserts the detector built by `inference::InferenceFactory` when mode==Inference. Ensure the checkpoint exists and matches the model config.
- TODO: add a smoke test invoking the factory with a tiny checkpoint once available.
