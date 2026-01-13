//! Shared data contracts for runs, manifests, and capture metadata.
//!
//! This crate defines serializable schemas for:
//! - `CaptureMetadata`: Per-frame capture metadata (labels, timestamps, provenance).
//! - `RunManifest`: Run-level configuration and metadata.
//! - `ImageStats`: Preprocessing statistics for normalization.
//!
//! These types are used across the CortenForge stack for dataset persistence, validation,
//! and reproducibility. All types implement `Serialize`/`Deserialize` for JSON storage.

pub mod capture;
pub mod manifest;
pub mod preprocess;

pub use capture::{CaptureMetadata, DetectionLabel, LabelSource, ValidationError};
pub use manifest::{RunManifest, RunManifestSchemaVersion};
pub use preprocess::{ImageStats, ImageStatsError};
