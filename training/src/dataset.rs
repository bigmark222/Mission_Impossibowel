use data_contracts::capture::CaptureMetadata;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use burn::tensor::{backend::Backend, Tensor};
use burn::tensor::TensorData;

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
    /// Normalized boxes per sample (shape: [batch, max_boxes, 4]).
    pub boxes: Tensor<B, 3>,
    /// Mask indicating which box slots are populated (shape: [batch, max_boxes]).
    pub box_mask: Tensor<B, 2>,
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
    if samples.is_empty() {
        anyhow::bail!("cannot collate empty batch");
    }

    // Load first image to establish dimensions.
    let first = image::open(&samples[0].image)
        .map_err(|e| anyhow::anyhow!("failed to open image {:?}: {e}", samples[0].image))?
        .to_rgb8();
    let (width, height) = first.dimensions();

    let batch = samples.len();
    let num_pixels = (width * height) as usize;
    let mut image_buf: Vec<f32> = Vec::with_capacity(batch * num_pixels * 3);

    // Gather normalized boxes; track the maximum per-sample count.
    let mut all_boxes: Vec<Vec<[f32; 4]>> = Vec::with_capacity(batch);
    let mut max_boxes = 0usize;

    for (idx, sample) in samples.iter().enumerate() {
        let img = if idx == 0 {
            first.clone()
        } else {
            let img = image::open(&sample.image).map_err(|e| {
                anyhow::anyhow!("failed to open image {:?}: {e}", sample.image)
            })?;
            let rgb = img.to_rgb8();
            let (w, h) = rgb.dimensions();
            if w != width || h != height {
                anyhow::bail!(
                    "image dimensions differ within batch: {:?} is {}x{}, expected {}x{}",
                    sample.image,
                    w,
                    h,
                    width,
                    height
                );
            }
            rgb
        };

        // Push normalized pixel data in CHW order.
        for c in 0..3 {
            for y in 0..height {
                for x in 0..width {
                    let p = img.get_pixel(x, y);
                    image_buf.push(p[c] as f32 / 255.0);
                }
            }
        }

        let mut boxes = Vec::new();
        for label in &sample.metadata.polyp_labels {
            let bbox = if let Some(norm) = label.bbox_norm {
                norm
            } else if let Some(px) = label.bbox_px {
                [
                    px[0] / width as f32,
                    px[1] / height as f32,
                    px[2] / width as f32,
                    px[3] / height as f32,
                ]
            } else {
                continue;
            };
            boxes.push(bbox);
        }
        max_boxes = max_boxes.max(boxes.len());
        all_boxes.push(boxes);
    }

    if max_boxes == 0 {
        // Ensure downstream tensors have at least one slot.
        max_boxes = 1;
    }

    let mut boxes_buf = vec![0.0f32; batch * max_boxes * 4];
    let mut mask_buf = vec![0.0f32; batch * max_boxes];
    for (b, boxes) in all_boxes.iter().enumerate() {
        for (i, bbox) in boxes.iter().take(max_boxes).enumerate() {
            let base = (b * max_boxes + i) * 4;
            boxes_buf[base..base + 4].copy_from_slice(bbox);
            mask_buf[b * max_boxes + i] = 1.0;
        }
    }

    let device = &B::Device::default();
    let images = Tensor::<B, 4>::from_data(
        TensorData::new(
            image_buf,
            [batch, 3, height as usize, width as usize],
        ),
        device,
    );
    let boxes = Tensor::<B, 3>::from_data(
        TensorData::new(boxes_buf, [batch, max_boxes, 4]),
        device,
    );
    let box_mask = Tensor::<B, 2>::from_data(
        TensorData::new(mask_buf, [batch, max_boxes]),
        device,
    );

    Ok(CollatedBatch {
        images,
        boxes,
        box_mask,
    })
}
