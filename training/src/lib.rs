pub mod model;
pub mod dataset;
pub mod util;

pub use model::{TinyDetConfig, TinyDet};
pub use dataset::{DatasetConfig, RunSample, collate};
pub use util::{TrainArgs, run_train};
/// Backend alias for training/eval (NdArray by default; WGPU if enabled).
#[cfg(feature = "backend-wgpu")]
pub type TrainBackend = burn_wgpu::Wgpu<f32>;
#[cfg(not(feature = "backend-wgpu"))]
pub type TrainBackend = burn_ndarray::NdArray<f32>;
