# data_contracts

Shared schemas for runs, manifests, and capture metadata.

Contents
- `manifest`: `RunManifest` + schema version.
- `capture`: `CaptureMetadata`, `PolypLabel`, validation helpers.

Usage
- Add `data_contracts` as a dependency and import the structs you need.
- Call `validate()` on manifests/metadata before writing or ingesting.

Tests
- See `tests/capture_validation.rs` for bbox validation examples.
