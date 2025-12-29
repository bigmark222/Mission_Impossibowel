use burn::backend::Autodiff;
use burn::module::Module;
use burn::nn::loss::{MseLoss, Reduction};
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::record::{BinFileRecorder, FullPrecisionSettings, RecorderError};
use std::path::Path;

use crate::{DatasetConfig, TinyDet, TinyDetConfig, TrainBackend};
use clap::Parser;
use std::fs;

pub fn load_tinydet_from_checkpoint<P: AsRef<Path>>(
    path: P,
    device: &<TrainBackend as burn::tensor::backend::Backend>::Device,
) -> Result<TinyDet<TrainBackend>, RecorderError> {
    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    TinyDet::<TrainBackend>::new(TinyDetConfig::default(), device)
        .load_file(path.as_ref(), &recorder, device)
}

#[derive(Parser, Debug)]
#[command(name = "train", about = "Train TinyDet on capture metadata")]
pub struct TrainArgs {
    /// Dataset root containing labels/ and images/ (uses data_contracts schemas).
    #[arg(long, default_value = "assets/datasets/captures_filtered")]
    pub dataset_root: String,
    /// Labels subdirectory relative to dataset root.
    #[arg(long, default_value = "labels")]
    pub labels_subdir: String,
    /// Images subdirectory relative to dataset root.
    #[arg(long, default_value = ".")]
    pub images_subdir: String,
    /// Number of epochs.
    #[arg(long, default_value_t = 1)]
    pub epochs: usize,
    /// Batch size.
    #[arg(long, default_value_t = 1)]
    pub batch_size: usize,
    /// Learning rate.
    #[arg(long, default_value_t = 1e-3)]
    pub lr: f32,
    /// Objectness threshold (for future eval).
    #[arg(long, default_value_t = 0.3)]
    pub infer_obj_thresh: f32,
    /// IoU threshold (for future eval).
    #[arg(long, default_value_t = 0.5)]
    pub infer_iou_thresh: f32,
    /// Checkpoint output path.
    #[arg(long, default_value = "checkpoints/tinydet.bin")]
    pub checkpoint_out: String,
}

pub fn run_train(args: TrainArgs) -> anyhow::Result<()> {
    let cfg = DatasetConfig {
        root: args.dataset_root.into(),
        labels_subdir: args.labels_subdir,
        images_subdir: args.images_subdir,
    };
    let samples = cfg.load()?;
    if samples.is_empty() {
        println!("No samples found under {}", cfg.root.display());
        return Ok(());
    }

    type ADBackend = Autodiff<TrainBackend>;
    let device = <ADBackend as burn::tensor::backend::Backend>::Device::default();
    let mut model = TinyDet::<ADBackend>::new(TinyDetConfig::default(), &device);
    let mut optim = AdamConfig::new().init();

    let batch_size = args.batch_size.max(1);
    let data = samples.clone();
    for epoch in 0..args.epochs {
        let mut losses = Vec::new();
        for batch in data.chunks(batch_size) {
            let batch = crate::collate::<ADBackend>(batch)?;
            // Feature: take the first box (or zeros) as the input vector.
            let boxes = batch.boxes.clone();
            let first_box = boxes
                .clone()
                .slice([0..boxes.dims()[0], 0..1, 0..4])
                .reshape([boxes.dims()[0], 4]);

            // Target: 1.0 if any box present, else 0.0.
            let mask = batch.box_mask.clone();
            let has_box = mask
                .clone()
                .sum_dim(1)
                .reshape([mask.dims()[0], 1]);

            let preds = model.forward(first_box);
            let mse = MseLoss::new();
            let loss = mse.forward(preds, has_box, Reduction::Mean);
            let loss_detached = loss.clone().detach();
            let grads = GradientsParams::from_grads(loss.backward(), &model);
            model = optim.step(args.lr as f64, model, grads);

            let loss_val: f32 = loss_detached
                .into_data()
                .to_vec::<f32>()
                .unwrap_or_default()
                .into_iter()
                .next()
                .unwrap_or(0.0);
            losses.push(loss_val);
        }
        let avg_loss: f32 = if losses.is_empty() {
            0.0
        } else {
            losses.iter().sum::<f32>() / losses.len() as f32
        };
        println!("epoch {epoch}: avg loss {avg_loss:.4}");
    }

    if let Some(parent) = Path::new(&args.checkpoint_out).parent() {
        fs::create_dir_all(parent)?;
    }
    let recorder = BinFileRecorder::<FullPrecisionSettings>::new();
    model
        .clone()
        .save_file(Path::new(&args.checkpoint_out), &recorder)
        .map_err(|e| anyhow::anyhow!("failed to save checkpoint: {e}"))?;
    println!("Saved checkpoint to {}", args.checkpoint_out);
    Ok(())
}
