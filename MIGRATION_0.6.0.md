# Migration Guide: 0.5.1 → 0.6.0

This guide provides detailed instructions for upgrading from CortenForge 0.5.1 to 0.6.0.

Version 0.6.0 includes extensive breaking changes focused on terminology standardization, API cleanup, and architectural improvements. These changes make the codebase more maintainable and domain-agnostic.

## Table of Contents

1. [Quick Migration Checklist](#quick-migration-checklist)
2. [Terminology Changes](#terminology-changes)
3. [Type Renames](#type-renames)
4. [Module Reorganization](#module-reorganization)
5. [Feature Flag Changes](#feature-flag-changes)
6. [Dependency Architecture](#dependency-architecture)
7. [Plugin Naming](#plugin-naming)
8. [Naming Conventions](#naming-conventions)

## Quick Migration Checklist

Use this checklist to track your migration progress:

- [ ] Replace `PolypLabel` with `DetectionLabel`
- [ ] Update probe terminology to articulated_instrument
- [ ] Rename FrontCamera types to PrimaryCapture types
- [ ] Update TinyDet/BigDet to LinearDetector/ConvolutionalDetector
- [ ] Rename `polyp_seed` field to `label_seed`
- [ ] Remove tinydet/bigdet feature flags
- [ ] Update Bevy app integration (no Bevy in inference crate)
- [ ] Update prelude imports to canonical modules
- [ ] Update InferencePlugin to InferenceRuntimePlugin
- [ ] Update recorder imports to sim_core::recorder
- [ ] Standardize Config suffix on custom types
- [ ] Test and verify all changes

## Terminology Changes

### PolypLabel → DetectionLabel

**Why**: Domain-agnostic terminology makes the stack reusable beyond medical applications.

**Affected crates**: `data_contracts`, `capture_utils`, `burn_dataset`, `training`, `vision_core`, `vision_runtime`

**Before**:
```rust
use data_contracts::PolypLabel;

let label = PolypLabel {
    frame_id: 42,
    box_xywh: [0.5, 0.5, 0.1, 0.1],
    source: LabelSource::Ground,
};
```

**After**:
```rust
use data_contracts::DetectionLabel;

let label = DetectionLabel {
    frame_id: 42,
    box_xywh: [0.5, 0.5, 0.1, 0.1],
    source: LabelSource::Ground,
};
```

**Migration steps**:
1. Find all `PolypLabel` references: `rg "PolypLabel" --type rust`
2. Replace with `DetectionLabel`
3. Update imports: `use data_contracts::DetectionLabel;`

### probe → articulated_instrument

**Why**: More precise terminology for flexible robotic instruments.

**Affected crates**: `sim_core`, documentation

**Before**:
```rust
// Documentation and comments
"The probe navigates the tunnel..."
```

**After**:
```rust
// Documentation and comments
"The articulated instrument navigates the tunnel..."
```

**Migration steps**:
1. Search for "probe" in documentation and comments
2. Replace with "articulated instrument" or "articulated_instrument" in code
3. This is primarily a documentation change; code symbols remain similar

### polyp_seed → label_seed

**Why**: Aligns with DetectionLabel terminology for domain-agnostic usage.

**Affected crates**: `vision_core`, `capture_utils`, all recorder implementations

**Before**:
```rust
use vision_core::FrameRecord;

let record = FrameRecord {
    frame: my_frame,
    labels: &my_labels,
    camera_active: true,
    polyp_seed: 12345, // OLD
};
```

**After**:
```rust
use vision_core::FrameRecord;

let record = FrameRecord {
    frame: my_frame,
    labels: &my_labels,
    camera_active: true,
    label_seed: 12345, // NEW
};
```

**Migration steps**:
1. Find all `polyp_seed` field accesses: `rg "polyp_seed" --type rust`
2. Replace with `label_seed`
3. Update any custom recorder implementations

## Type Renames

### FrontCamera → PrimaryCapture

**Why**: Clearer terminology that doesn't assume camera position.

**Affected crates**: `vision_core`, `vision_runtime`, `sim_core`

**Type mapping**:
- `FrontCamera` → `PrimaryCaptureCamera`
- `FrontCameraTarget` → `PrimaryCaptureTarget`
- `FrontCameraReadback` → `PrimaryCaptureReadback`

**Before**:
```rust
use vision_core::capture::{FrontCamera, FrontCameraTarget};

app.insert_resource(FrontCamera::default());
```

**After**:
```rust
use vision_core::capture::{PrimaryCaptureCamera, PrimaryCaptureTarget};

app.insert_resource(PrimaryCaptureCamera::default());
```

**Migration steps**:
1. Find all FrontCamera references: `rg "FrontCamera" --type rust`
2. Replace systematically:
   - `FrontCamera` → `PrimaryCaptureCamera`
   - `FrontCameraTarget` → `PrimaryCaptureTarget`
   - `FrontCameraReadback` → `PrimaryCaptureReadback`
3. Update imports from `vision_core::capture`

### TinyDet/BigDet → LinearDetector/ConvolutionalDetector

**Why**: Architecture-descriptive names instead of informal nicknames.

**Affected crates**: `inference`

**Before**:
```rust
use inference::TinyDet;

let detector = TinyDet::load("model.bin")?;
```

**After**:
```rust
use inference::LinearDetector;

let detector = LinearDetector::load("model.bin")?;
```

**Type mapping**:
- `TinyDet` → `LinearDetector`
- `BigDet` → `ConvolutionalDetector`

**Migration steps**:
1. Find detector type references: `rg "TinyDet|BigDet" --type rust`
2. Replace with architectural names
3. Update checkpoint loading code
4. Update any documentation or comments

## Module Reorganization

### Recorder Module Consolidation

**Why**: Centralized recorder types for easier discovery.

**Before**:
```rust
// Types scattered across multiple modules
use sim_core::some_module::RecorderType;
use capture_utils::another::AnotherRecorder;
```

**After**:
```rust
// All recorder types in one module
use sim_core::recorder::{RecorderType, AnotherRecorder};
```

**Migration steps**:
1. Find recorder imports: `rg "use.*Recorder" --type rust`
2. Update to import from `sim_core::recorder::*`
3. Check that all recorder types are accessible

### Prelude Re-export Removal

**Why**: Explicit imports over implicit re-exports for clarity.

**Before**:
```rust
// Using legacy prelude re-exports
use some_crate::prelude::*;
```

**After**:
```rust
// Explicit imports from canonical modules
use some_crate::types::SpecificType;
use some_crate::interfaces::Detector;
```

**Migration steps**:
1. Identify prelude wildcard imports: `rg "use.*prelude::\*" --type rust`
2. Replace with explicit imports from source modules
3. Check compiler errors for missing imports
4. Add specific imports as needed

## Feature Flag Changes

### Removed: tinydet/bigdet Features

**Why**: Model selection should be via runtime API, not compile-time features.

**Before** (Cargo.toml):
```toml
[dependencies]
cortenforge-inference = { version = "0.5.1", features = ["tinydet"] }
```

**After** (Cargo.toml):
```toml
[dependencies]
cortenforge-inference = { version = "0.6.0" }
```

**Before** (code):
```rust
// Feature-gated model selection
#[cfg(feature = "tinydet")]
let detector = load_tinydet()?;
```

**After** (code):
```rust
// Runtime model selection via checkpoint path or API
let detector = LinearDetector::load("path/to/linear_model.bin")?;
// or
let detector = ConvolutionalDetector::load("path/to/conv_model.bin")?;
```

**Migration steps**:
1. Remove `tinydet` and `bigdet` from feature lists in Cargo.toml
2. Replace feature-gated code with runtime model selection
3. Use checkpoint paths or factory methods to choose detector type

## Dependency Architecture

### Bevy Removed from Inference Crate

**Why**: Lighter weight inference deployments without game engine dependencies.

**Affected**: Applications using `cortenforge-inference` directly

**Before**:
```toml
[dependencies]
# Inference pulled in Bevy transitively
cortenforge-inference = "0.5.1"
```

**After**:
```toml
[dependencies]
# Inference is Bevy-free
cortenforge-inference = "0.6.0"

# Use vision_runtime for Bevy integration
cortenforge-vision-runtime = "0.6.0"  # If you need Bevy plugins
```

**Migration steps**:
1. If using inference in non-Bevy context: No changes needed
2. If using inference in Bevy app: Add `cortenforge-vision-runtime` and use plugins
3. Update detector instantiation to use appropriate factory

**Bevy integration example**:
```rust
use vision_runtime::InferenceRuntimePlugin;

fn main() {
    App::new()
        .add_plugins(InferenceRuntimePlugin::default())
        .run();
}
```

## Plugin Naming

### InferencePlugin → InferenceRuntimePlugin

**Why**: Clarifies this is the runtime/Bevy-specific plugin.

**Before**:
```rust
use vision_runtime::InferencePlugin;

app.add_plugins(InferencePlugin::default());
```

**After**:
```rust
use vision_runtime::InferenceRuntimePlugin;

app.add_plugins(InferenceRuntimePlugin::default());
```

**Migration steps**:
1. Find plugin references: `rg "InferencePlugin" --type rust`
2. Replace with `InferenceRuntimePlugin`
3. Update imports

### Burn Prefix Clarification

**Why**: Consistently prefix Burn-specific types.

**Before**:
```rust
use vision_runtime::DetectorFactory;
```

**After**:
```rust
use vision_runtime::BurnDetectorFactory;
```

**Migration steps**:
1. Find detector factory references in vision_runtime
2. Add `Burn` prefix to factory types
3. This primarily affects advanced integrations

## Naming Conventions

### Config Suffix Standardization

**Why**: Consistent naming across the workspace.

**Before**:
```rust
struct MyDetectorParams {
    hidden: usize,
}

struct OtherOptions {
    depth: usize,
}
```

**After**:
```rust
struct MyDetectorConfig {
    hidden: usize,
}

struct OtherConfig {
    depth: usize,
}
```

**Migration steps**:
1. Review custom configuration types
2. Rename types ending in `Params`, `Options`, `Settings` to use `Config` suffix
3. Update constructor calls

### Abbreviation Standardization

**Why**: Consistent parameter naming in public APIs.

**Common changes**:
- `cfg` → `config`
- `rect` → `rectangle`
- `img` → `image`

**Before**:
```rust
fn process(cfg: &MyConfig, img: &Image) { }
```

**After**:
```rust
fn process(config: &MyConfig, image: &Image) { }
```

**Migration steps**:
1. This primarily affects function signatures
2. Update function calls if parameter names changed
3. Most changes are internal; check compiler errors

### State Suffix Semantics

**Why**: Types ending in `State` consistently represent mutable runtime state.

**Migration steps**:
1. Review custom types ending in `State`
2. Ensure they represent mutable runtime state
3. Rename if they're actually configuration or static data
4. This is mostly an internal convention

## Testing Your Migration

After completing the migration, verify everything works:

```bash
# 1. Format code
cargo fmt --all

# 2. Check for compiler errors
cargo check --workspace --all-targets

# 3. Run clippy
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 4. Run tests
cargo test --workspace --locked

# 5. Build documentation
cargo doc --workspace --all-features --no-deps
```

## Getting Help

If you encounter issues during migration:

1. Check the [CHANGELOG.md](./CHANGELOG.md) for detailed breaking changes
2. Review the [documentation book](https://via-balaena.github.io/CortenForge/)
3. Check type definitions in the source code with `rg "struct TypeName" --type rust`
4. Open an issue at https://github.com/via-balaena/CortenForge/issues

## Summary

The 0.6.0 release focuses on:
- **Terminology standardization** for domain-agnostic usage
- **Module consolidation** for better organization
- **Naming conventions** for consistency
- **Dependency cleanup** for lighter deployments

While this release includes many breaking changes, they improve maintainability and make the codebase more accessible to new users. Most migrations involve straightforward find-and-replace operations.

Thank you for upgrading to CortenForge 0.6.0!
