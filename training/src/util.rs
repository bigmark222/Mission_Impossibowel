use burn::record::{BinBytesRecorder, RecorderError};
use std::path::Path;

use crate::{DatasetConfig, TinyDet, TinyDetConfig, TrainBackend};
use clap::Parser;
use std::fs;

pub fn load_tinydet_from_checkpoint<P: AsRef<Path>>(
    _path: P,
    _device: &<TrainBackend as burn::tensor::backend::Backend>::Device,
) -> Result<TinyDet<TrainBackend>, RecorderError> {
    let _recorder: BinBytesRecorder<burn::record::HalfPrecisionSettings> = BinBytesRecorder::new();
    // Placeholder: real checkpoint loading to be implemented.
    Err(RecorderError::Unknown("checkpoint loading not implemented yet".into()))
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

    let device = <TrainBackend as burn::tensor::backend::Backend>::Device::default();
    let _model = TinyDet::<TrainBackend>::new(TinyDetConfig::default(), &device);

    let batch_size = args.batch_size.max(1);
    let data = samples.clone();
    for epoch in 0..args.epochs {
        // Placeholder loop; shuffle omitted for now.
        let mut losses = Vec::new();
        for batch in data.chunks(batch_size) {
            let _batch = crate::collate::<TrainBackend>(batch)?;
            losses.push(0.0f32);
        }
        let avg_loss: f32 = if losses.is_empty() {
            0.0
        } else {
            losses.iter().sum::<f32>() / losses.len() as f32
        };
        println!("epoch {epoch}: avg loss {avg_loss:.4} (stub)");
    }

    if let Some(parent) = Path::new(&args.checkpoint_out).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&args.checkpoint_out, b"stub checkpoint")?;
    println!("Saved checkpoint to {}", args.checkpoint_out);
    Ok(())
}
