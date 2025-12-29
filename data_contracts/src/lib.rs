//! Shared data contracts for runs, manifests, and capture metadata.

pub mod manifest;
pub mod capture;

pub use manifest::{RunManifest, RunManifestSchemaVersion};
pub use capture::{PolypLabel, CaptureMetadata, ValidationError};
