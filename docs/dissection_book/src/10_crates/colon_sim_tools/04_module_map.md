# colon_sim_tools (shared): Module Map

- `lib.rs`: Exposes modules and re-exports capture_utils.
- `overlay`: Re-exports overlay helpers from vision_core.
- `recorder`: Re-exports recorder helpers from capture_utils (JsonRecorder, generate_overlays, prune_run).
- `services`: Shared CLI/service helpers (RunManifestSummary, RunInfo, ServiceCommand, DatagenOptions, TrainOptions, list_runs, spawn, datagen_command, train_command, read_metrics/logs/status).
- `warehouse_commands`: Common/Builder submodules for warehouse command generation (WarehouseStore, ModelKind, CmdConfig, DEFAULT_CONFIG, Shell, build_command).
- `bin/`: Binaries (overlay_labels, prune_empty, warehouse_etl/export/cmd, single_infer; app-gated bins: datagen, datagen_scheduler, tui, gpu_macos_helper).

Cross-module dependencies: services/warehouse_commands use cli_support and substrate crates; overlay/recorder wrap vision_core/capture_utils; bins wire these helpers into CLIs.
