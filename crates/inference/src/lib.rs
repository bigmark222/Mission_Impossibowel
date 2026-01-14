//! Detector factory and runtime configuration for CortenForge inference.
//!
//! This crate bridges the `models` crate (Burn Modules) and the `vision_core::interfaces::Detector`
//! trait. The `InferenceFactory` loads weights and constructs detector implementations that can be
//! used in vision pipelines.
//!
//! ## Backend Selection
//! - `backend-wgpu`: Uses WGPU for GPU acceleration (recommended for production).
//! - Default: Falls back to NdArray CPU backend.
//!
//! ## Model Selection
//! - `convolutional_detector`: Uses `MultiboxModel` for multi-box detection.
//! - Default: Uses `LinearClassifier` for binary classification.
//!
//! Type aliases `InferenceModel` and `InferenceModelConfig` adapt to the selected features.

#![recursion_limit = "256"]

pub mod factory;

#[cfg(feature = "backend-wgpu")]
pub type InferenceBackend = burn_wgpu::Wgpu<f32>;
#[cfg(not(feature = "backend-wgpu"))]
pub type InferenceBackend = burn_ndarray::NdArray<f32>;

#[cfg(feature = "convolutional_detector")]
pub type InferenceModel<B> = models::MultiboxModel<B>;
#[cfg(feature = "convolutional_detector")]
pub type InferenceModelConfig = models::MultiboxModelConfig;
#[cfg(not(feature = "convolutional_detector"))]
pub type InferenceModel<B> = models::LinearClassifier<B>;
#[cfg(not(feature = "convolutional_detector"))]
pub type InferenceModelConfig = models::LinearClassifierConfig;

pub use factory::{InferenceFactory, InferenceThresholds};

pub mod prelude {
    pub use crate::factory::{InferenceFactory, InferenceThresholds};
    pub use crate::{InferenceBackend, InferenceModel, InferenceModelConfig};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inference_factory_falls_back_without_weights() {
        let factory = InferenceFactory;
        let mut detector = factory.build(InferenceThresholds::default(), None);
        // Should not panic and should produce a detector.
        assert!(
            detector
                .detect(&vision_core::interfaces::Frame {
                    id: 0,
                    timestamp: 0.0,
                    rgba: None,
                    size: (1, 1),
                    path: None,
                })
                .frame_id
                == 0
        );
    }
}
