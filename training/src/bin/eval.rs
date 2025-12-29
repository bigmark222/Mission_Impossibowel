use clap::Parser;
use training::dataset::{DatasetConfig, collate};
use training::{TinyDet, TinyDetConfig, TrainBackend};
use training::util::load_tinydet_from_checkpoint;
use burn::nn::loss::{MseLoss, Reduction};

#[derive(Parser, Debug)]
#[command(name = "eval", about = "Evaluate TinyDet checkpoint on a dataset")]
struct Args {
    /// Dataset root containing labels/ and images/ (uses data_contracts schemas).
    #[arg(long, default_value = "assets/datasets/captures_filtered")]
    dataset_root: String,
    /// Labels subdirectory relative to dataset root.
    #[arg(long, default_value = "labels")]
    labels_subdir: String,
    /// Images subdirectory relative to dataset root.
    #[arg(long, default_value = ".")]
    images_subdir: String,
    /// Checkpoint path to load (stub; real loading TBD).
    #[arg(long, default_value = "checkpoints/tinydet.bin")]
    checkpoint: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
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
    let model = match load_tinydet_from_checkpoint(&args.checkpoint, &device) {
        Ok(m) => {
            println!("Loaded checkpoint from {}", args.checkpoint);
            m
        }
        Err(e) => {
            println!(
                "Failed to load checkpoint {}; using fresh model ({e})",
                args.checkpoint
            );
            TinyDet::<TrainBackend>::new(TinyDetConfig::default(), &device)
        }
    };

    let batch_size = 8usize;
    let mut total_loss = 0.0f32;
    let mut count = 0usize;
    let mse = MseLoss::new();
    for chunk in samples.chunks(batch_size) {
        let batch = collate::<TrainBackend>(chunk)?;
        let boxes = batch.boxes.clone();
        let first_box = boxes
            .clone()
            .slice([0..boxes.dims()[0], 0..1, 0..4])
            .reshape([boxes.dims()[0], 4]);

        let mask = batch.box_mask.clone();
        let has_box = mask
            .clone()
            .sum_dim(1)
            .reshape([mask.dims()[0], 1]);

        let preds = model.forward(first_box);
        let loss = mse.forward(preds, has_box, Reduction::Mean);
        let loss_val: f32 = loss
            .into_data()
            .to_vec::<f32>()
            .unwrap_or_default()
            .into_iter()
            .next()
            .unwrap_or(0.0);
        total_loss += loss_val;
        count += 1;
    }

    let avg_loss = if count == 0 {
        0.0
    } else {
        total_loss / count as f32
    };
    println!(
        "Eval complete over {} batches ({} samples): avg MSE vs box presence = {:.4}",
        count,
        samples.len(),
        avg_loss
    );
    Ok(())
}
