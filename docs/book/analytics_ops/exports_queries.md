# Exports (Parquet) and queries

## Export manifest summaries
```bash
cargo run -p colon_sim_tools --bin warehouse_export -- \
  --manifest artifacts/tensor_warehouse/v<version>/manifest.json \
  --out logs/warehouse_summary.parquet
```

## Quick queries
- Polars (Rust):
```rust
let lf = polars::prelude::LazyFrame::scan_parquet(
    "logs/warehouse_summary.parquet",
    Default::default()
)?; // add filters/projections as needed
```
- DataFusion (SQL):
```rust
let ctx = datafusion::prelude::SessionContext::new();
ctx.register_parquet("summary", "logs/warehouse_summary.parquet", Default::default()).await?;
let df = ctx.sql("SELECT * FROM summary LIMIT 5").await?;
df.show().await?;
```

## Log locations
- ETL logs: stdout/stderr from `warehouse_etl` (`cargo run -p colon_sim_tools --bin warehouse_etl ...`); optional trace if enabled.
- Training logs: `logs/train_status.json` (if `--status-file` set) plus stdout/stderr.
- Export logs: stdout/stderr from `warehouse_export` (`cargo run -p colon_sim_tools --bin warehouse_export ...`).

## Operational checks
- Verify manifest + shard checksums before training.
- Confirm WGPU envs match target GPU/backend; check adapter name when multiple GPUs exist.
- Spot-check summary stats: sample counts, class balance, invalid/missing/empty counts.
- Storage health: ensure output roots have free space and reasonable IO throughput.

## Escalation steps
- Capture failing command, manifest version, env vars, and log snippet.
- Reproduce on a small subset or single shard to isolate.
- Roll back to last known-good manifest if training blockers persist.
- File an issue with logs, manifest version, and hardware details (GPU/driver/OS).
