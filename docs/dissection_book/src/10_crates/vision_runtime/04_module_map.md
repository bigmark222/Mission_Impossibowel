# vision_runtime: Module Map

- `lib.rs`: Houses capture and inference pipeline types/functions and Bevy plugins (`CapturePlugin`, `InferencePlugin`). Contains resources (FrontCamera*, DetectorHandle, thresholds) and systems for capture/readback/inference scheduling, overlay state, hotkeys, polling.
- `prelude`: Re-export of commonly used types from lib.rs.

Cross-module dependencies: single-module crate; relies on vision_core types, inference detectors, and sim_core/Bevy app context.
