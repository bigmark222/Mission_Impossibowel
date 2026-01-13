//! Framework-agnostic vision interfaces for CortenForge detection pipelines.
//!
//! This crate defines the core abstractions for detection, capture, and recording:
//! - `Detector`: Trait for running inference on frames.
//! - `Recorder`: Trait for persisting frame records and labels.
//! - `Frame`, `FrameRecord`, `Label`: Core data types for vision pipelines.
//! - Capture resources and overlay utilities.
//!
//! ## Design Philosophy
//! `vision_core` is intentionally framework-agnostic. It has no Bevy dependencies and can be
//! used in CLI tools, web services, or any Rust application. The `vision_runtime` crate
//! provides Bevy-specific integration on top of these interfaces.

pub mod capture;
pub mod interfaces;
pub mod overlay;

pub mod prelude {
    pub use crate::capture::{
        CaptureLimit, PrimaryCaptureCamera, PrimaryCaptureReadback, PrimaryCaptureTarget,
    };
    pub use crate::interfaces::*;
    pub use crate::overlay::*;
}
