# Dependency Analysis

**Date:** 2026-01-13
**Branch:** hacky-crates-publishing-fix

## Summary

This document captures the dependency structure of the CortenForge workspace, both internal (workspace crates) and external (third-party dependencies).

## Workspace Internal Dependencies

### Dependency Graph (Our Crates Only)

```
cortenforge (umbrella)
├── sim_core
├── vision_core
├── vision_runtime
│   ├── sim_core
│   └── vision_core
├── data_contracts
├── capture_utils
│   ├── data_contracts
│   └── vision_core
├── models
├── training
│   ├── burn_dataset
│   ├── data_contracts
│   └── models
├── inference
│   ├── data_contracts
│   ├── models
│   ├── sim_core
│   └── vision_core
└── burn_dataset
    └── data_contracts

cli_support (standalone)

cortenforge-tools (standalone)
├── burn_dataset
├── capture_utils
├── cli_support
├── data_contracts
├── inference
└── vision_core
```

### Dependency Analysis

**Foundation crates (no internal deps):**
- `data_contracts`: Pure schema definitions
- `vision_core`: Core vision interfaces
- `cli_support`: CLI argument parsing

**Single-dependency crates:**
- `models`: No internal deps (pure model definitions)
- `sim_core`: Depends on `vision_core`
- `burn_dataset`: Depends on `data_contracts`

**Multi-dependency crates:**
- `capture_utils`: Depends on `data_contracts`, `vision_core`
- `vision_runtime`: Depends on `sim_core`, `vision_core`
- `training`: Depends on `burn_dataset`, `data_contracts`, `models`
- `inference`: Depends on `data_contracts`, `models`, `sim_core`, `vision_core`

**High-dependency crates:**
- `cortenforge-tools`: Depends on 6 workspace crates (burn_dataset, capture_utils, cli_support, data_contracts, inference, vision_core)
- `cortenforge` (umbrella): Re-exports 9 core crates

### Observations

1. **Good separation of concerns**: Foundation crates (data_contracts, vision_core) have no internal dependencies
2. **Clear layering**: Dependencies flow from higher-level crates to lower-level foundations
3. **No circular dependencies**: Clean acyclic dependency graph
4. **Potential issue**: `inference` crate depends on `sim_core` (runtime dependency in a library crate - to investigate in H3)

## External Dependencies

### Key Third-Party Dependencies

**Machine Learning:**
- `burn` (v0.19.1): Core ML framework for training/inference
- `burn-ndarray` (v0.19.1): CPU backend
- `burn-wgpu` (v0.19.1): GPU backend (optional, feature-gated)

**Data Processing:**
- `arrow` (v57.1.0): In-memory columnar data
- `parquet` (v57.1.0): Parquet file format
- `serde` (v1.0.228): Serialization framework
- `bincode` (v2.0.1): Binary serialization

**Runtime/Simulation:**
- `bevy` (v0.17.3): Game engine for simulation runtime
- `image` (v0.25.2): Image processing

**CLI/Tools:**
- `clap` (v4.4.18): Command-line argument parsing
- `ratatui` (v0.30.0): Terminal UI framework
- `sysinfo` (v0.37.2): System information (GPU probing)

### Dependency Health

**Recent updates (completed in v0.2.0):**
- Burn: 0.14.1 → 0.19.1 (major update)
- Arrow/Parquet: ~v53 → v57.1.0
- bincode: v1.x → v2.0.1
- sysinfo: v0.30.x → v0.37.2
- ratatui: v0.27.x → v0.30.0

**Deferred updates:**
- bincode 3.x: Currently 3.0.0 on crates.io is a stub/placeholder

**Removed patches:**
- Previously vendored burn-core (now using official 0.19.1 release)

## Files Generated

- `docs/dependency_tree.txt`: Full dependency tree (all transitive deps, ~thousands of lines)
- `docs/workspace_deps_summary.txt`: Workspace crates only (depth=1)

## Usage

To regenerate these files:

```bash
# Full dependency tree
cargo tree --workspace --no-dedupe -e normal --prefix depth > docs/dependency_tree.txt

# Workspace summary
cargo tree --workspace --no-dedupe -e normal --depth 1 --prefix depth | grep "cortenforge" > docs/workspace_deps_summary.txt
```

## Next Steps

- **C1 priority**: Audit Cargo.toml files for version drift (0.5.0 vs 0.5.1)
- **H3 priority**: Investigate inference → sim_core dependency (runtime coupling in library)
- **M4 priority**: Monitor burn 0.20.x releases for future updates
