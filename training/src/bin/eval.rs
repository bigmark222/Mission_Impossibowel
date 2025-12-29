use clap::Parser;
use training::dataset::DatasetConfig;
use training::{TinyDet, TinyDetConfig};

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

    let device = <training::TrainBackend as burn::tensor::backend::Backend>::Device::default();
    let _model = TinyDet::<training::TrainBackend>::new(TinyDetConfig::default(), &device);

    println!(
        "Stub eval: would load checkpoint {} and run over {} samples",
        args.checkpoint,
        samples.len()
    );
    Ok(())
}
