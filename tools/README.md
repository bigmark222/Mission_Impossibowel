# colon_sim_tools

This crate hosts CLI utilities that are not part of the core runtime:
- Data capture helpers: `overlay_labels`, `prune_empty`, `datagen_scheduler`.
- Warehouse tooling: `warehouse_etl`, `warehouse_export`, `warehouse_cmd` (with shared builder/common under `src/warehouse_commands`).
- Optional TUI: `tui` (if enabled).

Run via `cargo run -p colon_sim_tools --bin <tool> -- ...`. See docs/book/reference/cli_api.md for flag details.
