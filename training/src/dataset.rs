use data_contracts::capture::CaptureMetadata;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use burn::tensor::{backend::Backend, Tensor};

#[derive(Debug, Clone, Deserialize)]
pub struct DatasetConfig {
    pub root: PathBuf,
    pub labels_subdir: String,
    pub images_subdir: String,
}

#[derive(Debug, Clone)]
pub struct RunSample {
    pub image: PathBuf,
    pub metadata: CaptureMetadata,
}

#[derive(Debug, Clone)]
pub struct CollatedBatch<B: Backend> {
    pub images: Tensor<B, 4>,
    pub targets: Tensor<B, 2>,
}

impl DatasetConfig {
    pub fn load(&self) -> anyhow::Result<Vec<RunSample>> {
        let mut samples = Vec::new();
        let labels_dir = self.root.join(&self.labels_subdir);
        for entry in fs::read_dir(&labels_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let meta: CaptureMetadata = serde_json::from_slice(&fs::read(&path)?)?;
            meta.validate()
                .map_err(|e| anyhow::anyhow!("invalid metadata {:?}: {e}", path))?;
            let img_path = self.root.join(&self.images_subdir).join(&meta.image);
            samples.push(RunSample {
                image: img_path,
                metadata: meta,
            });
        }
        Ok(samples)
    }
}

pub fn collate<B: Backend>(samples: &[RunSample]) -> anyhow::Result<CollatedBatch<B>> {
    // Placeholder: load images and targets; for now, just create dummy tensors sized to batch.
    let batch = samples.len();
    // Dummy shapes: [batch, channels, height, width] and [batch, 1] targets.
    let images = Tensor::zeros([batch, 3, 1, 1], &B::Device::default());
    let targets = Tensor::zeros([batch, 1], &B::Device::default());
    Ok(CollatedBatch { images, targets })
}
