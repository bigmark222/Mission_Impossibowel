# Dev Guide

For developers working on the data/training pipeline: setup, standards (fmt/clippy/tests), how to extend schemas/warehouse, and where to find key code paths. Update docs whenever interfaces change.

## Crate map (where code lives)
- Root (`colon_sim`): orchestration/CLI glue (`cli.rs`, `common_cli.rs`, `seed.rs`, `run_app`); no domain systems.
- App (`apps/colon_sim`): reference world/entities, HUD, controls/autopilot hooks.
- Core: `sim_core` (Bevy plumbing), `vision_core`/`vision_runtime` (detector interfaces + capture/inference plugins), `models` (TinyDet/BigDet).
- Training/Inference: `training` (loop/CLI), `inference` (Burn-backed detector factory).
- Tools: `colon_sim_tools` (overlay/prune/warehouse/datagen/scheduler/tui) plus shared helpers.
