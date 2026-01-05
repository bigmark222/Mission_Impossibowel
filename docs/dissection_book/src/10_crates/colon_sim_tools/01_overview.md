# colon_sim_tools (shared): Overview

## Problem statement
Bundle tooling bins and helpers for capture/overlay/ETL/export and simple inference, shared across apps. Currently contains both shared and app-specific bins; plan to split app-facing pieces into app repos and keep only shared utilities here.

## Scope
- Bins: overlay_labels, prune_empty, warehouse_etl/export/cmd, single_infer (shared-ish); app-facing bins gated by features (datagen_scheduler, tui, gpu_macos_helper, datagen).
- Shared helpers: CLI services and warehouse commands in `services` / `warehouse_commands` (only live here today).
- Uses substrate crates: capture_utils, data_contracts, vision_core, inference/models, cli_support, burn_dataset.

## Non-goals
- No app-specific world/config baked into shared bins; app-facing bins should move to app repos.
- No recorder/meta/world definitions; uses shared schemas/helpers.

## Who should use it
- Consumers needing CLI tooling for captures/warehouse/inference without app-specific logic.
- Contributors trimming/splitting the crate: move app-only bins out, fold shared helpers into existing crates (cli_support/capture_utils) and keep a thin bin crate.
