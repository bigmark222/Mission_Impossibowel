//! Prints a Bash one-liner for running training with a warehouse manifest.
//! Edit `CONFIG` below (or swap to `DEFAULT_CONFIG`) and run:
//! `cargo run --bin warehouse_sh_command`.

mod lib;

use lib::{build_bash_command, CmdConfig, ModelKind, WarehouseStore, DEFAULT_CONFIG};

const CONFIG: CmdConfig = CmdConfig {
    manifest: "artifacts/tensor_warehouse/v<version>/manifest.json",
    store: WarehouseStore::Stream,
    prefetch: Some(8),
    model: ModelKind::Big,
    batch_size: 32,
    log_every: 1,
    extra_args: "",
};

fn main() {
    // Use CONFIG for ad-hoc tweaks; switch to DEFAULT_CONFIG for shared defaults.
    let cmd = build_bash_command(&CONFIG);
    println!("{cmd}");
}
