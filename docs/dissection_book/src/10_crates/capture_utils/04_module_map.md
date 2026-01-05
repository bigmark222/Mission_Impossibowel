# capture_utils: Module Map

- `lib.rs`: Single-module crate containing:
  - `JsonRecorder`: Default JSON recorder sink implementing `vision_core::Recorder` for labels.
  - `generate_overlays`: Helper to render overlays from label JSONs in a run dir.
  - `prune_run`: Helper to prune empty-label frames into a filtered run.
  - Local helper `draw_rect` for overlay rendering.

Cross-module dependencies: uses `data_contracts` for schemas, `vision_core` overlay helpers via capture_utils re-export, and standard image/fs utilities. Consumed by runtime/apps/tools.
