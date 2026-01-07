# burn_dataset: Module Map

- `lib.rs`: Single-module crate defining dataset types (DatasetConfig, SampleIndex, DatasetSample), transforms, summaries/validation, shard metadata/manifest, iterators (BatchIter, WarehouseBatchIter), and helper functions (index/summarize/load/split/build iterators).

Cross-module dependencies: pure Rust with Burn/backends; consumed by training/tools; no submodules beyond lib.rs.

## Links
- Source: `crates/burn_dataset/src/lib.rs`
