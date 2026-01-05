# cortenforge (umbrella): Module Map

- `lib.rs`: Single-module facade; re-exports member crates: sim_core, vision_core, vision_runtime, data_contracts, capture_utils, models, inference, training, burn_dataset, cli_support, tools.
- No additional submodules; feature wiring aligns with member crates.

Cross-module dependencies: none internally; this crate is purely a facade delegating to underlying crates.
