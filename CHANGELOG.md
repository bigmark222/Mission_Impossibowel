# Changelog

All notable changes to the CortenForge workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-01-13

### Breaking Changes

This release includes extensive API cleanup and standardization. See MIGRATION_0.6.0.md for detailed upgrade instructions.

#### Terminology Standardization (C0a-C0g)
- **PolypLabel → DetectionLabel** (C0a): Renamed `PolypLabel` to `DetectionLabel` across all crates for domain-agnostic terminology
  - Affected: `data_contracts`, `capture_utils`, `burn_dataset`, `training`, `vision_core`, `vision_runtime`
  - Migration: Replace all `PolypLabel` references with `DetectionLabel`

- **probe → articulated_instrument** (C0c): Replaced probe terminology with articulated instrument for clarity
  - Affected: `sim_core`, documentation
  - Migration: Update terminology in custom code and configurations

- **FrontCamera → PrimaryCapture** (C0d): Renamed camera/capture terminology for consistency
  - Affected: `vision_core`, `vision_runtime`, `sim_core`
  - Types renamed:
    - `FrontCamera` → `PrimaryCaptureCamera`
    - `FrontCameraTarget` → `PrimaryCaptureTarget`
    - `FrontCameraReadback` → `PrimaryCaptureReadback`
  - Migration: Update all camera component references

- **TinyDet/BigDet → LinearDetector/ConvolutionalDetector** (C0e): Renamed detector types for architectural clarity
  - Affected: `inference` crate
  - Migration: Update detector type references and checkpoint loading code

- **polyp_seed → label_seed** (C0b): Renamed `FrameRecord::polyp_seed` field to `label_seed`
  - Affected: `vision_core`, `capture_utils`, all recorder implementations
  - Migration: Update `FrameRecord` construction and field access

#### Feature Flag Changes (BC-1)
- **Removed legacy feature aliases** `tinydet` and `bigdet` in `cortenforge-inference`
  - Migration: Use model selection via runtime API or checkpoint path instead of features

#### Dependency Architecture (BC-2)
- **Removed Bevy dependency from inference crate** for lighter weight deployment
  - Affected: `cortenforge-inference`
  - Migration: Inference crate now has no Bevy dependency; integrate via `vision_runtime` for Bevy apps

#### Type Consolidation (BC-3, BC-4)
- **Removed backward-compatible prelude re-exports** for cleaner module boundaries
  - Migration: Import types from their canonical modules instead of legacy re-exports

- **Consolidated Label type hierarchy**: Single `DetectionLabel` type in `data_contracts`
  - Removed duplicate/deprecated label types
  - Migration: Use `data_contracts::DetectionLabel` everywhere

#### Plugin Naming (BC-5, BC-6)
- **InferencePlugin → InferenceRuntimePlugin**: Renamed for clarity in Bevy runtime context
  - Affected: `vision_runtime`
  - Migration: Update plugin registration: `app.add_plugins(InferenceRuntimePlugin)`

- **Clarified Burn prefix usage**: Burn-specific types now consistently prefixed
  - Affected: `vision_runtime` detector factory types
  - Migration: Update type imports for Burn-specific implementations

#### Module Consolidation
- **Consolidated recorder types** into `sim_core::recorder` module
  - Previously scattered across multiple modules
  - Migration: Import recorder types from `sim_core::recorder::*`

#### Naming Standardization (C0h-1 through C0h-10)
- **Standardized Config suffix**: All configuration types now use `Config` suffix (not `Params`, `Options`, etc.)
  - Examples: `LinearClassifierConfig`, `MultiboxModelConfig`
  - Migration: Rename custom config types to match pattern

- **Standardized abbreviations** in public APIs: `cfg` → `config`, `rect` → `rectangle`, etc.
  - Affected: Function parameters and struct fields
  - Migration: Update function calls with renamed parameters

- **Clarified State suffix semantics**: Types ending in `State` represent mutable runtime state
  - Internal resource types standardized
  - Migration: Minimal - mostly internal types

- **Standardized detector naming**: Detectors follow pattern `{Architecture}Detector`
  - Examples: `LinearDetector`, `ConvolutionalDetector`
  - Migration: Update detector type references

### Added

#### Documentation
- **API stability markers**: Added explicit stability documentation to core crates
  - Stable: `Detector`, `Recorder`, `FrameSource`, model architectures
  - Experimental: warehouse format, training utilities

- **Comprehensive module-level docs** (C0h-6): All public modules now have detailed documentation
  - Purpose, design notes, and usage examples

- **Versioning strategy**: Added SemVer policy to README
  - Documented stability guarantees and coordinated release approach

- **Publishing preparation**: Complete crates.io metadata and docs.rs configuration
  - All 10 publishable crates have:
    - Complete Cargo.toml metadata (description, keywords, categories)
    - README.md with badges
    - docs.rs configuration for optimal documentation

#### Conventions Documentation (C0h-11 through C0h-17)
- Documented crate naming strategy and vision architecture split
- Documented function verb usage conventions (new, create, load, etc.)
- Documented feature flag organization patterns
- Documented module naming conventions
- Documented trait naming conventions
- Documented type alias feature-gating patterns

### Changed

- **Code quality sweep**: Applied rustfmt and resolved clippy warnings across workspace
- **Module organization**: Improved internal module structure for better discoverability

### Internal

- Consolidated duplicate `InferenceThresholds` definitions
- Improved CI checks for version consistency and path dependencies
- Enhanced documentation coverage across all crates

## [0.5.1] - 2026-01-12

- Added automated version synchronization script (`scripts/sync_versions.sh`)
- Added CI validation for version consistency
- Refactored `burn_dataset` into focused modules (capture, warehouse, batch, splits, aug, types, validation, parquet)
- Added comprehensive integration tests for E2E workflows
- All workspace crates synchronized to 0.5.1

## [0.5.0] - 2026-01-12

- Removed `gpu_amd_windows` alias and legacy warehouse command fallback/tests in tools
- Removed legacy crate-name deprecation notices from crate README/Cargo descriptions
- Docs updated to remove legacy alias guidance and note the breaking changes

---

For older releases, see the git history or the book's changelog at `docs/cortenforge_book/src/00_workspace/changelog.md`.
