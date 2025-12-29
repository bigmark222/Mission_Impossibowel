use training::util::{TrainArgs, run_train};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = TrainArgs::parse();
    run_train(args)
}
